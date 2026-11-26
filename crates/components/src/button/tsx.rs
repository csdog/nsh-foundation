//! Native Shell TSX bindings for the styled Foundation button.

use nsh_core::prelude::*;

use super::{ButtonSize, ButtonVariant};

/// Registers the styled Button in an app native module.
pub fn register(registry: &mut Registry) {
    registry.register::<Button>();
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// The spoken name for buttons without text.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// The visual treatment.
    #[prop(default)]
    pub variant: Prop<ButtonVariant>,
    /// The control size.
    #[prop(default)]
    pub size: Prop<ButtonSize>,
    /// Prevents activation.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Shows a spinner and prevents activation.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// The visible button content.
    pub children: Prop<Children>,
}

#[derive(Debug, Clone, PartialEq, Event)]
pub enum ButtonEvent {
    /// The user activated the button.
    Press,
}

/// A focusable Foundation button with child content.
#[component]
pub fn Button(props: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    super::button()
        .variant(props.variant.get())
        .size(props.size.get())
        .disabled(props.disabled)
        .loading(props.loading)
        .child(props.children)
        .surface()
        .optional_label(props.accessibility_label)
        .on_press(move || cx.emit(ButtonEvent::Press))
}
