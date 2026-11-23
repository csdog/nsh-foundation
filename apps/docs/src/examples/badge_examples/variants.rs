use components::badge::{Badge, BadgeVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(Badge::new("Default"))
        .child(Badge::new("Secondary").variant(BadgeVariant::Secondary))
        .child(Badge::new("Destructive").variant(BadgeVariant::Destructive))
        .child(Badge::new("Outline").variant(BadgeVariant::Outline))
        .child(Badge::new("Ghost").variant(BadgeVariant::Ghost))
}
