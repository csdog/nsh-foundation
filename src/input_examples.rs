use nsh_base::{SelectItem, theme};
use nsh_core::{element::Input, picker::FilePickRequest, prelude::*};

use crate::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
    button_group::ButtonGroup,
    direction::{Direction, DirectionProvider},
    field::{Field, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldTitle},
    field_examples::selection,
    input::{file_input, input, invalid_input},
    input_group::{
        InputGroup, InputGroupAddon, InputGroupAlign, input_group_input, input_group_text,
    },
    ui::muted,
};

fn narrow(content: impl IntoElement) -> View {
    view().w_full().max_w(px(320.0)).child(content)
}

fn labeled(title: FieldLabel, control: Input, description: Option<&'static str>) -> Field {
    let target = element_ref();
    let detail = element_ref();
    Field::new()
        .child(title.for_control(target))
        .child(
            control
                .element_ref(&target)
                .accessibility_described_by(description.map(|_| detail)),
        )
        .child(
            description.map(|description| FieldDescription::new(description).element_ref(&detail)),
        )
}

fn entry(label: &'static str, placeholder: &'static str) -> Input {
    input().label(label).placeholder(placeholder)
}

fn bind_value(control: Input, value: Signal<String>) -> Input {
    control
        .value(value)
        .on_change(move |text: &str| value.set(text.to_owned()))
}

fn password(label: &'static str) -> Input {
    entry(label, "sk-...")
        .masked(true)
        .auto_correct(false)
        .auto_capitalize(AutoCapitalize::None)
}

pub fn demo() -> impl IntoElement {
    narrow(labeled(
        FieldLabel::new("API Key"),
        password("API Key"),
        Some("Use a sample key to try the masked input."),
    ))
}

pub fn basic() -> impl IntoElement {
    narrow(entry("Text", "Enter text"))
}

pub fn field() -> impl IntoElement {
    narrow(labeled(
        FieldLabel::new("Username"),
        entry("Username", "Enter your username"),
        Some("Choose a unique username for your account."),
    ))
}

pub fn disabled() -> impl IntoElement {
    narrow(labeled(
        FieldLabel::new("Email"),
        entry("Email", "Email")
            .keyboard_type(KeyboardType::Email)
            .disabled(true),
        Some("This field is currently disabled."),
    ))
}

pub fn invalid() -> impl IntoElement {
    narrow(
        labeled(
            FieldLabel::new("Invalid Input"),
            invalid_input(true)
                .label("Invalid Input")
                .placeholder("Error"),
            Some("This field contains validation errors."),
        )
        .invalid(true),
    )
}

pub fn file() -> impl IntoElement {
    narrow(
        Field::new()
            .child(FieldLabel::new("Picture"))
            .child(file_input(
                "Choose File",
                FilePickRequest::default().max_size(0),
                |_| {},
            )),
    )
}

fn search(joined: bool) -> impl IntoElement {
    let query = signal(String::new());
    let result = signal(String::new());
    let submit = move || {
        let value = query.get();
        let query = value.trim();
        if !query.is_empty() {
            result.set(format!("Search preview · {query}"));
        }
    };
    let focus = element_ref();
    let control = bind_value(
        entry(
            "Search",
            if joined {
                "Type to search..."
            } else {
                "Search..."
            },
        ),
        query,
    )
    .flex_1()
    .element_ref(&focus)
    .return_key_type(ReturnKeyType::Search)
    .on_submit(move |_: &str| submit());
    let button = Button::new("Search")
        .variant(if joined {
            ButtonVariant::Outline
        } else {
            ButtonVariant::Default
        })
        .on_press(submit);
    narrow(
        if joined {
            Field::new()
                .child(FieldLabel::new("Search").for_control(focus))
                .child(
                    ButtonGroup::new("Search controls")
                        .child(control)
                        .child(button),
                )
        } else {
            Field::new().child(
                view()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(control)
                    .child(button),
            )
        }
        .child(when(
            move || !result.with(String::is_empty),
            move || muted(move || result.get()),
        )),
    )
}

pub fn inline() -> impl IntoElement {
    search(false)
}

pub fn button_group() -> impl IntoElement {
    search(true)
}

