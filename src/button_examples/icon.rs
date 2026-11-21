use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    let submitted = signal(false);
    view()
        .items_center()
        .gap_2()
        .child(
            Button::new("Submit")
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Icon)
                .icon_start(text("↑"))
                .on_press(move || submitted.set(true)),
        )
        .child(when(submitted, || {
            text("Submitted in this preview").text_sm()
        }))
}
