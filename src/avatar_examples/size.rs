use nsh_core::prelude::*;
use nsh_foundation::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};

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