pub fn grid() -> impl IntoElement {
    narrow(
        view()
            .flex_row()
            .gap_3()
            .child(
                labeled(
                    FieldLabel::new("First Name"),
                    entry("First Name", "Jordan"),
                    None,
                )
                .flex_1(),
            )
            .child(
                labeled(
                    FieldLabel::new("Last Name"),
                    entry("Last Name", "Lee"),
                    None,
                )
                .flex_1(),
            ),
    )
}

pub fn required() -> impl IntoElement {
    let value = signal(String::new());
    let touched = signal(false);
    let invalid = move || touched.get() && value.with(|value| value.trim().is_empty());
    let control = bind_value(
        invalid_input(invalid)
            .label("Required Field")
            .placeholder("This field is required"),
        value,
    )
    .on_submit(move |_: &str| touched.set(true))
    .on_blur(move || touched.set(true));
    let title = FieldLabel::render(
        view()
            .flex_row()
            .items_center()
            .gap_1()
            .child(FieldTitle::new("Required Field"))
            .child(
                text("*")
                    .text_sm()
                    .text_color(move || theme().destructive)
                    .accessibility_elements_hidden(true),
            ),
    );
    narrow(
        labeled(title, control, Some("This field must be filled out."))
            .invalid(invalid)
            .child(when(invalid, || FieldError::new("Enter a value."))),
    )
}

pub fn badge() -> impl IntoElement {
    let title = FieldLabel::render(
        view()
            .flex_row()
            .items_center()
            .justify_between()
            .child(FieldTitle::new("Webhook URL"))
            .child(Badge::new("Beta").variant(BadgeVariant::Secondary)),
    );
    narrow(labeled(
        title,
        entry("Webhook URL", "https://api.example.com/webhook").keyboard_type(KeyboardType::Url),
        None,
    ))
}

pub fn input_group() -> impl IntoElement {
    let control = element_ref();
    narrow(
        Field::new()
            .child(FieldLabel::new("Website URL").for_control(control))
            .child(
                InputGroup::new(
                    input_group_input()
                        .label("Website URL")
                        .placeholder("example.com")
                        .keyboard_type(KeyboardType::Url),
                )
                .control_ref(control)
                .addon(
                    InputGroupAddon::new(InputGroupAlign::InlineStart)
                        .child(input_group_text("https://")),
                )
                .addon(
                    InputGroupAddon::new(InputGroupAlign::InlineEnd)
                        .child(input_group_text("ⓘ").accessibility_elements_hidden(true)),
                ),
            ),
    )
}

const EMAIL_HELP: &str = "Use an address like name@example.com.";

fn valid_email(value: &str) -> bool {
    let value = value.trim();
    let Some((name, host)) = value.split_once('@') else {
        return false;
    };
    !name.is_empty()
        && !host.contains('@')
        && !value.chars().any(char::is_whitespace)
        && host.split('.').all(|part| !part.is_empty())
        && host.contains('.')
}

fn validated(
    label: &'static str,
    placeholder: &'static str,
    value: Signal<String>,
    keyboard: KeyboardType,
    invalid: impl Fn() -> bool + Copy + 'static,
    error: &'static str,
    submit: impl Fn() + Copy + 'static,
) -> Field {
    let control = element_ref();
    let detail = element_ref();
    let help = (keyboard == KeyboardType::Email).then_some(EMAIL_HELP);
    Field::new()
        .invalid(invalid)
        .child(FieldLabel::new(label).for_control(control))
        .child(
            bind_value(
                invalid_input(invalid)
                    .label(label)
                    .placeholder(placeholder)
                    .keyboard_type(keyboard)
                    .element_ref(&control)
                    .accessibility_described_by(move || {
                        (invalid() || help.is_some()).then_some(detail)
                    }),
                value,
            )
            .on_submit(move |_: &str| submit()),
        )
        .child(
            when(invalid, move || FieldError::new(error).element_ref(&detail)).otherwise(
                move || help.map(|help| FieldDescription::new(help).element_ref(&detail)),
            ),
        )
}

pub fn field_group() -> impl IntoElement {
    form_preview(false)
}

pub fn form() -> impl IntoElement {
    form_preview(true)
}

