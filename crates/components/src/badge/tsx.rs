//! Native Shell TSX bindings for Badge.

use nsh_core::prelude::*;

use super::BadgeVariant;

/// Registers Badge in an app native module.
pub fn register(registry: &mut Registry) {
    registry.register::<Badge>();
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// The visual treatment.
    #[prop(default)]
    pub variant: Prop<BadgeVariant>,
    /// The visible badge content.
    pub children: Prop<Children>,
}

/// A compact label for a status or category.
#[component]
pub fn Badge(props: BadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    super::badge()
        .variant(props.variant.get())
        .child(props.children)
}
