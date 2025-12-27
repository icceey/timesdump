use log::debug;
use tauri::WebviewWindow;

use crate::HudPosition;

/// Padding from screen edges in pixels
const SCREEN_EDGE_PADDING: i32 = 20;

/// Setup the ghost window with platform-specific non-activating behavior
pub fn setup_ghost_window(window: &WebviewWindow) {
    #[cfg(target_os = "macos")]
    setup_ghost_window_macos(window);

    #[cfg(target_os = "windows")]
    setup_ghost_window_windows(window);

    #[cfg(target_os = "linux")]
    setup_ghost_window_linux(window);
}

#[cfg(target_os = "macos")]
fn setup_ghost_window_macos(window: &WebviewWindow) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{NSFloatingWindowLevel, NSWindowCollectionBehavior, NSWindowStyleMask};

    // NSNonactivatingPanelMask is not exposed directly in objc2-app-kit
    // This mask prevents the window from activating when clicked
    const NS_NONACTIVATING_PANEL_MASK: usize = 1 << 7;

    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as *mut AnyObject;

            // Set window level to floating (above normal windows)
            let _: () = msg_send![ns_win, setLevel: NSFloatingWindowLevel];

            // Configure collection behavior for all spaces and full screen support
            let behavior = NSWindowCollectionBehavior::CanJoinAllSpaces
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::Transient;
            let _: () = msg_send![ns_win, setCollectionBehavior: behavior];

            // Make window non-activating (won't steal focus)
            let style_mask: NSWindowStyleMask = msg_send![ns_win, styleMask];
            let new_style = NSWindowStyleMask(style_mask.0 | NS_NONACTIVATING_PANEL_MASK);
            let _: () = msg_send![ns_win, setStyleMask: new_style];

            // Ignore mouse events for focus purposes but allow clicks
            let _: () = msg_send![ns_win, setIgnoresMouseEvents: false];

            // Don't show in mission control
            let _: () = msg_send![ns_win, setHidesOnDeactivate: false];

            debug!("macOS ghost window configured");
        }
    }

    // Apply vibrancy effect
    if let Err(e) = window_vibrancy::apply_vibrancy(
        window,
        window_vibrancy::NSVisualEffectMaterial::HudWindow,
        None,
        None,
    ) {
        log::warn!("Failed to apply vibrancy: {:?}", e);
    }
}

#[cfg(target_os = "macos")]
pub fn position_hud_macos(window: &WebviewWindow, position: HudPosition) {
    use tauri::PhysicalPosition;

    if let Some(monitor) = window.current_monitor().ok().flatten() {
        let screen_size = monitor.size();
        let scale_factor = monitor.scale_factor();

        // Get window size (use outer_size for total window dimensions)
        let window_size = window
            .outer_size()
            .unwrap_or(tauri::PhysicalSize::new(380, 120));

        let screen_width = screen_size.width as i32;
        let screen_height = screen_size.height as i32;
        let win_width = window_size.width as i32;
        let win_height = window_size.height as i32;
        let padding = (SCREEN_EDGE_PADDING as f64 * scale_factor) as i32;

        let (x, y) = match position {
            HudPosition::TopLeft => (padding, padding),
            HudPosition::TopRight => (screen_width - win_width - padding, padding),
            HudPosition::BottomLeft => (padding, screen_height - win_height - padding),
            HudPosition::BottomRight => (
                screen_width - win_width - padding,
                screen_height - win_height - padding,
            ),
            HudPosition::TopCenter => ((screen_width - win_width) / 2, padding),
            HudPosition::BottomCenter => (
                (screen_width - win_width) / 2,
                screen_height - win_height - padding,
            ),
        };

        let _ = window.set_position(PhysicalPosition::new(x, y));
        debug!(
            "Positioned HUD window at ({}, {}) for position {:?}",
            x, y, position
        );
    }

    // Show window without activating
    show_without_focus_macos(window);
}

#[cfg(target_os = "macos")]
fn show_without_focus_macos(window: &WebviewWindow) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as *mut AnyObject;
            // orderFront: shows the window without making it key (no focus steal)
            // Pass nil (null_mut) as the sender parameter
            let nil: *mut AnyObject = std::ptr::null_mut();
            let _: () = msg_send![ns_win, orderFront: nil];
        }
    }
}

