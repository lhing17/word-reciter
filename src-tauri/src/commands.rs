use tauri::{command, AppHandle};

use crate::db;
use crate::services::word_import::{self, ImportResult};

/// Imports a word list from a text file into the application's SQLite database.
///
/// The file is expected to contain one `word|meaning` pair per line. The
/// `source` value is stored alongside each imported word for provenance.
#[command]
pub async fn import_word_list(path: String, source: String, app: AppHandle) -> Result<ImportResult, String> {
    let db_path = db::db_path(&app)?;

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))?;
        word_import::import_from_txt(&conn, &path, &source)
    })
    .await
    .map_err(|e| e.to_string())?
}
