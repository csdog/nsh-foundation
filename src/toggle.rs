//! Rhea-styled toggle buttons built on Native Shell's Toggle.

use nsh_base::{ToggleElement, theme, toggle as native_toggle};
use nsh_core::prelude::*;

use crate::direction::use_direction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    Small,
    #[default]
    Default,
    Large,
}

impl ToggleSize {
    fn height(self) -> f32 {
        match self {
            Self::Small => 28.0,
            Self::Default => 32.0,
            Self::Large => 36.0,
        }
    }
}

type Icon = Box<dyn FnOnce(Memo<bool>) -> AnyElement>;

pub struct Toggle {
    text: Reactive<SharedString>,
    label: Option<Reactive<SharedString>>,
    icon: Option<Icon>,
    variant: ToggleVariant,
    size: ToggleSize,
    initial: bool,
    controlled: Option<Reactive<bool>>,
    disabled: Reactive<bool>,
    change: Option<Box<dyn Fn(bool)>>,
}

impl Toggle {
    pub fn new(text: impl IntoReactive<SharedString>) -> Self {
        Self {
            text: text.into_reactive(),
            label: None,
            icon: None,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            initial: false,
            controlled: None,
            disabled: false.into_reactive(),
            change: None,
        }
    }

    pub fn label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.label = Some(label.into_reactive());
        self
    }

    /// Render a decorative icon that can react to the current pressed state.
    pub fn icon<E: IntoElement>(mut self, icon: impl FnOnce(Memo<bool>) -> E + 'static) -> Self {
        self.icon = Some(Box::new(move |pressed| icon(pressed).into_element()));
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.initial = pressed;
        self
    }

    /// Use on_change to apply requested changes to the bound state.
    pub fn pressed(mut self, value: impl IntoReactive<bool>) -> Self {
        self.controlled = Some(value.into_reactive());
        self
    }

    pub fn on_change(mut self, change: impl Fn(bool) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn surface(self) -> ToggleElement {
        let (pressed, local) = match self.controlled {
            Some(pressed) => (pressed, None),
            None => {
                let local = signal(self.initial);
                (local.into_reactive(), Some(local))
            }
        };
        let pressed = computed(move || pressed.get());
        let label = self.label.unwrap_or_else(|| self.text.clone());
        let height = self.size.height();
        let outline = self.variant == ToggleVariant::Outline;
        let direction = use_direction();
        native_toggle()
            .pressed(pressed)
            .disabled(self.disabled)
            .accessibility_label(label.map(Some))
            .on_change(move |next: &bool| {
                if let Some(local) = local {
                    local.set(*next);
                }
                if let Some(change) = &self.change {
                    change(*next);
                }
            })
            .direction(direction.map(LayoutDirection::from))
            .self_start()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_1()
            .h(px(height))
            .min_w(px(height))
            .px(px(10.0))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || {
                if outline {
                    theme().input
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .bg(AdaptiveColor::TRANSPARENT)
            .text_color(|| theme().foreground)
            .hover(|style| style.bg(theme().muted))
            .style_when(pressed, |style| style.bg(theme().muted))
            .focus_visible(|style| {
                style
                    .border_color(theme().ring)
                    .ring(px(3.0))
                    .ring_color(theme().ring.opacity(0.3))
            })
            .disabled_style(|style| style.opacity(0.5))
            .style_when(
                || !reduce_motion(),
                |style| style.transition_colors().duration(150.0),
            )
            .test_id("toggle-control")
            .child(self.icon.map(|icon| {
                view()
                    .size(px(16.0))
                    .flex_shrink_0()
                    .pointer_events(PointerEvents::None)
                    .accessibility_elements_hidden(true)
                    .test_id("toggle-icon")
                    .child(icon(pressed))
            }))
            .child(
                text(self.text)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .number_of_lines(1),
            )
    }
}

impl IntoElement for Toggle {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}
