use tauri::WebviewWindow;
use log::debug;

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
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSWindowLevel};
    use cocoa::base::id;
    use objc::runtime::YES;
    
    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as id;
            
            // Set window level to floating (above normal windows)
            ns_win.setLevel_(NSWindowLevel::NSFloatingWindowLevel as i64);
            
            // Configure collection behavior for all spaces and full screen support
            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorTransient;
            ns_win.setCollectionBehavior_(behavior);
            
            // Make window non-activating (won't steal focus)
            // NSNonactivatingPanelMask = 1 << 7 = 128
            let style_mask = ns_win.styleMask();
            ns_win.setStyleMask_(style_mask | (1 << 7));
            
            // Ignore mouse events for focus purposes but allow clicks
            ns_win.setIgnoresMouseEvents_(cocoa::base::NO);
            
            // Don't show in mission control
            ns_win.setHidesOnDeactivate_(cocoa::base::NO);
            
            debug!("macOS ghost window configured");
        }
    }
    
    // Apply vibrancy effect
    if let Err(e) = window_vibrancy::apply_vibrancy(window, window_vibrancy::NSVisualEffectMaterial::HudWindow, None, None) {
        log::warn!("Failed to apply vibrancy: {:?}", e);
    }
}

#[cfg(target_os = "macos")]
pub fn position_near_cursor_macos(window: &WebviewWindow) {
    use cocoa::appkit::NSEvent;
    use cocoa::foundation::NSPoint;
    use tauri::PhysicalPosition;
    
    unsafe {
        let mouse_location: NSPoint = NSEvent::mouseLocation(cocoa::base::nil);
        
        // Get screen height for coordinate conversion (macOS uses bottom-left origin)
        if let Some(monitor) = window.current_monitor().ok().flatten() {
            let screen_height = monitor.size().height as f64;
            let scale_factor = monitor.scale_factor();
            
            // Convert from macOS coordinates (bottom-left) to screen coordinates (top-left)
            let x = (mouse_location.x * scale_factor) as i32 + 20;
            let y = ((screen_height / scale_factor - mouse_location.y) * scale_factor) as i32 + 20;
            
            let _ = window.set_position(PhysicalPosition::new(x, y));
            debug!("Positioned window at ({}, {})", x, y);
        }
    }
    
    // Show window without activating
    show_without_focus_macos(window);
}

#[cfg(target_os = "macos")]
fn show_without_focus_macos(window: &WebviewWindow) {
    use cocoa::appkit::NSWindow;
    use cocoa::base::{id, nil};
    
    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as id;
            // orderFront: shows the window without making it key (no focus steal)
            let _: () = objc::msg_send![ns_win, orderFront: nil];
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
pub fn position_near_cursor_windows(window: &WebviewWindow) {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    use tauri::PhysicalPosition;
    
    unsafe {
        let mut point = POINT::default();
        if GetCursorPos(&mut point).is_ok() {
            // Position window slightly offset from cursor
            let x = point.x + 20;
            let y = point.y + 20;
            
            let _ = window.set_position(PhysicalPosition::new(x, y));
            debug!("Positioned window at ({}, {})", x, y);
        }
    }
    
    // Show window without activating
    show_without_focus_windows(window);
}

#[cfg(target_os = "windows")]
fn show_without_focus_windows(window: &WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, HWND_TOPMOST,
    };
    
    if let Ok(hwnd) = window.hwnd() {
        unsafe {
            let hwnd = HWND(hwnd.0);
            // Show window without activating using SWP_NOACTIVATE
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0, 0, 0, 0,
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
pub fn position_near_cursor_linux(window: &WebviewWindow) {
    use tauri::PhysicalPosition;
    
    // On Linux, we'll use a fixed position or try to get cursor position via GTK
    // For simplicity, show at a default position
    let _ = window.set_position(PhysicalPosition::new(100, 100));
    let _ = window.show();
    debug!("Linux: positioned window at default location");
}
