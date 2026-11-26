//! Native Shell component gallery and Web entry point.

mod blocks;
mod docs;
mod examples;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    components::drawer::tsx::register(registry);
    registry.assets(assets!());
}

#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}
