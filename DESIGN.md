# Timesdump 产品需求文档 (PRD)

## 1. 产品概述 (Overview)

* **产品名称**：Timesdump
* **产品标语**：The Silent Timestamp Decoder.
* **项目结构**：单一代码仓库 (Monorepo)，根目录下分设 `/macos` 和 `/windows` 两个独立工程目录。
* **核心理念**：
    1.  **极简 (Minimalist)**：无任务栏/Dock图标，后台驻留，无主窗口。
    2.  **静默 (Silent)**：无声、无通知、不抢焦点。
    3.  **原生 (Native)**：在双端均提供最符合该系统直觉的视觉与交互体验。

## 2. 平台与环境要求 (Platform Requirements)

### 2.1 macOS
* **系统版本**：macOS 14.0 (Sonoma) 及以上 (兼容最新三个大版本)。
* **开发语言**：Swift 6。
* **UI 框架**：SwiftUI (主) + AppKit (窗口管理)。
* **设计规范**：Apple Human Interface Guidelines (HIG)。

### 2.2 Windows
* **系统版本**：Windows 10 (Build 19041+) 及 Windows 11。
* **开发语言**：C# (.NET 10)。
* **UI 框架**：WinUI 3 (Windows App SDK)。
* **设计规范**：Microsoft Fluent Design System。

## 3. 核心功能逻辑 (Core Logic) - 双端通用

后台静默监听系统剪贴板（仅监听纯文本类型）。当内容变更时，执行以下**严格**的数据流处理：

1.  **数据清洗**：去除首尾空格。
2.  **格式校验**：
    * 若包含非数字字符：静默丢弃（不执行任何操作）。
    * 若为纯数字：进入下一步。
3.  **单位自适应**：
    * 数值长度 $\le$ 10位：按 **秒 (s)** 解析。
    * 数值长度 $>$ 10位：按 **毫秒 (ms)** 解析。
4.  **范围过滤器 (Filter)**：
    * 将数值转换为年份。
    * **判定逻辑**：仅当 `MinYear` (配置) $\le$ `年份` $\le$ `MaxYear` (配置) 时视为有效。
    * **结果**：不满足条件的（如手机号、验证码）直接丢弃。

## 4. UI/UX 设计规范 (UI Specifications)

### 4.1 结果浮窗 (The Ghost HUD)

浮窗是产品的核心交互界面，必须保证**不抢占焦点 (Non-activating)**。

#### A. macOS 端设计
* **风格**：Spotlight 风格或系统音量 HUD 风格。
* **材质**：采用 `UltraThinMaterial` (高斯模糊)，支持深色/浅色模式自适应。
* **形状**：圆角矩形 (Corner Radius 12pt)。
* **边框**：极细的内描边，增加通透感。
* **阴影**：轻微的 Drop Shadow 确保在浅色背景上的辨识度。
* **排版**：
    * 主标题（时间）使用 SF Mono 或 SF Pro Rounded 字体。
    * 副标题（元数据）使用 SF Pro 字体，字号较小，灰色。

#### B. Windows 端设计
* **风格**：Fluent Design 风格，类似系统通知 Toast 或现代 Flyout。
* **材质**：
    * Windows 11：采用 `Mica Alt` 或 `Acrylic` 材质，根据系统设置自适应。
    * Windows 10：采用 `Acrylic` (亚克力) 半透明效果。
* **形状**：圆角矩形 (Windows 11 标准圆角)。
* **主题**：严格跟随系统深色/浅色主题。
* **排版**：
    * 主标题使用 `Segoe UI Variable Display` 字体。
    * 副标题使用 `Segoe UI Variable Text` 字体。
* **细节**：窗口边缘应有 1px 的高亮描边 (Accent Color 或 灰度)，符合 WinUI 3 规范。

### 4.2 托盘/菜单栏交互 (Tray Integration)

