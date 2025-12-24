using System.Text.Json;

namespace Timesdump.Models;

public class AppSettings
{
    private static readonly string SettingsPath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
        "Timesdump",
        "settings.json"
    );

    public bool LaunchAtStartup { get; set; } = false;
    public HUDPosition Position { get; set; } = HUDPosition.FollowMouse;
    public double DisplayDuration { get; set; } = 3.0;
    public string TimeFormat { get; set; } = "yyyy-MM-dd HH:mm:ss";
    public int MinYear { get; set; } = 1990;
    public int MaxYear { get; set; } = 2050;

    public static AppSettings Load()
    {
        try
        {
            if (File.Exists(SettingsPath))
            {
                var json = File.ReadAllText(SettingsPath);
                return JsonSerializer.Deserialize<AppSettings>(json) ?? new AppSettings();
            }
        }
        catch
        {
            // Return default settings on error
        }
        return new AppSettings();
    }

    public void Save()
    {
        try
        {
            var directory = Path.GetDirectoryName(SettingsPath);
            if (!string.IsNullOrEmpty(directory) && !Directory.Exists(directory))
            {
                Directory.CreateDirectory(directory);
            }

            var json = JsonSerializer.Serialize(this, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(SettingsPath, json);
        }
        catch
        {
            // Silently fail
        }
    }
}

public enum HUDPosition
{
    FollowMouse,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center
}
