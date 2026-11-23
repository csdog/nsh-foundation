//! Composable inline markers for conversation threads.

use std::{cell::RefCell, time::Duration};

use nsh_base::{link, theme};
use nsh_core::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkerVariant {
    #[default]
    Default,
    Border,
    Separator,
}

pub struct Marker {
    variant: MarkerVariant,
    icon: Option<MarkerIcon>,
    content: MarkerContent,
    stacked: bool,
    status: bool,
}

impl Marker {
    pub fn new(content: MarkerContent) -> Self {
        Self {
            variant: MarkerVariant::Default,
            icon: None,
            content,
            stacked: false,
            status: false,
        }
    }

    pub fn icon(mut self, icon: MarkerIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn variant(mut self, variant: MarkerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn stacked(mut self) -> Self {
        self.stacked = true;
        self
    }

    /// Announces changes to the content through Native Shell's live region.
    pub fn status(mut self) -> Self {
        self.status = true;
        self
    }

    pub fn href(self, href: &'static str) -> impl IntoElement {
        let label = self.content.label.clone();
        link()
            .href(href)
            .accessibility_label(move || label.get())
            .w_full()
            .min_w(px(0.0))
            .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
            .child(
                self.surface()
                    .underline()
                    .hover(|style| style.text_color(theme().foreground)),
            )
    }

    pub fn on_press(self, handler: impl Fn() + 'static) -> View {
        let label = self.content.label.clone();
        self.surface()
            .on_press(handler)
            .role(AccessibilityRole::Button)
            .label(label)
            .cursor_pointer()
            .hover(|style| style.text_color(theme().foreground))
            .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
    }

    pub fn surface(self) -> View {
        if self.status {
            let label = self.content.label.clone();
            let previous = RefCell::new(None);
            effect(move || {
                let value = label.get();
                let previous = previous.replace(Some(value.clone()));
                if previous.is_some_and(|old| old != value) && !value.is_empty() {
                    announce_for_accessibility(value.to_string());
                }
            });
        }

        let mut parts = view().min_w(px(0.0)).gap_2().items_center();
        parts = if self.stacked {
            parts.flex_col()
        } else {
            parts.flex_row()
        };
        if let Some(icon) = self.icon {
            parts = parts.child(icon);
        }
        let mut root = view()
            .w_full()
            .min_h(px(16.0))
            .flex_row()
            .items_center()
            .text_color(|| theme().muted_foreground);
        match self.variant {
            MarkerVariant::Default => (),
            MarkerVariant::Border => {
                root = root
                    .border_b(px(1.0))
                    .border_color(|| theme().border)
                    .pb_2();
            }
            MarkerVariant::Separator => {
                root = root.gap_3().child(divider());
                parts = parts.text_center();
            }
        }
        root = root.child(parts.child(self.content));
        if self.variant == MarkerVariant::Separator {
            root = root.child(divider());
        }
        root
    }
}

impl IntoElement for Marker {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

fn divider() -> View {
    view()
        .test_id("marker-divider")
        .flex_1()
        .min_w(px(0.0))
        .h(px(1.0))
        .bg(|| theme().border)
        .accessibility_elements_hidden(true)
}

pub struct MarkerIcon(AnyElement);

impl MarkerIcon {
    pub fn new(icon: impl IntoElement) -> Self {
        Self(icon.into_element())
    }
}

impl IntoElement for MarkerIcon {
    fn into_element(self) -> AnyElement {
        view()
            .w(px(16.0))
            .h(px(16.0))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .accessibility_elements_hidden(true)
            .child(self.0)
            .into_element()
    }
}

pub struct MarkerContent {
    label: Reactive<SharedString>,
    shimmer: bool,
}

impl MarkerContent {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: label.into_reactive(),
            shimmer: false,
        }
    }

    pub fn shimmer(mut self) -> Self {
        self.shimmer = true;
        self
    }
}

impl IntoElement for MarkerContent {
    fn into_element(self) -> AnyElement {
        let label = self.label;
        let copy = text(label.clone()).text_sm().line_height(px(20.0));
        if !self.shimmer {
            return copy.min_w(px(0.0)).into_element();
        }
        view()
            .relative()
            .min_w(px(0.0))
            .child(copy)
            .child(when(move || !reduce_motion(), move || shine(label.clone())))
            .into_element()
    }
}

pub(crate) fn shine(label: Reactive<SharedString>) -> View {
    let step = signal(0_u32);
    let timer = set_interval(Duration::from_millis(40), move || {
        step.update(|step| *step = (*step + 1) % 50);
    });
    cleanup(move || timer.cancel());
    view()
        .test_id("marker-shimmer")
        .absolute()
        .inset_0()
        .pointer_events(PointerEvents::None)
        .accessibility_elements_hidden(true)
        .mask_linear(deg(90.0), move || {
            let center = step.get() as f32 / 49.0 * 1.5 - 0.25;
            vec![
                (transparent_black(), percentage(center - 0.25)).into(),
                (black(), percentage(center)).into(),
                (transparent_black(), percentage(center + 0.25)).into(),
            ]
        })
        .child(
            text(label)
                .text_sm()
                .line_height(px(20.0))
                .text_color(|| theme().foreground),
        )
}
