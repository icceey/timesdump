import AppKit
import SwiftUI

final class SettingsWindowController: NSWindowController {
    
    init(appState: AppState) {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 500, height: 400),
            styleMask: [.titled, .closable, .miniaturizable],
            backing: .buffered,
            defer: false
        )
        
        super.init(window: window)
        
        window.title = NSLocalizedString("settings.title", comment: "Settings")
        window.center()
        
        let settingsView = SettingsView()
            .environmentObject(appState)
        
        window.contentView = NSHostingView(rootView: settingsView)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
}
