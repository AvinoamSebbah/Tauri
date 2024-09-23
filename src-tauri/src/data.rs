#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]
  
  use rusqlite::{params, Connection, Result};
  use std::error::Error;
  use whoami; // Pour récupérer le username
  use std::ffi::OsString;
  use std::os::windows::ffi::OsStringExt;
  use std::slice::from_raw_parts;
  
  use grob::{RvIsError, winapi_small_binary};
  use windows::core::{PCWSTR, PWSTR};
  use windows::Win32::Foundation::{FALSE, HANDLE, HLOCAL, PSID};
  use windows::Win32::Globalization::lstrlenW;
  use windows::Win32::Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER};
  use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
  use windows::Win32::System::Memory::LocalFree;
  use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
  use windows::Win32::Foundation::BOOL;
 
#[derive(serde::Serialize)]
  struct User {
      username: String,
      sid: String,
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

  fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![
        get_current_user,
        get_users_list
    ])
      .setup(|_app| {
        initialize_db();
        let sid = get_user_sid().expect("Failed to get SID");
        let username = whoami::username();
        
        // Vérification ou ajout de l'utilisateur
        match check_or_insert_user(&sid, &username) {
          Ok(Some(name)) => println!("User {} already exists", name),
          Ok(None) => println!("New user {} registered", username),
          Err(e) => eprintln!("Error: {}", e),
        }
  
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
  
  /// Étape 2 : Récupérer le SID
  fn get_user_sid() -> Result<String, Box<dyn Error>> {
    let h = unsafe { GetCurrentProcess() };
    let mut h_token: HANDLE = Default::default();
    if unsafe { OpenProcessToken(h, TOKEN_QUERY, &mut h_token) } == BOOL(0) {
      return Err(Box::new(std::io::Error::last_os_error()));
    }
    let sid = get_user_sid_from_token(h_token)?;
    Ok(sid)
  }
  
  // Utilisez ici le code de conversion du SID que vous avez déjà fourni
  fn convert_sid_to_string(value: PSID) -> Result<String, std::io::Error> {
    let mut lhs = LocalHeapString::default();
    if unsafe { ConvertSidToStringSidW(value, lhs.as_mut_ptr()) } == FALSE {
        return Err(std::io::Error::last_os_error());
    }
    Ok(lhs.into())
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
  /// Étape 3 : Initialisation de la base de données et création des tables
  fn initialize_db() -> Connection {
    let base_path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let db_path = base_path.join("Tobleron.db");
    let conn = Connection::open(db_path).expect("Failed to open DB");
    conn.execute(
      "CREATE TABLE IF NOT EXISTS User (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        sid TEXT NOT NULL UNIQUE
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
  
    conn
  }
  
  /// Étape 4 : Vérification ou ajout d'un utilisateur
  fn check_or_insert_user(sid: &str, username: &str) -> Result<Option<String>, rusqlite::Error> {
    let conn = initialize_db();
    
    // Vérification si l'utilisateur existe
    let mut stmt = conn.prepare("SELECT username FROM User WHERE sid = ?1")?;
    let mut rows = stmt.query(params![sid])?;
  
    if let Some(row) = rows.next()? {
      let existing_username: String = row.get(0)?;
      return Ok(Some(existing_username));
    }
  
    // Insérer le nouvel utilisateur
    conn.execute(
      "INSERT INTO User (username, sid) VALUES (?1, ?2)",
      params![username, sid],
    )?;
  
    Ok(None)
  }
  
  #[tauri::command]
  fn get_current_user() -> Result<User, String> {
    let base_path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let db_path = base_path.join("Tobleron.db");
    let conn = Connection::open(db_path).expect("Failed to open DB");
    
    // Requête pour obtenir l'utilisateur actuellement connecté
    let mut stmt = conn
        .prepare("SELECT username, sid FROM User WHERE sid = ?1")
        .map_err(|e| e.to_string())?;

    // Récupérer le SID et username actuel (vous devrez passer le SID ici, ou le calculer)
    let sid = get_user_sid().map_err(|e| e.to_string())?;

    // Exécution de la requête pour récupérer l'utilisateur avec le SID correspondant
    let user = stmt
        .query_row([&sid], |row| {
            Ok(User {
                username: row.get(0)?,
                sid: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(user)
}

#[tauri::command]
fn get_users_list() -> Result<Vec<User>, String> {
    let base_path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let db_path = base_path.join("Tobleron.db");
    let conn = Connection::open(db_path).expect("Failed to open DB");

    // Requête pour obtenir tous les utilisateurs
    let mut stmt = conn
        .prepare("SELECT username, sid FROM User")
        .map_err(|e| e.to_string())?;

    let users_iter = stmt
        .query_map([], |row| {
            Ok(User {
                username: row.get(0)?,
                sid: row.get(1)?,
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