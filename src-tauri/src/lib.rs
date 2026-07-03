mod commands;
mod db;
mod services;

use std::sync::{Arc, Mutex};
use tauri::Manager;

/// Application-wide state that guards against commands running before the
/// database has finished initializing.
pub struct AppState {
    init_complete: Mutex<bool>,
    init_error: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            init_complete: Mutex::new(false),
            init_error: Mutex::new(None),
        })
    }

    pub fn mark_complete(&self) {
        *self.init_complete.lock().unwrap() = true;
    }

    pub fn mark_error(&self, error: String) {
        *self.init_error.lock().unwrap() = Some(error);
        *self.init_complete.lock().unwrap() = true;
    }

    pub fn is_complete(&self) -> bool {
        *self.init_complete.lock().unwrap()
    }

    pub fn error(&self) -> Option<String> {
        self.init_error.lock().unwrap().clone()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .setup(|app| {
            let handle = app.handle().clone();
            let state: Arc<AppState> = app.state::<Arc<AppState>>().inner().clone();
            tauri::async_runtime::spawn(async move {
                match db::init_db(&handle).await {
                    Ok(()) => state.mark_complete(),
                    Err(e) => {
                        eprintln!("Database initialization failed: {}", e);
                        state.mark_error(e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::import_word_list,
            commands::get_stats,
            commands::get_next_unmarked_word,
            commands::mark_word,
            commands::generate_quiz,
            commands::submit_study_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
