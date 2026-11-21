use nsh_core::{AccessibilityState, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    field::{Field, FieldDescription, FieldLabel},
    textarea::{invalid_textarea, textarea},
    ui::muted,
};

const MESSAGE: &str = "Message";
const PLACEHOLDER: &str = "Type your message here.";

fn narrow() -> View {
    view().w_full().max_w(px(320.0))
}

pub fn demo() -> View {
    narrow().child(textarea().label(MESSAGE).placeholder(PLACEHOLDER))
}

pub fn field() -> View {
    let control = element_ref();
    let description = element_ref();
    narrow().child(
        Field::new()
            .child(FieldLabel::new(MESSAGE).for_control(control))
            .child(FieldDescription::new("Enter your message below.").element_ref(&description))
            .child(
                textarea()
                    .label(MESSAGE)
                    .placeholder(PLACEHOLDER)
                    .element_ref(&control)
                    .accessibility_described_by(Some(description)),
            ),
    )
}

pub fn disabled() -> View {
    narrow().child(
        Field::new()
            .child(FieldLabel::new(MESSAGE).opacity(0.5))
            .child(
                textarea()
                    .label(MESSAGE)
                    .placeholder(PLACEHOLDER)
                    .disabled(true),
            ),
    )
}

pub fn invalid() -> View {
    let control = element_ref();
    let error = element_ref();
    let value = signal(String::new());
    let invalid = move || value.with(|text| text.trim().is_empty());
    narrow().child(
        Field::new()
            .invalid(invalid)
            .child(FieldLabel::new(MESSAGE).for_control(control))
            .child(
                invalid_textarea(invalid)
                    .label(MESSAGE)
                    .placeholder(PLACEHOLDER)
                    .value(value)
                    .on_change(move |text: &str| value.set(text.to_owned()))
                    .element_ref(&control)
                    .accessibility_described_by(move || invalid().then_some(error)),
            )
            .child(when(invalid, move || {
                FieldDescription::new("Please enter a valid message.").element_ref(&error)
            })),
    )
}

pub fn button() -> View {
    let draft = signal(String::new());
    let status = signal("");
    narrow()
        .gap_2()
        .child(
            textarea()
                .label(MESSAGE)
                .placeholder(PLACEHOLDER)
                .value(draft)
                .on_change(move |text: &str| {
                    draft.set(text.to_owned());
                    status.set("");
                }),
        )
        .child(Button::new("Send message").on_press(move || {
            if draft.with_untracked(|text| text.trim().is_empty()) {
                status.set("Enter a message first.");
            } else {
                draft.set(String::new());
                status.set("Message sent in this preview.");
            }
        }))
        .child(when(
            move || !status.get().is_empty(),
            move || muted(move || SharedString::from(status.get())),
        ))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let value = signal(String::new());
    let translations = [
        (
            "English",
            Direction::Ltr,
            "Feedback",
            "Your feedback helps us improve...",
            "Share your thoughts about our service.",
        ),
        (
            "Arabic",
            Direction::Rtl,
            "التعليقات",
            "تعليقاتك تساعدنا على التحسين...",
            "شاركنا أفكارك حول خدمتنا.",
        ),
        (
            "Hebrew",
            Direction::Rtl,
            "משוב",
            "המשוב שלך עוזר לנו להשתפר...",
            "שתף את מחשבותיך על השירות שלנו.",
        ),
    ];
    narrow()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(
                    translations
                        .into_iter()
                        .enumerate()
                        .map(|(index, (name, ..))| {
                            Button::new(name)
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Small)
                                .surface()
                                .accessibility_state(move || AccessibilityState {
                                    selected: language.get() == index,
                                    ..Default::default()
                                })
                                .on_press(move || language.set(index))
                        }),
                ),
        )
        .child(dynamic(move || {
            let (_, direction, label, placeholder, description) = translations[language.get()];
            DirectionProvider::new(direction, move || {
                let control = element_ref();
                let help = element_ref();
                Field::new()
                    .child(FieldLabel::new(label).for_control(control))
                    .child(
                        textarea()
                            .label(label)
                            .placeholder(placeholder)
                            .min_h(|| {
                                px(if window().width.0 < 768.0 {
                                    114.0
                                } else {
                                    98.0
                                })
                            })
                            .value(value)
                            .on_change(move |text: &str| value.set(text.to_owned()))
                            .element_ref(&control)
                            .accessibility_described_by(Some(help)),
                    )
                    .child(FieldDescription::new(description).element_ref(&help))
            })
        }))
}