#[cfg(target_os = "windows")]
fn setup_ghost_window_windows(window: &WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
    };

    if let Ok(hwnd) = window.hwnd() {
        unsafe {
            let hwnd = HWND(hwnd.0);
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

            // Add WS_EX_NOACTIVATE to prevent focus stealing
            // Add WS_EX_TOOLWINDOW to hide from taskbar and Alt+Tab
            let new_style = ex_style | WS_EX_NOACTIVATE.0 as i32 | WS_EX_TOOLWINDOW.0 as i32;
            SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);

            debug!("Windows ghost window configured with WS_EX_NOACTIVATE");
        }
    }

    // Apply Mica or Acrylic effect
    if let Err(e) = window_vibrancy::apply_mica(window, Some(true)) {
        log::warn!("Failed to apply Mica, trying Acrylic: {:?}", e);
        if let Err(e2) = window_vibrancy::apply_acrylic(window, Some((18, 18, 18, 200))) {
            log::warn!("Failed to apply Acrylic: {:?}", e2);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn position_hud_windows(window: &WebviewWindow, position: HudPosition) {
    use tauri::PhysicalPosition;

    if let Some(monitor) = window.current_monitor().ok().flatten() {
        let screen_size = monitor.size();
        let scale_factor = monitor.scale_factor();

        // Get window size
        let window_size = window
            .outer_size()
            .unwrap_or(tauri::PhysicalSize::new(380, 120));

        let screen_width = screen_size.width as i32;
        let screen_height = screen_size.height as i32;
        let win_width = window_size.width as i32;
        let win_height = window_size.height as i32;
        let padding = (SCREEN_EDGE_PADDING as f64 * scale_factor) as i32;

        let (x, y) = match position {
            HudPosition::TopLeft => (padding, padding),
            HudPosition::TopRight => (screen_width - win_width - padding, padding),
            HudPosition::BottomLeft => (padding, screen_height - win_height - padding),
            HudPosition::BottomRight => (
                screen_width - win_width - padding,
                screen_height - win_height - padding,
            ),
            HudPosition::TopCenter => ((screen_width - win_width) / 2, padding),
            HudPosition::BottomCenter => (
                (screen_width - win_width) / 2,
                screen_height - win_height - padding,
            ),
        };

        let _ = window.set_position(PhysicalPosition::new(x, y));
        debug!(
            "Positioned HUD window at ({}, {}) for position {:?}",
            x, y, position
        );
    }

    // Show window without activating
    show_without_focus_windows(window);
}

#[cfg(target_os = "windows")]
fn show_without_focus_windows(window: &WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
    };

    if let Ok(hwnd) = window.hwnd() {
        unsafe {
            let hwnd = HWND(hwnd.0);
            // Show window without activating using SWP_NOACTIVATE
            let _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW | SWP_NOACTIVATE,
            );
        }
    }
}

#[cfg(target_os = "linux")]
fn setup_ghost_window_linux(window: &WebviewWindow) {
    // Linux/GTK doesn't have the same vibrancy APIs
    // Just set up basic window hints
    let _ = window.set_always_on_top(true);
    debug!("Linux ghost window configured");
}

#[cfg(target_os = "linux")]
pub fn position_hud_linux(window: &WebviewWindow, position: HudPosition) {
    use tauri::PhysicalPosition;

    if let Some(monitor) = window.current_monitor().ok().flatten() {
        let screen_size = monitor.size();
        let scale_factor = monitor.scale_factor();

        // Get window size
        let window_size = window
            .outer_size()
            .unwrap_or(tauri::PhysicalSize::new(380, 120));

        let screen_width = screen_size.width as i32;
        let screen_height = screen_size.height as i32;
        let win_width = window_size.width as i32;
        let win_height = window_size.height as i32;
        let padding = (SCREEN_EDGE_PADDING as f64 * scale_factor) as i32;

        let (x, y) = match position {
            HudPosition::TopLeft => (padding, padding),
            HudPosition::TopRight => (screen_width - win_width - padding, padding),
            HudPosition::BottomLeft => (padding, screen_height - win_height - padding),
            HudPosition::BottomRight => (
                screen_width - win_width - padding,
                screen_height - win_height - padding,
            ),
            HudPosition::TopCenter => ((screen_width - win_width) / 2, padding),
            HudPosition::BottomCenter => (
                (screen_width - win_width) / 2,
                screen_height - win_height - padding,
            ),
        };

        let _ = window.set_position(PhysicalPosition::new(x, y));
        debug!(
            "Positioned HUD window at ({}, {}) for position {:?}",
            x, y, position
        );
    }

    let _ = window.show();
}
