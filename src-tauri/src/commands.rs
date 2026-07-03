use std::path::PathBuf;

use tauri::{command, path::BaseDirectory, AppHandle, Manager, State};

use crate::db;
use crate::db::word_states::Stats;
use crate::db::words::Word;
use crate::services::study::{self, Quiz};
use crate::services::word_import::{self, ImportResult};
use crate::AppState;

/// Name of the bundled default word list. The backend never trusts the frontend
/// path and always resolves this file from known-safe locations.
const DEFAULT_WORD_LIST_FILE: &str = "unique_words_with_chinese.txt";

/// Resolves the default word-list path from a known-safe location.
///
/// * Rejects absolute paths, paths containing `..`, and filenames that do not
///   match [`DEFAULT_WORD_LIST_FILE`].
/// * In production/bundled builds, resolves the file inside the resource
///   directory (`BaseDirectory::Resource`).
/// * In development, also allows resolving the file relative to the project
///   root (`src-tauri/..`) for convenience when running `cargo run`/`tauri dev`.
fn resolve_word_list_path(path: &str, app: &AppHandle) -> Result<PathBuf, String> {
    let p = PathBuf::from(path);

    // Only plain relative references to the default filename are accepted.
    if p.is_absolute() {
        return Err("absolute word list paths are not allowed".into());
    }
    if p.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return Err("word list path cannot contain '..'".into());
    }
    if p.file_name().and_then(|n| n.to_str()) != Some(DEFAULT_WORD_LIST_FILE) {
        return Err(format!(
            "only the default word list '{}' is allowed",
            DEFAULT_WORD_LIST_FILE
        ));
    }

    // 1. Try bundled resources (production builds).
    if let Ok(resource_path) = app.path().resolve(path, BaseDirectory::Resource) {
        if resource_path.exists() {
            return Ok(resource_path);
        }
    }

    // 2. Dev fallback: resolve relative to the project root. When running
    //    `cargo run`/`tauri dev`, the cwd is `src-tauri`, so the reference file
    //    is at `../references/<path>`.
    let from_project_root = PathBuf::from("..").join(path);
    if from_project_root.exists() {
        return Ok(from_project_root);
    }

    Err(format!("Word list file not found: {}", path))
}

/// Returns an error if the database has not finished initializing or if it
/// failed to initialize.
fn check_db_ready(state: &State<'_, AppState>) -> Result<(), String> {
    if !state.is_complete() {
        return Err("Database is still initializing".into());
    }
    if let Some(err) = state.error() {
        return Err(err);
    }
    Ok(())
}

/// Imports the default word list from the bundled resource directory.
///
/// The `path` argument is ignored; the backend always resolves the known-safe
/// default reference file. The `source` value is stored alongside each imported
/// word for provenance.
#[command]
pub async fn import_word_list(
    _path: String,
    source: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ImportResult, String> {
    check_db_ready(&state)?;
    let db_path = db::db_path(&app)?;
    // Always use the safe default relative path inside the resource directory.
    let resolved_path = resolve_word_list_path(
        &format!("references/{}", DEFAULT_WORD_LIST_FILE),
        &app,
    )?;

    tokio::task::spawn_blocking(move || {
        let mut conn = db::open_connection(&db_path)
            .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))?;
        word_import::import_from_txt(
            &mut conn,
            resolved_path.to_string_lossy().as_ref(),
            &source,
        )
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_stats(app: AppHandle, state: State<'_, AppState>) -> Result<Stats, String> {
    check_db_ready(&state)?;
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = db::open_connection(&path).map_err(|e| e.to_string())?;
        db::word_states::get_stats(&conn)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn get_next_unmarked_word(
    offset: i64,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<Word>, String> {
    check_db_ready(&state)?;
    if offset < 0 {
        return Err("offset cannot be negative".into());
    }
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = db::open_connection(&path).map_err(|e| e.to_string())?;
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
    state: State<'_, AppState>,
) -> Result<(), String> {
    check_db_ready(&state)?;
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = db::open_connection(&path).map_err(|e| e.to_string())?;
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
pub async fn generate_quiz(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<Quiz>, String> {
    check_db_ready(&state)?;
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let conn = db::open_connection(&path).map_err(|e| e.to_string())?;
        let pool = db::words::get_study_pool(&conn)?;
        study::generate_quiz(&conn, &pool)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
pub async fn submit_study_result(
    payload: StudyResultPayload,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    check_db_ready(&state)?;
    let path = crate::db::db_path(&app)?;
    tokio::task::spawn_blocking(move || {
        let mut conn = db::open_connection(&path).map_err(|e| e.to_string())?;
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
