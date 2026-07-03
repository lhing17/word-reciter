mod commands;
mod db;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = db::init_db(&handle).await {
                    eprintln!("Database init failed: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::import_word_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
