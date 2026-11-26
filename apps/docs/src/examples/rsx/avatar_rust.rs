use components::avatar::{Avatar, AvatarFallback};
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    Avatar::new("Ada Lovelace").fallback(AvatarFallback::new("AL"))
}
