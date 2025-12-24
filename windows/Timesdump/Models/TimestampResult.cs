namespace Timesdump.Models;

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

    public string UnitDescription => IsMilliseconds
        ? LocalizationHelper.GetString("Result_Unit_Milliseconds")
        : LocalizationHelper.GetString("Result_Unit_Seconds");
}
