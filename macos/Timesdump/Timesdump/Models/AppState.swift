import Foundation
import SwiftUI

@MainActor
final class AppState: ObservableObject {
    @Published var isPaused: Bool = false
    @Published var history: [TimestampResult] = []
    
    // Settings
    @Published var launchAtLogin: Bool = false {
        didSet { saveSettings() }
    }
    
    @Published var hudPosition: HUDPosition = .followMouse {
        didSet { saveSettings() }
    }
    
    @Published var hudDisplayDuration: Double = 3.0 {
        didSet { saveSettings() }
    }
    
    @Published var timeFormat: TimeFormat = .iso8601 {
        didSet { saveSettings() }
    }
    
    @Published var minYear: Int = 1990 {
        didSet { saveSettings() }
    }
    
    @Published var maxYear: Int = 2050 {
        didSet { saveSettings() }
    }
    
    private let defaults = UserDefaults.standard
    
    enum HUDPosition: String, CaseIterable, Codable {
        case followMouse = "followMouse"
        case topLeft = "topLeft"
        case topRight = "topRight"
        case bottomLeft = "bottomLeft"
        case bottomRight = "bottomRight"
        case center = "center"
        
        var localizedName: String {
            switch self {
            case .followMouse: return NSLocalizedString("settings.position.followMouse", comment: "Follow Mouse")
            case .topLeft: return NSLocalizedString("settings.position.topLeft", comment: "Top Left")
            case .topRight: return NSLocalizedString("settings.position.topRight", comment: "Top Right")
            case .bottomLeft: return NSLocalizedString("settings.position.bottomLeft", comment: "Bottom Left")
            case .bottomRight: return NSLocalizedString("settings.position.bottomRight", comment: "Bottom Right")
            case .center: return NSLocalizedString("settings.position.center", comment: "Center")
            }
        }
    }
    
    enum TimeFormat: String, CaseIterable, Codable {
        case iso8601 = "yyyy-MM-dd HH:mm:ss"
        case usDate = "MM/dd/yyyy HH:mm:ss"
        case euDate = "dd/MM/yyyy HH:mm:ss"
        case chinese = "yyyy年MM月dd日 HH:mm:ss"
        case compact = "yyyyMMdd HHmmss"
        
        var localizedName: String {
            switch self {
            case .iso8601: return "YYYY-MM-DD HH:mm:ss"
            case .usDate: return "MM/DD/YYYY HH:mm:ss"
            case .euDate: return "DD/MM/YYYY HH:mm:ss"
            case .chinese: return NSLocalizedString("settings.format.chinese", comment: "YYYY年MM月DD日 HH:mm:ss")
            case .compact: return "YYYYMMDD HHmmss"
            }
        }
    }
    
    func loadSettings() {
        launchAtLogin = defaults.bool(forKey: "launchAtLogin")
        
        if let positionRaw = defaults.string(forKey: "hudPosition"),
           let position = HUDPosition(rawValue: positionRaw) {
            hudPosition = position
        }
        
        let duration = defaults.double(forKey: "hudDisplayDuration")
        if duration >= 1.5 && duration <= 10.0 {
            hudDisplayDuration = duration
        }
        
        if let formatRaw = defaults.string(forKey: "timeFormat"),
           let format = TimeFormat(rawValue: formatRaw) {
            timeFormat = format
        }
        
        let minYearValue = defaults.integer(forKey: "minYear")
        if minYearValue > 0 {
            minYear = minYearValue
        }
        
        let maxYearValue = defaults.integer(forKey: "maxYear")
        if maxYearValue > 0 {
            maxYear = maxYearValue
        }
    }
    
    func saveSettings() {
        defaults.set(launchAtLogin, forKey: "launchAtLogin")
        defaults.set(hudPosition.rawValue, forKey: "hudPosition")
        defaults.set(hudDisplayDuration, forKey: "hudDisplayDuration")
        defaults.set(timeFormat.rawValue, forKey: "timeFormat")
        defaults.set(minYear, forKey: "minYear")
        defaults.set(maxYear, forKey: "maxYear")
    }
}
