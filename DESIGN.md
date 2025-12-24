# Timesdump 产品需求文档 (PRD)

## 1. 产品概述 (Overview)
* **产品名称**：Timesdump
* **产品标语**：The Silent Timestamp Decoder.
* **支持平台**：**macOS 14.0 (Sonoma) 及以上**。
    * *兼容范围*：完全支持 macOS 14 (Sonoma), macOS 15 (Sequoia), macOS 26 (Tahoe) 及后续版本。
* **核心理念**：
    1.  **极简 (Minimalist)**：无Dock图标，菜单栏驻留，无主窗口。
    2.  **静默 (Silent)**：无声、无通知、不抢焦点。
    3.  **原生 (Native)**：完美融入 macOS 生态的视觉体验。

## 2. 核心功能逻辑 (Core Logic)

### 2.1 剪贴板监听与解析管道
后台静默监听系统剪贴板（仅文本类型 `public.utf8-plain-text`）。数据流处理如下：

1.  **触发**：剪贴板内容变化 `changeCount` 增加。
2.  **清洗**：去除首尾空格 (`trim`)。
3.  **预判**：
    * 若包含非数字字符：静默丢弃。
    * 若为纯数字：进入下一步。
4.  **单位自适应**：
    * 数值长度 $\le$ 10位：按 **秒 (s)** 解析。
    * 数值长度 $>$ 10位：按 **毫秒 (ms)** 解析。
5.  **范围过滤器 (Filter)**：
    * 将数值转换为年份 `Year`。
    * **判定**：`MinYear` (配置) $\le$ `Year` $\le$ `MaxYear` (配置)。
    * *结果*：不满足条件的（如手机号、验证码）直接丢弃，不执行任何 UI 操作。

### 2.2 多语言支持 (Localization)
* **策略**：App 启动时检测系统语言 `Locale.current`。
* **支持列表**：
    1.  **简体中文 (zh-Hans)** - 优先匹配
    2.  **English (en)** - 默认兜底 (Fallback)
* **覆盖范围**：菜单栏文案、设置面板、HUD 浮窗中的辅助文字。

## 3. UI/UX 设计规范 (Design Specifications)

本应用无主窗口，所有交互通过 **HUD 浮窗** 和 **菜单栏** 完成。设计需严格遵循 Apple Human Interface Guidelines (HIG)。

### 3.1 结果浮窗 (HUD Popover)
这是一个悬浮的、非交互式的（不抢焦点）、极简的信息展示卡片。

* **形态与材质**：
    * **形状**：圆角矩形，Corner Radius **12pt**。
    * **背景**：使用 SwiftUI `UltraThinMaterial` (高斯模糊磨砂效果)，使背景色能通过模糊透出，呈现高级的通透感。
    * **适配**：自动适配 Light/Dark Mode。
    * **阴影**：添加轻微的 Drop Shadow (Radius: 6, Y: 3, Opacity: 0.12) 以确保在白色背景应用上也能清晰分辨。
    * **边框**：1px 的极细描边，颜色为 `Color.primary.opacity(0.1)` (Inner Stroke)，增加精致感。

* **布局与排版**：
    * **内边距 (Padding)**：水平 16pt，垂直 12pt。
    * **主标题 (转换后时间)**：
        * 字体：**SF Mono** (等宽字体) 或 **SF Pro Rounded**。
        * 字号：**15pt**。
        * 字重：**Semibold**。
        * 颜色：`Color.primary` (高对比度)。
    * **副标题 (元数据)**：
        * 位置：主标题正下方，间距 3pt。
        * 字体：**SF Pro**。
        * 字号：**11pt**。
        * 字重：**Regular**。
        * 颜色：`Color.secondary` (次级灰度)。
        * 内容格式：`[图标] 本地时间 · [毫秒/秒]`
            * *中文示例*：`ClockIcon 本地时间 · 毫秒`
            * *英文示例*：`ClockIcon Local Time · ms`

* **动效 (Animation)**：
    * **出现**：`.spring(response: 0.35, dampingFraction: 0.75)`。轻微的弹簧效果，从原本大小的 95% 放大到 100%。
    * **消失**：`.easeOut(duration: 0.2)`。平滑淡出。
    * **点击反馈**：点击瞬间，卡片缩小至 98% (`scaleEffect`) 并恢复。

### 3.2 菜单栏 (Menu Bar)
* **图标**：使用 SF Symbols 中的 `clock` 或 `arrow.triangle.2.circlepath.clock` 变体。
* **菜单结构**：
    * `State: Running` (灰色，指示状态)
    * `Pause Monitoring` (暂停/恢复开关)
    * ---
    * `History` (子菜单：显示最近 5 条成功记录)
    * ---
    * `Settings...`
    * `Quit`

## 4. 设置面板 (Settings Requirements)

设置面板应使用标准的 macOS App 设置风格（TabView 结构）。

### Tab 1: 通用 (General)
* **启动 (Launch)**：
    * `[Checkbox]` 开机自动启动 (Launch at login)
* **显示 (Display)**：
    * **弹窗位置 (Position)**：`[Picker]` 跟随鼠标 (Follow Mouse) | 屏幕右上角 (Top-Right)
    * **弹窗停留 (Duration)**：`[Slider]` **1.5s - 10.0s** (默认 3.0s)。
        * *交互*：滑动时滑块旁显示具体秒数 (例如 "3.0s")。
    * **时间格式 (Format)**：`[Menu]` `YYYY-MM-DD HH:mm:ss` (默认) | `ISO8601` | `自定义...`

### Tab 2: 过滤 (Filter)
* **年份限制 (Year Range)**：
    * `Min Year [TextField]` - `Max Year [TextField]`
    * *默认值*：1990 - 2050
    * *逻辑*：在此范围之外的数字将被视为无效数据，不触发弹窗。

## 5. 交互行为规范 (Interaction Specs)

1.  **焦点保护 (Focus Safety)**：
    * **高优先级**：弹窗出现时，**严禁**窃取当前应用（如 VS Code, Terminal）的焦点。
    * 实现参考：`NSPanel` with `.nonactivatingPanel` style mask。
    * 窗口层级：`NSWindow.Level.floating`。

2.  **点击复制**：
    * 用户点击 HUD 任意区域 -> 将**格式化后的时间字符串**写入剪贴板。
    * HUD 立即执行消失动画。
    * **反馈**：仅视觉反馈（缩放/闪烁），**无**系统提示音。

3.  **鼠标悬停 (Hover)**：
    * 当鼠标指针位于 HUD 区域内时，自动消失计时器**暂停**。
    * 鼠标移出后，重新开始倒计时。

## 6. 文案键值表 (Localization Keys)

| Key | English (Default) | Chinese (Simplified) |
| :--- | :--- | :--- |
| `menu.status.running` | Status: Monitoring | 状态：监听中 |
| `menu.status.paused` | Status: Paused | 状态：已暂停 |
| `hud.label.local` | Local Time | 本地时间 |
| `settings.tab.general` | General | 通用 |
| `settings.duration` | Display Duration | 弹窗停留时间 |
| `settings.year_range` | Year Range | 年份范围 |
| `settings.launch_at_login` | Launch at login | 开机自动启动 |
