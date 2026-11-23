//! Selectable, themed code snippets with composable headers and actions.

mod highlight;

use std::time::Duration;

use nsh_base::theme;
use nsh_core::{
    element::{LayoutDirection, idiom, set_clipboard},
    platform::Idiom,
    prelude::*,
};

use crate::button::{Button, ButtonSize, ButtonVariant};
pub use highlight::strip_ansi;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeBlockSize {
    #[default]
    Default,
    Small,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeBlockVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeLineVariant {
    #[default]
    Default,
    Added,
    Removed,
    Highlighted,
}

pub struct CodeBlock {
    content: CodeBlockContent,
    header: Option<CodeBlockHeader>,
    actions: Option<CodeBlockActions>,
    footer: Option<CodeBlockFooter>,
    size: CodeBlockSize,
    variant: CodeBlockVariant,
    direction: LayoutDirection,
}

impl CodeBlock {
    pub fn new(content: CodeBlockContent) -> Self {
        Self {
            content,
            header: None,
            actions: None,
            footer: None,
            size: CodeBlockSize::Default,
            variant: CodeBlockVariant::Default,
            direction: LayoutDirection::Inherit,
        }
    }

    pub fn header(mut self, header: CodeBlockHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn actions(mut self, actions: CodeBlockActions) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn footer(mut self, footer: CodeBlockFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CodeBlockVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn surface(self) -> View {
        let compact = self.size == CodeBlockSize::Small;
        let padding = if compact { 12.0 } else { 16.0 };
        let font = if compact { 12.0 } else { 14.0 };
        let destructive = self.variant == CodeBlockVariant::Destructive;
        let has_header = self.header.is_some();
        let has_footer = self.footer.is_some();
        view()
            .direction(self.direction)
            .group()
            .relative()
            .w_full()
            .min_w(px(0.0))
            .rounded(px(padding))
            .ring(px(1.0))
            .ring_color(move || {
                if destructive {
                    theme()
                        .destructive
                        .opacity(if dark() { 0.20 } else { 0.15 })
                } else {
                    theme().foreground.opacity(if dark() { 0.10 } else { 0.05 })
                }
            })
            .bg(move || {
                if destructive {
                    theme()
                        .destructive
                        .opacity(if dark() { 0.10 } else { 0.05 })
                } else {
                    theme().muted
                }
            })
            .text_color(move || {
                if destructive {
                    theme().destructive
                } else {
                    theme().foreground
                }
            })
            .child(self.header.map(|header| header.surface(padding, font)))
            .child(self.content.surface(
                font,
                padding,
                has_header,
                has_footer,
                self.actions.is_some(),
            ))
            .child(self.actions.map(|actions| {
                view()
                    .absolute()
                    .top(px(padding - 4.0))
                    .when(idiom() == Idiom::Desktop, |actions| {
                        actions
                            .opacity(0.0)
                            .group_hover(|style| style.opacity(1.0))
                            .group_focus_within(|style| style.opacity(1.0))
                    })
                    .style_with(move |style| {
                        if self.direction == LayoutDirection::Rtl {
                            style.left(px(padding - 4.0))
                        } else {
                            style.right(px(padding - 4.0))
                        }
                    })
                    .child(actions)
            }))
            .child(self.footer.map(|footer| {
                view()
                    .px(px(padding))
                    .pb(px(padding))
                    .flex_row()
                    .justify_center()
                    .child(footer.0)
            }))
    }
}

impl IntoElement for CodeBlock {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct CodeBlockTitle(SharedString);

impl CodeBlockTitle {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self(title.into())
    }
}

pub struct CodeBlockActions(View);

impl CodeBlockActions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.0 = self.0.child(child);
        self
    }
}

impl Default for CodeBlockActions {
    fn default() -> Self {
        Self(view().flex_row().flex_shrink_0().items_center().gap_1())
    }
}

impl IntoElement for CodeBlockActions {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct CodeBlockHeader {
    title: CodeBlockTitle,
    actions: Option<CodeBlockActions>,
}

impl CodeBlockHeader {
    pub fn new(title: CodeBlockTitle) -> Self {
        Self {
            title,
            actions: None,
        }
    }

    pub fn actions(mut self, actions: CodeBlockActions) -> Self {
        self.actions = Some(actions);
        self
    }

    fn surface(self, padding: f32, font: f32) -> View {
        view()
            .px(px(padding))
            .pt(px(padding))
            .flex_row()
            .items_center()
            .justify_between()
            .gap_2()
            .child(
                text(self.title.0)
                    .text_size(px(font))
                    .font_weight(FontWeight::MEDIUM)
                    .truncate(),
            )
            .child(self.actions)
    }
}

pub struct CodeBlockFooter(AnyElement);

impl CodeBlockFooter {
    pub fn new(content: impl IntoElement) -> Self {
        Self(content.into_element())
    }
}

pub struct CodeBlockCopyButton {
    value: Reactive<SharedString>,
    disabled: Reactive<bool>,
}

impl CodeBlockCopyButton {
    pub fn new(value: impl IntoReactive<SharedString>) -> Self {
        Self {
            value: value.into_reactive(),
            disabled: false.into_reactive(),
        }
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }
}

impl IntoElement for CodeBlockCopyButton {
    fn into_element(self) -> AnyElement {
        let copied = signal(false);
        let reset = stored(None::<Spawned<()>>);
        Button::new("Copy")
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::ExtraSmall)
            .disabled(self.disabled)
            .on_press(move || {
                set_clipboard(self.value.get().to_string());
                copied.set(true);
                if let Some(task) = reset.replace(None) {
                    task.cancel();
                }
                reset.replace(Some(set_timeout(Duration::from_secs(2), move || {
                    copied.set(false)
                })));
            })
            .child(when(copied, || {
                text("Copied").text_xs().role(AccessibilityRole::Summary)
            }))
            .into_element()
    }
}

pub struct CodeBlockLine {
    text: SharedString,
    variant: CodeLineVariant,
}

impl CodeBlockLine {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            variant: CodeLineVariant::Default,
        }
    }

    pub fn variant(mut self, variant: CodeLineVariant) -> Self {
        self.variant = variant;
        self
    }
}

