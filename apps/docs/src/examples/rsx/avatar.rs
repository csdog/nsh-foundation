use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Avatar name="Ada Lovelace">
            <AvatarFallback>AL</AvatarFallback>
        </Avatar>
    }
}
