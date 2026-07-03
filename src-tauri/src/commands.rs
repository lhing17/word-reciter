use std::path::PathBuf;

use tauri::{command, path::BaseDirectory, AppHandle, Manager};

use crate::db;
use crate::db::word_states::Stats;
use crate::db::words::Word;
use crate::services::study::{self, Quiz};
use crate::services::word_import::{self, ImportResult};

/// Resolves a word-list path in a deterministic order.
///
/// 1. Absolute paths are returned unchanged.
/// 2. Bundled resources are checked first (works in packaged apps).
/// 3. The path is resolved relative to the current working directory for dev
///    convenience. During `cargo run`/`tauri dev` the binary runs from
///    `src-tauri/target/debug`, so the cwd is `src-tauri` and a relative path
///    like `references/unique_words_with_chinese.txt` points there.
/// 4. The project-root version (`../<path>`) is tried as a final fallback.
fn resolve_word_list_path(path: &str, app: &AppHandle) -> Result<PathBuf, String> {
    let p = PathBuf::from(path);
    if p.is_absolute() {
        return Ok(p);
    }

    // 1. Try bundled resources (production builds).
    if let Ok(resource_path) = app.path().resolve(&p, BaseDirectory::Resource) {
        if resource_path.exists() {
            return Ok(resource_path);
        }
    }

    // 2. Resolve relative to the current working directory (dev convenience).
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_relative = cwd.join(&p);
        if cwd_relative.exists() {
            return Ok(cwd_relative);
        }
    }

    // 3. Fall back to project root (when cwd is `src-tauri`).
    let from_project_root = PathBuf::from("..").join(&p);
    if from_project_root.exists() {
        return Ok(from_project_root);
    }

    Err(format!("Word list file not found: {}", path))
}

/// Imports a word list from a text file into the application's SQLite database.
///
/// The file is expected to contain one `word|meaning` pair per line. The
/// `source` value is stored alongside each imported word for provenance.
#[command]
pub async fn import_word_list(path: String, source: String, app: AppHandle) -> Result<ImportResult, String> {
    let db_path = db::db_path(&app)?;
    let resolved_path = resolve_word_list_path(&path, &app)?;

    tokio::task::spawn_blocking(move || {
        let mut conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))?;
        word_import::import_from_txt(&mut conn,
            resolved_path.to_string_lossy().as_ref(),
            &source,
        )
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
        Ok(study::generate_quiz(&conn, &pool)?)
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
        let mut conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        db::word_states::mark_word(&tx, &payload.word, &payload.familiarity_after)?;
        db::study_logs::log_study(
            &tx,
            &payload.word,
            &payload.quiz_type,
            &payload.result,
            &payload.familiarity_after,
        )?;
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
