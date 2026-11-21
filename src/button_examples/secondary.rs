use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonVariant};

pub fn preview() -> impl IntoElement {
    let presses = signal(0u32);
    view()
        .items_center()
        .gap_2()
        .child(
            Button::new("Secondary")
                .variant(ButtonVariant::Secondary)
                .on_press(move || presses.update(|count| *count += 1)),
        )
        .child(when(
            move || presses.get() > 0,
            move || text(move || format!("Pressed {}", presses.get())).text_xs(),
        ))
}
