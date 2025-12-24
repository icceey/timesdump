using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Composition.SystemBackdrops;
using Timesdump.Models;
using Timesdump.Services;
using Windows.ApplicationModel.DataTransfer;

namespace Timesdump.Views;

public sealed partial class HUDWindow : Window
{
    private readonly TimestampResult _result;
    private readonly AppSettings _settings;
    private DispatcherTimer? _dismissTimer;
    private bool _isMouseInside = false;

    public HUDWindow(TimestampResult result, AppSettings settings)
    {
        _result = result;
        _settings = settings;

        InitializeWindow();
        SetupContent();
        PositionWindow();
        ApplyNoActivateStyle();
        StartDismissTimer();
    }

    private void InitializeWindow()
    {
        Title = "Timesdump HUD";
        ExtendsContentIntoTitleBar = true;

        // Apply Mica or Acrylic backdrop
        if (MicaController.IsSupported())
        {
            SystemBackdrop = new MicaBackdrop { Kind = MicaKind.BaseAlt };
        }
        else if (DesktopAcrylicController.IsSupported())
        {
            SystemBackdrop = new DesktopAcrylicBackdrop();
        }
    }

    private void SetupContent()
    {
        var mainGrid = new Grid
        {
            Padding = new Thickness(20, 16, 20, 16),
            RowSpacing = 8,
            Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent)
        };

