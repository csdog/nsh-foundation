use components::button::{Button, ButtonSize, ButtonVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let started = signal(false);
    view()
        .items_center()
        .gap_2()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Get Started")
                        .radius(999.0)
                        .on_press(move || started.set(true)),
                )
                .child(
                    Button::new("Continue")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .radius(999.0)
                        .icon_start(text("↑"))
                        .on_press(move || started.set(true)),
                ),
        )
        .child(when(started, || text("Started in this preview").text_sm()))
}