fn form_preview(full_form: bool) -> impl IntoElement {
    let name = signal(String::new());
    let email = signal(String::new());
    let phone = signal(String::new());
    let address = signal(String::new());
    let country = signal(Some(SharedString::from("us")));
    let submitted = signal(false);
    let submit = move || submitted.set(true);
    let missing_name = move || full_form && name.with(|name| name.trim().is_empty());
    let bad_email = move || email.with(|email| !email.trim().is_empty() && !valid_email(email));
    let reset = move || {
        batch(|| {
            for input in [name, email, phone, address] {
                input.set(String::new());
            }
            submitted.set(false);
            country.set(Some(SharedString::from("us")));
        })
    };
    let mut fields = FieldGroup::new()
        .child(validated(
            "Name",
            if full_form {
                "Evil Rabbit"
            } else {
                "Jordan Lee"
            },
            name,
            KeyboardType::Default,
            move || submitted.get() && missing_name(),
            "Enter your name.",
            submit,
        ))
        .child(validated(
            "Email",
            if full_form {
                "john@example.com"
            } else {
                "name@example.com"
            },
            email,
            KeyboardType::Email,
            move || submitted.get() && bad_email(),
            "Enter a valid email address.",
            submit,
        ));
    if full_form {
        let countries = [
            ("us", "United States"),
            ("uk", "United Kingdom"),
            ("ca", "Canada"),
        ];
        fields = fields
            .child(
                view()
                    .flex_row()
                    .gap_3()
                    .child(
                        labeled(
                            FieldLabel::new("Phone"),
                            bind_value(
                                entry("Phone", "+1 (555) 123-4567")
                                    .keyboard_type(KeyboardType::Phone),
                                phone,
                            ),
                            None,
                        )
                        .flex_1(),
                    )
                    .child(
                        Field::new()
                            .flex_1()
                            .child(FieldLabel::new("Country"))
                            .child(selection(
                                "Country",
                                "Choose country",
                                countries
                                    .into_iter()
                                    .map(|(value, title)| SelectItem::new(value, title))
                                    .collect(),
                                country,
                            )),
                    ),
            )
            .child(labeled(
                FieldLabel::new("Address"),
                bind_value(entry("Address", "123 Main St"), address),
                None,
            ));
    }
    view().w_full().max_w(px(384.0)).child(
        fields
            .child(
                view()
                    .flex_row()
                    .gap_2()
                    .flex_wrap()
                    .child(
                        Button::new(if full_form { "Cancel" } else { "Reset" })
                            .variant(ButtonVariant::Outline)
                            .on_press(reset),
                    )
                    .child(Button::new("Submit").on_press(submit)),
            )
            .child(when(
                move || submitted.get() && !missing_name() && !bad_email(),
                || muted("Sample form validated."),
            )),
    )
}

pub fn rtl() -> impl IntoElement {
    let selected = signal(1_usize);
    let key = signal(String::new());
    let direction = move || {
        if selected.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    let options = [
        (
            "English",
            "API Key",
            "Use a sample key to try the masked input.",
        ),
        (
            "Arabic",
            "مفتاح API",
            "استخدم مفتاحًا تجريبيًا لتجربة حقل الإدخال المخفي.",
        ),
        (
            "Hebrew",
            "מפתח API",
            "השתמשו במפתח לדוגמה כדי לנסות את השדה המוסתר.",
        ),
    ];
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(
                    options
                        .into_iter()
                        .enumerate()
                        .map(|(index, (language, _, _))| {
                            Button::new(language)
                                .variant(ButtonVariant::Outline)
                                .on_press(move || selected.set(index))
                        }),
                ),
        )
        .child(DirectionProvider::new(direction, move || {
            dynamic(move || {
                let (_, label, help) = options[selected.get()];
                labeled(
                    FieldLabel::new(label),
                    bind_value(password(label), key),
                    Some(help),
                )
            })
        }))
}

#[cfg(test)]
mod tests {
    use super::valid_email;

    #[test]
    fn sample_email_validation_rejects_missing_parts_and_extra_at_signs() {
        assert!(valid_email(" hello+test@example.com "));
        for value in [
            "",
            "@example.com",
            "hello@.com",
            "hello@example..com",
            "a@b@example.com",
            "a b@example.com",
        ] {
            assert!(!valid_email(value), "{value:?}");
        }
    }
}
