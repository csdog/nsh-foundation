//! Native Shell documentation and component examples.

extern crate self as nsh_foundation;

pub mod accordion;
mod accordion_examples;
pub mod alert;
pub mod alert_dialog;
mod alert_dialog_examples;
mod alert_examples;
pub mod aspect_ratio;
mod aspect_ratio_examples;
pub mod attachment;
mod attachment_examples;
pub mod avatar;
mod avatar_examples;
pub mod badge;
mod badge_examples;
pub mod breadcrumb;
mod breadcrumb_examples;
pub mod bubble;
mod bubble_examples;
pub mod button;
mod button_examples;
pub mod button_group;
mod button_group_examples;
pub mod calendar;
mod calendar_examples;
pub mod card;
mod card_examples;
pub mod carousel;
mod carousel_examples;
pub mod chart;
mod chart_examples;
pub mod checkbox;
mod checkbox_examples;
mod docs;
pub mod drawer;
mod examples;
mod ui;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    registry.assets(assets!());
}

#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}
