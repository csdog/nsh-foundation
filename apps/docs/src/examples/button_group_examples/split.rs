use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupSeparator},
};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let result = signal("");
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Split button")
                .child(
                    Button::new("Button")
                        .variant(ButtonVariant::Secondary)
                        .on_press(move || result.set("Main action selected")),
                )
                .separator(ButtonGroupSeparator::new())
                .child(
                    Button::new("Add")
                        .variant(ButtonVariant::Secondary)
                        .size(ButtonSize::Icon)
                        .icon_start(text("+"))
                        .on_press(move || result.set("Add selected")),
                ),
        )
        .child(text(move || result.get()).text_xs())
}
