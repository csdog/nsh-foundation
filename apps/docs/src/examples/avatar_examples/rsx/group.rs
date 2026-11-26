use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <AvatarGroup>
            <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")} grayscale>
                <AvatarFallback>CN</AvatarFallback>
            </Avatar>
            <Avatar name="@maxleiter" src={asset!("avatar-maxleiter.png")} grayscale>
                <AvatarFallback>LR</AvatarFallback>
            </Avatar>
            <Avatar name="@evilrabbit" src={asset!("avatar-evilrabbit.png")} grayscale>
                <AvatarFallback>ER</AvatarFallback>
            </Avatar>
        </AvatarGroup>
    }
}
