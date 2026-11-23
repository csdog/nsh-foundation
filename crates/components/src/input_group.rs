//! Compose one native input with inline and block addons.

use nsh_base::theme;
use nsh_core::{AccessibilityState, element::Input, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    input::bare_input,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputGroupAlign {
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

pub struct InputGroupAddon {
    align: InputGroupAlign,
    children: Vec<AnyElement>,
    interactive: bool,
    separator: bool,
}

impl InputGroupAddon {
    pub fn new(align: InputGroupAlign) -> Self {
        Self {
            align,
            children: Vec::new(),
            interactive: false,
            separator: false,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    /// Adds a button or other interactive content that handles its own focus.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.interactive = true;
        self.child(action)
    }

    /// Adds a rule between a block addon and the control.
    pub fn separator(mut self) -> Self {
        self.separator = true;
        self
    }

    fn render(self, control: ElementRef, disabled: Reactive<bool>) -> View {
        let block = matches!(
            self.align,
            InputGroupAlign::BlockStart | InputGroupAlign::BlockEnd
        );
        let can_focus = disabled.clone();
        view()
            .role(AccessibilityRole::Group)
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .gap_2()
            .px_2()
            .when(block, |part| part.w_full().py_1())
            .when(
                self.separator && self.align == InputGroupAlign::BlockStart,
                |part| part.border_b(px(1.0)),
            )
            .when(
                self.separator && self.align == InputGroupAlign::BlockEnd,
                |part| part.border_t(px(1.0)),
            )
            .border_color(move || theme().border)
            .style_when(disabled, |style| style.opacity(0.5))
            .when(!self.interactive, |part| {
                part.on_press(move || {
                    if !can_focus.get() {
                        control.focus();
                    }
                })
                .focusable(false)
                .role(AccessibilityRole::Group)
            })
            .children(self.children)
    }
}

/// A single native input or textarea with addons in logical layout order.
pub struct InputGroup {
    control: Input,
    control_ref: Option<ElementRef>,
    label: Option<Reactive<SharedString>>,
    addons: Vec<InputGroupAddon>,
    disabled: Option<Reactive<bool>>,
    invalid: Option<Reactive<bool>>,
}

impl InputGroup {
    pub fn new(control: Input) -> Self {
        Self {
            control,
            control_ref: None,
            label: None,
            addons: Vec::new(),
            disabled: None,
            invalid: None,
        }
    }

    pub fn addon(mut self, addon: InputGroupAddon) -> Self {
        self.addons.push(addon);
        self
    }

    /// Shares the control's focus handle with a visible label.
    pub fn control_ref(mut self, control_ref: ElementRef) -> Self {
        self.control_ref = Some(control_ref);
        self
    }

    pub fn label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.label = Some(label.into_reactive());
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = Some(disabled.into_reactive());
        self
    }

    pub fn invalid(mut self, invalid: impl IntoReactive<bool>) -> Self {
        self.invalid = Some(invalid.into_reactive());
        self
    }
}

impl IntoElement for InputGroup {
    fn into_element(self) -> AnyElement {
        let target = self.control_ref.unwrap_or_else(element_ref);
        let set_disabled = self.disabled.is_some();
        let set_invalid = self.invalid.is_some();
        let disabled = computed(move || self.disabled.as_ref().is_some_and(Reactive::get));
        let invalid = computed(move || self.invalid.as_ref().is_some_and(Reactive::get));
        let (mut top, mut before, mut after, mut bottom) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for addon in self.addons {
            let slots = match addon.align {
                InputGroupAlign::InlineStart => &mut before,
                InputGroupAlign::InlineEnd => &mut after,
                InputGroupAlign::BlockStart => &mut top,
                InputGroupAlign::BlockEnd => &mut bottom,
            };
            slots.push(addon.render(target, (move || disabled.get()).into_reactive()));
        }
        let control = self
            .control
            .element_ref(&target)
            .when(set_disabled, |input| input.disabled(move || disabled.get()))
            .disabled_style(|style| style.opacity(0.5))
            .when(set_invalid, |input| {
                input.accessibility_state(move || AccessibilityState {
                    invalid: invalid.get(),
                    ..Default::default()
                })
            })
            .flex_1();
        view()
            .role(AccessibilityRole::Group)
            .optional_label(
                self.label
                    .map_or(Reactive::Static(None), |label| label.map(Some)),
            )
            .w_full()
            .min_w(px(0.0))
            .rounded(px(16.0))
            .bg(move || theme().input.opacity(0.5))
            .border(px(1.0))
            .border_color(move || {
                if invalid.get() {
                    theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .transition()
            .duration(200.0)
            .focus_within(move |style| {
                style
                    .border_color(move || {
                        if invalid.get() {
                            theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
                        } else {
                            theme().ring
                        }
                    })
                    .ring(px(3.0))
                    .ring_color(move || {
                        if invalid.get() {
                            invalid_ring()
                        } else {
                            theme().ring.opacity(0.3)
                        }
                    })
            })
            .style_when(invalid, |style| {
                style.ring(px(3.0)).ring_color(invalid_ring)
            })
            .children(top)
            .child(
                view()
                    .min_w(px(0.0))
                    .flex_row()
                    .items_center()
                    .children(before)
                    .child(control)
                    .children(after),
            )
            .children(bottom)
            .into_element()
    }
}

fn invalid_ring() -> AdaptiveColor {
    theme().destructive.opacity(if dark() { 0.4 } else { 0.2 })
}

pub fn input_group_input() -> Input {
    bare_input()
}

pub fn input_group_textarea() -> Input {
    bare_input().multiline(true).h(px(80.0)).py_2()
}

pub fn input_group_text(text_value: impl IntoReactive<SharedString>) -> Text {
    text(text_value)
        .text_sm()
        .text_color(move || theme().muted_foreground)
}

pub fn input_group_button(label: impl IntoReactive<SharedString>) -> Button {
    Button::new(label)
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::ExtraSmall)
        .radius(12.0)
}
