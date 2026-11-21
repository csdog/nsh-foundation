use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    use ButtonSize::*;
    let mut examples = view().flex_row().flex_wrap().items_center().gap_3();
    for (label, size, icon_size) in [
        ("Extra Small", ExtraSmall, IconExtraSmall),
        ("Small", Small, IconSmall),
        ("Default", Default, Icon),
        ("Large", Large, IconLarge),
    ] {
        examples = examples.child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    Button::new(label)
                        .variant(ButtonVariant::Outline)
                        .size(size),
                )
                .child(
                    Button::new("Submit")
                        .variant(ButtonVariant::Outline)
                        .size(icon_size)
                        .icon_start(text("↗").text_sm()),
                ),
        );
    }
    examples
}
