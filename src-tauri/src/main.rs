// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .map_err(|e| anyhow::anyhow!("Error running Tauri application: {}", e))?;
    
    Ok(())
}
