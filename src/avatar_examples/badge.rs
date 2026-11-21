use nsh_core::prelude::*;
use nsh_foundation::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

pub fn preview() -> impl IntoElement {
    Avatar::new("@shadcn")
        .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
        .fallback(AvatarFallback::new("CN"))
        .badge(AvatarBadge::new("Online").color(AdaptiveColor::rgb(0x16a34a, 0x166534)))
}
