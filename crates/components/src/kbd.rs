//! Inline keyboard labels and shortcut groups.

use nsh_base::theme;
use nsh_core::{element::ElNode, prelude::*};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum KbdTone {
    #[default]
    Default,
    Tooltip,
    InputGroup,
}

pub struct Kbd {
    text: Reactive<SharedString>,
    name: Option<Reactive<SharedString>>,
    tone: KbdTone,
}

impl Kbd {
    pub fn new(text: impl IntoReactive<SharedString>) -> Self {
        Self {
            text: text.into_reactive(),
            name: None,
            tone: KbdTone::Default,
        }
    }

    /// Supplies a spoken name for a symbol, such as “Command” for “⌘”.
    pub fn named(mut self, name: impl IntoReactive<SharedString>) -> Self {
        self.name = Some(name.into_reactive());
        self
    }

    pub fn tone(mut self, tone: KbdTone) -> Self {
        self.tone = tone;
        self
    }
}

impl IntoElement for Kbd {
    fn into_element(self) -> AnyElement {
        let tone = self.tone;
        view()
            .h(px(20.0))
            .min_w(px(20.0))
            .flex_shrink_0()
            .px_1()
            .rounded(px(8.0))
            .items_center()
            .justify_center()
            .pointer_events(PointerEvents::None)
            .bg(move || match tone {
                KbdTone::Default => theme().muted,
                KbdTone::Tooltip => theme().background.opacity(0.2),
                KbdTone::InputGroup => theme().input,
            })
            .child(
                text(self.text)
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .selectable(false)
                    .optional_label(
                        self.name
                            .map_or(Reactive::Static(None), |name| name.map(Some)),
                    )
                    .text_color(move || match tone {
                        KbdTone::Tooltip => theme().background,
                        KbdTone::Default | KbdTone::InputGroup => theme().muted_foreground,
                    }),
            )
            .into_element()
    }
}

pub struct KbdGroup(View);

impl KbdGroup {
    pub fn new() -> Self {
        Self(
            view()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_1()
                .min_w(px(0.0)),
        )
    }
}

impl Default for KbdGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Element for KbdGroup {
    fn node(&mut self) -> &mut ElNode {
        self.0.node()
    }
}

impl ParentElement for KbdGroup {}

impl IntoElement for KbdGroup {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}
