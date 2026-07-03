use tauri::{command, AppHandle};

use crate::db;
use crate::db::word_states::Stats;
use crate::db::words::Word;
use crate::services::study::{self, Quiz};
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
    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db::word_states::get_stats(&conn)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_next_unmarked_word(
    offset: i64,
    app: AppHandle,
) -> Result<Option<Word>, String> {
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db::words::get_next_unmarked(&conn, offset)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn mark_word(
    word: String,
    familiarity: String,
    app: AppHandle,
) -> Result<(), String> {
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db::word_states::mark_word(&conn, &word, &familiarity)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[derive(serde::Deserialize)]
pub struct StudyResultPayload {
    pub word: String,
    pub quiz_type: String,
    pub result: String,
    pub familiarity_after: String,
}

#[command]
pub async fn generate_quiz(app: AppHandle) -> Result<Option<Quiz>, String> {
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        let pool = db::words::get_study_pool(&conn)?;
        Ok(study::generate_quiz(&pool))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn submit_study_result(
    payload: StudyResultPayload,
    app: AppHandle,
) -> Result<(), String> {
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db::word_states::mark_word(&conn, &payload.word, &payload.familiarity_after)?;
        db::study_logs::log_study(
            &conn,
            &payload.word,
            &payload.quiz_type,
            &payload.result,
            &payload.familiarity_after,
        )?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
