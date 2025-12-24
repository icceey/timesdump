using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Composition.SystemBackdrops;
using Timesdump.Models;
using Timesdump.Services;

namespace Timesdump.Views;

public sealed class SettingsWindow : Window
{
    private readonly AppSettings _settings;

    public SettingsWindow(AppSettings settings)
    {
        _settings = settings;
        Title = LocalizationHelper.GetString("Settings_Title");
        ExtendsContentIntoTitleBar = true;

        // Apply Mica backdrop
        if (MicaController.IsSupported())
        {
            SystemBackdrop = new MicaBackdrop();
        }

        SetupWindow();
        SetupContent();
    }

    private void SetupWindow()
    {
        var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
        var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hwnd);
        var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(windowId);

        appWindow.Resize(new Windows.Graphics.SizeInt32(600, 500));

        // Center the window
        var displayArea = Microsoft.UI.Windowing.DisplayArea.GetFromWindowId(windowId, Microsoft.UI.Windowing.DisplayAreaFallback.Primary);
        var workArea = displayArea.WorkArea;
        var x = (workArea.Width - 600) / 2 + workArea.X;
        var y = (workArea.Height - 500) / 2 + workArea.Y;
        appWindow.Move(new Windows.Graphics.PointInt32(x, y));
    }

    private void SetupContent()
    {
        var navigationView = new NavigationView
        {
            IsBackButtonVisible = NavigationViewBackButtonVisible.Collapsed,
            IsSettingsVisible = false,
            PaneDisplayMode = NavigationViewPaneDisplayMode.Left
        };

        // Add navigation items
        var generalItem = new NavigationViewItem
        {
            Content = LocalizationHelper.GetString("Settings_Tab_General"),
            Icon = new SymbolIcon(Symbol.Setting),
            Tag = "General"
        };
        navigationView.MenuItems.Add(generalItem);

        var filterItem = new NavigationViewItem
        {
            Content = LocalizationHelper.GetString("Settings_Tab_Filter"),
            Icon = new SymbolIcon(Symbol.Filter),
            Tag = "Filter"
        };
        navigationView.MenuItems.Add(filterItem);

        var aboutItem = new NavigationViewItem
        {
            Content = LocalizationHelper.GetString("Settings_Tab_About"),
            Icon = new SymbolIcon(Symbol.Help),
            Tag = "About"
        };
        navigationView.MenuItems.Add(aboutItem);

        navigationView.SelectionChanged += NavigationView_SelectionChanged;
        navigationView.SelectedItem = generalItem;

        Content = navigationView;
    }

    private void NavigationView_SelectionChanged(NavigationView sender, NavigationViewSelectionChangedEventArgs args)
    {
        if (args.SelectedItem is NavigationViewItem item)
        {
            var tag = item.Tag?.ToString();
            sender.Content = tag switch
            {
                "General" => CreateGeneralPage(),
                "Filter" => CreateFilterPage(),
                "About" => CreateAboutPage(),
                _ => CreateGeneralPage()
            };
        }
    }

    private UIElement CreateGeneralPage()
    {
        var panel = new StackPanel
        {
            Spacing = 16,
            Padding = new Thickness(24)
        };

        // Launch at startup toggle
        var startupToggle = new ToggleSwitch
        {
            Header = LocalizationHelper.GetString("Settings_LaunchAtStartup"),
            IsOn = _settings.LaunchAtStartup
        };
        startupToggle.Toggled += (s, e) =>
        {
            _settings.LaunchAtStartup = startupToggle.IsOn;
            _settings.Save();
        };
        panel.Children.Add(startupToggle);

        // HUD Position
        var positionCombo = new ComboBox
        {
            Header = LocalizationHelper.GetString("Settings_HUDPosition"),
            Width = 200
        };
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_FollowMouse"), Tag = HUDPosition.FollowMouse });
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_TopLeft"), Tag = HUDPosition.TopLeft });
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_TopRight"), Tag = HUDPosition.TopRight });
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_BottomLeft"), Tag = HUDPosition.BottomLeft });
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_BottomRight"), Tag = HUDPosition.BottomRight });
        positionCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Position_Center"), Tag = HUDPosition.Center });
        positionCombo.SelectedIndex = (int)_settings.Position;
        positionCombo.SelectionChanged += (s, e) =>
        {
            if (positionCombo.SelectedItem is ComboBoxItem item && item.Tag is HUDPosition pos)
            {
                _settings.Position = pos;
                _settings.Save();
            }
        };
        panel.Children.Add(positionCombo);

        // Display Duration
        var durationPanel = new StackPanel { Spacing = 8 };
        var durationHeader = new TextBlock
        {
            Text = LocalizationHelper.GetString("Settings_DisplayDuration")
        };
        durationPanel.Children.Add(durationHeader);

        var durationSliderPanel = new StackPanel
        {
            Orientation = Orientation.Horizontal,
            Spacing = 12
        };

        var durationSlider = new Slider
        {
            Minimum = 1.5,
            Maximum = 10,
            StepFrequency = 0.5,
            Width = 200,
            Value = _settings.DisplayDuration
        };

        var durationValue = new TextBlock
        {
            Text = $"{_settings.DisplayDuration:F1}s",
            VerticalAlignment = VerticalAlignment.Center
        };

        durationSlider.ValueChanged += (s, e) =>
        {
            _settings.DisplayDuration = durationSlider.Value;
            durationValue.Text = $"{durationSlider.Value:F1}s";
            _settings.Save();
        };

        durationSliderPanel.Children.Add(durationSlider);
        durationSliderPanel.Children.Add(durationValue);
        durationPanel.Children.Add(durationSliderPanel);
        panel.Children.Add(durationPanel);

        // Time Format
        var formatCombo = new ComboBox
        {
            Header = LocalizationHelper.GetString("Settings_TimeFormat"),
            Width = 250
        };
        formatCombo.Items.Add(new ComboBoxItem { Content = "YYYY-MM-DD HH:mm:ss", Tag = "yyyy-MM-dd HH:mm:ss" });
        formatCombo.Items.Add(new ComboBoxItem { Content = "MM/DD/YYYY HH:mm:ss", Tag = "MM/dd/yyyy HH:mm:ss" });
        formatCombo.Items.Add(new ComboBoxItem { Content = "DD/MM/YYYY HH:mm:ss", Tag = "dd/MM/yyyy HH:mm:ss" });
        formatCombo.Items.Add(new ComboBoxItem { Content = LocalizationHelper.GetString("Format_Chinese"), Tag = "yyyy年MM月dd日 HH:mm:ss" });
        formatCombo.Items.Add(new ComboBoxItem { Content = "YYYYMMDD HHmmss", Tag = "yyyyMMdd HHmmss" });

        for (int i = 0; i < formatCombo.Items.Count; i++)
        {
            if (formatCombo.Items[i] is ComboBoxItem item && item.Tag?.ToString() == _settings.TimeFormat)
            {
                formatCombo.SelectedIndex = i;
                break;
            }
        }

        formatCombo.SelectionChanged += (s, e) =>
        {
            if (formatCombo.SelectedItem is ComboBoxItem item && item.Tag is string format)
            {
                _settings.TimeFormat = format;
                _settings.Save();
            }
        };
        panel.Children.Add(formatCombo);

        return new ScrollViewer { Content = panel };
    }

    private UIElement CreateFilterPage()
    {
        var panel = new StackPanel
        {
            Spacing = 16,
            Padding = new Thickness(24)
        };

        var headerText = new TextBlock
        {
            Text = LocalizationHelper.GetString("Settings_YearRange"),
            Style = (Style)Application.Current.Resources["SubtitleTextBlockStyle"]
        };
        panel.Children.Add(headerText);

        var descriptionText = new TextBlock
        {
            Text = LocalizationHelper.GetString("Settings_YearRange_Description"),
            Foreground = (Microsoft.UI.Xaml.Media.Brush)Application.Current.Resources["TextFillColorSecondaryBrush"]
        };
        panel.Children.Add(descriptionText);

        var yearPanel = new StackPanel
        {
            Orientation = Orientation.Horizontal,
            Spacing = 24,
            Margin = new Thickness(0, 16, 0, 0)
        };

        // Min Year
        var minYearPanel = new StackPanel { Spacing = 8 };
        var minYearLabel = new TextBlock { Text = LocalizationHelper.GetString("Settings_MinYear") };
        var minYearBox = new NumberBox
        {
            Value = _settings.MinYear,
            Minimum = 1970,
            Maximum = 2100,
            SpinButtonPlacementMode = NumberBoxSpinButtonPlacementMode.Compact
        };
        minYearBox.ValueChanged += (s, e) =>
        {
            if (!double.IsNaN(minYearBox.Value))
            {
                _settings.MinYear = (int)minYearBox.Value;
                _settings.Save();
            }
        };
        minYearPanel.Children.Add(minYearLabel);
        minYearPanel.Children.Add(minYearBox);
        yearPanel.Children.Add(minYearPanel);

        // Max Year
        var maxYearPanel = new StackPanel { Spacing = 8 };
        var maxYearLabel = new TextBlock { Text = LocalizationHelper.GetString("Settings_MaxYear") };
        var maxYearBox = new NumberBox
        {
            Value = _settings.MaxYear,
            Minimum = 1970,
            Maximum = 2100,
            SpinButtonPlacementMode = NumberBoxSpinButtonPlacementMode.Compact
        };
        maxYearBox.ValueChanged += (s, e) =>
        {
            if (!double.IsNaN(maxYearBox.Value))
            {
                _settings.MaxYear = (int)maxYearBox.Value;
                _settings.Save();
            }
        };
        maxYearPanel.Children.Add(maxYearLabel);
        maxYearPanel.Children.Add(maxYearBox);
        yearPanel.Children.Add(maxYearPanel);

        panel.Children.Add(yearPanel);

        return new ScrollViewer { Content = panel };
    }

    private UIElement CreateAboutPage()
    {
        var panel = new StackPanel
        {
            Spacing = 16,
            Padding = new Thickness(24),
            HorizontalAlignment = HorizontalAlignment.Center,
            VerticalAlignment = VerticalAlignment.Center
        };

        var icon = new FontIcon
        {
            Glyph = "\uE823",
            FontSize = 64,
            Foreground = (Microsoft.UI.Xaml.Media.Brush)Application.Current.Resources["AccentFillColorDefaultBrush"]
        };
        panel.Children.Add(icon);

        var titleText = new TextBlock
        {
            Text = "Timesdump",
            Style = (Style)Application.Current.Resources["TitleTextBlockStyle"],
            HorizontalAlignment = HorizontalAlignment.Center
        };
        panel.Children.Add(titleText);

        var taglineText = new TextBlock
        {
            Text = LocalizationHelper.GetString("About_Tagline"),
            Foreground = (Microsoft.UI.Xaml.Media.Brush)Application.Current.Resources["TextFillColorSecondaryBrush"],
            HorizontalAlignment = HorizontalAlignment.Center
        };
        panel.Children.Add(taglineText);

        var versionText = new TextBlock
        {
            Text = "Version 1.0.0",
            Foreground = (Microsoft.UI.Xaml.Media.Brush)Application.Current.Resources["TextFillColorTertiaryBrush"],
            HorizontalAlignment = HorizontalAlignment.Center,
            Margin = new Thickness(0, 16, 0, 0)
        };
        panel.Children.Add(versionText);

        var copyrightText = new TextBlock
        {
            Text = "© 2024 Timesdump",
            Foreground = (Microsoft.UI.Xaml.Media.Brush)Application.Current.Resources["TextFillColorTertiaryBrush"],
            HorizontalAlignment = HorizontalAlignment.Center,
            FontSize = 12
        };
        panel.Children.Add(copyrightText);

        return panel;
    }
}
