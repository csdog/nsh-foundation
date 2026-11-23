use components::badge::{Badge, BadgeVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(
            Badge::new("Deleting")
                .variant(BadgeVariant::Destructive)
                .icon_start(spinner(true)),
        )
        .child(
            Badge::new("Generating")
                .variant(BadgeVariant::Secondary)
                .icon_end(spinner(false)),
        )
}

fn spinner(destructive: bool) -> View {
    components::spinner::ring(10.0, move || {
        let t = nsh_base::theme();
        if destructive {
            t.destructive
        } else {
            t.secondary_foreground
        }
    })
}
