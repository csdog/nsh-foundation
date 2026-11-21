use nsh_base::{theme, tooltip};
use nsh_core::{element::input, prelude::*};
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};

pub fn preview() -> impl IntoElement {
    let status = signal("");
    let message = signal(String::new());
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Message composer")
                .separate(
                    ButtonGroup::new("Attachments").child(
                        Button::new("Add attachment")
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Icon)
                            .icon_start(text("+"))
                            .on_press(move || status.set("Attachment action selected")),
                    ),
                )
                .separate(
                    ButtonGroup::new("Message input")
                        .child(
                            input()
                                .label("Message")
                                .placeholder("Send a message…")
                                .value(message)
                                .w(px(160.0))
                                .min_w(px(55.0))
                                .h(px(32.0))
                                .px_3()
                                .rounded(px(16.0))
                                .text_sm()
                                .border(px(1.0))
                                .border_color(move || theme().border)
                                .bg(move || theme().input.opacity(0.5))
                                .text_color(move || theme().foreground)
                                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                                .on_change(move |value: &str| message.set(value.to_owned()))
                                .on_submit(move |_: &str| {
                                    status.set("Message submitted in this preview")
                                }),
                        )
                        .child(
                            Button::new("Voice Mode")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Icon)
                                .icon_start(text("≋"))
                                .on_press(move || status.set("Voice mode selected"))
                                .tooltip(
                                    tooltip()
                                        .px_2()
                                        .py_1()
                                        .rounded(px(8.0))
                                        .bg(move || theme().foreground)
                                        .text_color(move || theme().background)
                                        .child(text("Voice Mode").text_xs()),
                                ),
                        ),
                ),
        )
        .child(text(move || status.get()).text_xs())
}
