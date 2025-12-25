#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use arboard::Clipboard;
use log::info;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use timesdump_lib::{create_tray_menu, setup_ghost_window, ClipboardMonitor, TimestampConfig};

/// Get the system locale
#[tauri::command]
fn get_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string())
}

/// Copy the result to clipboard
#[tauri::command]
fn copy_result(text: String) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

/// Save settings to store
#[tauri::command]
async fn save_settings(
    app: AppHandle,
    min_year: i32,
    max_year: i32,
    display_duration_ms: u64,
    time_format: String,
) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;

    let store = app.store("settings.json").map_err(|e| e.to_string())?;

    store.set("min_year", serde_json::json!(min_year));
    store.set("max_year", serde_json::json!(max_year));
    store.set(
        "display_duration_ms",
        serde_json::json!(display_duration_ms),
    );
    store.set("time_format", serde_json::json!(time_format));
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

/// Load settings from store
#[tauri::command]
async fn load_settings(app: AppHandle) -> Result<TimestampConfig, String> {
    use tauri_plugin_store::StoreExt;

    let store = app.store("settings.json").map_err(|e| e.to_string())?;

    let min_year = store
        .get("min_year")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(1970);

    let max_year = store
        .get("max_year")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or(2100);

    let display_duration_ms = store
        .get("display_duration_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(3000);

    let time_format = store
        .get("time_format")
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "%Y-%m-%d %H:%M:%S".to_string());

    Ok(TimestampConfig {
        min_year,
        max_year,
        display_duration_ms,
        time_format,
    })
}

/// Toggle monitoring pause state
#[tauri::command]
fn toggle_pause(state: tauri::State<Arc<ClipboardMonitor>>) -> bool {
    state.toggle_pause()
}

/// Hide the HUD window
#[tauri::command]
async fn hide_hud(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("hud") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Show the settings window
#[tauri::command]
async fn show_settings(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    env_logger::init();
    info!("Starting Timesdump - The Silent Timestamp Decoder");

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            info!("Setting up Timesdump application");

            // Initialize clipboard monitor with default config
            let config = TimestampConfig::default();
            let monitor = Arc::new(ClipboardMonitor::new(config));

            // Store monitor in app state
            app.manage(Arc::clone(&monitor));

            // Start clipboard monitoring
            monitor.start(app.handle().clone());
            info!("Clipboard monitor started");

            // Setup ghost window behavior for the HUD
            if let Some(hud_window) = app.get_webview_window("hud") {
                setup_ghost_window(&hud_window);
                info!("Ghost window configured");
            }

            // Create system tray
            create_tray_menu(app.handle())?;
            info!("System tray created");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_locale,
            copy_result,
            hide_hud,
            show_settings,
            toggle_pause,
            save_settings,
            load_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
