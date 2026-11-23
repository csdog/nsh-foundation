use components::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage,
};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_4()
        .child(
            Avatar::new("@shadcn")
                .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
                .fallback(AvatarFallback::new("CN")),
        )
        .child(
            Avatar::new("@evilrabbit")
                .image(AvatarImage::new(asset!("avatar-evilrabbit.png")))
                .fallback(AvatarFallback::new("ER"))
                .badge(AvatarBadge::new("Online").color(AdaptiveColor::rgb(0x16a34a, 0x166534))),
        )
        .child(
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
                .count(AvatarGroupCount::new("3 more people", text("+3").text_sm())),
        )
}
