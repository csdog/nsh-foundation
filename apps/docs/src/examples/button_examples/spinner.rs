use components::button::{Button, ButtonVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .child(
            Button::new("Generating")
                .variant(ButtonVariant::Outline)
                .loading(true),
        )
        .child(
            Button::new("Downloading")
                .variant(ButtonVariant::Secondary)
                .loading(true),
        )
}