pub struct CodeBlockContent {
    source: Reactive<SharedString>,
    language: &'static str,
    variants: Vec<CodeLineVariant>,
    highlighted: &'static [usize],
    diff: bool,
    wrap: Reactive<bool>,
    numbered: bool,
    first: u32,
    limit: Reactive<Option<usize>>,
    max_height: Option<f32>,
    focusable: bool,
}

impl CodeBlockContent {
    pub fn new(source: impl IntoReactive<SharedString>) -> Self {
        Self {
            source: source.into_reactive(),
            language: "",
            variants: Vec::new(),
            highlighted: &[],
            diff: false,
            wrap: false.into_reactive(),
            numbered: false,
            first: 1,
            limit: None.into_reactive(),
            max_height: None,
            focusable: true,
        }
    }

    pub fn lines(lines: impl IntoIterator<Item = CodeBlockLine>) -> Self {
        let lines = lines.into_iter().collect::<Vec<_>>();
        let source = lines
            .iter()
            .map(|line| &*line.text)
            .collect::<Vec<_>>()
            .join("\n");
        let mut content = Self::new(source);
        content.variants = lines.into_iter().map(|line| line.variant).collect();
        content
    }

    pub fn language(mut self, language: &'static str) -> Self {
        self.language = language;
        self
    }

    /// Reads the first character of each line as a unified-diff marker.
    pub fn diff(mut self) -> Self {
        self.diff = true;
        self
    }

    /// Highlights the supplied one-based line positions.
    pub fn highlighted(mut self, positions: &'static [usize]) -> Self {
        self.highlighted = positions;
        self
    }

    pub fn wrap(mut self, wrap: impl IntoReactive<bool>) -> Self {
        self.wrap = wrap.into_reactive();
        self
    }

    pub fn line_numbers(mut self, start: u32) -> Self {
        self.numbered = true;
        self.first = start;
        self
    }

