using Timesdump.Models;

namespace Timesdump.Services;

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

    /// <summary>
    /// Parses clipboard content and returns a TimestampResult if valid
    /// </summary>
    /// <param name="content">The raw clipboard content</param>
    /// <returns>A TimestampResult if the content is a valid timestamp, null otherwise</returns>
    public TimestampResult? Parse(string content)
    {
        // Step 1: Trim whitespace
        var trimmed = content.Trim();

        // Step 2: Validate - must contain only digits
        if (string.IsNullOrEmpty(trimmed) || !trimmed.All(char.IsDigit))
        {
            return null;
        }

        // Step 3: Parse as number
        if (!double.TryParse(trimmed, out var numericValue))
        {
            return null;
        }

        // Step 4: Determine unit (seconds vs milliseconds)
        var isMilliseconds = trimmed.Length > 10;
        var timestamp = isMilliseconds ? numericValue / 1000.0 : numericValue;

        // Step 5: Create date and validate year range
        var date = DateTimeOffset.FromUnixTimeSeconds((long)timestamp).DateTime;
        var year = date.Year;

        if (year < _minYear || year > _maxYear)
        {
            return null;
        }

        // Step 6: Return result
        return new TimestampResult(
            trimmed,
            timestamp,
            date,
            _timeFormat,
            isMilliseconds
        );
    }
}
