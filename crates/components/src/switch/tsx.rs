//! Native Shell TSX bindings for the Foundation switch.

use nsh_core::prelude::*;

use super::SwitchSize;

/// Registers the styled Switch in an app native module.
pub fn register(registry: &mut Registry) {
    registry.register::<Switch>();
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// The setting that this switch controls.
    pub accessibility_label: Prop<SharedString>,
    /// The current value. Update it from onChange to accept a toggle.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// Prevents activation.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Highlights a validation error.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// The control size.
    #[prop(default)]
    pub size: Prop<SwitchSize>,
}

#[derive(Debug, Clone, PartialEq, Event)]
pub enum SwitchEvent {
    /// The checked value requested by the user.
    #[args(checked)]
    Change(bool),
}

/// A styled, controlled switch for an on or off setting.
#[component]
pub fn Switch(props: SwitchProps, cx: Scope<SwitchEvent>) -> impl IntoElement {
    super::switch()
        .accessibility_label(props.accessibility_label)
        .checked(props.checked)
        .disabled(props.disabled)
        .invalid(props.invalid)
        .size(props.size.get())
        .on_change(move |checked| cx.emit(SwitchEvent::Change(checked)))
}
