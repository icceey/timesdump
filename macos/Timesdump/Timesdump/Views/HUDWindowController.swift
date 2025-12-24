import AppKit
import SwiftUI

final class HUDWindowController: NSWindowController {
    
    private var dismissTimer: Timer?
    private var isMouseInside = false
    private let result: TimestampResult
    private let appState: AppState
    
    init(result: TimestampResult, appState: AppState) {
        self.result = result
        self.appState = appState
        
        let panel = HUDPanel(
            contentRect: NSRect(x: 0, y: 0, width: 320, height: 100),
            styleMask: [.nonactivatingPanel, .fullSizeContentView, .borderless],
            backing: .buffered,
            defer: false
        )
        
        super.init(window: panel)
        
        setupPanel(panel)
        setupContent()
        positionWindow()
        startDismissTimer()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupPanel(_ panel: HUDPanel) {
        panel.level = .floating
        panel.backgroundColor = .clear
        panel.isOpaque = false
        panel.hasShadow = true
        panel.isMovableByWindowBackground = false
        panel.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary, .transient]
        panel.hidesOnDeactivate = false
        panel.becomesKeyOnlyIfNeeded = true
        
        panel.onMouseEntered = { [weak self] in
            self?.handleMouseEntered()
        }
        
        panel.onMouseExited = { [weak self] in
            self?.handleMouseExited()
        }
        
        panel.onClicked = { [weak self] in
            self?.handleClick()
        }
    }
    
    private func setupContent() {
        guard let panel = window else { return }
        
        let hudView = HUDView(result: result)
        let hostingView = NSHostingView(rootView: hudView)
        hostingView.frame = panel.contentView?.bounds ?? .zero
        hostingView.autoresizingMask = [.width, .height]
        
        panel.contentView = hostingView
    }
    
    private func positionWindow() {
        guard let panel = window else { return }
        
        let windowSize = panel.frame.size
        var position: NSPoint
        
        switch appState.hudPosition {
        case .followMouse:
            let mouseLocation = NSEvent.mouseLocation
            position = NSPoint(
                x: mouseLocation.x + 20,
                y: mouseLocation.y - windowSize.height - 20
            )
            
        case .topLeft:
            if let screen = NSScreen.main {
                position = NSPoint(
                    x: screen.visibleFrame.minX + 20,
                    y: screen.visibleFrame.maxY - windowSize.height - 20
                )
            } else {
                position = .zero
            }
            
        case .topRight:
            if let screen = NSScreen.main {
                position = NSPoint(
                    x: screen.visibleFrame.maxX - windowSize.width - 20,
                    y: screen.visibleFrame.maxY - windowSize.height - 20
                )
            } else {
                position = .zero
            }
            
        case .bottomLeft:
            if let screen = NSScreen.main {
                position = NSPoint(
                    x: screen.visibleFrame.minX + 20,
                    y: screen.visibleFrame.minY + 20
                )
            } else {
                position = .zero
            }
            
        case .bottomRight:
            if let screen = NSScreen.main {
                position = NSPoint(
                    x: screen.visibleFrame.maxX - windowSize.width - 20,
                    y: screen.visibleFrame.minY + 20
                )
            } else {
                position = .zero
            }
            
        case .center:
            if let screen = NSScreen.main {
                position = NSPoint(
                    x: screen.visibleFrame.midX - windowSize.width / 2,
                    y: screen.visibleFrame.midY - windowSize.height / 2
                )
            } else {
                position = .zero
            }
        }
        
        // Ensure window stays on screen
        if let screen = NSScreen.screens.first(where: { NSPointInRect(position, $0.frame) }) ?? NSScreen.main {
            let visibleFrame = screen.visibleFrame
            
            if position.x + windowSize.width > visibleFrame.maxX {
                position.x = visibleFrame.maxX - windowSize.width - 20
            }
            if position.x < visibleFrame.minX {
                position.x = visibleFrame.minX + 20
            }
            if position.y < visibleFrame.minY {
                position.y = visibleFrame.minY + 20
            }
            if position.y + windowSize.height > visibleFrame.maxY {
                position.y = visibleFrame.maxY - windowSize.height - 20
            }
        }
        
        panel.setFrameOrigin(position)
    }
    
    override func showWindow(_ sender: Any?) {
        guard let panel = window else { return }
        panel.orderFront(nil)
        
        // Fade in animation
        panel.alphaValue = 0
        NSAnimationContext.runAnimationGroup { context in
            context.duration = 0.2
            panel.animator().alphaValue = 1
        }
    }
    
    private func startDismissTimer() {
        dismissTimer?.invalidate()
        dismissTimer = Timer.scheduledTimer(
            withTimeInterval: appState.hudDisplayDuration,
            repeats: false
        ) { [weak self] _ in
            Task { @MainActor in
                self?.dismissWindow()
            }
        }
    }
    
    private func handleMouseEntered() {
        isMouseInside = true
        dismissTimer?.invalidate()
        dismissTimer = nil
    }
    
    private func handleMouseExited() {
        isMouseInside = false
        startDismissTimer()
    }
    
    private func handleClick() {
        // Copy formatted time to clipboard
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(result.formattedTime, forType: .string)
        
        // Visual feedback - scale animation
        guard let panel = window else { return }
        
        NSAnimationContext.runAnimationGroup({ context in
            context.duration = 0.1
            panel.animator().alphaValue = 0.5
        }, completionHandler: { [weak self] in
            self?.dismissWindow()
        })
    }
    
    private func dismissWindow() {
        guard let panel = window else { return }
        
        NSAnimationContext.runAnimationGroup({ context in
            context.duration = 0.2
            panel.animator().alphaValue = 0
        }, completionHandler: { [weak self] in
            self?.close()
        })
    }
    
    override func close() {
        dismissTimer?.invalidate()
        dismissTimer = nil
        super.close()
    }
}

final class HUDPanel: NSPanel {
    
    var onMouseEntered: (() -> Void)?
    var onMouseExited: (() -> Void)?
    var onClicked: (() -> Void)?
    
    private var trackingArea: NSTrackingArea?
    
    override var canBecomeKey: Bool { false }
    override var canBecomeMain: Bool { false }
    
    override func awakeFromNib() {
        super.awakeFromNib()
        setupTrackingArea()
    }
    
    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        setupTrackingArea()
    }
    
    private func setupTrackingArea() {
        guard let contentView = contentView else { return }
        
        if let existing = trackingArea {
            contentView.removeTrackingArea(existing)
        }
        
        trackingArea = NSTrackingArea(
            rect: contentView.bounds,
            options: [.mouseEnteredAndExited, .activeAlways, .inVisibleRect],
            owner: self,
            userInfo: nil
        )
        
        contentView.addTrackingArea(trackingArea!)
    }
    
    override func mouseEntered(with event: NSEvent) {
        onMouseEntered?()
    }
    
    override func mouseExited(with event: NSEvent) {
        onMouseExited?()
    }
    
    override func mouseDown(with event: NSEvent) {
        onClicked?()
    }
}
