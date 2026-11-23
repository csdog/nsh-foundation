use components::avatar::{Avatar, AvatarFallback, AvatarImage};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Avatar::new("@shadcn")
        .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
        .fallback(AvatarFallback::new("CN"))
}
