#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]
  
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice::from_raw_parts;
use uuid::Uuid;
use grob::{RvIsError, winapi_small_binary};
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{FALSE, HANDLE, HLOCAL, PSID};
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER};
use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
use windows::Win32::System::Memory::LocalFree;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::ptr::null_mut;
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::Shell::IsUserAnAdmin;
use winapi::um::wincrypt::DATA_BLOB;
use winapi::um::dpapi::{CryptUnprotectData,CryptProtectData};
use std::fs;

const JSON_PATH: &str = "C:\\tobleron\\auth.json";
const DB_PATH: &str = "C:\\tobleron\\Tobleron.db";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfo {
    id: String,
    username: String,
    sid: String,
    role: String,
    valid: bool,
}

#[derive(serde::Serialize)]
  struct User {
      username: String,
      id: String,
      first_name: String,
    last_name: String,
 }
#[derive(Serialize)]
struct UserWaitingList {
    username: String,
    id: String
}

 struct LocalHeapString {
    inner: PWSTR,
}

impl LocalHeapString {  
    fn as_mut_ptr(&mut self) -> &mut PWSTR {
        &mut self.inner
    }
}

impl Default for LocalHeapString {
    fn default() -> Self {
        Self {
            inner: PWSTR::null(),
        }
    }
}

impl Drop for LocalHeapString {
    fn drop(&mut self) {
        if self.inner != PWSTR::null() {
            let free_me: HLOCAL = HLOCAL(self.inner.0 as isize);
            self.inner = PWSTR::null();
            let _ = unsafe { LocalFree(free_me) };
        }
    }
}

impl From<LocalHeapString> for String {
    fn from(value: LocalHeapString) -> Self {
        let as_constant_wide_string: PCWSTR = PCWSTR(value.inner.0);
        let s = unsafe { lstrlenW(as_constant_wide_string) };
        let v = unsafe { from_raw_parts(as_constant_wide_string.0, s as usize) };
        let as_os_string = OsString::from_wide(v);
        let as_rust_string = as_os_string.to_string_lossy();
        as_rust_string.into_owned()
    }
}

fn get_current_username() -> String {
    let output = std::process::Command::new("whoami")
        .output()
        .expect("Erreur lors de l'exécution de la commande whoami");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn is_user_admin() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

fn protect_data(data: &[u8]) -> Vec<u8> {
    let mut data_blob = DATA_BLOB {
        pbData: data.as_ptr() as _,
        cbData: data.len() as _,
    };
    let mut encrypted_blob = DATA_BLOB {
        pbData: null_mut(),
        cbData: 0,
    };

    let result = unsafe {
        CryptProtectData(
            &mut data_blob,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut encrypted_blob,
        )
    };

    if result != 0 {
        let encrypted_data = unsafe {
            std::slice::from_raw_parts(encrypted_blob.pbData as *const u8, encrypted_blob.cbData as usize)
        };
        encrypted_data.to_vec()
    } else {
        panic!("Erreur lors du chiffrement des données");
    }
}

fn unprotect_data(encrypted_data: &[u8]) -> Vec<u8> {
    let mut encrypted_blob = DATA_BLOB {
        pbData: encrypted_data.as_ptr() as _,
        cbData: encrypted_data.len() as _,
    };
    let mut decrypted_blob = DATA_BLOB {
        pbData: null_mut(),
        cbData: 0,
    };

    let result = unsafe {
        CryptUnprotectData(
            &mut encrypted_blob,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut decrypted_blob,
        )
    };

    if result != 0 {
        let decrypted_data = unsafe {
            std::slice::from_raw_parts(decrypted_blob.pbData as *const u8, decrypted_blob.cbData as usize)
        };
        decrypted_data.to_vec()
    } else {
        panic!("Erreur lors du déchiffrement des données");
    }
}

fn read_json(json_path: &str) -> Vec<UserInfo> {
    if !std::path::Path::new(json_path).exists() {
        return Vec::new(); // Retourne une liste vide si le fichier n'existe pas
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(json_path)
        .expect("Erreur lors de l'ouverture du fichier JSON");

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)
        .expect("Erreur lors de la lecture des données");

    let decrypted_data = unprotect_data(&encrypted_data);

    // Essaye de désérialiser comme un tableau
    let users: Result<Vec<UserInfo>, _> = serde_json::from_slice(&decrypted_data);
    
    match users {
        Ok(users) => users,
        Err(_) => {
            // Si la désérialisation échoue, essaie de traiter comme un objet
            let single_user: UserInfo = serde_json::from_slice(&decrypted_data)
                .expect("Erreur lors de la désérialisation du JSON");
            vec![single_user] // Retourne un vecteur contenant l'utilisateur
        }
    }
}

fn create_or_update_json(json_path: &str, user_info: &UserInfo) -> std::io::Result<()> {
    let json_data = serde_json::to_string(user_info).expect("Erreur lors de la sérialisation en JSON");
    let encrypted_data = protect_data(json_data.as_bytes());

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(json_path)?;

    file.write_all(&encrypted_data)?;

    Ok(())
}

fn initialize_db() -> Connection {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute(
      "CREATE TABLE IF NOT EXISTS User (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL
      )",
      [],
    ).expect("Failed to create User table");
  
    conn.execute(
      "CREATE TABLE IF NOT EXISTS Machine (
        id INTEGER PRIMARY KEY,
        machine_name TEXT NOT NULL,
        user_id INTEGER,
        FOREIGN KEY(user_id) REFERENCES User(id)
      )",
      [],
    ).expect("Failed to create Machine table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Measure (
          id INTEGER PRIMARY KEY,
          user_id INTEGER,
          machine_id INTEGER,
          measure_date TEXT NOT NULL,
          temperature REAL NOT NULL,
          FOREIGN KEY(user_id) REFERENCES User(id),
          FOREIGN KEY(machine_id) REFERENCES Machine(id)
        )",
        [],
      ).expect("Failed to create Measure table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS WaitingList (
          id INTEGER PRIMARY KEY,
          user_id TEXT NOT NULL,
          username TEXT NOT NULL
        )",
        [],
      ).expect("Failed to create WaitingList table");
  
    conn
  }
 
