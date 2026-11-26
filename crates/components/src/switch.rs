//! Rhea-styled switches driven by Native Shell's switch control.

use nsh_base::theme;
use nsh_core::{Checked, geometry::Edges, prelude::*};

use crate::direction::{Direction, use_direction};

pub mod tsx;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PropEnum)]
pub enum SwitchSize {
    Small,
    #[default]
    Default,
}

pub struct Switch {
    label: Option<Reactive<SharedString>>,
    description: Option<Reactive<SharedString>>,
    controlled: Option<Reactive<bool>>,
    initial: bool,
    disabled: Reactive<bool>,
    invalid: Reactive<bool>,
    size: SwitchSize,
    control: Option<ElementRef>,
    change: Option<Box<dyn Fn(bool)>>,
}

impl Switch {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        switch().accessibility_label(label)
    }

    /// Names the setting toggled by the switch for assistive technology.
    pub fn accessibility_label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.label = Some(label.into_reactive());
        self
    }

    fn empty() -> Self {
        Self {
            label: None,
            description: None,
            controlled: None,
            initial: false,
            disabled: false.into_reactive(),
            invalid: false.into_reactive(),
            size: SwitchSize::Default,
            control: None,
            change: None,
        }
    }

    pub fn description(mut self, description: impl IntoReactive<SharedString>) -> Self {
        self.description = Some(description.into_reactive());
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.initial = checked;
        self
    }

    /// When bound, apply changes to the shared value in on_change.
    pub fn checked(mut self, checked: impl IntoReactive<bool>) -> Self {
        self.controlled = Some(checked.into_reactive());
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

    pub fn invalid(mut self, invalid: impl IntoReactive<bool>) -> Self {
        self.invalid = invalid.into_reactive();
        self
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn control_ref(mut self, control: ElementRef) -> Self {
        self.control = Some(control);
        self
    }

    pub fn surface(self) -> View {
        let owned = signal(self.initial);
        let controlled = self.controlled.is_some();
        let checked = self.controlled.unwrap_or_else(|| owned.into_reactive());
        let checked = computed(move || checked.get());
        let disabled = computed(move || self.disabled.get());
        let invalid = computed(move || self.invalid.get());
        let direction = use_direction();
        let small = self.size == SwitchSize::Small;
        let thumb_size = if small { 12.0 } else { 16.0 };
        let travel = if small { 8.0 } else { 12.0 };
        let rtl = computed(move || direction.get() == Direction::Rtl);
        let thumb = view()
            .absolute()
            .top_0()
            .size(px(thumb_size))
            .rounded_full()
            .shadow_sm()
            .bg(move || {
                let t = theme();
                if dark() && checked.get() {
                    t.primary_foreground
                } else if dark() {
                    t.foreground
                } else {
                    t.background
                }
            })
            .style_with(move |style| {
                if rtl.get() {
                    style
                        .right_0()
                        .translate_x(px(if checked.get() { -travel } else { 0.0 }))
                } else {
                    style
                        .left_0()
                        .translate_x(px(if checked.get() { travel } else { 0.0 }))
                }
            })
            .style_when(
                || !reduce_motion(),
                |style| style.transition_transform().duration(150.0),
            )
            .pointer_events(PointerEvents::None)
            .accessibility_elements_hidden(true)
            .test_id("switch-thumb");

        let control = view()
            .role(AccessibilityRole::Switch)
            .focusable(move || !disabled.get())
            .disabled(disabled)
            .on_press(move || {
                if disabled.get_untracked() {
                    return;
                }
                let next = !checked.get_untracked();
                if !controlled {
                    owned.set(next);
                }
                if let Some(change) = &self.change {
                    change(next);
                }
            })
            .accessibility_state(move || AccessibilityState {
                checked: Some(if checked.get() {
                    Checked::True
                } else {
                    Checked::False
                }),
                disabled: disabled.get(),
                invalid: invalid.get(),
                ..Default::default()
            })
            .hit_slop(Edges {
                top: px(8.0),
                right: px(12.0),
                bottom: px(8.0),
                left: px(12.0),
            })
            .w(px(if small { 24.0 } else { 32.0 }))
            .h(px(if small { 16.0 } else { 20.0 }))
            .rounded(px(16.0))
            .flex_shrink_0()
            .border(px(2.0))
            .bg(move || {
                if checked.get() {
                    theme().primary
                } else {
                    theme().input.opacity(0.90)
                }
            })
            .border_color(move || {
                if invalid.get() {
                    theme().destructive
                } else if checked.get() {
                    theme().primary
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .style_when(
                || !reduce_motion(),
                |style| style.transition_colors().duration(150.0),
            )
            .style_when(invalid, |style| {
                style.ring(px(3.0)).ring_color(invalid_ring)
            })
            .focus_visible(|style| {
                style
                    .border_color(move || {
                        if invalid.get() {
                            theme().destructive
                        } else {
                            theme().ring
                        }
                    })
                    .ring(px(3.0))
                    .ring_color(move || {
                        if invalid.get() {
                            invalid_ring()
                        } else {
                            theme().ring.opacity(0.30)
                        }
                    })
            })
            .disabled_style(|style| style.opacity(0.5).cursor(Cursor::NotAllowed))
            .test_id("switch-control")
            .child(thumb);
        let control = match self.label {
            Some(label) => control.label(label),
            None => control,
        };
        let control = match self.description {
            Some(description) => control.accessibility_description(description),
            None => control,
        };
        match self.control {
            Some(reference) => control.element_ref(&reference),
            None => control,
        }
    }
}

/// Builds a switch that is off by default.
pub fn switch() -> Switch {
    Switch::empty()
}

impl IntoElement for Switch {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

fn invalid_ring() -> AdaptiveColor {
    theme().destructive.opacity(if dark() { 0.4 } else { 0.2 })
}
