use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    let selected = signal("");
    view()
        .items_center()
        .gap_2()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Button")
                        .variant(ButtonVariant::Outline)
                        .on_press(move || selected.set("Button pressed")),
                )
                .child(
                    Button::new("Submit")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .icon_start(text("↑"))
                        .on_press(move || selected.set("Submitted in this preview")),
                ),
        )
        .child(text(move || selected.get()).text_sm())
}
