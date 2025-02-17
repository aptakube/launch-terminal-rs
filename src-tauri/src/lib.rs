use launch_terminal::Terminal;

#[tauri::command]
async fn launch(terminal: Terminal, command: &str) -> Result<(), String> {
    match launch_terminal::open(terminal.clone(), command) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            launch_terminal::Error::NotSupported => Err(format!("Terminal {:?} is not supported on this OS", terminal)),
            launch_terminal::Error::IOError(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
async fn is_installed(terminal: Terminal) -> Result<bool, String> {
    match launch_terminal::is_installed(terminal.clone()) {
        Ok(installed) => Ok(installed),
        Err(err) => match err {
            launch_terminal::Error::NotSupported => Err(format!("Terminal {:?} is not supported on this OS", terminal)),
            launch_terminal::Error::IOError(err) => Err(err.to_string()),
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![launch, is_installed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
