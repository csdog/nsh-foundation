use nsh_core::prelude::*;
use nsh_foundation::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

pub fn preview() -> impl IntoElement {
    Avatar::new("@pranathip")
        .image(AvatarImage::new(asset!("avatar-pranathip.png")))
        .fallback(AvatarFallback::new("PP"))
        .grayscale(true)
        .badge(AvatarBadge::new("New").icon(text("+").text_size(px(8.0))))
}
