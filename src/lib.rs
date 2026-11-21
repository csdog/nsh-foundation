//! Native Shell documentation and component examples.

mod docs;
pub mod drawer;
mod examples;
mod ui;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
}

#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}
