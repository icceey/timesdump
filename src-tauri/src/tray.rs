use crate::ClipboardMonitor;
use log::info;
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager,
};

pub fn setup_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let status_item = MenuItem::with_id(app, "status", "Timesdump - Active", false, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit Timesdump", true, None::<&str>)?;

    // Create the menu
    let menu = Menu::with_items(
        app,
        &[
            &status_item,
            &separator,
            &pause_item,
            &settings_item,
            &PredefinedMenuItem::separator(app)?,
            &quit_item,
        ],
    )?;

    // Get the existing tray icon from config and set up menu and events
    let tray = app
        .tray_by_id("main-tray")
        .ok_or("Tray icon 'main-tray' not found")?;

    tray.set_menu(Some(menu))?;
    tray.set_show_menu_on_left_click(false)?;

    tray.on_menu_event(move |app, event| match event.id.as_ref() {
        "pause" => {
            if let Some(monitor) = app.try_state::<Arc<ClipboardMonitor>>() {
                let is_running = monitor.toggle_pause();
                info!(
                    "Monitoring {}",
                    if is_running { "resumed" } else { "paused" }
                );
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
    });

    tray.on_tray_icon_event(|tray, event| {
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
    });

    Ok(())
}
