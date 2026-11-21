# nsh-foundation 组件进度

更新于 2026-11-21。已发布 **14 / 66**，待开始 **52 / 66**。下一项是 **Carousel（#14）**。[实施计划](PLAN.md)

## 已交付基线

- **Drawer** 已拆成纯 Rust 组合组件，包括 `DrawerTrigger`、`DrawerContent`、`DrawerHeader`、`DrawerTitle`、`DrawerDescription`、`DrawerFooter` 和 `DrawerClose`。文档已有 9 个交互示例及 Preview / Rust / TSX 标签。面板默认留 8 点外边距，示例正文内边距为 16 点，并与标题和底部操作对齐。普通滑入和滑出分别用时 450 ms 和 400 ms。甩动关闭按剩余距离和松手速度调整时长，退出动画播完再卸载内容。测试覆盖退出途中重新打开及内容间距。[组件源码](src/drawer.rs) · [TSX 参考](references/drawer-tsx/)
- **线上版本**为 20，发布提交为 `09deb05`，新增 Card。Web 打包、WASM 校验和封面图校验已完成。云端预览端口启动受限，浏览器视觉核验仍待复核。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#card)
- **基线复测**于 2026-11-21 通过 `cargo fmt --check` 和 `cargo test --offline --locked`，共 40 项 Rust 测试。
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
| 14 | Carousel | 待开始 · 下一项 |
| 15 | Chart | 待开始 |
| 16 | Checkbox | 待开始 |
| 17 | Code Block | 待开始 |
| 18 | Collapsible | 待开始 |
| 19 | Combobox | 待开始 |
| 20 | Command | 待开始 |
| 21 | Context Menu | 待开始 |
| 22 | Data Table | 待开始 |
| 23 | Date Picker | 待开始 |
| 24 | Dialog | 待开始 |
| 25 | Direction | 待开始 |
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
