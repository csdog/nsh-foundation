use nsh_core::prelude::*;
use nsh_foundation::badge::{Badge, BadgeVariant};

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(
            Badge::new("Verified")
                .variant(BadgeVariant::Secondary)
                .icon_start(text("✓").text_xs()),
        )
        .child(
            Badge::new("Bookmark")
                .variant(BadgeVariant::Outline)
                .icon_end(text("♧").text_xs()),
        )
}
