use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_base::{theme, tooltip};
use nsh_core::{Checked, element::input, prelude::*};

pub fn preview() -> impl IntoElement {
    let voice = signal(false);
    let message = signal(String::new());
    let status = signal("");
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Rounded message composer")
                .separate(
                    Button::new("Add attachment to message")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .radius(999.0)
                        .icon_start(text("+"))
                        .on_press(move || status.set("Attachment action selected")),
                )
                .separate(
                    ButtonGroup::new("Voice or text")
                        .child(
                            input()
                                .label("Voice or text message")
                                .placeholder(move || {
                                    if voice.get() {
                                        "Listening…"
                                    } else {
                                        "Send a message…"
                                    }
                                })
                                .disabled(voice)
                                .value(message)
                                .w(px(168.0))
                                .min_w(px(50.0))
                                .h(px(32.0))
                                .px_3()
                                .rounded_full()
                                .text_sm()
                                .border(px(1.0))
                                .border_color(move || theme().border)
                                .bg(move || theme().input.opacity(0.5))
                                .text_color(move || theme().foreground)
                                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                                .on_change(move |value: &str| message.set(value.to_owned())),
                        )
                        .child(
                            Button::new("Toggle voice mode")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Icon)
                                .radius(999.0)
                                .icon_start(text("≋"))
                                .on_press(move || voice.update(|enabled| *enabled = !*enabled))
                                .accessibility_state(move || AccessibilityState {
                                    pressed: Some(if voice.get() {
                                        Checked::True
                                    } else {
                                        Checked::False
                                    }),
                                    ..Default::default()
                                })
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
        .child(when(voice, || {
            text("Voice mode enabled in this preview").text_xs()
        }))
        .child(text(move || status.get()).text_xs())
}
