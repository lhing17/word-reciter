use tauri::{command, AppHandle};

use crate::db;
use crate::db::word_states::Stats;
use crate::services::word_import::{self, ImportResult};

/// Imports a word list from a text file into the application's SQLite database.
///
/// The file is expected to contain one `word|meaning` pair per line. The
/// `source` value is stored alongside each imported word for provenance.
#[command]
pub async fn import_word_list(path: String, source: String, app: AppHandle) -> Result<ImportResult, String> {
    let db_path = db::db_path(&app)?;

    tokio::task::spawn_blocking(move || {
        let mut conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))?;
        word_import::import_from_txt(&mut conn, &path, &source)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_stats(app: AppHandle) -> Result<Stats, String> {
    let path = crate::db::db_path(&app)?;
    let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
    db::word_states::get_stats(&conn)
}
