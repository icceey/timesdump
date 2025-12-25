use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use log::info;
use crate::ClipboardMonitor;

pub fn create_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let status_item = MenuItem::with_id(app, "status", "Timesdump - Active", false, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit Timesdump", true, None::<&str>)?;
    
    // Create the menu
    let menu = Menu::with_items(app, &[
        &status_item,
        &separator,
        &pause_item,
        &settings_item,
        &PredefinedMenuItem::separator(app)?,
        &quit_item,
    ])?;

    // Get the tray icon if it exists, or create a new one
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "pause" => {
                    if let Some(monitor) = app.try_state::<Arc<ClipboardMonitor>>() {
                        let is_running = monitor.toggle_pause();
                        info!("Monitoring {}", if is_running { "resumed" } else { "paused" });
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("settings") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    info!("Quitting Timesdump");
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Left click opens settings
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;
    
    Ok(())
}
