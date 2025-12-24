using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using H.NotifyIcon;
using Timesdump.Services;
using Timesdump.Views;
using Timesdump.Models;

namespace Timesdump;

public partial class App : Application
{
    private TaskbarIcon? _trayIcon;
    private ClipboardMonitor? _clipboardMonitor;
    private HUDWindow? _hudWindow;
    private SettingsWindow? _settingsWindow;
    private readonly AppSettings _settings;

    public static App Current => (App)Application.Current;
    public AppSettings Settings => _settings;
    public Window? MainWindow { get; private set; }

    public App()
    {
        this.InitializeComponent();
        _settings = AppSettings.Load();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        // Create a hidden main window (required for WinUI 3)
        MainWindow = new Window
        {
            Title = "Timesdump"
        };

        // Hide the window immediately
        var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(MainWindow);
        PInvoke.User32.ShowWindow(hwnd, PInvoke.User32.SW_HIDE);

        SetupTrayIcon();
        SetupClipboardMonitor();
    }

    private void SetupTrayIcon()
    {
        var contextMenu = new MenuFlyout();

        var statusItem = new MenuFlyoutItem
        {
            Text = LocalizationHelper.GetString("Menu_Status_Monitoring"),
            IsEnabled = false
        };
        contextMenu.Items.Add(statusItem);

        contextMenu.Items.Add(new MenuFlyoutSeparator());

        var pauseItem = new MenuFlyoutItem
        {
            Text = LocalizationHelper.GetString("Menu_Pause")
        };
        pauseItem.Click += (s, e) => TogglePause(pauseItem, statusItem);
        contextMenu.Items.Add(pauseItem);

        contextMenu.Items.Add(new MenuFlyoutSeparator());

        var settingsItem = new MenuFlyoutItem
        {
            Text = LocalizationHelper.GetString("Menu_Settings")
        };
        settingsItem.Click += (s, e) => OpenSettings();
        contextMenu.Items.Add(settingsItem);

        contextMenu.Items.Add(new MenuFlyoutSeparator());

        var exitItem = new MenuFlyoutItem
        {
            Text = LocalizationHelper.GetString("Menu_Exit")
        };
        exitItem.Click += (s, e) => ExitApplication();
        contextMenu.Items.Add(exitItem);

        _trayIcon = new TaskbarIcon
        {
            ToolTipText = "Timesdump",
            ContextMenuMode = ContextMenuMode.SecondWindow
        };
        _trayIcon.ContextFlyout = contextMenu;
        _trayIcon.LeftClickCommand = new RelayCommand(OpenSettings);
        _trayIcon.ForceCreate();
    }

    private bool _isPaused = false;

    private void TogglePause(MenuFlyoutItem pauseItem, MenuFlyoutItem statusItem)
    {
        _isPaused = !_isPaused;

        if (_isPaused)
        {
            _clipboardMonitor?.Stop();
            pauseItem.Text = LocalizationHelper.GetString("Menu_Resume");
            statusItem.Text = LocalizationHelper.GetString("Menu_Status_Paused");
        }
        else
        {
            _clipboardMonitor?.Start();
            pauseItem.Text = LocalizationHelper.GetString("Menu_Pause");
            statusItem.Text = LocalizationHelper.GetString("Menu_Status_Monitoring");
        }
    }

    private void SetupClipboardMonitor()
    {
        _clipboardMonitor = new ClipboardMonitor(OnTimestampDetected);
        _clipboardMonitor.Start();
    }

    private void OnTimestampDetected(TimestampResult result)
    {
        MainWindow?.DispatcherQueue.TryEnqueue(() =>
        {
            ShowHUD(result);
        });
    }

    private void ShowHUD(TimestampResult result)
    {
        _hudWindow?.Close();
        _hudWindow = new HUDWindow(result, _settings);
        _hudWindow.Activate();
    }

    private void OpenSettings()
    {
        if (_settingsWindow == null)
        {
            _settingsWindow = new SettingsWindow(_settings);
            _settingsWindow.Closed += (s, e) => _settingsWindow = null;
        }
        _settingsWindow.Activate();
    }

    private void ExitApplication()
    {
        _clipboardMonitor?.Stop();
        _trayIcon?.Dispose();
        _settings.Save();
        Environment.Exit(0);
    }
}

public class RelayCommand : System.Windows.Input.ICommand
{
    private readonly Action _execute;

    public RelayCommand(Action execute)
    {
        _execute = execute;
    }

    public event EventHandler? CanExecuteChanged;

    public bool CanExecute(object? parameter) => true;

    public void Execute(object? parameter) => _execute();
}
