//! Message bubbles, message groups, and edge-anchored reactions.

use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReactionSide {
    Top,
    #[default]
    Bottom,
}

pub struct Bubble {
    content: BubbleContent,
    reactions: Option<BubbleReactions>,
    align: BubbleAlign,
    variant: BubbleVariant,
}

impl Bubble {
    pub fn new(content: BubbleContent) -> Self {
        Self {
            content,
            reactions: None,
            align: BubbleAlign::Start,
            variant: BubbleVariant::Default,
        }
    }

    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }

    pub fn reactions(mut self, reactions: BubbleReactions) -> Self {
        self.reactions = Some(reactions);
        self
    }
}

impl IntoElement for Bubble {
    fn into_element(self) -> AnyElement {
        let ghost = self.variant == BubbleVariant::Ghost;
        let end = self.align == BubbleAlign::End;
        let mut frame = view()
            .min_w(px(0.0))
            .max_w(percentage(if ghost { 1.0 } else { 0.8 }))
            .when(end, |view| view.self_end())
            .when(!end, |view| view.self_start())
            .child(self.content.render(self.variant));
        if let Some(reactions) = self.reactions {
            frame = frame
                .when(reactions.side == ReactionSide::Top, |frame| {
                    frame.mt(px(16.0))
                })
                .when(reactions.side == ReactionSide::Bottom, |frame| {
                    frame.mb(px(16.0))
                })
                .child(reactions);
        }
        frame.into_element()
    }
}

pub struct BubbleContent {
    surface: View,
    interactive: bool,
}

impl BubbleContent {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            surface: view().child(child),
            interactive: false,
        }
    }

    pub fn text(message: impl IntoReactive<SharedString>) -> Self {
        Self::new(text(message).text_sm().line_height(px(20.0)))
    }

    pub fn button(label: &'static str, press: impl Fn() + 'static) -> Self {
        Self::interactive(
            view()
                .role(AccessibilityRole::Button)
                .label(label)
                .on_press(press)
                .child(text(label).text_sm()),
        )
    }

    pub fn link(label: &'static str, href: &'static str) -> Self {
        Self::interactive(
            view()
                .role(AccessibilityRole::Link)
                .label(label)
                .on_press(move || {
                    let _ = open_url(href);
                })
                .child(text(label).text_sm()),
        )
    }

    pub fn interactive(surface: View) -> Self {
        Self {
            surface,
            interactive: true,
        }
    }

    fn render(self, variant: BubbleVariant) -> View {
        use BubbleVariant::{Default, Destructive, Ghost, Muted, Outline, Secondary, Tinted};
        let colors = move || {
            let t = theme();
            match variant {
                Default => (t.primary, t.primary_foreground),
                Secondary => (t.secondary, t.secondary_foreground),
                Muted => (t.muted, t.foreground),
                Tinted => (
                    AdaptiveColor::new(
                        hsla(t.primary.light.h, 0.32, 0.93, 1.0),
                        hsla(t.primary.dark.h, 0.28, 0.20, 1.0),
                    ),
                    t.foreground,
                ),
                Outline => (t.background, t.foreground),
                Ghost => (AdaptiveColor::TRANSPARENT, t.foreground),
                Destructive => (
                    AdaptiveColor::new(
                        t.destructive.light.opacity(0.1),
                        t.destructive.dark.opacity(0.2),
                    ),
                    t.destructive,
                ),
            }
        };
        self.surface
            .min_w(px(0.0))
            .bg(move || colors().0)
            .text_color(move || colors().1)
            .when(variant != Ghost, |surface| {
                surface
                    .rounded(px(24.0))
                    .px_3()
                    .py(px(10.0))
                    .border(px(1.0))
                    .border_color(move || {
                        if variant == Outline {
                            theme().border
                        } else {
                            AdaptiveColor::TRANSPARENT
                        }
                    })
            })
            .when(self.interactive, |surface| {
                surface
                    .cursor_pointer()
                    .focus_visible(|s| s.ring(px(3.0)).ring_color(theme().ring.opacity(0.3)))
                    .hover(move |style| {
                        if matches!(variant, Outline | Ghost) {
                            style.bg(theme().muted)
                        } else {
                            style.opacity(0.85)
                        }
                    })
            })
    }
}

pub struct BubbleGroup(View);

impl BubbleGroup {
    pub fn new() -> Self {
        Self(view().role(AccessibilityRole::Group).min_w(px(0.0)).gap_2())
    }

    pub fn child(mut self, bubble: Bubble) -> Self {
        self.0 = self.0.child(bubble);
        self
    }
}

impl Default for BubbleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for BubbleGroup {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct BubbleReactions {
    children: View,
    label: Option<&'static str>,
    side: ReactionSide,
    align: BubbleAlign,
}

impl BubbleReactions {
    pub fn new() -> Self {
        Self {
            children: view().flex_row().items_center().gap_1(),
            label: None,
            side: ReactionSide::Bottom,
            align: BubbleAlign::End,
        }
    }

    pub fn child(mut self, content: impl IntoElement) -> Self {
        self.children = self.children.child(content);
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn side(mut self, side: ReactionSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for BubbleReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for BubbleReactions {
    fn into_element(self) -> AnyElement {
        let top = self.side == ReactionSide::Top;
        let start = self.align == BubbleAlign::Start;
        let label = self.label;
        view()
            .absolute()
            .z_index(10)
            .rounded_full()
            .bg(move || theme().muted)
            .text_color(move || theme().foreground)
            .ring(px(3.0))
            .ring_color(move || theme().card)
            .when(top, |view| view.top_0().translate_y(percentage(-0.75)))
            .when(!top, |view| view.bottom_0().translate_y(percentage(0.75)))
            .when(start, |view| view.left(px(12.0)))
            .when(!start, |view| view.right(px(12.0)))
            .when(label.is_some(), |view| {
                view.role(AccessibilityRole::Image)
                    .label(label.unwrap_or_default())
                    .px(px(6.0))
                    .py(px(2.0))
            })
            .child(self.children.when(label.is_some(), |row| {
                row.accessibility_elements_hidden(true)
            }))
            .into_element()
    }
}
