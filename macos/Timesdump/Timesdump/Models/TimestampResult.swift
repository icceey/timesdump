import Foundation

struct TimestampResult: Identifiable, Equatable {
    let id = UUID()
    let originalValue: String
    let timestamp: TimeInterval
    let date: Date
    let formattedTime: String
    let isMilliseconds: Bool
    
    init(originalValue: String, timestamp: TimeInterval, date: Date, format: String, isMilliseconds: Bool) {
        self.originalValue = originalValue
        self.timestamp = timestamp
        self.date = date
        self.isMilliseconds = isMilliseconds
        
        let formatter = DateFormatter()
        formatter.dateFormat = format
        formatter.locale = Locale.current
        formatter.timeZone = TimeZone.current
        self.formattedTime = formatter.string(from: date)
    }
    
    var unitDescription: String {
        isMilliseconds 
            ? NSLocalizedString("result.unit.milliseconds", comment: "milliseconds")
            : NSLocalizedString("result.unit.seconds", comment: "seconds")
    }
}
