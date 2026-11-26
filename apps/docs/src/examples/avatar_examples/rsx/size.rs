use components::{avatar::AvatarSize, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View style={|style| style.flex_row().items_center().gap_4()}>
            <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")} size={AvatarSize::Small}>
                <AvatarFallback>CN</AvatarFallback>
            </Avatar>
            <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")}>
                <AvatarFallback>CN</AvatarFallback>
            </Avatar>
            <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")} size={AvatarSize::Large}>
                <AvatarFallback>CN</AvatarFallback>
            </Avatar>
        </View>
    }
}
