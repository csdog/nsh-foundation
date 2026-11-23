use components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_4()
        .child(
            Avatar::new("@shadcn")
                .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
                .fallback(AvatarFallback::new("CN"))
                .size(AvatarSize::Small),
        )
        .child(
            Avatar::new("@shadcn")
                .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
                .fallback(AvatarFallback::new("CN")),
        )
        .child(
            Avatar::new("@shadcn")
                .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
                .fallback(AvatarFallback::new("CN"))
                .size(AvatarSize::Large),
        )
}