    pub fn limit(mut self, lines: impl IntoReactive<Option<usize>>) -> Self {
        self.limit = lines.into_reactive();
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height.max(0.0));
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    fn surface(self, font: f32, padding: f32, header: bool, footer: bool, floating: bool) -> View {
        let font_gap = font * 1.5;
        let source = self.source;
        let language = self.language;
        let wrap = computed(move || self.wrap.get());
        let limit = self.limit;
        let variants = self.variants;
        let numbered = self.numbered;
        let first = self.first;
        let diff = self.diff;
        let highlighted = self.highlighted;
        let syntax_ready = highlight::ready(language);
        let color = computed({
            let source = source.clone();
            move || {
                if language.is_empty() || (language != "ansi" && !syntax_ready()) {
                    return Vec::new();
                }
                let source_text = source.get();
                if language == "ansi" {
                    return highlight::ansi(&source_text);
                }
                let syntax_source = diff.then(|| {
                    source_text
                        .split('\n')
                        .map(|line| diff_line(line).0)
                        .collect::<Vec<_>>()
                        .join("\n")
                });
                highlight::syntax(
                    syntax_source.as_deref().unwrap_or(&source_text),
                    language,
                    dark(),
                )
            }
        });
        let render = move || {
            let source_text = source.get();
            let raw = source_text.split('\n');
            let lines = raw.clone().count();
            let count = limit.get().unwrap_or(lines).min(lines);
            let wraps = wrap.get();
            let gutter =
                font * 0.62 * (first.saturating_add(count as u32).to_string().len().max(3) as f32);
            let mut rows = view()
                .min_w(px(0.0))
                .selectable(true)
                .pt(px(if header { padding * 0.5 } else { padding }))
                .pb(px(if footer { padding * 0.5 } else { padding }));
            for (i, line) in raw.into_iter().take(count).enumerate() {
                let (line, variant) = if diff {
                    diff_line(line)
                } else {
                    let explicit = variants.get(i).copied().unwrap_or_default();
                    let variant =
                        if explicit == CodeLineVariant::Default && highlighted.contains(&(i + 1)) {
                            CodeLineVariant::Highlighted
                        } else {
                            explicit
                        };
                    (line, variant)
                };
                let body = color.with(|lines| {
                    let mut body = monospace("", font).line_height(px(font_gap));
                    if let Some(tokens) = lines.get(i).filter(|tokens| !tokens.is_empty()) {
                        for token in tokens {
                            let mut item = text(token.text.clone());
                            if let Some(color) = token.rgba {
                                item = item.text_color(rgba(color));
                            }
                            if token.dim {
                                item = item.text_color(theme().muted_foreground);
                            }
                            body = body.child(item);
                        }
                    } else {
                        body = body.child(text(if line.is_empty() {
                            " ".to_owned()
                        } else {
                            line.trim_end_matches('\r').to_owned()
                        }));
                    }
                    body
                });
                let row = view()
                    .flex_row()
                    .items_start()
                    .min_h(px(font_gap))
                    .px(px(padding))
                    .gap(px(9.0))
                    .when(!wraps, |row| {
                        row.min_w(px((line.chars().count() as f32 * font * 0.64)
                            + padding * 2.0
                            + if numbered { gutter + 9.0 } else { 0.0 }))
                    })
                    .when(variant != CodeLineVariant::Default, |row| {
                        row.bg(move || match variant {
                            CodeLineVariant::Removed => theme().destructive.opacity(0.10),
                            _ => theme().foreground.opacity(0.05),
                        })
                    })
                    .child(numbered.then(|| {
                        monospace(first.saturating_add(i as u32).to_string(), font)
                            .w(px(gutter))
                            .flex_shrink_0()
                            .text_right()
                            .line_height(px(font_gap))
                            .selectable(false)
                            .text_color(move || theme().muted_foreground)
                    }))
                    .child(match variant {
                        CodeLineVariant::Added | CodeLineVariant::Removed => Some(
                            monospace(
                                if variant == CodeLineVariant::Added {
                                    "+"
                                } else {
                                    "−"
                                },
                                font,
                            )
                            .line_height(px(font_gap))
                            .selectable(false),
                        ),
                        _ => None,
                    })
                    .child(body.flex_grow(1.0).when(wraps, |body| body.min_w(px(0.0))));
                rows = rows.child(row);
            }
            rows
        };
        let max_height = self.max_height;
        let focusable = self.focusable;
        let content = dynamic(move || {
            let rows = dynamic(render.clone());
            let body = if wrap.get() {
                view().child(rows).into_element()
            } else {
                scroll()
                    .horizontal()
                    .focusable(focusable && max_height.is_none())
                    .child(rows)
                    .into_element()
            };
            if let Some(height) = max_height {
                scroll()
                    .max_h(px(height))
                    .focusable(focusable)
                    .child(body)
                    .into_element()
            } else if wrap.get() {
                view().focusable(focusable).child(body).into_element()
            } else {
                body
            }
        });
        view()
            .direction(LayoutDirection::Ltr)
            .w_full()
            .min_w(px(0.0))
            .when(floating, |view| view.pr(px(42.0)))
            .label("Code")
            .child(content)
    }
}

fn diff_line(line: &str) -> (&str, CodeLineVariant) {
    match line.as_bytes().first() {
        Some(b'+') => (&line[1..], CodeLineVariant::Added),
        Some(b'-') => (&line[1..], CodeLineVariant::Removed),
        Some(b' ') => (&line[1..], CodeLineVariant::Default),
        _ => (line, CodeLineVariant::Default),
    }
}

fn monospace(value: impl IntoReactive<SharedString>, size: f32) -> Text {
    text(value).text_size(px(size)).font_family("monospace")
}
