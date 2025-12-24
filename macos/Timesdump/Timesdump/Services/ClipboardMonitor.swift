import Foundation
import AppKit

@MainActor
final class ClipboardMonitor {
    
    private var timer: Timer?
    private var lastChangeCount: Int
    private let pasteboard = NSPasteboard.general
    private let onTimestampDetected: (TimestampResult) -> Void
    private var configuration: TimeParser.Configuration
    
    init(onTimestampDetected: @escaping (TimestampResult) -> Void) {
        self.lastChangeCount = pasteboard.changeCount
        self.onTimestampDetected = onTimestampDetected
        self.configuration = TimeParser.Configuration()
    }
    
    func updateConfiguration(minYear: Int, maxYear: Int, timeFormat: String) {
        self.configuration = TimeParser.Configuration(
            minYear: minYear,
            maxYear: maxYear,
            timeFormat: timeFormat
        )
    }
    
    func start() {
        stop()
        lastChangeCount = pasteboard.changeCount
        
        timer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { [weak self] _ in
            Task { @MainActor in
                self?.checkClipboard()
            }
        }
        
        RunLoop.main.add(timer!, forMode: .common)
    }
    
    func stop() {
        timer?.invalidate()
        timer = nil
    }
    
    private func checkClipboard() {
        let currentChangeCount = pasteboard.changeCount
        
        guard currentChangeCount != lastChangeCount else {
            return
        }
        
        lastChangeCount = currentChangeCount
        
        guard let content = pasteboard.string(forType: .string) else {
            return
        }
        
        let parser = TimeParser(configuration: configuration)
        
        if let result = parser.parse(content) {
            onTimestampDetected(result)
        }
    }
}
