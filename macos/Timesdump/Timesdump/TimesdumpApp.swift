import SwiftUI
import AppKit

@main
struct TimesdumpApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        Settings {
            SettingsView()
                .environmentObject(appDelegate.appState)
        }
    }
}

@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate {
    let appState = AppState()
    private var statusItem: NSStatusItem!
    private var hudWindowController: HUDWindowController?
    private var clipboardMonitor: ClipboardMonitor?
    private var settingsWindowController: SettingsWindowController?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        setupMenuBar()
        setupClipboardMonitor()
        appState.loadSettings()
    }
    
    private func setupMenuBar() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        if let button = statusItem.button {
            button.image = NSImage(systemSymbolName: "clock.badge.checkmark", accessibilityDescription: NSLocalizedString("menu.icon.description", comment: "Timesdump icon"))
            button.image?.isTemplate = true
        }
        
        let menu = NSMenu()
        
        let statusItem = NSMenuItem(title: NSLocalizedString("menu.status.monitoring", comment: "Monitoring"), action: nil, keyEquivalent: "")
        statusItem.tag = 100
        menu.addItem(statusItem)
        
        menu.addItem(NSMenuItem.separator())
        
        let pauseItem = NSMenuItem(title: NSLocalizedString("menu.pause", comment: "Pause"), action: #selector(togglePause), keyEquivalent: "p")
        pauseItem.target = self
        pauseItem.tag = 101
        menu.addItem(pauseItem)
        
        menu.addItem(NSMenuItem.separator())
        
        let historyItem = NSMenuItem(title: NSLocalizedString("menu.history", comment: "History"), action: nil, keyEquivalent: "")
        let historySubmenu = NSMenu()
        historySubmenu.addItem(NSMenuItem(title: NSLocalizedString("menu.history.empty", comment: "No history"), action: nil, keyEquivalent: ""))
        historyItem.submenu = historySubmenu
        historyItem.tag = 102
        menu.addItem(historyItem)
        
        menu.addItem(NSMenuItem.separator())
        
        let settingsItem = NSMenuItem(title: NSLocalizedString("menu.settings", comment: "Settings..."), action: #selector(openSettings), keyEquivalent: ",")
        settingsItem.target = self
        menu.addItem(settingsItem)
        
        menu.addItem(NSMenuItem.separator())
        
        let quitItem = NSMenuItem(title: NSLocalizedString("menu.quit", comment: "Quit Timesdump"), action: #selector(quitApp), keyEquivalent: "q")
        quitItem.target = self
        menu.addItem(quitItem)
        
        self.statusItem.menu = menu
    }
    
    private func setupClipboardMonitor() {
        clipboardMonitor = ClipboardMonitor { [weak self] result in
            guard let self = self else { return }
            Task { @MainActor in
                self.showHUD(with: result)
                self.addToHistory(result)
            }
        }
        clipboardMonitor?.start()
    }
    
    @objc private func togglePause() {
        appState.isPaused.toggle()
        
        if appState.isPaused {
            clipboardMonitor?.stop()
        } else {
            clipboardMonitor?.start()
        }
        
        updateMenuState()
    }
    
    private func updateMenuState() {
        guard let menu = statusItem.menu else { return }
        
        if let statusMenuItem = menu.item(withTag: 100) {
            statusMenuItem.title = appState.isPaused 
                ? NSLocalizedString("menu.status.paused", comment: "Paused")
                : NSLocalizedString("menu.status.monitoring", comment: "Monitoring")
        }
        
        if let pauseItem = menu.item(withTag: 101) {
            pauseItem.title = appState.isPaused 
                ? NSLocalizedString("menu.resume", comment: "Resume")
                : NSLocalizedString("menu.pause", comment: "Pause")
        }
    }
    
    private func addToHistory(_ result: TimestampResult) {
        appState.history.insert(result, at: 0)
        if appState.history.count > 10 {
            appState.history.removeLast()
        }
        updateHistoryMenu()
    }
    
    private func updateHistoryMenu() {
        guard let menu = statusItem.menu,
              let historyItem = menu.item(withTag: 102),
              let historySubmenu = historyItem.submenu else { return }
        
        historySubmenu.removeAllItems()
        
        if appState.history.isEmpty {
            historySubmenu.addItem(NSMenuItem(title: NSLocalizedString("menu.history.empty", comment: "No history"), action: nil, keyEquivalent: ""))
        } else {
            for (index, result) in appState.history.prefix(10).enumerated() {
                let item = NSMenuItem(title: result.formattedTime, action: #selector(copyHistoryItem(_:)), keyEquivalent: "")
                item.target = self
                item.tag = 1000 + index
                historySubmenu.addItem(item)
            }
        }
    }
    
    @objc private func copyHistoryItem(_ sender: NSMenuItem) {
        let index = sender.tag - 1000
        guard index >= 0 && index < appState.history.count else { return }
        
        let result = appState.history[index]
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(result.formattedTime, forType: .string)
    }
    
    @objc private func openSettings() {
        if settingsWindowController == nil {
            settingsWindowController = SettingsWindowController(appState: appState)
        }
        settingsWindowController?.showWindow(nil)
        NSApp.activate(ignoringOtherApps: true)
    }
    
    @objc private func quitApp() {
        NSApp.terminate(nil)
    }
    
    private func showHUD(with result: TimestampResult) {
        hudWindowController?.close()
        hudWindowController = HUDWindowController(result: result, appState: appState)
        hudWindowController?.showWindow(nil)
    }
}
