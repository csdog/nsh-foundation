use components::button::{Button, ButtonVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let status = signal("");
    view()
        .items_center()
        .gap_2()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_2()
                .child(
                    Button::new("New Branch")
                        .variant(ButtonVariant::Outline)
                        .icon_start(text("⑂"))
                        .on_press(move || status.set("New branch selected")),
                )
                .child(
                    Button::new("Fork")
                        .variant(ButtonVariant::Outline)
                        .icon_end(text("⑂"))
                        .on_press(move || status.set("Fork selected")),
                ),
        )
        .child(text(move || status.get()).text_sm())
}
