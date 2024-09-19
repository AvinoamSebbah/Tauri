use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::command;

// Structure pour l'utilisateur
#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

// Initialiser la base de données
#[command]
fn initialize_db() -> Result<(), String> {
    let conn = Connection::open("app_data.db").map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Ajouter un utilisateur
#[command]
fn insert_user(name: String) -> Result<(), String> {
    let conn = Connection::open("app_data.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO users (name) VALUES (?1)",
        params![name],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Récupérer tous les utilisateurs
#[command]
fn get_users() -> Result<Vec<User>, String> {
    let conn = Connection::open("app_data.db").map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name FROM users")
        .map_err(|e| e.to_string())?;
    let user_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user.map_err(|e| e.to_string())?);
    }

    Ok(users)
}

// Mettre à jour un utilisateur
#[command]
fn update_user(id: i32, name: String) -> Result<(), String> {
    let conn = Connection::open("app_data.db").map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE users SET name = ?1 WHERE id = ?2",
        params![name, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Supprimer un utilisateur
#[command]
fn delete_user(id: i32) -> Result<(), String> {
    let conn = Connection::open("app_data.db").map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM users WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Fonction principale
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initialize_db,
            insert_user,
            get_users,
            update_user,
            delete_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
