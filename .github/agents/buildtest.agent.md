# Build & Test Agent

用于指导 Agent 在 Windows 和 macOS 上设置测试环境、编译项目、运行测试以及打包应用。

## 项目概述

Timesdump 是一个跨平台的时间戳解码工具，支持 macOS 和 Windows 平台。

- **macOS 版本**: Swift 6 + SwiftUI/AppKit
- **Windows 版本**: C# .NET 10 + WinUI 3

## 环境要求

### macOS 环境

- **操作系统**: macOS 14.0 (Sonoma) 或更高版本
- **Xcode**: 15.0 或更高版本
- **Swift**: 6.0

### Windows 环境

- **操作系统**: Windows 10 (Build 19041+) 或 Windows 11
- **.NET SDK**: 10.0 (preview)
- **Visual Studio 2022**: 可选，需安装 WinUI 3 工作负载

---

## Windows 平台

### 安装测试环境

1. **安装 .NET 10 SDK (Preview)**

   ```powershell
   # 使用 winget 安装
   winget install Microsoft.DotNet.SDK.Preview

   # 或者从官网下载安装
   # https://dotnet.microsoft.com/download/dotnet/10.0
   ```

2. **验证安装**

   ```powershell
   dotnet --version
   # 应该显示 10.0.x 版本
   ```

3. **安装 Visual Studio 2022 (可选)**

   如需使用 IDE 开发，请安装 Visual Studio 2022 并选择以下工作负载：
   - .NET Desktop Development
   - Windows App SDK C# Templates (WinUI 3)

### 编译项目

```powershell
cd windows

# 还原依赖
dotnet restore Timesdump.sln

# 编译 Release 版本
dotnet build Timesdump.sln --configuration Release

# 编译 Debug 版本 (用于调试)
dotnet build Timesdump.sln --configuration Debug
```

### 运行测试

```powershell
cd windows

# 运行所有测试
dotnet test Timesdump.Tests/Timesdump.Tests.csproj --configuration Release --verbosity normal

# 运行测试并生成代码覆盖率报告
dotnet test Timesdump.Tests/Timesdump.Tests.csproj --collect:"XPlat Code Coverage"
```

### 打包发布

```powershell
cd windows

# 发布为独立应用 (x64)
dotnet publish Timesdump/Timesdump.csproj -c Release -r win-x64 --self-contained true

# 发布为独立应用 (x86)
dotnet publish Timesdump/Timesdump.csproj -c Release -r win-x86 --self-contained true

# 发布为独立应用 (ARM64)
dotnet publish Timesdump/Timesdump.csproj -c Release -r win-arm64 --self-contained true
```

发布后的文件位于 `windows/Timesdump/bin/Release/net10.0-windows10.0.19041.0/{rid}/publish/` 目录。

---

## macOS 平台

### 安装测试环境

1. **安装 Xcode**

   ```bash
   # 从 App Store 安装 Xcode 15.0 或更高版本
   # 或使用 xcode-select 安装命令行工具
   xcode-select --install
   ```

2. **选择 Xcode 版本 (如果安装了多个版本)**

   ```bash
   # 查看已安装的 Xcode 版本
   ls /Applications/ | grep Xcode

   # 选择要使用的 Xcode 版本 (示例)
   sudo xcode-select -s /Applications/Xcode_15.4.app/Contents/Developer
   ```

3. **验证安装**

   ```bash
   xcodebuild -version
   # 应该显示 Xcode 15.x 或更高版本
   
   swift --version
   # 应该显示 Swift 6.x
   ```

### 编译项目

```bash
cd macos/Timesdump

# 编译 Release 版本
xcodebuild build \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -configuration Release

# 编译 Release 版本 (无代码签名，用于 CI/CD)
xcodebuild build \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -configuration Release \
  CODE_SIGN_IDENTITY="" \
  CODE_SIGNING_REQUIRED=NO \
  CODE_SIGNING_ALLOWED=NO

# 编译 Debug 版本 (用于调试)
xcodebuild build \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -configuration Debug
```

### 运行测试

```bash
cd macos/Timesdump

# 运行所有测试
xcodebuild test \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS'

# 运行测试 (无代码签名，用于 CI/CD)
xcodebuild test \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  CODE_SIGN_IDENTITY="" \
  CODE_SIGNING_REQUIRED=NO \
  CODE_SIGNING_ALLOWED=NO
```

### 打包发布

```bash
cd macos/Timesdump

# 归档应用
xcodebuild archive \
  -project Timesdump.xcodeproj \
  -scheme Timesdump \
  -destination 'platform=macOS' \
  -archivePath build/Timesdump.xcarchive \
  -configuration Release

# 导出应用 (需要配置 ExportOptions.plist)
xcodebuild -exportArchive \
  -archivePath build/Timesdump.xcarchive \
  -exportPath build/export \
  -exportOptionsPlist ExportOptions.plist
```

打包后的 `.app` 文件位于 `build/export/` 目录。

---

## CI/CD 工作流

项目已配置 GitHub Actions 工作流 (`.github/workflows/build.yml`)，会在以下情况自动触发：

- Push 到 `main` 或 `develop` 分支
- Pull Request 到 `main` 或 `develop` 分支

工作流会自动执行：
1. macOS 编译和测试
2. Windows 编译和测试

---

## 故障排除

### Windows 常见问题

1. **dotnet 命令未找到**
   - 确保 .NET SDK 已正确安装
   - 尝试重新启动终端或 IDE

2. **WinUI 3 组件缺失**
   - 确保 Windows App SDK 已正确安装
   - 尝试运行 `dotnet restore` 重新还原依赖

3. **构建平台错误**
   - 默认支持 x86、x64 和 ARM64 平台
   - 可以通过 `-p:Platform=x64` 指定平台

### macOS 常见问题

1. **xcodebuild 命令未找到**
   - 运行 `xcode-select --install` 安装命令行工具
   - 确保 Xcode 已从 App Store 安装

2. **代码签名错误**
   - 在 CI/CD 环境中添加 `CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO CODE_SIGNING_ALLOWED=NO`
   - 本地开发时确保已配置有效的开发者证书

3. **Swift 版本不兼容**
   - 确保使用 Xcode 15.0 或更高版本
   - 项目需要 Swift 6 支持
