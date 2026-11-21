# nsh-foundation 组件进度

更新于 2026-11-21。已发布 **25 / 66**，待开始 **41 / 66**。下一项是 **Direction（#25）**。[实施计划](PLAN.md)

## 已交付基线

- **Drawer** 含 9 个交互示例及 Preview / Rust / TSX 标签。面板默认预留 8 px 外边距，示例正文、标题和底部操作对齐在 16 px 内边距。手柄按抽屉方向放在可拖动的内侧边缘。滑入用时 450 ms，普通滑出用时 400 ms，退出时先缓慢起步再滑出。遮罩随拖动位移渐淡，短拖松手后面板和遮罩恢复。甩动关闭按剩余距离和松手速度调整时长，退出后卸载。Chromium 已验证 1280 × 900 和 390 × 844 的间距、进出场位移和模拟触摸回弹。[组件源码](src/drawer.rs) · [TSX 参考](references/drawer-tsx/)
- **线上版本**为 32，发布提交为 `fc5f352`，新增 Dialog。Web 打包和 WASM 内容校验已完成。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#dialog)
- **基线复测**于 2026-11-21 通过 `cargo fmt --check` 和 `cargo test --offline --locked`，共 68 项 Rust 测试。
- **设计参考**已固定 shadcn/ui 源码版本，Obra 预览图已保存。可编辑的 Figma 原文件仍未获取。[参考资料记录](references/README.md)
- **Accordion** 已发布于 v7（`7abbcb5`），含 7 个官方示例及 Preview / Rust / TSX。支持默认与受控状态、单项与多项展开、禁用、方向键和 RTL。15 项 Rust 测试、Web 打包及 WASM 校验已通过，浏览器视觉核验受预览端口限制。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#accordion) · [组件源码](src/accordion.rs) · [TSX 参考](references/accordion-tsx/)
- **Alert** 已发布于 v8（`1e44dc4`），含 6 个官方示例及 Preview / Rust / TSX。支持普通和 destructive 样式、自定义主题颜色、操作按钮和 RTL。无头测试覆盖移动端交互、复制代码和历史导航。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#alert) · [组件源码](src/alert.rs) · [TSX 参考](references/alert-tsx/)
- **Alert Dialog** 已发布于 v9（`6090b69`），含 7 个预览及 Preview / Rust / TSX。无头测试覆盖桌面和移动端尺寸、打开后聚焦取消、Escape 关闭和显式确认。Rust 预览的媒体徽章使用字符图标。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#alert-dialog) · [组件源码](src/alert_dialog.rs)
- **Aspect Ratio** 已发布于 v10（`b15b302`），含 4 个预览及 Preview / Rust / TSX。无头测试覆盖桌面和移动端比例、响应式比例更新及无效比例的方形回退。示例图片随站点打包。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#aspect-ratio) · [组件源码](src/aspect_ratio.rs)
- **Attachment** 已发布于 v11（`68ff01e`），含 6 个预览及 Preview / Rust / TSX。覆盖 5 种状态、3 种尺寸、键盘横向滚动、预览与独立操作。重试是本地演示。组件导航改用可滚动菜单。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#attachment) · [组件源码](src/attachment.rs)
- **Avatar** 已发布于 v12（`0bd7a00`），含 10 个预览及 Preview / Rust / TSX。测试覆盖图片失败回退、状态徽章的可访问名称、下拉菜单选择与 Escape 关闭。头像图片随站点打包，下拉选项只更新预览反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#avatar) · [组件源码](src/avatar.rs)
- **Badge** 已发布于 v13（`034d1b7`），含 7 个预览及 Preview / Rust / TSX。支持变体、主题自定义颜色、内联图标、加载动画和链接。测试覆盖链接打开和 RTL 图标顺序。链接示例指向上游文档的 Link 小节。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#badge) · [组件源码](src/badge.rs)
- **Breadcrumb** 已发布于 v14（`3cf8695`），含 7 个预览及 Preview / Rust / TSX。测试覆盖当前页语义、折叠菜单、打开链接和 Escape 关闭。Rust 示例导航指向上游文档及相关页面。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#breadcrumb) · [组件源码](src/breadcrumb.rs)
- **Bubble** 已发布于 v15（`4cb00ec`），含 9 个预览及 Preview / Rust / TSX。支持七种样式、两侧对齐、Markdown、反应按钮、展开、Tooltip 和 Popover。无头测试覆盖展开、点击已读提示及关闭详情。命令示例仅更新本地反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#bubble) · [组件源码](src/bubble.rs)
- **Button** 已发布于 v16（`d4ea8a2`），含 15 个预览及 Preview / Rust / TSX。支持 6 种变体、8 种尺寸、响应式加载与禁用、图标、链接和 RTL。无头测试覆盖键盘激活、禁用、链接与组合菜单。组合布局是此页的演示，Button Group 组件排在下一项。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#button) · [组件源码](src/button.rs)
- **Button Group** 已发布于 v17（`b55977a`），含 12 个预览及 Preview / Rust / TSX。支持横竖排列、连接圆角、嵌套、分隔符、文字和 RTL。无头测试覆盖 Tab 激活、输入查询、币种选择及弹层关闭。金额和任务提交仅更新预览反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#button-group) · [组件源码](src/button_group.rs)
- **Calendar** 已发布于 v18（`312d026`），含 12 个预览及 Preview / Rust / TSX。支持日期与范围、单月或双月、禁用日期、周数、波斯历、RTL 和本地日期。月份和年份由标题按钮进入选择网格。时区辅助 API 支持设备本地时区与固定偏移。测试覆盖波斯历转换、键盘导航、预约禁用、范围选择和跨日边界。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#calendar) · [组件源码](src/calendar.rs)
- **Card** 已发布于 v20（`09deb05`），含 6 个预览及 Preview / Rust / TSX。支持组合部件、普通与紧凑尺寸、动态间距、通栏正文和封面、阿拉伯语及希伯来语 RTL。无头测试覆盖登录验证、间距更新、通栏布局与 RTL 动作位置。账号操作和表单提交仅显示本地反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#card) · [组件源码](src/card.rs) · [TSX 参考](references/card-tsx/)
- **Carousel** 已发布于 v21（`5198b2e`），含 7 个预览及 Preview / Rust / TSX。支持横竖排列、响应式尺寸、可配间距、索引信号、循环和自动播放。Rust 示例使用原生滚动吸附和 Rust 定时器。无头测试覆盖拖动、边界按钮、垂直方向键、悬停暂停和局部 RTL。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#carousel) · [组件源码](src/carousel.rs) · [TSX 参考](references/carousel-tsx/)
- **Chart** 已发布于 v22（`412bbdf`），含 8 个预览及 Preview / Rust / TSX。纯 Rust 分组柱状图复用 Native Shell 绘图，支持系列切换、主题色、网格、坐标轴、图例及 RTL。提示框支持悬停、键盘导航和自定义格式。无头测试覆盖系列切换、提示框关闭和反向坐标轴。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#chart) · [组件源码](src/chart.rs) · [TSX 参考](references/chart-tsx/)
- **Checkbox** 已发布于 v23（`f82eaaa`），含 8 个预览及 Preview / Rust / TSX。支持受控与非受控状态、描述、禁用、无效状态、混合选择和 RTL。Rust 表格在只有部分行选中时显示混合状态。无头测试覆盖空格键切换、无障碍状态、全选和 RTL 布局。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#checkbox) · [组件源码](src/checkbox.rs) · [TSX 参考](references/checkbox-tsx/)
- **Code Block** 已发布于 v24（`3791d21`），含 25 个预览及 Preview / Rust / TSX。支持组合式标题和操作、行号及起始行、折行、差异标记、复制、展开、RTL、基于 Syntect 的着色、基础 ANSI 颜色和本地流式回放。Rust 演示仍缺文件下载，流式着色会重新处理当前文本。无头测试校验复制内容、ANSI 控制码去除、折行、展开、文件切换和回放。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#code-block) · [组件源码](src/code_block/mod.rs) · [TSX 参考](references/code-block-tsx/)
- **Collapsible** 已发布于 v25（`0a51eef`），含 5 个预览及 Preview / Rust / TSX。支持 Trigger 和 Content 组合、受控与非受控状态、禁用、原生展开动画和 RTL。无头测试覆盖订单详情、设置面板、嵌套折叠保留状态、无障碍控件关联和键盘操作。文件选择与 Learn More 使用本地演示反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#collapsible) · [组件源码](src/collapsible.rs) · [TSX 参考](references/collapsible-tsx/)
- **Combobox** 已发布于 v26（`b46c33b`），含 12 个预览及 Preview / Rust / TSX。支持过滤、自动高亮、分组、可移除的多选标签、自定义选项、搜索弹层和 RTL。无头测试覆盖键盘选择、禁用项、清空、多选及 RTL 切换保留选中。Native Shell 在触屏平台使用底部弹层。无效状态的无障碍标记在字段容器上。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#combobox) · [组件源码](src/combobox.rs) · [TSX 参考](references/combobox-tsx/)
- **Command** 已发布于 v27（`910ed43`），含 7 个预览及 Preview / Rust / TSX。支持内嵌和对话框、关键词搜索、自动高亮、分组、快捷键提示、长列表和 RTL。无头测试覆盖禁用项、双阶段 Escape、快捷键打开和关闭、滚动与选择。项目命令仅更新本地反馈。各命令的快捷键标记用于显示，应用可为实际操作绑定按键。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#command) · [组件源码](src/command.rs) · [TSX 参考](references/command-tsx/)
- **Context Menu** 已发布于 v29（`7bb6db6`），含 11 个官方示例及 Preview / Rust / TSX。支持右键、触屏长按、键盘打开、子菜单、分组、禁用项、复选、单选和四向定位。Chromium 在桌面和手机尺寸验证了焦点恢复、状态保留、四向定位和模拟触摸长按。示例用字符图标与本地操作反馈。RTL 示例支持阿拉伯语和希伯来语，子菜单沿用 Native Shell 的 → 展开和 ← 返回规则。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#context-menu) · [组件源码](src/context_menu.rs) · [TSX 参考](references/context-menu-tsx/)
- **Data Table** 已发布于 v30（`e81bfca`）。含官方付款和 RTL 示例，以及每页两行的分页预览，均可切换 Preview / Rust / TSX。支持自定义单元格、排序、筛选、列显隐、分页、行选择和行操作。付款与客户详情使用本地示例数据。Chromium 验证了桌面列头对齐、排序、列显隐、复制付款 ID、RTL，以及手机横向滑动、分页、跨页选择和行操作。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#data-table) · [组件源码](src/data_table.rs) · [TSX 参考](references/data-table-tsx/)
- **Date Picker** 已发布于 v31（`b1ca5ab`），含 8 个官方示例及 Preview / Rust / TSX。复用 Calendar，支持单日、范围、年份导航、手动日期、时分秒和阿拉伯语、希伯来语 RTL。自然语言解析覆盖常用英文短语，使用设备本地日期。日期范围在桌面并排显示两个月，在窄屏按月切换。Chromium 验证了键盘打开、闰日验证、范围端点高亮、时间编辑、RTL 切换和手机跨月选择。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#date-picker) · [组件源码](src/date_picker.rs) · [TSX 参考](references/date-picker-tsx/)
- **Dialog** 已发布于 v32（`fc5f352`），含 6 个官方示例及 Preview / Rust / TSX。提供组合式 Trigger、Content、Header、Footer、Overlay 和 Close，支持自定义关闭按钮、独立的 Escape 与外点关闭设置、正文滚动和固定底栏。使用 Native Shell 淡入淡出，表单保存到示例本地状态。Chromium 验证了 Tab / Shift+Tab 循环、输入框内 Escape 关闭、焦点恢复、RTL，以及手机滑动正文时底栏保持位置。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#dialog) · [组件源码](src/dialog.rs) · [TSX 参考](references/dialog-tsx/)

