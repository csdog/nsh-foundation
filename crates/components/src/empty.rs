//! Composable empty states with theme-aware media and responsive spacing.

use nsh_base::theme;
use nsh_core::{element::ElNode, prelude::*};

macro_rules! empty_container {
    ($( $name:ident ),+ $(,)?) => {
        $(
            pub struct $name(View);

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl IntoElement for $name {
                fn into_element(self) -> AnyElement {
                    self.0.into_element()
                }
            }

            impl Element for $name {
                fn node(&mut self) -> &mut ElNode {
                    self.0.node()
                }
            }

            impl ParentElement for $name {}
        )+
    };
}

empty_container!(Empty, EmptyHeader, EmptyMedia, EmptyContent);

impl Empty {
    pub fn new() -> Self {
        Self(
            view()
                .w_full()
                .min_w(px(0.0))
                .flex_1()
                .items_center()
                .justify_center()
                .gap_4()
                .rounded(px(24.0))
                .px(move || px(if window().width.0 < 640.0 { 16.0 } else { 48.0 }))
                .py(px(48.0)),
        )
    }
}

impl EmptyHeader {
    pub fn new() -> Self {
        Self(view().w_full().max_w(px(384.0)).items_center().gap_2())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMedia {
    pub fn new() -> Self {
        Self(
            view()
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .mb_2(),
        )
    }

    /// Adds a muted icon tile while allowing arbitrary content in the default variant.
    pub fn variant(self, variant: EmptyMediaVariant) -> Self {
        Self(self.0.when(variant == EmptyMediaVariant::Icon, |tile| {
            tile.size(px(40.0))
                .rounded(px(12.0))
                .bg(move || theme().muted)
                .text_color(move || theme().foreground)
        }))
    }
}

impl EmptyContent {
    pub fn new() -> Self {
        Self(
            view()
                .w_full()
                .max_w(px(384.0))
                .min_w(px(0.0))
                .items_center()
                .gap_4(),
        )
    }
}

pub struct EmptyTitle(Text);

impl EmptyTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(
            text(title)
                .text_lg()
                .font_weight(FontWeight::MEDIUM)
                .text_center()
                .text_color(move || theme().foreground),
        )
    }
}

impl IntoElement for EmptyTitle {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct EmptyDescription(View);

impl EmptyDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self::render(text(description).text_sm().text_center())
    }

    /// Accepts styled text, inline links or other composed content.
    pub fn render(content: impl IntoElement) -> Self {
        Self(
            view()
                .w_full()
                .items_center()
                .text_color(move || theme().muted_foreground)
                .child(content),
        )
    }
}

impl IntoElement for EmptyDescription {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}
