use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <Avatar name="@pranathip" src={asset!("avatar-pranathip.png")} grayscale>
            <AvatarFallback>PP</AvatarFallback>
            <AvatarBadge status="New" icon={text("+").text_size(px(8.0))} />
        </Avatar>
    }
}
