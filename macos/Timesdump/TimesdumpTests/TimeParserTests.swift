import XCTest
@testable import Timesdump

final class TimeParserTests: XCTestCase {
    
    // MARK: - Valid Timestamp Tests
    
    func testValidSecondTimestamp() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Unix timestamp for 2023-12-25 00:00:00 UTC
        let result = parser.parse("1703462400")
        
        XCTAssertNotNil(result)
        XCTAssertEqual(result?.originalValue, "1703462400")
        XCTAssertFalse(result?.isMilliseconds ?? true)
    }
    
    func testValidMillisecondTimestamp() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Unix timestamp for 2023-12-25 00:00:00 UTC in milliseconds
        let result = parser.parse("1703462400000")
        
        XCTAssertNotNil(result)
        XCTAssertEqual(result?.originalValue, "1703462400000")
        XCTAssertTrue(result?.isMilliseconds ?? false)
    }
    
    // MARK: - Year Range Filter Tests
    
    func testTimestampBeforeMinYear1990() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Timestamp for 1989-12-31 23:59:59 UTC (before 1990)
        let result = parser.parse("631151999")
        
        XCTAssertNil(result, "Timestamps before 1990 should be rejected")
    }
    
    func testTimestampAtMinYear1990() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Timestamp for 1990-01-01 00:00:00 UTC
        let result = parser.parse("631152000")
        
        XCTAssertNotNil(result, "Timestamps in 1990 should be accepted")
    }
    
    func testTimestampAfterMaxYear2050() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Timestamp for 2051-01-01 00:00:00 UTC
        let result = parser.parse("2556144000")
        
        XCTAssertNil(result, "Timestamps after 2050 should be rejected")
    }
    
    func testTimestampAtMaxYear2050() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Timestamp for 2050-12-31 23:59:59 UTC
        let result = parser.parse("2556143999")
        
        XCTAssertNotNil(result, "Timestamps in 2050 should be accepted")
    }
    
    // MARK: - Input Validation Tests
    
    func testInvalidInputWithNonDigits() {
        let parser = TimeParser()
        
        XCTAssertNil(parser.parse("170346240a"))
        XCTAssertNil(parser.parse("hello"))
        XCTAssertNil(parser.parse("1703462400.5"))
        XCTAssertNil(parser.parse("-1703462400"))
        XCTAssertNil(parser.parse("170 346 240"))
    }
    
    func testInputWithWhitespace() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Leading/trailing whitespace should be trimmed
        let result = parser.parse("  1703462400  ")
        
        XCTAssertNotNil(result)
        XCTAssertEqual(result?.originalValue, "1703462400")
    }
    
    func testEmptyInput() {
        let parser = TimeParser()
        
        XCTAssertNil(parser.parse(""))
        XCTAssertNil(parser.parse("   "))
    }
    
    // MARK: - Phone Number Rejection Tests
    
    func testPhoneNumberRejection() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // Phone numbers typically parse to years far in the future or past
        // 13812345678 as seconds = year ~2407
        let result = parser.parse("13812345678")
        
        XCTAssertNil(result, "Phone numbers should be rejected by year filter")
    }
    
    // MARK: - Verification Code Rejection Tests
    
    func testShortVerificationCodeRejection() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // 6-digit code like 123456 as seconds = year 1970
        let result = parser.parse("123456")
        
        XCTAssertNil(result, "Short verification codes should be rejected by year filter")
    }
    
    // MARK: - Unit Detection Tests (10-digit vs >10-digit)
    
    func testTenDigitAsSeconds() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        let result = parser.parse("1703462400")
        
        XCTAssertNotNil(result)
        XCTAssertFalse(result?.isMilliseconds ?? true)
    }
    
    func testElevenDigitAsMilliseconds() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        // 11 digits should be treated as milliseconds
        let result = parser.parse("17034624000")
        
        XCTAssertNotNil(result)
        XCTAssertTrue(result?.isMilliseconds ?? false)
    }
    
    func testThirteenDigitAsMilliseconds() {
        let parser = TimeParser(configuration: .init(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss"))
        
        let result = parser.parse("1703462400000")
        
        XCTAssertNotNil(result)
        XCTAssertTrue(result?.isMilliseconds ?? false)
    }
}
