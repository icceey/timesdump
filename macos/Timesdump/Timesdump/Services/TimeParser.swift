import Foundation

struct TimeParser {
    
    struct Configuration {
        let minYear: Int
        let maxYear: Int
        let timeFormat: String
        
        init(minYear: Int = 1990, maxYear: Int = 2050, timeFormat: String = "yyyy-MM-dd HH:mm:ss") {
            self.minYear = minYear
            self.maxYear = maxYear
            self.timeFormat = timeFormat
        }
    }
    
    private let configuration: Configuration
    
    init(configuration: Configuration = Configuration()) {
        self.configuration = configuration
    }
    
    /// Parses clipboard content and returns a TimestampResult if valid
    /// - Parameter content: The raw clipboard content
    /// - Returns: A TimestampResult if the content is a valid timestamp, nil otherwise
    func parse(_ content: String) -> TimestampResult? {
        // Step 1: Trim whitespace
        let trimmed = content.trimmingCharacters(in: .whitespacesAndNewlines)
        
        // Step 2: Validate - must contain only digits
        guard !trimmed.isEmpty, trimmed.allSatisfy({ $0.isNumber }) else {
            return nil
        }
        
        // Step 3: Parse as number
        guard let numericValue = Double(trimmed) else {
            return nil
        }
        
        // Step 4: Determine unit (seconds vs milliseconds)
        let isMilliseconds = trimmed.count > 10
        let timestamp: TimeInterval = isMilliseconds ? numericValue / 1000.0 : numericValue
        
        // Step 5: Create date and validate year range
        let date = Date(timeIntervalSince1970: timestamp)
        let calendar = Calendar.current
        let year = calendar.component(.year, from: date)
        
        guard year >= configuration.minYear && year <= configuration.maxYear else {
            return nil
        }
        
        // Step 6: Return result
        return TimestampResult(
            originalValue: trimmed,
            timestamp: timestamp,
            date: date,
            format: configuration.timeFormat,
            isMilliseconds: isMilliseconds
        )
    }
}
