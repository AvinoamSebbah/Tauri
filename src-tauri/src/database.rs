use rusqlite::{params,Connection};
use crate::types::*;
use crate::user::*;

pub const DB_PATH: &str = "C:\\tobleron\\Tobleron.db";

pub fn initialize_db() -> Connection {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS User (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL
      )",
        [],
    )
    .expect("Failed to create User table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Machine (
        id INTEGER PRIMARY KEY,
        machine_name TEXT NOT NULL,
        user_id INTEGER,
        FOREIGN KEY(user_id) REFERENCES User(id)
      )",
        [],
    )
    .expect("Failed to create Machine table");

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
    )
    .expect("Failed to create Measure table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS WaitingList (
          id INTEGER PRIMARY KEY,
          user_id TEXT NOT NULL,
          username TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create WaitingList table");

    conn
}

pub fn get_user_by_id(user_id: String) -> Result<User, String> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.query_row(
        "SELECT username, first_name, last_name, id FROM User WHERE id = ?1",
        params![user_id.as_str()],
        |row| {
            Ok(User {
                username: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                id: row.get(3)?,
            })
        },
    ).map_err(|e| e.to_string())
}

pub fn user_exists(user_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    let mut stmt = conn.prepare("SELECT COUNT(1) FROM User WHERE id = ?1")?;
    let user_exists: i64 = stmt.query_row([user_id], |row| row.get(0))?;
    Ok(user_exists > 0)
}

pub fn insert_user(user: &UserInfo) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute(
        "INSERT INTO User (id, username, first_name, last_name) VALUES (?1, ?2, '', '')",
        params![user.id, user.username],
    )?;
    println!("User added to database : {:?}", user);
    Ok(())
}

pub fn delete_user_from_waiting_list(user_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute(
        "DELETE FROM WaitingList WHERE user_id = ?1",
        params![user_id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn get_current_user() -> Result<User, String> {
    let current_user = CURRENT_USER.lock().unwrap();
    if let Some(user) = &*current_user {
        return Ok(user.clone());
    }
    Err("User not found".to_string())
}

#[tauri::command]
pub fn register_to_waiting_list() -> Result<bool, String> {
    let users = read_json(JSON_PATH);
    let sid = get_user_sid().expect("Failed to get SID");
    let actual_user = users.iter().find(|u| u.sid == sid);

    if let Some(user) = actual_user {
        let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO WaitingList (user_id, username) VALUES (?1, ?2)",
            params![user.id, user.username],
        )
        .map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Err("User not found".to_string())
    }
}

#[tauri::command]
pub fn get_users_waiting_list() -> Result<Vec<UserWaitingList>, String> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");

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

    let mut users = Vec::new();
    for user_result in users_iter {
        match user_result {
            Ok(user) => users.push(user),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(users)
}

#[tauri::command]
pub fn get_table_data() -> Result<Vec<Measure>, String> {
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, user_id, machine_id, measure_date, temperature FROM Measure")
        .map_err(|e| e.to_string())?;

    let measure_iter = stmt
        .query_map([], |row| {
            Ok(Measure {
                id: row.get(0)?,
                user_id: row.get(1)?,
                machine_id: row.get(2)?,
                measure_date: row.get(3)?,
                temperature: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut measures = Vec::new();
    for measure_result in measure_iter {
        match measure_result {
            Ok(measure) => measures.push(measure),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(measures)
}

#[tauri::command]
pub fn add_data_to_table(data: Vec<Measure>) -> Result<bool, String> {
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    for measure in data {
        conn.execute(
            "INSERT INTO Measure (id, user_id, machine_id, measure_date, temperature)
             VALUES (
                CASE WHEN ?1 IS NOT NULL THEN ?1 ELSE (SELECT MAX(id) + 1 FROM Measure) END,
                ?2, ?3, ?4, ?5
             )",
            params![
                measure.id,
                measure.user_id,
                measure.machine_id,
                measure.measure_date,
                measure.temperature
            ],
        ).map_err(|e| e.to_string())?;
    }

    Ok(true)
}

#[tauri::command]
pub fn clear_measure_table() -> Result<(), String> {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute("DELETE FROM Measure", []).map_err(|e| e.to_string())?;
    Ok(())
}