## 按官方顺序跟踪

以下按 2026-11-21 的 **All Components** 排列。新增组件从第一个「待开始」条目开始。每完成一项，更新状态并记录已覆盖的示例、验证结果、已知差异和部署版本。[shadcn/ui 组件目录](https://ui.shadcn.com/docs/components#all-components)

| # | 组件 | 状态 |
| ---: | --- | --- |
| 1 | Accordion | 已发布 |
| 2 | Alert | 已发布 |
| 3 | Alert Dialog | 已发布 |
| 4 | Aspect Ratio | 已发布 |
| 5 | Attachment | 已发布 |
| 6 | Avatar | 已发布 |
| 7 | Badge | 已发布 |
| 8 | Breadcrumb | 已发布 |
| 9 | Bubble | 已发布 |
| 10 | Button | 已发布 |
| 11 | Button Group | 已发布 |
| 12 | Calendar | 已发布 |
| 13 | Card | 已发布 |
| 14 | Carousel | 已发布 |
| 15 | Chart | 已发布 |
| 16 | Checkbox | 已发布 |
| 17 | Code Block | 已发布 |
| 18 | Collapsible | 已发布 |
| 19 | Combobox | 已发布 |
| 20 | Command | 已发布 |
| 21 | Context Menu | 已发布 |
| 22 | Data Table | 已发布 |
| 23 | Date Picker | 已发布 |
| 24 | Dialog | 已发布 |
| 25 | Direction | 已实现 · 待发布 |
| 26 | Drawer | 已发布 |
| 27 | Dropdown Menu | 待开始 |
| 28 | Empty | 待开始 |
| 29 | Field | 待开始 |
| 30 | Hover Card | 待开始 |
| 31 | Input | 待开始 |
| 32 | Input Group | 待开始 |
| 33 | Input OTP | 待开始 |
| 34 | Item | 待开始 |
| 35 | Kbd | 待开始 |
| 36 | Label | 待开始 |
| 37 | Marker | 待开始 |
| 38 | Menubar | 待开始 |
| 39 | Message | 待开始 |
| 40 | Message Scroller | 待开始 |
| 41 | Native Select | 待开始 |
| 42 | Navigation Menu | 待开始 |
| 43 | Pagination | 待开始 |
| 44 | Popover | 待开始 |
| 45 | Progress | 待开始 |
| 46 | Questionnaire | 待开始 |
| 47 | Radio Group | 待开始 |
| 48 | Resizable | 待开始 |
| 49 | Scroll Area | 待开始 |
| 50 | Select | 待开始 |
| 51 | Separator | 待开始 |
| 52 | Sheet | 待开始 |
| 53 | Sidebar | 待开始 |
| 54 | Skeleton | 待开始 |
| 55 | Slider | 待开始 |
| 56 | Spinner | 待开始 |
| 57 | Switch | 待开始 |
| 58 | Table | 待开始 |
| 59 | Tabs | 待开始 |
| 60 | Textarea | 待开始 |
| 61 | Toast | 待开始 |
| 62 | Toggle | 待开始 |
| 63 | Toggle Group | 待开始 |
| 64 | Tooltip | 待开始 |
| 65 | Trace | 待开始 |
| 66 | Typography | 待开始 |
