//! Native Shell component gallery and Web entry point.

mod blocks;
mod demo_source;
mod docs;
mod examples;
pub mod native_examples;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    components::drawer::tsx::register(registry);
    components::avatar::tsx::register(registry);
    components::badge::tsx::register(registry);
    components::button::tsx::register(registry);
    components::switch::tsx::register(registry);
    native_examples::register(registry);
    registry.assets(assets!());
}

#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}
