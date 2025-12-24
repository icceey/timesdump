using Xunit;

namespace Timesdump.Tests;

/// <summary>
/// Standalone TimeParser for testing purposes.
/// This duplicates the logic from the main project to avoid WinUI dependencies in tests.
/// </summary>
public class TimeParser
{
    private readonly int _minYear;
    private readonly int _maxYear;
    private readonly string _timeFormat;

    public TimeParser(int minYear = 1990, int maxYear = 2050, string timeFormat = "yyyy-MM-dd HH:mm:ss")
    {
        _minYear = minYear;
        _maxYear = maxYear;
        _timeFormat = timeFormat;
    }

    public TimestampResult? Parse(string content)
    {
        var trimmed = content.Trim();

        if (string.IsNullOrEmpty(trimmed) || !trimmed.All(char.IsDigit))
        {
            return null;
        }

        if (!double.TryParse(trimmed, out var numericValue))
        {
            return null;
        }

        var isMilliseconds = trimmed.Length > 10;
        var timestamp = isMilliseconds ? numericValue / 1000.0 : numericValue;

        var date = DateTimeOffset.FromUnixTimeSeconds((long)timestamp).DateTime;
        var year = date.Year;

        if (year < _minYear || year > _maxYear)
        {
            return null;
        }

        return new TimestampResult(
            trimmed,
            timestamp,
            date,
            _timeFormat,
            isMilliseconds
        );
    }
}

public class TimestampResult
{
    public string OriginalValue { get; }
    public double Timestamp { get; }
    public DateTime Date { get; }
    public string FormattedTime { get; }
    public bool IsMilliseconds { get; }

    public TimestampResult(string originalValue, double timestamp, DateTime date, string format, bool isMilliseconds)
    {
        OriginalValue = originalValue;
        Timestamp = timestamp;
        Date = date;
        IsMilliseconds = isMilliseconds;
        FormattedTime = date.ToString(format);
    }
}

public class TimeParserTests
{
    // MARK: - Valid Timestamp Tests

    [Fact]
    public void TestValidSecondTimestamp()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Unix timestamp for 2023-12-25 00:00:00 UTC
        var result = parser.Parse("1703462400");

        Assert.NotNull(result);
        Assert.Equal("1703462400", result.OriginalValue);
        Assert.False(result.IsMilliseconds);
    }

    [Fact]
    public void TestValidMillisecondTimestamp()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Unix timestamp for 2023-12-25 00:00:00 UTC in milliseconds
        var result = parser.Parse("1703462400000");

        Assert.NotNull(result);
        Assert.Equal("1703462400000", result.OriginalValue);
        Assert.True(result.IsMilliseconds);
    }

    // MARK: - Year Range Filter Tests

    [Fact]
    public void TestTimestampBeforeMinYear1990()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Timestamp for 1989-12-31 23:59:59 UTC (before 1990)
        var result = parser.Parse("631151999");

        Assert.Null(result);
    }

    [Fact]
    public void TestTimestampAtMinYear1990()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Timestamp for 1990-01-01 00:00:00 UTC
        var result = parser.Parse("631152000");

        Assert.NotNull(result);
    }

    [Fact]
    public void TestTimestampAfterMaxYear2050()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Timestamp for 2051-01-01 00:00:00 UTC
        var result = parser.Parse("2556144000");

        Assert.Null(result);
    }

    [Fact]
    public void TestTimestampAtMaxYear2050()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Timestamp for 2050-12-31 23:59:59 UTC
        var result = parser.Parse("2556143999");

        Assert.NotNull(result);
    }

    // MARK: - Input Validation Tests

    [Theory]
    [InlineData("170346240a")]
    [InlineData("hello")]
    [InlineData("1703462400.5")]
    [InlineData("-1703462400")]
    [InlineData("170 346 240")]
    public void TestInvalidInputWithNonDigits(string input)
    {
        var parser = new TimeParser();

        var result = parser.Parse(input);

        Assert.Null(result);
    }

    [Fact]
    public void TestInputWithWhitespace()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Leading/trailing whitespace should be trimmed
        var result = parser.Parse("  1703462400  ");

        Assert.NotNull(result);
        Assert.Equal("1703462400", result.OriginalValue);
    }

    [Theory]
    [InlineData("")]
    [InlineData("   ")]
    public void TestEmptyInput(string input)
    {
        var parser = new TimeParser();

        var result = parser.Parse(input);

        Assert.Null(result);
    }

    // MARK: - Phone Number Rejection Tests

    [Fact]
    public void TestPhoneNumberRejection()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // Phone numbers typically parse to years far in the future or past
        // 13812345678 as seconds = year ~2407
        var result = parser.Parse("13812345678");

        Assert.Null(result);
    }

    // MARK: - Verification Code Rejection Tests

    [Fact]
    public void TestShortVerificationCodeRejection()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // 6-digit code like 123456 as seconds = year 1970
        var result = parser.Parse("123456");

        Assert.Null(result);
    }

    // MARK: - Unit Detection Tests (10-digit vs >10-digit)

    [Fact]
    public void TestTenDigitAsSeconds()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        var result = parser.Parse("1703462400");

        Assert.NotNull(result);
        Assert.False(result.IsMilliseconds);
    }

    [Fact]
    public void TestElevenDigitAsMilliseconds()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        // 11 digits should be treated as milliseconds
        var result = parser.Parse("17034624000");

        Assert.NotNull(result);
        Assert.True(result.IsMilliseconds);
    }

    [Fact]
    public void TestThirteenDigitAsMilliseconds()
    {
        var parser = new TimeParser(minYear: 1990, maxYear: 2050, timeFormat: "yyyy-MM-dd HH:mm:ss");

        var result = parser.Parse("1703462400000");

        Assert.NotNull(result);
        Assert.True(result.IsMilliseconds);
    }
}
