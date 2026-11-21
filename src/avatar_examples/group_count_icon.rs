use nsh_core::prelude::*;
use nsh_foundation::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};

pub fn preview() -> impl IntoElement {
    AvatarGroup::new()
        .child(
            Avatar::new("@shadcn")
                .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
                .fallback(AvatarFallback::new("CN"))
                .grayscale(true),
        )
        .child(
            Avatar::new("@maxleiter")
                .image(AvatarImage::new(asset!("avatar-maxleiter.png")))
                .fallback(AvatarFallback::new("LR"))
                .grayscale(true),
        )
        .child(
            Avatar::new("@evilrabbit")
                .image(AvatarImage::new(asset!("avatar-evilrabbit.png")))
                .fallback(AvatarFallback::new("ER"))
                .grayscale(true),
        )
        .count(AvatarGroupCount::new("3 more people", text("+").text_sm()))
}
