# nsh-foundation 组件计划

> 历史归档，记录首批 66 个组件的实施与交付。当前工作以 [AGENTS.md 的验收 SOP](../AGENTS.md#acceptance-sop) 为准。

## 范围与顺序

以 2026-11-21 的 shadcn/ui **All Components** 目录为顺序快照，共 66 项。用每一项的 Base UI 文档和示例定义行为，以 Rhea 样式和固定版本源码核验视觉。目录中的 New Components 是推荐入口，其中的组件仍按 All Components 的位置排队。AI Elements、Blocks 和 Forms 采用各自的官方目录，此计划覆盖 All Components 中列出的项目。[官方顺序](https://ui.shadcn.com/docs/components#all-components) · [源码与设计参考](../references/README.md)

从 **Accordion → Alert → Alert Dialog → Aspect Ratio → Attachment** 开始，逐个推进。Drawer 已在第 26 位标记为已发布，推进到该位置时复核它与新增组件是否共用一致的主题、导航和演示结构。完整顺序及状态见 [PROGRESS.md](PROGRESS.md)。

## 每个组件的执行方式

1. **对照参考。** 打开该组件的 Base UI 文档，列出页面上每个可运行示例，并核对它的状态、键盘操作、触控及无障碍行为。用仓库的上游版本定位对应源码；若当前页面有更新，把实际采用的参考版本记到进度里。
2. **实现 Rust 组件。** 优先复用 `nsh-base`、`nsh-core` 和现有主题变量，应用 UI、状态和交互继续用纯 Rust。组合组件按官方示例暴露可独立使用的部件。若当前示例需要排在后面的组件，先复用 NSH 原语或已有的局部实现，把后续组件的正式交付留在原顺序。
3. **接入文档。** 每项提供可直达的文档入口、使用方法和可操作的预览。沿用 **Preview / Rust / TSX** 模式，Rust 展示本项目 API，TSX 展示对应的 shadcn/ui React + Base UI 写法。用同一个文档框架接入新页面，保持现有 Drawer 入口可访问。
4. **核验并记录。** 运行 `cargo fmt --check`、`cargo test` 和 `nsh package --target web --out dist`。按场景实测桌面和手机宽度的页面、主题和交互，把浏览器实测与无头测试分别记下来。发布后记录版本或链接，再推进下一项。

## 第一项 · Accordion

| 工作 | 验收 |
| --- | --- |
| 确定公共 API | 组件可组合使用 `Accordion`、`AccordionItem`、`AccordionTrigger`、`AccordionContent`，支持默认展开、受控状态及单项和多项展开。优先复用 NSH 的 Accordion 原语。 |
| 对齐官方展示 | 主预览与 Basic、Multiple、Disabled、Borders、Card、RTL 示例都可单独打开，按页面展示的顺序排列。 |
| 核对交互 | 验证点击、触控、Tab 聚焦及 Enter / Space 展开，核对上游方向键行为，并验证展开状态、禁用项、焦点样式和 RTL 布局。 |
| 接入文档导航 | 能从组件目录打开 Accordion，通过 Rust 和 TSX 标签查看并复制对应示例，原 Drawer 页仍可达。 |

[Accordion 官方文档](https://ui.shadcn.com/docs/components/base/accordion) · [Native Shell Accordion](https://nsh.dev/elements/accordion/)
