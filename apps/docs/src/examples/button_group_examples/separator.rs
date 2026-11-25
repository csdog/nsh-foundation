use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupSeparator},
};
use nsh_core::{element::set_clipboard, prelude::*};

pub fn preview() -> impl IntoElement {
    let status = signal("");
    let copied = signal(false);
    let button = |label| {
        Button::new(label)
            .variant(ButtonVariant::Secondary)
            .size(ButtonSize::Small)
    };
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Clipboard actions")
                .child(button("Copy").on_press(move || {
                    let _ = set_clipboard("Button Group sample");
                    copied.set(true);
                    status.set("Sample text copied");
                }))
                .separator(ButtonGroupSeparator::new())
                .child(button("Paste").on_press(move || {
                    status.set(if copied.get() {
                        "Pasted the sample text"
                    } else {
                        "Copy the sample text first"
                    });
                })),
        )
        .child(text(move || status.get()).text_xs())
}
