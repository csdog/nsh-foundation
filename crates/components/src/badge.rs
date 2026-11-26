//! Rhea-styled badges with optional inline icons and link behavior.

use nsh_base::theme;
use nsh_core::prelude::*;

pub mod tsx;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

pub struct Badge {
    label: Option<Reactive<SharedString>>,
    children: Vec<AnyElement>,
    variant: BadgeVariant,
    colors: Option<(AdaptiveColor, AdaptiveColor)>,
    start: Option<AnyElement>,
    end: Option<AnyElement>,
    href: Option<&'static str>,
}

impl Badge {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: Some(label.into_reactive()),
            ..badge()
        }
    }

    /// Adds visible badge content.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    fn empty() -> Self {
        Self {
            label: None,
            children: Vec::new(),
            variant: BadgeVariant::Default,
            colors: None,
            start: None,
            end: None,
            href: None,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn colors(mut self, background: AdaptiveColor, foreground: AdaptiveColor) -> Self {
        self.colors = Some((background, foreground));
        self
    }

    pub fn icon_start(mut self, icon: impl IntoElement) -> Self {
        self.start = Some(icon.into_element());
        self
    }

    pub fn icon_end(mut self, icon: impl IntoElement) -> Self {
        self.end = Some(icon.into_element());
        self
    }

    pub fn href(mut self, href: &'static str) -> Self {
        self.href = Some(href);
        self
    }
}

impl IntoElement for Badge {
    fn into_element(self) -> AnyElement {
        use BadgeVariant::{Default, Destructive, Ghost, Link, Outline, Secondary};
        let variant = self.variant;
        let colors = move || {
            self.colors.unwrap_or_else(|| {
                let t = theme();
                let transparent = AdaptiveColor::TRANSPARENT;
                match variant {
                    Default => (t.primary, t.primary_foreground),
                    Secondary => (t.secondary, t.secondary_foreground),
                    Destructive => (
                        AdaptiveColor::new(
                            t.destructive.light.opacity(0.1),
                            t.destructive.dark.opacity(0.2),
                        ),
                        t.destructive,
                    ),
                    Ghost | Outline => (transparent, t.foreground),
                    Link => (transparent, t.primary),
                }
            })
        };
        let label = self.label;
        let mut pill = view()
            .flex_row()
            .items_center()
            .gap_1()
            .h(px(20.0))
            .flex_shrink_0()
            .px_2()
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || {
                if variant == Outline {
                    theme().border
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .bg(move || colors().0)
            .text_color(move || colors().1);
        if variant == Ghost {
            pill = pill.hover(|s| s.bg(theme().muted));
        }
        if let Some(href) = self.href {
            pill = pill
                .role(AccessibilityRole::Link)
                .cursor_pointer()
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                .hover(move |s| match variant {
                    Outline | Ghost => s.bg(theme().muted),
                    Destructive => s.bg(theme().destructive.opacity(0.2)),
                    Link => s.underline(),
                    _ => s.bg(colors().0.opacity(0.8)),
                })
                .on_press(move || {
                    let _ = open_url(href);
                });
            if let Some(label) = &label {
                pill = pill.label(label.clone());
            }
        }
        if let Some(icon) = self.start {
            pill = pill.pl(px(6.0)).child(icon_frame(icon));
        }
        if let Some(label) = label {
            pill = pill.child(text(label).text_xs().font_weight(FontWeight::MEDIUM));
        }
        pill = pill.children(self.children);
        if let Some(icon) = self.end {
            pill = pill.pr(px(6.0)).child(icon_frame(icon));
        }
        pill.into_element()
    }
}

fn icon_frame(icon: AnyElement) -> View {
    view()
        .w(px(12.0))
        .h(px(12.0))
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .accessibility_elements_hidden(true)
        .child(icon)
}

/// Builds a badge with content supplied by its children.
pub fn badge() -> Badge {
    Badge::empty()
}
