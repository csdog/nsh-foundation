use nsh_core::prelude::*;
use nsh_foundation::badge::{Badge, BadgeVariant};

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
    let rotation = animated_value(0.0);
    let animation =
        Animation::timing(&rotation, 360.0, Timing::new(800.0, Easing::Linear)).looped(None);
    effect({
        let animation = animation.clone();
        move || {
            if reduce_motion() {
                animation.stop();
            } else {
                let _ = animation.start();
            }
        }
    });
    cleanup(move || animation.stop());
    let color = move || {
        let t = nsh_base::theme();
        if destructive {
            t.destructive
        } else {
            t.secondary_foreground
        }
    };
    view()
        .w(px(10.0))
        .h(px(10.0))
        .rounded_full()
        .border(px(1.0))
        .border_color(move || color().opacity(0.30))
        .border_t_color(color)
        .bind_rotate(&rotation)
}
