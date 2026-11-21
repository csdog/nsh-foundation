# nsh-foundation 组件进度

更新于 2026-11-21。已发布 **58 / 66**，本地完成待发布 **4 / 66**，待开始 **4 / 66**。下一项是 **Toggle Group（#63）**。[实施计划](PLAN.md)

## 已交付基线

- **Drawer** 含 9 个交互示例及 Preview / Rust / TSX 标签。v51（`1857e44`）按 Rhea 将 16 px 留白放回 Header、Footer 和各示例正文，正文可独立设置内边距。边缘面板默认留 8 px 外边距，浮动示例为 16 px。滑入 450 ms，普通滑出 320 ms，面板和遮罩在普通关闭时使用同一条缓动曲线。拖拽关闭继续按松手速度调整时长。Chromium 在 1280 × 900 和 390 × 844 验证了间距、逐帧位移、遮罩淡出、触摸拖拽、嵌套层、浮动间距、自定义尺寸和紧凑吸附点；减少动态效果模式也已复测。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#drawer) · [组件源码](src/drawer.rs) · [TSX 参考](references/drawer-tsx/)
- **线上版本**为 67，发布提交为 `ffd9488`，新增 Table。Web 打包和 WASM 内容校验已完成。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#table)
- **基线复测**于 2026-11-21 通过 `cargo fmt --check` 和 `cargo test --offline --locked`，共 111 项 Rust 测试（全量复测使用去除调试符号的构建）。
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
- **Direction** 已发布于 v33（`d13dc58`）。提供可响应 Rust 信号的 `DirectionProvider` 和读取最近作用域的 `use_direction()`。复用 Card RTL 示例，可切换英语、阿拉伯语和希伯来语，含 Preview / Rust / TSX。Rust 测试覆盖嵌套覆盖、同级隔离和保留节点，Chromium 验证了桌面、手机的实际布局变化。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#direction) · [组件源码](src/direction.rs) · [TSX 参考](references/direction-tsx/)
- **Dropdown Menu** 已发布于 v34（`7756fc8`），含 13 个官方示例及 Preview / Rust / TSX。复用 Context Menu 的菜单项，支持多级子菜单、复选与单选、头像和 RTL。Chromium 验证了键盘焦点恢复、禁用项、三级菜单、状态保留、RTL 右对齐、深色主题和手机全宽底部菜单。示例使用字符图标、快捷键提示和本地操作反馈。RTL 子菜单沿用 Native Shell 的 → 展开和 ← 返回规则。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#dropdown-menu) · [组件源码](src/dropdown_menu.rs) · [TSX 参考](references/dropdown-menu-tsx/)
- **Empty** 已发布于 v36（`9ead589`），含 7 个官方示例及 Preview / Rust / TSX。提供组合式 Header、Media、Title、Description 和 Content。Chromium 在 1280 × 900 和 390 × 844 验证了虚线边框、头像重叠、键盘与触屏操作、搜索框提交、RTL 按钮顺序、代码标签、深色主题和组件导航。头像沿用项目的大号尺寸，操作仅显示本地演示反馈，搜索框中的斜杠为视觉提示。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#empty) · [组件源码](src/empty.rs) · [TSX 参考](references/empty-tsx/)
- **Field** 已发布于 v37（`411e36f`），含 13 个官方示例及 Preview / Rust / TSX。提供组合式字段、字段集、标签、说明、分隔线和错误提示，按字段宽度切换响应式布局。Chromium 在 1280 × 900 和 390 × 844 验证了点按标签聚焦、表单错误关联与清除、输入、下拉、双滑块、单选、开关、RTL 值保留、布局和导航。控件复用 Native Shell 原语，结账和资料表单只在本地校验示例值。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#field) · [组件源码](src/field.rs) · [TSX 参考](references/field-tsx/)
- **Hover Card** 已发布于 v38（`ea14f44`），含 4 个官方预览位置和 3 份 TSX 示例，Basic 复用首页示例。提供组合式 Trigger、Content、受控状态、开关延迟、外点与 Escape 关闭、四向定位及 RTL 逻辑侧。Chromium 在 1280 × 900 和 390 × 844 验证了悬停和移入卡片、触屏开关、窄屏侧向限宽、RTL 定位、深色主题和导航。桌面沿用 Native Shell 的指针触发规则，键盘聚焦保持在触发控件上。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#hover-card) · [组件源码](src/hover_card.rs) · [TSX 参考](references/hover-card-tsx/)
- **Input** 已发布于 v39（`990dd8b`），含 15 个官方示例及 Preview / Rust / TSX。提供 Rhea 样式的原生输入、密码遮蔽、禁用、响应式错误状态和系统文件选择，复用 Field、Badge、Button Group。Chromium 在 1280 × 900 和 390 × 844 验证了聚焦、提交与重置、文件选择与取消、网格、RTL 对齐与值保留、主题及导航。文件示例只保留文件名，搜索和表单只在本地显示反馈。Website URL 预览从 v40 起使用正式的 Input Group。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#input) · [组件源码](src/input.rs) · [TSX 参考](references/input-tsx/)
- **Input Group** 已发布于 v40（`69f312d`），含 14 个官方示例及 Preview / Rust / TSX。组合原生单行输入、多行输入、四向附加区、文本、按钮、菜单和加载指示。Chromium 在 1280 × 900 和 390 × 844 验证了附加区定位与聚焦、剪贴板、弹层、菜单、光标行列号、文本域自动增高、RTL 对齐和值保留。原 Input 页通过回归复测。运行、搜索、评论和提交使用本地演示反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#input-group) · [组件源码](src/input_group.rs) · [TSX 参考](references/input-group-tsx/)
- **Input OTP** 已发布于 v41（`ea92533`），含 10 个官方示例及 Preview / Rust / TSX。使用一个原生输入框和组合式 Group、Slot、Separator，支持自定义字符规则、受控值、格式化粘贴、错误状态和验证码自动填充提示。Chromium 在 1280 × 900 和 390 × 844 验证了键盘编辑、数字归一化、完整粘贴替换、禁用、错误说明关联、RTL 视觉顺序和值保留。验证与重发使用本地示例数据。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#input-otp) · [组件源码](src/input_otp.rs) · [TSX 参考](references/input-otp-tsx/)
- **Item** 已发布于 v42（`9affb99`），含 11 个官方示例及 Preview / Rust / TSX。提供三种样式、三种尺寸、媒体、正文、操作区、整行链接和通栏头尾。Chromium 在 1280 × 900 和 390 × 844 验证了本地图片、链接和独立操作、菜单选项、RTL、明暗主题和导航。Header 示例在窄屏使用单列。歌曲链接指向组件文档，邀请和选择只更新本地预览。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#item) · [组件源码](src/item.rs) · [TSX 参考](references/item-tsx/)
- **Kbd** 已发布于 v43（`bf5b7bc`），含 6 个官方示例及 Preview / Rust / TSX。提供按键标签、符号名称、组合键和不同容器的配色。Chromium 在 1280 × 900 和 390 × 844 验证了按钮操作、Tooltip 悬停与键盘焦点、RTL 顺序、主题和导航。搜索框绑定平台对应的组合键，旧 Input Group 页采用相同样式并通过回归复测。Save 和 Print 使用本地演示反馈，Tooltip 内的快捷键用于展示。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#kbd) · [组件源码](src/kbd.rs) · [TSX 参考](references/kbd-tsx/)
- **Label** 已发布于 v44（`da71207`），含 3 个官方预览及 Preview / Rust / TSX。通过 `ElementRef` 关联控件，控件自身提供可访问名称，`FieldLabel` 共用关联逻辑。Rust 测试覆盖禁用状态。Chromium 在桌面和触屏尺寸验证了标签激活、Tab 顺序、空格键切换、RTL 状态保留，并复测 Field 和 Input OTP。桌面点击标签可聚焦复选框，触屏点按会切换状态。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#label) · [组件源码](src/label.rs) · [TSX 参考](references/label-tsx/)
- **Marker** 已发布于 v45（`65b451b`），含 8 个官方示例及 Preview / Rust / TSX。支持行内、底边框、分隔线、饰图、流光和可操作标记。状态变化使用 Native Shell 全局实时区域，示例使用字符图标。Chromium 验证了桌面和触屏布局、动画与减少动态效果设置、键盘操作、链接及撤回按钮的本地反馈。示例链接指向上游 PR 列表。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#marker) · [组件源码](src/marker.rs) · [TSX 参考](references/marker-tsx/)
- **Menubar** 已发布于 v46（`8323785`），含 6 个官方示例及 Preview / Rust / TSX。支持顶层菜单切换、复选、单选、嵌套菜单、图标、快捷键提示和 RTL。Rust 测试覆盖打开后连续左右切换和键盘选中。Chromium 在 1280 × 900 和 390 × 844 验证了 6 个预览、复选和单选、子菜单、RTL、主题和导航，并通过 Dropdown Menu 与 Context Menu 回归。手机使用 Native Shell 底部菜单，子菜单沿用 Native Shell 展开键，示例命令只更新本地反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#menubar) · [组件源码](src/menubar.rs) · [TSX 参考](references/menubar-tsx/)
- **Message** 已发布于 v47（`75249fb`），含 6 个官方示例及 Preview / Rust / TSX。支持两侧对齐、头像占位、连续消息、页眉和页脚，并组合现有 Bubble、Avatar、Marker 和 Attachment。Rust 测试确认页脚变高时头像仍与正文底边对齐。Chromium 在 1280 × 900 和 390 × 844 验证了 6 个预览、两侧对齐、头像与页脚、剪贴板内容、反馈切换、重试和附件图片的可访问名称。重试模拟本地发送状态，PDF 操作显示请求反馈。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#message) · [组件源码](src/message.rs) · [TSX 参考](references/message-tsx/)
- **Message Scroller** 已发布于 v48（`da2de72`），含 11 个官方场景及 Preview / Rust / TSX。使用稳定消息 ID 支持回合锚定和上下文保留、流式跟随、三种打开位置、加载历史时保留可见行、跳转、入场动效及可见范围。Chromium 在 1280 × 900 和 390 × 844 验证了打开位置、跳转、上下文间距、历史定位、流式暂停和恢复、可见范围、入场动画和减少动态效果设置，并复测 Message 页面。Rust 预览使用本地对话及模拟流式输出，滚动区按常规原生布局渲染全部消息。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#message-scroller) · [组件源码](src/message_scroller.rs) · [TSX 参考](references/message-scroller-tsx/)
- **Native Select** 已发布于 v49（`a76572b`），含 5 个官方示例及 Preview / Rust / TSX。支持选项组、默认值与绑定值、禁用、无效状态和 RTL。Rust 实现复用 Native Shell Select，桌面使用锚定列表，触屏使用底部面板。Rust 测试覆盖键盘跳过禁用选项和禁用组。Chromium 在 1280 × 900 和 390 × 844 验证了 5 个预览、键盘输入匹配、组选项选择、触屏底部面板、无效状态、RTL 选中值保留，并复测 Field。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#native-select) · [组件源码](src/native_select.rs) · [TSX 参考](references/native-select-tsx/)
- **Navigation Menu** 已发布于 v50（`94a9e7a`），含 2 个官方示例及 Preview / Rust / TSX。支持顶层直达链接、单列及多列面板、悬停切换、点击关闭、键盘焦点、可选指示标记和 RTL。文档链接指向 shadcn，状态图标链接保留演示占位。Chromium 在 1280 × 900 和 390 × 844 验证了双列面板、悬停、触控关闭、方向键、Escape 焦点恢复、链接和 RTL，并复测 Menubar。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#navigation-menu) · [组件源码](src/navigation_menu.rs) · [TSX 参考](references/navigation-menu-tsx/)
- **Pagination** 已发布于 v52（`cf27889`），含 4 个官方示例及 Preview / Rust / TSX。支持普通链接、原地翻页、当前页语义、首尾禁用、图标模式和 RTL。行数切换示例使用 150 条的本地计数并回到第 1 页。88 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了各示例、首尾边界、行数切换、阿拉伯数字、阿拉伯语和希伯来语 RTL、主题及代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#pagination) · [组件源码](src/pagination.rs) · [TSX 参考](references/pagination-tsx/)
- **Popover** 已发布于 v53（`b8ad101`），含 5 个官方示例及 Preview / Rust / TSX。支持可组合部件、受控状态、三种对齐、物理和逻辑方向、空间不足时自动翻转、点击外部及 Escape 关闭。表单只编辑弹层内的值，重开时恢复示例默认值。桌面自动聚焦首个指定输入框，触屏等用户点输入框。88 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了弹层位置、边缘翻转、表单、关闭、桌面焦点恢复、RTL 和代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#popover) · [组件源码](src/popover.rs) · [TSX 参考](references/popover-tsx/)
- **Progress** 已发布于 v54（`0968031`），含 4 个官方示例及 Preview / Rust / TSX。支持受控进度、自定义范围及标签、当前值的无障碍语义、RTL 和减少动态效果。未知进度显示空轨道并省略当前值。89 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了定时更新、逐帧填充、减少动态效果、滑块键盘和触控操作、RTL 对齐和本地化百分比。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#progress) · [组件源码](src/progress.rs) · [TSX 参考](references/progress-tsx/)
- **Questionnaire** 已发布于 v55（`11e3df4`），含 14 个官方示例及 Preview / Rust / TSX。支持单选、多选、自由输入、显式跳过、快捷键、跨题校验、受控步骤、草稿重置、条件项、自定义进度、动画、Card 和 Dialog。演示使用预置草稿，提交仅更新本地反馈。90 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了全部示例、校验焦点、草稿重置、隐藏答案过滤、键盘和触控操作。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#questionnaire) · [组件源码](src/questionnaire.rs) · [TSX 参考](references/questionnaire-tsx/)
- **Radio Group** 已发布于 v56（`8fe9b87`），含 7 个官方示例及 Preview / Rust / TSX。支持默认值、受控状态、描述、选项卡片、禁用和无效态。方向键按当前组的阅读方向移动，循环并跳过禁用项。91 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了互斥选中、Tab 焦点、卡片点击、无障碍状态、RTL 切换保留选中以及代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#radio-group) · [组件源码](src/radio_group.rs) · [TSX 参考](references/radio-group-tsx/)
- **Resizable** 已发布于 v57（`9ae2dab`），含 4 个官方示例及 Preview / Rust / TSX。支持横竖嵌套、百分比尺寸、最小和最大约束、受控布局、可见抓手及 RTL。键盘步长为一个百分点，Home 和 End 到达边界。94 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了鼠标、键盘和触控拖动、嵌套尺寸、无障碍数值、RTL 切换保留比例及代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#resizable) · [组件源码](src/resizable.rs) · [TSX 参考](references/resizable-tsx/)
- **Scroll Area** 已发布于 v58（`a94b767`），含 3 个官方示例及 Preview / Rust / TSX。每个视口可选横向或纵向，支持可拖动滚动条、键盘滚动、快速按键和 RTL。画廊三张照片随站点打包。95 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了滚轮、滚动条拖动、横竖触控和键盘、RTL 切换保留位置、图像可访问名称及代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#scroll-area) · [组件源码](src/scroll_area.rs) · [TSX 参考](references/scroll-area-tsx/)
- **Select** 已发布于 v59（`d32416c`），含 7 个官方示例及 Preview / Rust / TSX。支持组合式触发器、分组选项、受控状态、禁用和无效态、RTL 及桌面选中行对齐。触屏使用 Native Shell 底部面板，长列表使用其滚动条与键盘导航。96 项 Rust 测试通过。Chromium 在 1280 × 900 和 390 × 844 验证了七个示例、键盘搜索、选中标记、长列表滚动、两种桌面对齐、RTL 选项宽度和代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#select) · [组件源码](src/select.rs) · [TSX 参考](references/select-tsx/)

