//! Native Shell component gallery and Web entry point.

mod blocks;
mod demo_source;
mod docs;
mod examples;
pub mod native_examples;
mod tsx_examples;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    registry.register::<WebGallery>();
    components::drawer::tsx::register(registry);
    components::avatar::tsx::register(registry);
    components::badge::tsx::register(registry);
    components::button::tsx::register(registry);
    components::switch::tsx::register(registry);
    native_examples::register(registry);
    registry.assets(assets!());
}

/// The gallery's Rust entry point.
#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}

#[derive(Clone, PartialEq, Props)]
pub struct WebGalleryProps {
    /// The selected example rendered by the Native Shell TSX entry point.
    pub example: Prop<Option<Slot>>,
}

#[derive(Clone, PartialEq, Event)]
pub enum WebGalleryEvent {
    /// Selects an example by ID. An empty ID closes the current example.
    #[args(id)]
    RunTsx(SharedString),
}

/// The gallery with an inline slot for running Native Shell TSX examples.
#[component]
pub fn WebGallery(props: WebGalleryProps, cx: Scope<WebGalleryEvent>) -> impl IntoElement {
    provide_context(tsx_examples::TsxRunner {
        selected: signal(None),
        slot: props.example,
        scope: cx,
    });
    docs::page()
}
