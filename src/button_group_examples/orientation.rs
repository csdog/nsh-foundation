use nsh_core::prelude::*;
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupOrientation},
};

pub fn preview() -> impl IntoElement {
    let volume = signal(5_i32);
    let control = |label, icon, change: i32| {
        Button::new(label)
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Icon)
            .icon_start(text(icon))
            .on_press(move || {
                volume.update(|value| *value = (*value + change).clamp(0, 10));
            })
    };
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Media controls")
                .orientation(ButtonGroupOrientation::Vertical)
                .child(control("Increase volume", "+", 1))
                .child(control("Decrease volume", "−", -1)),
        )
        .child(text(move || format!("Volume {}", volume.get())).text_xs())
}