- **Separator** 已发布于 v60（`849b8d1`），含 5 个官方示例及 Preview / Rust / TSX。支持横竖排列和主题色，装饰线跳过键盘焦点及无障碍树。带名称的分隔线用 Native Shell 静态文字角色提供名称。Chromium 在 1280 × 900 和 390 × 844 验证了横竖尺寸、响应式菜单、RTL、装饰语义、主题和代码标签。Rust 测试覆盖两轴尺寸与带名称的分隔线。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#separator) · [组件源码](src/separator.rs) · [TSX 参考](references/separator-tsx/)
- **Sheet** 已发布于 v61（`387a059`），含 4 个官方示例及 Preview / Rust / TSX。支持四边定位、组合部件、独立的面板和遮罩动画、可滚动正文及可选关闭按钮。示例表单保存到页面内状态。Chromium 在 1280 × 900 和 390 × 844 验证了边缘布局、进出场、滚轮和触控滚动、表单、键盘焦点、关闭方式、RTL 及代码标签，也复测了减少动态效果。Rust 测试覆盖四边定位和 Escape。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#sheet) · [组件源码](src/sheet.rs) · [TSX 参考](references/sheet-tsx/)
- **Sidebar** 已发布于 v62（`effff22`），含工作区主示例、受控模式和 RTL 示例及 Preview / Rust / TSX。支持左右边缘、普通／浮动／内嵌样式、三种折叠方式、可配置宽度及桌面状态持久化。手机使用 Sheet。Chromium 在 1280 × 900 和 390 × 844 验证了折叠、动画、菜单、快捷键、触摸导航、RTL 和代码标签，并复测减少动态效果。工作区命令使用本地反馈，图标使用字符和随站头像。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#sidebar) · [组件源码](src/sidebar.rs) · [TSX 参考](references/sidebar-tsx/)
- **Skeleton** 已发布于 v63（`3699695`），含 7 个官方示例及 Preview / Rust / TSX。支持自定义尺寸与形状、主题色和可关闭的脉冲动画。装饰形状跳过键盘与无障碍树。Sidebar 菜单占位复用该组件。Chromium 在 1280 × 900、390 × 844 和 320 × 844 验证了尺寸、比例、卡片间距、动画、减少动态效果、RTL、主题和代码标签。Rust 测试覆盖 RTL 镜像时尺寸保持一致。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#skeleton) · [组件源码](src/skeleton.rs) · [TSX 参考](references/skeleton-tsx/)
- **Slider** 已发布于 v64（`524fd48`），含 7 个官方示例及 Preview / Rust / TSX。支持固定数量的滑块、水平和垂直、受控信号、小数步进、相邻滑块推移、禁用和 RTL。纵向示例由可访问名称注明方向。Chromium 在 1280 × 900 和 390 × 844 验证了轨道点按、鼠标和触屏拖动、松手焦点、键盘大步进、链式推移、无障碍数值、主题和代码标签。Rust 测试覆盖提交次数和 RTL 小数键盘操作。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#slider) · [组件源码](src/slider.rs) · [TSX 参考](references/slider-tsx/)
- **Spinner** 已发布于 v65（`2279e37`），含 8 个官方示例及 Preview / Rust / TSX。默认圆弧和自定义辐条由 Native Shell 绘制，可替换图标、设置尺寸和颜色。具名 Spinner 使用无确定进度的进度条语义，内联图标可设为装饰。Chromium 验证了绘制、主题色、动态切换减少动态效果、RTL 和示例操作，并复测旧 Button 加载状态。Send 和 Cancel 更新本地预览状态。Rust 测试覆盖加载名称和取消。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#spinner) · [组件源码](src/spinner.rs) · [TSX 参考](references/spinner-tsx/)
- **Switch** 已发布于 v66（`9a288ca`），含 7 个官方示例及 Preview / Rust / TSX。支持受控与初始状态、说明、禁用、无效态、两种尺寸、RTL 和减少动态效果。Field 示例复用相同组件。接受条款后，无效示例会清除错误状态。Chromium 验证了标签和卡片切换、手机焦点、键盘、尺寸、动画、主题、RTL 保留值及代码标签。Rust 测试覆盖默认值、空格键与卡片装配。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#switch) · [组件源码](src/switch.rs) · [TSX 参考](references/switch-tsx/)
- **Table** 已发布于 v67（`ffd9488`），含 4 个官方示例及 Preview / Rust / TSX。组合部件使用 Native Shell 表格语义，支持固定和弹性列、合计行视觉跨列、表注、行菜单和 RTL。跨列由布局表达，Native Shell 当前无跨列无障碍元数据。Rust 示例按可见发票计算总计，行菜单只显示本地反馈。Chromium 验证了列宽对齐、排序目录、菜单与焦点、RTL 起始位置、触摸和键盘横滑、主题及代码标签。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#table) · [组件源码](src/table.rs) · [TSX 参考](references/table-tsx/)

