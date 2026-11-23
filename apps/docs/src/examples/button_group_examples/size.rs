use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let mut examples = view().items_center().gap_3();
    for (name, size, icon_size) in [
        ("Small", ButtonSize::Small, ButtonSize::IconSmall),
        ("Default", ButtonSize::Default, ButtonSize::Icon),
        ("Large", ButtonSize::Large, ButtonSize::IconLarge),
    ] {
        let button = |label| {
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(size)
                .surface()
        };
        examples = examples.child(
            ButtonGroup::new(name)
                .child(button(name))
                .child(button("Button"))
                .child(button("Group"))
                .child(
                    Button::new("More")
                        .variant(ButtonVariant::Outline)
                        .size(icon_size)
                        .icon_start(text("…"))
                        .surface(),
                ),
        );
    }
    examples
}
