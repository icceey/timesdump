using System.Runtime.InteropServices;
using Timesdump.Models;

namespace Timesdump.Services;

public class ClipboardMonitor : IDisposable
{
    private readonly Action<TimestampResult> _onTimestampDetected;
    private Timer? _timer;
    private string? _lastContent;
    private bool _isRunning;
    private int _minYear = 1990;
    private int _maxYear = 2050;
    private string _timeFormat = "yyyy-MM-dd HH:mm:ss";

    public ClipboardMonitor(Action<TimestampResult> onTimestampDetected)
    {
        _onTimestampDetected = onTimestampDetected;
    }

    public void UpdateConfiguration(int minYear, int maxYear, string timeFormat)
    {
        _minYear = minYear;
        _maxYear = maxYear;
        _timeFormat = timeFormat;
    }

    public void Start()
    {
        if (_isRunning) return;

        _isRunning = true;
        _lastContent = GetClipboardText();
        _timer = new Timer(CheckClipboard, null, 500, 500);
    }

    public void Stop()
    {
        _isRunning = false;
        _timer?.Dispose();
        _timer = null;
    }

    private void CheckClipboard(object? state)
    {
        if (!_isRunning) return;

        try
        {
            var currentContent = GetClipboardText();

            if (currentContent != null && currentContent != _lastContent)
            {
                _lastContent = currentContent;
                var parser = new TimeParser(_minYear, _maxYear, _timeFormat);
                var result = parser.Parse(currentContent);

                if (result != null)
                {
                    _onTimestampDetected(result);
                }
            }
        }
        catch
        {
            // Silently ignore clipboard access errors
        }
    }

    private static string? GetClipboardText()
    {
        try
        {
            if (OpenClipboard(IntPtr.Zero))
            {
                try
                {
                    var handle = GetClipboardData(CF_UNICODETEXT);
                    if (handle != IntPtr.Zero)
                    {
                        var pointer = GlobalLock(handle);
                        if (pointer != IntPtr.Zero)
                        {
                            try
                            {
                                return Marshal.PtrToStringUni(pointer);
                            }
                            finally
                            {
                                GlobalUnlock(handle);
                            }
                        }
                    }
                }
                finally
                {
                    CloseClipboard();
                }
            }
        }
        catch
        {
            // Silently ignore errors
        }
        return null;
    }

    public void Dispose()
    {
        Stop();
    }

    private const uint CF_UNICODETEXT = 13;

    [DllImport("user32.dll", SetLastError = true)]
    private static extern bool OpenClipboard(IntPtr hWndNewOwner);

    [DllImport("user32.dll", SetLastError = true)]
    private static extern bool CloseClipboard();

    [DllImport("user32.dll", SetLastError = true)]
    private static extern IntPtr GetClipboardData(uint uFormat);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr GlobalLock(IntPtr hMem);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool GlobalUnlock(IntPtr hMem);
}
