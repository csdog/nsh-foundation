use components::badge::{BadgeVariant, badge};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    badge()
        .variant(BadgeVariant::Outline)
        .child(text("In progress"))
}
