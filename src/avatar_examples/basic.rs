use nsh_core::prelude::*;
use nsh_foundation::avatar::{Avatar, AvatarFallback, AvatarImage};

pub fn preview() -> impl IntoElement {
    Avatar::new("@shadcn")
        .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
        .fallback(AvatarFallback::new("CN"))
}
