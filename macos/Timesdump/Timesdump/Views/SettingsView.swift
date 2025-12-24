import SwiftUI
import ServiceManagement

struct SettingsView: View {
    @EnvironmentObject var appState: AppState
    
    var body: some View {
        TabView {
            GeneralSettingsView()
                .tabItem {
                    Label(NSLocalizedString("settings.tab.general", comment: "General"), systemImage: "gear")
                }
                .environmentObject(appState)
            
            FilterSettingsView()
                .tabItem {
                    Label(NSLocalizedString("settings.tab.filter", comment: "Filter"), systemImage: "line.3.horizontal.decrease.circle")
                }
                .environmentObject(appState)
            
            AboutView()
                .tabItem {
                    Label(NSLocalizedString("settings.tab.about", comment: "About"), systemImage: "info.circle")
                }
        }
        .frame(width: 480, height: 320)
        .padding()
    }
}

struct GeneralSettingsView: View {
    @EnvironmentObject var appState: AppState
    
    var body: some View {
        Form {
            Section {
                Toggle(isOn: $appState.launchAtLogin) {
                    Text(NSLocalizedString("settings.launchAtLogin", comment: "Launch at Login"))
                }
                .onChange(of: appState.launchAtLogin) { _, newValue in
                    updateLaunchAtLogin(enabled: newValue)
                }
            }
            
            Section {
                Picker(NSLocalizedString("settings.hudPosition", comment: "HUD Position"), selection: $appState.hudPosition) {
                    ForEach(AppState.HUDPosition.allCases, id: \.self) { position in
                        Text(position.localizedName).tag(position)
                    }
                }
                .pickerStyle(.menu)
                
                VStack(alignment: .leading) {
                    Text(NSLocalizedString("settings.displayDuration", comment: "Display Duration"))
                    HStack {
                        Slider(value: $appState.hudDisplayDuration, in: 1.5...10.0, step: 0.5)
                        Text(String(format: "%.1fs", appState.hudDisplayDuration))
                            .font(.system(.body, design: .monospaced))
                            .frame(width: 50)
                    }
                }
            }
            
            Section {
                Picker(NSLocalizedString("settings.timeFormat", comment: "Time Format"), selection: $appState.timeFormat) {
                    ForEach(AppState.TimeFormat.allCases, id: \.self) { format in
                        Text(format.localizedName).tag(format)
                    }
                }
                .pickerStyle(.menu)
            }
        }
        .formStyle(.grouped)
    }
    
    private func updateLaunchAtLogin(enabled: Bool) {
        do {
            if enabled {
                try SMAppService.mainApp.register()
            } else {
                try SMAppService.mainApp.unregister()
            }
        } catch {
            print("Failed to update launch at login: \(error)")
        }
    }
}

struct FilterSettingsView: View {
    @EnvironmentObject var appState: AppState
    
    var body: some View {
        Form {
            Section {
                VStack(alignment: .leading, spacing: 12) {
                    Text(NSLocalizedString("settings.yearRange", comment: "Valid Year Range"))
                        .font(.headline)
                    
                    Text(NSLocalizedString("settings.yearRange.description", comment: "Timestamps outside this range will be ignored"))
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    HStack(spacing: 20) {
                        VStack(alignment: .leading) {
                            Text(NSLocalizedString("settings.minYear", comment: "Min Year"))
                                .font(.caption)
                            TextField("", value: $appState.minYear, format: .number)
                                .textFieldStyle(.roundedBorder)
                                .frame(width: 100)
                        }
                        
                        VStack(alignment: .leading) {
                            Text(NSLocalizedString("settings.maxYear", comment: "Max Year"))
                                .font(.caption)
                            TextField("", value: $appState.maxYear, format: .number)
                                .textFieldStyle(.roundedBorder)
                                .frame(width: 100)
                        }
                    }
                }
            }
        }
        .formStyle(.grouped)
    }
}

struct AboutView: View {
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "clock.badge.checkmark")
                .font(.system(size: 64))
                .foregroundColor(.accentColor)
            
            Text("Timesdump")
                .font(.title)
                .fontWeight(.semibold)
            
            Text(NSLocalizedString("about.tagline", comment: "The Silent Timestamp Decoder"))
                .font(.subheadline)
                .foregroundColor(.secondary)
            
            if let version = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String,
               let build = Bundle.main.infoDictionary?["CFBundleVersion"] as? String {
                Text("Version \(version) (\(build))")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Text("© 2024 Timesdump")
                .font(.caption2)
                .foregroundColor(.secondary)
        }
        .padding()
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

#Preview {
    SettingsView()
        .environmentObject(AppState())
}
