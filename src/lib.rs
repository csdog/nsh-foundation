//! Native Shell documentation and component examples.

extern crate self as nsh_foundation;

pub mod accordion;
mod accordion_examples;
pub mod alert;
mod alert_examples;
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
