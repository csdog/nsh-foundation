use nsh_core::prelude::*;
use nsh_foundation::badge::{Badge, BadgeVariant};

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(Badge::new("Badge"))
        .child(Badge::new("Secondary").variant(BadgeVariant::Secondary))
        .child(Badge::new("Destructive").variant(BadgeVariant::Destructive))
        .child(Badge::new("Outline").variant(BadgeVariant::Outline))
}
