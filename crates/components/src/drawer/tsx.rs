//! Native Shell TSX bindings for Drawer parts.

use nsh_core::prelude::*;

use super::{Drawer, DrawerContext, drawer_from_root};
use crate::ui::{Variant, button_container};

/// Registers the Drawer components in the app's native module.
pub fn register(registry: &mut Registry) {
    registry
        .register::<DrawerRoot>()
        .register::<DrawerTrigger>()
        .register::<DrawerContent>()
        .register::<DrawerClose>();
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DrawerRootProps {
    pub children: Prop<Children>,
}

/// Shares an open state with its Drawer parts.
#[component]
pub fn DrawerRoot(props: DrawerRootProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    provide_context(DrawerContext(drawer));
    view().child(props.children)
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DrawerTriggerProps {
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}

/// Opens the nearest DrawerRoot.
#[component]
pub fn DrawerTrigger(props: DrawerTriggerProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    super::drawer_trigger()
        .accessibility_label(props.accessibility_label)
        .child(props.children)
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DrawerContentProps {
    pub accessibility_label: Prop<SharedString>,
    pub children: Prop<Children>,
}

/// The Drawer panel and its window layer.
#[component]
pub fn DrawerContent(props: DrawerContentProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    super::DrawerContent::from_root(props.accessibility_label, move || {
        view().child(props.children)
    })
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DrawerCloseProps {
    pub accessibility_label: Prop<Option<SharedString>>,
    pub children: Prop<Children>,
}

/// Closes the nearest DrawerRoot.
#[component]
pub fn DrawerClose(props: DrawerCloseProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    super::DrawerClose::render(
        drawer_from_root(),
        button_container(Variant::Primary)
            .optional_label(props.accessibility_label)
            .child(props.children),
    )
}
