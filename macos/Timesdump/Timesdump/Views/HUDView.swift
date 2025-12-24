import SwiftUI

struct HUDView: View {
    let result: TimestampResult
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // Main time display
            Text(result.formattedTime)
                .font(.system(size: 20, weight: .medium, design: .monospaced))
                .foregroundColor(.primary)
            
            // Metadata
            HStack(spacing: 16) {
                Label(result.originalValue, systemImage: "number")
                    .font(.system(size: 12))
                    .foregroundColor(.secondary)
                
                Text("(\(result.unitDescription))")
                    .font(.system(size: 11))
                    .foregroundColor(.secondary)
            }
            
            // Hint
            Text(NSLocalizedString("hud.clickToCopy", comment: "Click to copy"))
                .font(.system(size: 10))
                .foregroundColor(.secondary.opacity(0.7))
        }
        .padding(.horizontal, 20)
        .padding(.vertical, 16)
        .frame(minWidth: 280, alignment: .leading)
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(.ultraThinMaterial)
                .overlay(
                    RoundedRectangle(cornerRadius: 12)
                        .strokeBorder(Color.primary.opacity(0.1), lineWidth: 0.5)
                )
        )
        .shadow(color: .black.opacity(0.15), radius: 12, x: 0, y: 4)
    }
}

#Preview("Light Mode") {
    HUDView(result: TimestampResult(
        originalValue: "1703472000",
        timestamp: 1703472000,
        date: Date(timeIntervalSince1970: 1703472000),
        format: "yyyy-MM-dd HH:mm:ss",
        isMilliseconds: false
    ))
    .padding()
    .preferredColorScheme(.light)
}

#Preview("Dark Mode") {
    HUDView(result: TimestampResult(
        originalValue: "1703472000000",
        timestamp: 1703472000,
        date: Date(timeIntervalSince1970: 1703472000),
        format: "yyyy-MM-dd HH:mm:ss",
        isMilliseconds: true
    ))
    .padding()
    .preferredColorScheme(.dark)
}
