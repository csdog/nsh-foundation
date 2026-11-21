//! Rhea checkbox styling and state using Native Shell press and accessibility APIs.

use nsh_base::theme;
use nsh_core::{AccessibilityState, geometry::Edges, prelude::*};

pub use nsh_core::Checked as CheckboxState;

pub struct Checkbox {
    label: SharedString,
    description: Option<SharedString>,
    checked: Option<Reactive<CheckboxState>>,
    initial: bool,
    disabled: Reactive<bool>,
    invalid: Reactive<bool>,
    change: Option<Box<dyn Fn(bool)>>,
    icon_only: bool,
    card: bool,
    id: Option<&'static str>,
}

impl Checkbox {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            description: None,
            checked: None,
            initial: false,
            disabled: false.into_reactive(),
            invalid: false.into_reactive(),
            change: None,
            icon_only: false,
            card: false,
            id: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.initial = checked;
        self
    }

    /// Controls the value. Use on_change to apply the requested change.
    pub fn checked(mut self, state: impl IntoReactive<CheckboxState>) -> Self {
        self.checked = Some(state.into_reactive());
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

    /// Receives the next boolean value. Pressing a mixed box requests true.
    pub fn on_change(mut self, change: impl Fn(bool) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    /// Shows the indicator and keeps the supplied label for screen readers.
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }

    pub fn card(mut self) -> Self {
        self.card = true;
        self
    }

    pub fn test_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn surface(self) -> View {
        let owned = signal(self.initial);
        let controlled = self.checked.is_some();
        let state = self.checked.unwrap_or_else(|| {
            (move || {
                if owned.get() {
                    CheckboxState::True
                } else {
                    CheckboxState::False
                }
            })
            .into_reactive()
        });
        let state = computed(move || state.get());
        let disabled = self.disabled;
        let disabled = computed(move || disabled.get());
        let invalid = self.invalid;
        let invalid = computed(move || invalid.get());
        let change = self.change;
        let filled = move || state.get() != CheckboxState::False;
        let mark_background = move || {
            if filled() {
                theme().primary
            } else {
                theme().input.opacity(0.9)
            }
        };
        let mark_border = move || {
            if filled() {
                theme().primary
            } else if invalid.get() {
                theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
            } else {
                AdaptiveColor::TRANSPARENT
            }
        };
        let mark = view()
            .w(px(16.0))
            .h(px(16.0))
            .flex_shrink_0()
            .rounded(px(5.0))
            .border(px(1.0))
            .bg(mark_background)
            .border_color(mark_border)
            .style_when(
                move || invalid.get(),
                |style| {
                    style
                        .ring(px(3.0))
                        .ring_color(theme().destructive.opacity(if dark() { 0.4 } else { 0.2 }))
                },
            )
            .test_id("checkbox-mark")
            .when(!self.card, |mark| {
                mark.group_focus_visible(|style| {
                    style
                        .border_color(theme().ring)
                        .ring(px(3.0))
                        .ring_color(theme().ring.opacity(0.3))
                })
            })
            .items_center()
            .justify_center()
            .child(when(
                move || state.get() == CheckboxState::True,
                || {
                    view()
                        .w(px(9.0))
                        .h(px(5.0))
                        .border_l(px(1.5))
                        .border_b(px(1.5))
                        .border_color(move || theme().primary_foreground)
                        .rotate(deg(-45.0))
                        .mb(px(2.0))
                },
            ))
            .child(when(
                move || state.get() == CheckboxState::Mixed,
                || {
                    view()
                        .w(px(8.0))
                        .h(px(2.0))
                        .rounded_full()
                        .bg(move || theme().primary_foreground)
                },
            ));
        let content = (!self.icon_only).then(|| {
            view()
                .flex_1()
                .min_w(px(0.0))
                .gap_1()
                .child(
                    text(self.label.clone())
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM),
                )
                .child(self.description.clone().map(|description| {
                    text(description)
                        .text_sm()
                        .text_color(move || theme().muted_foreground)
                }))
        });
        view()
            .group()
            .role(AccessibilityRole::Checkbox)
            .focusable(true)
            .label(self.label)
            .when_some(self.description, |control, description| {
                control.accessibility_description(description)
            })
            .accessibility_state(move || AccessibilityState {
                checked: Some(state.get()),
                invalid: invalid.get(),
                disabled: disabled.get(),
                ..Default::default()
            })
            .disabled(move || disabled.get())
            .on_press(move || {
                let next = state.get_untracked() != CheckboxState::True;
                if !controlled {
                    owned.set(next);
                }
                if let Some(change) = &change {
                    change(next);
                }
            })
            .flex_row()
            .gap(px(10.0))
            .min_w(px(0.0))
            .items_start()
            .rounded(px(5.0))
            .disabled_style(|style| style.opacity(0.5))
            .when(self.icon_only, |control| {
                control.hit_slop(Edges::all(px(6.0)))
            })
            .when(self.card, |control| {
                control
                    .p_3()
                    .rounded(px(12.0))
                    .focus_visible(|style| {
                        style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3))
                    })
                    .border(px(1.0))
                    .border_color(move || {
                        if filled() {
                            theme().primary
                        } else {
                            theme().border
                        }
                    })
                    .style_when(filled, |style| style.bg(theme().muted))
            })
            .when_some(self.id, |control, id| control.test_id(id))
            .child(mark)
            .child(content)
    }
}

impl IntoElement for Checkbox {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}
