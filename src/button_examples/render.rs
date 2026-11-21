use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    Button::new("Login")
        .variant(ButtonVariant::Secondary)
        .size(ButtonSize::Small)
        .href("https://ui.shadcn.com/docs/components/base/button#as-link")
}