fn get_user_sid() -> Result<String, Box<dyn Error>> {
    let h = unsafe { GetCurrentProcess() };
    let mut h_token: HANDLE = Default::default();
    if unsafe { OpenProcessToken(h, TOKEN_QUERY, &mut h_token) } == BOOL(0) {
      return Err(Box::new(std::io::Error::last_os_error()));
    }
    let sid = get_user_sid_from_token(h_token)?;
    Ok(sid)
  }

fn get_user_sid_from_token(token: HANDLE) -> Result<String, std::io::Error> {
    winapi_small_binary(
        |argument| {
            let rv = unsafe {
                GetTokenInformation(
                    token,
                    TokenUser,
                    Some(argument.pointer()),
                    *argument.size(),
                argument.size(),
                )
            };
            RvIsError::new(rv)
        },
        |frozen_buffer| {
            if let Some(fbp) = frozen_buffer.pointer() {
                let tup = fbp as *const TOKEN_USER;
                convert_sid_to_string( unsafe {*tup}.User.Sid)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "something went terribly wrong"))
            }
        }
    )
}

fn convert_sid_to_string(value: PSID) -> Result<String, std::io::Error> {
    let mut lhs = LocalHeapString::default();
    if unsafe { ConvertSidToStringSidW(value, lhs.as_mut_ptr()) } == FALSE {
        return Err(std::io::Error::last_os_error());
    }
    Ok(lhs.into())
}

#[tauri::command]
fn get_current_user() -> Result<User, String> {
    let users = read_json(JSON_PATH);
    let sid = get_user_sid().expect("Failed to get SID");
    let actual_user = users.iter().find(|u| u.sid == sid);
    if actual_user.is_none() {
        return Err("User not found".to_string().to_string());
    }

  let conn = Connection::open(DB_PATH).expect("Failed to open DB");
  
    let user = conn.query_row(
        "SELECT username, first_name, last_name, id FROM User WHERE id = ?1",
        params![actual_user.unwrap().id],
        |row| {
            Ok(User {
                username: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                id: row.get(3)?,
            })
        },
    ).expect("Erreur lors de la récupération de l'utilisateur");
  Ok(user)
}

#[tauri::command]
fn register_to_waiting_list() -> Result<bool, String> {
    let users = read_json(JSON_PATH);
    let sid = get_user_sid().expect("Failed to get SID");
    let actual_user = users.iter().find(|u| u.sid == sid);

    if let Some(user) = actual_user {
        let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO WaitingList (user_id, username) VALUES (?1, ?2)",
            params![user.id, user.username],
        ).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Err("User not found".to_string())
    }
}


#[tauri::command]
fn get_users_waiting_list() -> Result<Vec<UserWaitingList>, String> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");

    // Requête pour obtenir tous les utilisateurs
    let mut stmt = conn
        .prepare("SELECT username, user_id FROM WaitingList")
        .map_err(|e| e.to_string())?;

    let users_iter = stmt
        .query_map([], |row| {
            Ok(UserWaitingList {
                username: row.get(0)?,
                id: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    // Construction de la liste des utilisateurs
    let mut users = Vec::new();
    for user_result in users_iter {
        match user_result {
            Ok(user) => users.push(user),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(users)
}

fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![
        get_current_user,
        register_to_waiting_list,
        get_users_waiting_list
    ])
      .setup(|_app| {
        initialize_db();
        let sid = get_user_sid().expect("Failed to get SID");

        if let Some(parent) = std::path::Path::new(JSON_PATH).parent() {
            fs::create_dir_all(parent).expect("Erreur lors de la création du dossier");
        }
    
        let username = get_current_username();
        let is_admin = is_user_admin();
        let role = if is_admin { "admin" } else { "user" };
        let valid = is_admin;
        let id = Uuid::new_v4().to_string();
    
        let user_info = UserInfo {
            id,
            username,
            sid,
            role: role.to_string(),
            valid,
        };
    
        let mut users = read_json(JSON_PATH);
        let existing_user = users.iter().find(|u| u.sid == user_info.sid);
    
        if let Some(existing) = existing_user {
            println!("L'utilisateur existe déjà : {:?}", existing);
        } else {
            users.push(user_info.clone());
            create_or_update_json(JSON_PATH, &user_info).expect("Erreur lors de la création/mise à jour du fichier JSON");
            println!("Nouveau user ajouté : {:?}", user_info);
            let conn = Connection::open(DB_PATH).expect("Failed to open DB");
            conn.execute(
                "INSERT INTO User (username, id, first_name, last_name) VALUES (?1, ?2, ?3, ?4)",
                params![user_info.username, user_info.id, "", ""],
            ).expect("Erreur lors de l'insertion dans la base de données");
        }
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
