use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub mod migrations;
pub mod study_logs;
pub mod word_states;
pub mod words;

/// Returns the path to the SQLite database file.
pub fn db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("word_reciter.db"))
}

/// Initializes the SQLite database in the application data directory.
///
/// Creates the database file and runs the migration SQL to ensure the
/// `words`, `word_states`, and `study_logs` tables exist.
pub async fn init_db(app: &AppHandle) -> Result<(), String> {
    let app = app.clone();
    tokio::task::spawn_blocking(move || {
        let db_path = db_path(&app)?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let mut conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
        migrations::run_migrations(&mut conn).map_err(|e| e.to_string())?;
        conn.execute("PRAGMA foreign_keys = ON;", [])
            .map_err(|e| e.to_string())?;

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