- **Tabs** 已在本地完成（`a4a01c0`），含 6 个官方示例及 Preview / Rust / TSX。默认方向键只移动焦点，Enter 或空格选中，可开启随焦点激活。支持受控值、禁用、垂直布局、线型指示器和 RTL。Native Shell 将当前面板暴露为有名称的可聚焦区域。Chromium 在 1280 × 900、390 × 844 和 320 × 844 验证了六个预览、键盘和触摸、RTL 切换语言保留选中项、主题和代码标签。Web 已打包并通过 WASM 校验，Sites 源码推送授权待确认。[组件源码](src/tabs.rs) · [TSX 参考](references/tabs-tsx/)

- **Textarea** 已在本地完成（`51e3771`），含 6 个官方示例及 Preview / Rust / TSX。复用 Native Shell 多行输入，支持随内容增减高度、主题、禁用、说明、动态校验和本地提交反馈。Chromium 在 1280 × 900、390 × 844 和 320 × 844 验证了换行、自动换行撑高、清空收缩、标签焦点、禁用、校验、提交、主题及代码标签。阿拉伯语和希伯来语右对齐，切换语言保留输入。Native Shell 0.6.1 的 Web 文本编辑器在 LTR 页面内仍沿用页面的基准书写方向。Rust 测试覆盖换行和实时校验，Web 已打包并通过 WASM 校验，Sites 源码推送授权待确认。[组件源码](src/textarea.rs) · [TSX 参考](references/textarea-tsx/)

