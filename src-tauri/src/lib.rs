mod commands;
mod db;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                db::init_db(&handle).await.map_err(|e| e.into())
            })
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
