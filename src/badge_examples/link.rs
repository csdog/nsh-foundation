use nsh_core::prelude::*;
use nsh_foundation::badge::Badge;

pub fn preview() -> impl IntoElement {
    Badge::new("Open Link")
        .href("https://ui.shadcn.com/docs/components/base/badge#link")
        .icon_end(text("↗").text_xs())
}