- **Toast** 已在本地完成（`5d595a9`），含 3 个官方示例及 Preview / Rust / TSX。复用 Native Shell 的通知栈、滑动识别与生命周期，支持撤销回调、自动消失、悬停或焦点暂停、Escape 关闭、向下滑动关闭及异步结果更新。状态图标使用原生图形和字符，所有类型沿用 Native Shell 的 alert 语义。Chromium 在 1280 × 900、390 × 844 和 320 × 844 验证了定位、类型、叠放、关闭、异步更新、主题和代码标签，在移动宽度额外验证了滑动早于超时。Rust 测试覆盖撤销、Escape、重开和异步失败。Web 已打包并通过 WASM 校验，Sites 源码推送授权待确认。[组件源码](src/toast.rs) · [TSX 参考](references/toast-tsx/)

- **Toggle** 已在本地完成（`27fa85c`），含 6 个官方示例及 Preview / Rust / TSX。复用 Native Shell 的 Toggle 和按钮按下语义，支持受控与本地状态、透明和描边样式、28/32/36 px 三档高度、禁用和 RTL。图标由 Native Shell 画布绘制，书签随状态切换描边和填充。Chromium 在 1280 × 900、390 × 844 和 320 × 844 验证了键盘和触摸、禁用、RTL 图标顺序与语言切换保留状态、主题、尺寸及代码标签。Rust 测试覆盖初始值、键盘、共享值与禁用状态。Web 已打包并通过 WASM 校验，Sites 源码推送授权待确认。[组件源码](src/toggle.rs) · [TSX 参考](references/toggle-tsx/)

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
| 25 | Direction | 已发布 |
| 26 | Drawer | 已发布 · 已复核 |
| 27 | Dropdown Menu | 已发布 |
| 28 | Empty | 已发布 |
| 29 | Field | 已发布 |
| 30 | Hover Card | 已发布 |
| 31 | Input | 已发布 |
| 32 | Input Group | 已发布 |
| 33 | Input OTP | 已发布 |
| 34 | Item | 已发布 |
| 35 | Kbd | 已发布 |
| 36 | Label | 已发布 |
| 37 | Marker | 已发布 |
| 38 | Menubar | 已发布 |
| 39 | Message | 已发布 |
| 40 | Message Scroller | 已发布 |
| 41 | Native Select | 已发布 |
| 42 | Navigation Menu | 已发布 |
| 43 | Pagination | 已发布 |
| 44 | Popover | 已发布 |
| 45 | Progress | 已发布 |
| 46 | Questionnaire | 已发布 |
| 47 | Radio Group | 已发布 |
| 48 | Resizable | 已发布 |
| 49 | Scroll Area | 已发布 |
| 50 | Select | 已发布 |
| 51 | Separator | 已发布 |
| 52 | Sheet | 已发布 |
| 53 | Sidebar | 已发布 |
| 54 | Skeleton | 已发布 |
| 55 | Slider | 已发布 |
| 56 | Spinner | 已发布 |
| 57 | Switch | 已发布 |
| 58 | Table | 已发布 |
| 59 | Tabs | 本地完成 · 待发布 |
| 60 | Textarea | 本地完成 · 待发布 |
| 61 | Toast | 本地完成 · 待发布 |
| 62 | Toggle | 本地完成 · 待发布 |
| 63 | Toggle Group | 待开始 · 下一项 |
| 64 | Tooltip | 待开始 |
| 65 | Trace | 待开始 |
| 66 | Typography | 待开始 |
