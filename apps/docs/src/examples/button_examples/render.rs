use components::button::{Button, ButtonSize, ButtonVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Button::new("Login")
        .variant(ButtonVariant::Secondary)
        .size(ButtonSize::Small)
        .href("https://ui.shadcn.com/docs/components/base/button#as-link")
}
