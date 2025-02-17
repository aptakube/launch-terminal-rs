use launch_terminal::Terminal;

#[tauri::command]
fn launch(terminal: Terminal, command: &str) {
    launch_terminal::open(terminal, command).expect("error while launching terminal");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
