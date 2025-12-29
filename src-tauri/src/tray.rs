use crate::ClipboardMonitor;
use log::info;
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager,
};

/// Get translated text based on locale
fn get_tray_text(locale: &str) -> TrayTexts {
    if locale.starts_with("zh") {
        TrayTexts {
            status_active: "Timesdump - 运行中",
            status_paused: "Timesdump - 已暂停",
            pause: "暂停",
            resume: "恢复",
            settings: "设置...",
            quit: "退出 Timesdump",
        }
    } else {
        TrayTexts {
            status_active: "Timesdump - Active",
            status_paused: "Timesdump - Paused",
            pause: "Pause",
            resume: "Resume",
            settings: "Settings...",
            quit: "Quit Timesdump",
        }
    }
}

struct TrayTexts {
    status_active: &'static str,
    status_paused: &'static str,
    pause: &'static str,
    resume: &'static str,
    settings: &'static str,
    quit: &'static str,
}

/// Stores references to menu items that need to be updated
pub struct TrayMenuState {
    pub status_item: MenuItem<tauri::Wry>,
    pub pause_item: MenuItem<tauri::Wry>,
}

pub fn setup_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Get system locale for translations
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string());
    let texts = get_tray_text(&locale);

    // Create menu items
    let status_item = MenuItem::with_id(app, "status", texts.status_active, false, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", texts.pause, true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", texts.settings, true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", texts.quit, true, None::<&str>)?;

    // Store menu items for later updates
    app.manage(Mutex::new(TrayMenuState {
        status_item: status_item.clone(),
        pause_item: pause_item.clone(),
    }));

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

                // Update menu items text based on new state
                if let Some(menu_state) = app.try_state::<Mutex<TrayMenuState>>() {
                    if let Ok(state) = menu_state.lock() {
                        let locale =
                            sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string());
                        let texts = get_tray_text(&locale);

                        let new_text = if is_running {
                            texts.pause
                        } else {
                            texts.resume
                        };
                        let _ = state.pause_item.set_text(new_text);

                        let new_status = if is_running {
                            texts.status_active
                        } else {
                            texts.status_paused
                        };
                        let _ = state.status_item.set_text(new_status);
                    }
                }
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