        mainGrid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        mainGrid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        mainGrid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });

        // Main time display
        var timeText = new TextBlock
        {
            Text = _result.FormattedTime,
            FontFamily = new FontFamily("Segoe UI Variable Display"),
            FontSize = 20,
            FontWeight = Microsoft.UI.Text.FontWeights.Medium
        };
        Grid.SetRow(timeText, 0);
        mainGrid.Children.Add(timeText);

        // Metadata panel
        var metaPanel = new StackPanel
        {
            Orientation = Orientation.Horizontal,
            Spacing = 16
        };
        Grid.SetRow(metaPanel, 1);

        var originalText = new TextBlock
        {
            Text = $"# {_result.OriginalValue}",
            FontFamily = new FontFamily("Segoe UI Variable Text"),
            FontSize = 12,
            Foreground = (Brush)Application.Current.Resources["TextFillColorSecondaryBrush"]
        };
        metaPanel.Children.Add(originalText);

        var unitText = new TextBlock
        {
            Text = $"({_result.UnitDescription})",
            FontFamily = new FontFamily("Segoe UI Variable Text"),
            FontSize = 11,
            Foreground = (Brush)Application.Current.Resources["TextFillColorSecondaryBrush"]
        };
        metaPanel.Children.Add(unitText);
        mainGrid.Children.Add(metaPanel);

        // Hint text
        var hintText = new TextBlock
        {
            Text = LocalizationHelper.GetString("HUD_ClickToCopy"),
            FontFamily = new FontFamily("Segoe UI Variable Text"),
            FontSize = 10,
            Foreground = (Brush)Application.Current.Resources["TextFillColorTertiaryBrush"]
        };
        Grid.SetRow(hintText, 2);
        mainGrid.Children.Add(hintText);

        // Handle click event
        mainGrid.PointerPressed += OnContentClicked;
        mainGrid.PointerEntered += OnPointerEntered;
        mainGrid.PointerExited += OnPointerExited;

        Content = mainGrid;
    }

    private void PositionWindow()
    {
        var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
        var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hwnd);
        var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(windowId);

        // Set window size
        var width = 320;
        var height = 120;
        appWindow.Resize(new Windows.Graphics.SizeInt32(width, height));

        // Calculate position based on settings
        var displayArea = Microsoft.UI.Windowing.DisplayArea.GetFromWindowId(windowId, Microsoft.UI.Windowing.DisplayAreaFallback.Primary);
        var workArea = displayArea.WorkArea;

        int x, y;

        switch (_settings.Position)
        {
            case HUDPosition.FollowMouse:
                if (PInvoke.User32.GetCursorPos(out var cursorPos))
                {
                    x = cursorPos.X + 20;
                    y = cursorPos.Y + 20;

                    // Ensure window stays on screen
                    if (x + width > workArea.X + workArea.Width)
                        x = workArea.X + workArea.Width - width - 20;
                    if (y + height > workArea.Y + workArea.Height)
                        y = workArea.Y + workArea.Height - height - 20;
                }
                else
                {
                    x = workArea.X + workArea.Width - width - 20;
                    y = workArea.Y + 20;
                }
                break;

            case HUDPosition.TopLeft:
                x = workArea.X + 20;
                y = workArea.Y + 20;
                break;

            case HUDPosition.TopRight:
                x = workArea.X + workArea.Width - width - 20;
                y = workArea.Y + 20;
                break;

            case HUDPosition.BottomLeft:
                x = workArea.X + 20;
                y = workArea.Y + workArea.Height - height - 20;
                break;

            case HUDPosition.BottomRight:
                x = workArea.X + workArea.Width - width - 20;
                y = workArea.Y + workArea.Height - height - 20;
                break;

            case HUDPosition.Center:
                x = workArea.X + (workArea.Width - width) / 2;
                y = workArea.Y + (workArea.Height - height) / 2;
                break;

            default:
                x = workArea.X + workArea.Width - width - 20;
                y = workArea.Y + 20;
                break;
        }

        appWindow.Move(new Windows.Graphics.PointInt32(x, y));

        // Configure title bar
        if (appWindow.Presenter is Microsoft.UI.Windowing.OverlappedPresenter presenter)
        {
            presenter.IsAlwaysOnTop = true;
            presenter.IsMinimizable = false;
            presenter.IsMaximizable = false;
            presenter.IsResizable = false;
            presenter.SetBorderAndTitleBar(false, false);
        }
    }

    private void ApplyNoActivateStyle()
    {
        var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);

        // Add WS_EX_NOACTIVATE and WS_EX_TOOLWINDOW styles
        var exStyle = PInvoke.User32.GetWindowLong(hwnd, PInvoke.User32.GWL_EXSTYLE);
        exStyle |= PInvoke.User32.WS_EX_NOACTIVATE | PInvoke.User32.WS_EX_TOOLWINDOW | PInvoke.User32.WS_EX_TOPMOST;
        PInvoke.User32.SetWindowLong(hwnd, PInvoke.User32.GWL_EXSTYLE, exStyle);

        // Show window without activating it
        PInvoke.User32.SetWindowPos(
            hwnd,
            PInvoke.User32.HWND_TOPMOST,
            0, 0, 0, 0,
            PInvoke.User32.SWP_NOMOVE | PInvoke.User32.SWP_NOSIZE | PInvoke.User32.SWP_NOACTIVATE | PInvoke.User32.SWP_SHOWWINDOW
        );

        PInvoke.User32.ShowWindow(hwnd, PInvoke.User32.SW_SHOWNOACTIVATE);
    }

    private void StartDismissTimer()
    {
        _dismissTimer?.Stop();
        _dismissTimer = new DispatcherTimer
        {
            Interval = TimeSpan.FromSeconds(_settings.DisplayDuration)
        };
        _dismissTimer.Tick += (s, e) => DismissWindow();
        _dismissTimer.Start();
    }

    private void OnPointerEntered(object sender, PointerRoutedEventArgs e)
    {
        _isMouseInside = true;
        _dismissTimer?.Stop();
    }

    private void OnPointerExited(object sender, PointerRoutedEventArgs e)
    {
        _isMouseInside = false;
        StartDismissTimer();
    }

    private void OnContentClicked(object sender, PointerRoutedEventArgs e)
    {
        // Copy formatted time to clipboard
        var dataPackage = new DataPackage();
        dataPackage.SetText(_result.FormattedTime);
        Clipboard.SetContent(dataPackage);

        DismissWindow();
    }

    private void DismissWindow()
    {
        _dismissTimer?.Stop();
        Close();
    }
}
