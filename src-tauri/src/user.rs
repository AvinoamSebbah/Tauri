use grob::{winapi_small_binary, RvIsError};
use rusqlite::Result;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::ptr::null_mut;
use uuid::Uuid;
use std::fs;
use std::path::Path;
use winapi::um::dpapi::{CryptProtectData, CryptUnprotectData};
use winapi::um::wincrypt::DATA_BLOB;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::{FALSE, HANDLE, PSID};
use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
use windows::Win32::Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::IsUserAnAdmin;

use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::types::*;
use crate::database::*;

pub const JSON_PATH: &str = "C:\\tobleron\\auth.json";

lazy_static! {
    pub static ref CURRENT_USER: Mutex<Option<User>> = Mutex::new(None);
}

pub fn get_current_username() -> String {
    let output = std::process::Command::new("whoami")
        .output()
        .expect("Erreur lors de l'exécution de la commande whoami");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn is_user_admin() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

pub fn protect_data(data: &[u8]) -> Vec<u8> {
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
            std::slice::from_raw_parts(
                encrypted_blob.pbData as *const u8,
                encrypted_blob.cbData as usize,
            )
        };
        encrypted_data.to_vec()
    } else {
        panic!("Erreur lors du chiffrement des données");
    }
}

pub fn unprotect_data(encrypted_data: &[u8]) -> Vec<u8> {
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
            std::slice::from_raw_parts(
                decrypted_blob.pbData as *const u8,
                decrypted_blob.cbData as usize,
            )
        };
        decrypted_data.to_vec()
    } else {
        panic!("Erreur lors du déchiffrement des données");
    }
}

pub fn read_json(json_path: &str) -> Vec<UserInfo> {
    if !std::path::Path::new(json_path).exists() {
        return Vec::new();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(json_path)
        .expect("Erreur lors de l'ouverture du fichier JSON");

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)
        .expect("Erreur lors de la lecture des données");

    let decrypted_data = unprotect_data(&encrypted_data);

    let users: Result<Vec<UserInfo>, _> = serde_json::from_slice(&decrypted_data);

    match users {
        Ok(users) => users,
        Err(_) => {
            let single_user: UserInfo = serde_json::from_slice(&decrypted_data)
                .expect("Erreur lors de la désérialisation du JSON");
            vec![single_user]
        }
    }
}

pub fn create_or_update_json(json_path: &str, user_info: &UserInfo) -> std::io::Result<()> {
    let mut users: Vec<UserInfo> = read_json(json_path);
    if let Some(existing_user) = users.iter_mut().find(|u| u.sid == user_info.sid) {
        *existing_user = user_info.clone();
    } else {
        users.push(user_info.clone());
    }
    let json_data = serde_json::to_string(&users).expect("Erreur lors de la sérialisation en JSON");
    let encrypted_data = protect_data(json_data.as_bytes());
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Effacer l'ancien contenu
        .open(json_path)?;

    file.write_all(&encrypted_data)?;

    Ok(())
}

pub fn get_user_sid() -> Result<String, Box<dyn Error>> {
    let h = unsafe { GetCurrentProcess() };
    let mut h_token: HANDLE = Default::default();
    if unsafe { OpenProcessToken(h, TOKEN_QUERY, &mut h_token) } == BOOL(0) {
        return Err(Box::new(std::io::Error::last_os_error()));
    }
    let sid = get_user_sid_from_token(h_token)?;
    Ok(sid)
}

pub fn get_user_sid_from_token(token: HANDLE) -> Result<String, std::io::Error> {
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
                convert_sid_to_string(unsafe { *tup }.User.Sid)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "something went terribly wrong",
                ))
            }
        },
    )
}

pub fn convert_sid_to_string(value: PSID) -> Result<String, std::io::Error> {
    let mut lhs = LocalHeapString::default();
    if unsafe { ConvertSidToStringSidW(value, lhs.as_mut_ptr()) } == FALSE {
        return Err(std::io::Error::last_os_error());
    }
    Ok(lhs.into())
}

pub fn sync_json_with_db() -> Result<(), Box<dyn std::error::Error>> {
    let users: Vec<UserInfo> = read_json(JSON_PATH);
    for user in users.iter() {
        if user.valid {
            if !user_exists(&user.id)? {
                insert_user(user)?;
            }
        }
    }
    Ok(())
}

pub fn set_current_user(user: UserInfo) -> Result<User, String> {
    let mut current_user = CURRENT_USER.lock().unwrap();
    let user_data = get_user_by_id(user.id)?;

    *current_user = Some(user_data.clone());

    Ok(user_data)
}

pub fn initialize_user() {
    let sid = get_user_sid().expect("Failed to get SID");

    if let Some(parent) = Path::new(JSON_PATH).parent() {
        fs::create_dir_all(parent).expect("Error when creating directory");
    }
    let mut users = read_json(JSON_PATH);
    let existing_user = users.iter().find(|u| u.sid == sid);
    let user: UserInfo;

    if let Some(existing) = existing_user {
        println!("User already exists: {:?}", existing);
        user = existing.clone();
    } else {
        let username = get_current_username();
        let is_admin = is_user_admin();
        let role = if is_admin { "admin" } else { "user" };
        let valid = is_admin;
        let id = Uuid::new_v4().to_string();

        user = UserInfo {
            id,
            username,
            sid: sid.clone(),
            role: role.to_string(),
            valid,
        };
        users.push(user.clone());
        create_or_update_json(JSON_PATH, &user).expect("Error when creating or updating JSON file");
        println!("New user added: {:?}", user);
    }
    sync_json_with_db().expect("Failed to sync JSON with DB");
    set_current_user(user).expect("Failed to set current user");
}