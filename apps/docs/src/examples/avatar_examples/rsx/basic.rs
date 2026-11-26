use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")}>
            <AvatarFallback>CN</AvatarFallback>
        </Avatar>
    }
}
