# nsh-foundation 组件进度

更新于 2026-11-21。已发布 **4 / 66**，待开始 **62 / 66**。下一项是 **Aspect Ratio（#4）**。[实施计划](PLAN.md)

## 已交付基线

- **Drawer** 已拆成纯 Rust 组合组件，包括 `DrawerTrigger`、`DrawerContent`、`DrawerHeader`、`DrawerTitle`、`DrawerDescription`、`DrawerFooter` 和 `DrawerClose`。文档已有 9 个交互示例及 Preview / Rust / TSX 标签。贴边面板默认留 8 点外边距，按实际尺寸滑入和滑出，分别用时 450 ms 和 400 ms。[组件源码](src/drawer.rs) · [TSX 参考](references/drawer-tsx/)
- **线上版本**为 9，发布提交为 `6090b69`。Web 打包和 WASM 校验已完成。云端预览端口启动受限，浏览器视觉核验仍待复核。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site)
- **基线复测**于 2026-11-21 通过 `cargo fmt --check` 和 `cargo test --offline`，共 18 项 Rust 测试。
- **设计参考**已固定 shadcn/ui 源码版本，Obra 预览图已保存。可编辑的 Figma 原文件仍未获取。[参考资料记录](references/README.md)
- **Accordion** 已发布于 v7（`7abbcb5`），含 7 个官方示例及 Preview / Rust / TSX。支持默认与受控状态、单项与多项展开、禁用、方向键和 RTL。15 项 Rust 测试、Web 打包及 WASM 校验已通过，浏览器视觉核验受预览端口限制。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#accordion) · [组件源码](src/accordion.rs) · [TSX 参考](references/accordion-tsx/)
- **Alert** 已发布于 v8（`1e44dc4`），含 6 个官方示例及 Preview / Rust / TSX。支持普通和 destructive 样式、自定义主题颜色、操作按钮和 RTL。无头测试覆盖移动端交互、复制代码和历史导航。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#alert) · [组件源码](src/alert.rs) · [TSX 参考](references/alert-tsx/)
- **Alert Dialog** 已发布于 v9（`6090b69`），含 7 个预览及 Preview / Rust / TSX。无头测试覆盖桌面和移动端尺寸、打开后聚焦取消、Escape 关闭和显式确认。[在线演示](https://nsh-foundation-drawer.r8smtvbm6v.chatgpt.site/#alert-dialog) · [组件源码](src/alert_dialog.rs)

## 按官方顺序跟踪

以下按 2026-11-21 的 **All Components** 排列。新增组件从第一个「待开始」条目开始。每完成一项，更新状态并记录已覆盖的示例、验证结果、已知差异和部署版本。[shadcn/ui 组件目录](https://ui.shadcn.com/docs/components#all-components)

| # | 组件 | 状态 |
| ---: | --- | --- |
| 1 | Accordion | 已发布 |
| 2 | Alert | 已发布 |
| 3 | Alert Dialog | 已发布 |
| 4 | Aspect Ratio | 已实现 · 待发布 |
| 5 | Attachment | 待开始 |
| 6 | Avatar | 待开始 |
| 7 | Badge | 待开始 |
| 8 | Breadcrumb | 待开始 |
| 9 | Bubble | 待开始 |
| 10 | Button | 待开始 |
| 11 | Button Group | 待开始 |
| 12 | Calendar | 待开始 |
| 13 | Card | 待开始 |
| 14 | Carousel | 待开始 |
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
