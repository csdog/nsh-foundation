use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")}>
            <AvatarFallback>CN</AvatarFallback>
            <AvatarBadge status="Online" color={AdaptiveColor::rgb(0x16a34a, 0x166534)} />
        </Avatar>
    }
}