#### A. macOS 端 (Menu Bar)
* **位置**：顶部菜单栏右侧。
* **图标**：简洁的线条风格图标 (SF Symbols)。
* **菜单项**：标准下拉菜单，包含状态、暂停开关、历史记录(子菜单)、设置、退出。

#### B. Windows 端 (System Tray)
* **位置**：任务栏右下角系统托盘区。
* **图标**：符合 Windows 风格的线条图标 (`.ico`资源)。
* **交互**：
    * **右键单击**：弹出标准 Context Menu（菜单项内容与 macOS 一致）。
    * **左键单击/双击**：打开设置窗口。

## 5. 设置面板 (Settings)

设置面板需使用各平台原生的控件与布局方式。

* **功能项 (双端一致)**：
    1.  **通用**：
        * 开机自启开关。
        * 弹窗位置选择（跟随鼠标 / 屏幕固定角落）。
        * 弹窗停留时长滑动条（1.5s - 10s）。
        * 时间格式选择（YYYY-MM-DD HH:mm:ss 等）。
    2.  **过滤**：
        * 年份范围输入框 (Min - Max)。

* **UI 实现要求**：
    * **macOS**：标准的 `Settings` Scene，使用 TabView 分页。
    * **Windows**：独立的 WinUI 3 窗口，使用 `NavigationView` (左侧导航) 或顶部 `Pivot` 布局。

## 6. 交互行为规范 (Interaction Specs)

### 6.1 焦点保护 (Focus Safety - 最高优先级)
* **核心要求**：当浮窗弹出时，**严禁**窃取当前活动窗口（如 IDE、浏览器、编辑器）的输入焦点。
* **预期体验**：用户正在打字时复制了时间戳，浮窗出现，用户的打字流不应中断，光标不应丢失。

### 6.2 复制操作
* **触发**：点击浮窗任意区域。
* **行为**：将格式化后的时间字符串写入剪贴板，浮窗立即消失。
* **反馈**：仅视觉反馈（如缩放或背景闪烁），**无**提示音。

### 6.3 鼠标悬停
* **逻辑**：鼠标进入浮窗区域，自动消失倒计时暂停；鼠标移出，倒计时重置。

## 7. 国际化 (Localization)

* **策略**：根据操作系统当前语言自动匹配，不支持应用内手动切换语言。
* **支持语言**：
    1.  简体中文 (zh-Hans / zh-CN) - 优先
    2.  English (en-US) - 兜底
* **实现要求**：
    * **macOS**：使用 String Catalogs (`.xcstrings`) 或传统的 `.strings` 文件。
    * **Windows**：使用 `.resw` 资源文件进行多语言管理。

## 附录：技术实现建议 (Implementation Suggestions)

*(注：本节仅作为开发参考，非强制约束，旨在提示关键技术难点)*

### macOS 端建议
1.  **窗口管理**：建议使用 `NSPanel` 子类，配置 `.nonactivatingPanel` style mask 和 `.floating` level 来实现不抢焦点且置顶。
2.  **全屏支持**：配置 collection behavior 以支持 `.canJoinAllSpaces` 和 `.fullScreenAuxiliary`。
3.  **并发模型**：利用 Swift 6 的 Swift Concurrency (`async/await`, `Actors`) 处理剪贴板监听和数据解析，确保主线程流畅。

### Windows 端建议
1.  **窗口管理**：WinUI 3 默认窗口会抢焦点。建议通过 P/Invoke 调用 Win32 API (`SetWindowPos` 配合 `SWP_NOACTIVATE` 标志，或修改 Window Style 添加 `WS_EX_NOACTIVATE`) 来实现无焦点窗口。
2.  **位置计算**：需考虑不同 DPI 缩放比例下的坐标转换，确保跟随鼠标位置准确。
3.  **材质效果**：使用 Windows App SDK 提供的 `SystemBackdrop` API 应用 Mica 或 Acrylic 效果。
4.  **剪贴板监听**：WinUI 3 原生剪贴板 API 可能受限，必要时可引用 Win32 API 轮询或注册剪贴板钩子 (Clipboard Format Listener)。
