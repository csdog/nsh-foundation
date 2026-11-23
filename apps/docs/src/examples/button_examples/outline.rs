use components::button::{Button, ButtonVariant};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let presses = signal(0u32);
    view()
        .items_center()
        .gap_2()
        .child(
            Button::new("Outline")
                .variant(ButtonVariant::Outline)
                .on_press(move || presses.update(|count| *count += 1)),
        )
        .child(when(
            move || presses.get() > 0,
            move || text(move || format!("Pressed {}", presses.get())).text_xs(),
        ))
}
