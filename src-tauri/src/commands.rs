/// Placeholder command module for Tauri invoke handlers.
///
/// Add concrete commands (e.g. importing words, recording study results)
/// here as the application grows.

/// A simple health-check command used to verify the backend is reachable.
#[tauri::command]
pub fn ping() -> &'static str {
    "pong"
}
