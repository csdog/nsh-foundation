use components::badge::Badge;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Badge::new("Open Link")
        .href("https://ui.shadcn.com/docs/components/base/badge#link")
        .icon_end(text("↗").text_xs())
}
