use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    direction::{Direction, DirectionProvider},
    field::{Field, FieldDescription, FieldError, FieldLabel},
    input_otp::{InputOtp, InputOtpGroup, InputOtpPattern, InputOtpSeparator, InputOtpSlot},
    ui::muted,
};

fn centered(content: impl IntoElement) -> View {
    view()
        .w_full()
        .items_center()
        .child(view().max_w(relative(1.0)).child(content))
}

pub fn demo() -> impl IntoElement {
    centered(InputOtp::new("One-time password", 6).default_value("123456"))
}

pub fn pattern() -> impl IntoElement {
    let control = element_ref();
    centered(
        Field::new()
            .child(FieldLabel::new("Digits Only").for_control(control))
            .child(InputOtp::new("Digits Only", 6).control_ref(control)),
    )
}

pub fn separator() -> impl IntoElement {
    centered(
        InputOtp::new("Grouped verification code", 6)
            .group(InputOtpGroup::slots(0..2))
            .separator(InputOtpSeparator::new())
            .group(InputOtpGroup::slots(2..4))
            .separator(InputOtpSeparator::new())
            .group(InputOtpGroup::slots(4..6)),
    )
}

fn split(code: InputOtp) -> InputOtp {
    code.group(InputOtpGroup::slots(0..3))
        .separator(InputOtpSeparator::new())
        .group(InputOtpGroup::slots(3..6))
}

pub fn disabled() -> impl IntoElement {
    centered(split(
        InputOtp::new("Disabled verification code", 6)
            .default_value("123456")
            .disabled(true),
    ))
}

pub fn controlled() -> impl IntoElement {
    let value = signal(String::new());
    centered(
        view()
            .gap_2()
            .child(InputOtp::new("Controlled verification code", 6).value(value))
            .child(muted(move || {
                value.with(|value| {
                    if value.is_empty() {
                        "Enter your one-time password.".to_owned()
                    } else {
                        format!("You entered: {value}")
                    }
                })
            })),
    )
}

pub fn invalid() -> impl IntoElement {
    let group = |index| {
        InputOtpGroup::new()
            .child(InputOtpSlot::new(index).invalid(true))
            .child(InputOtpSlot::new(index + 1).invalid(true))
    };
    centered(
        InputOtp::new("Invalid verification code", 6)
            .default_value("000000")
            .group(group(0))
            .separator(InputOtpSeparator::new())
            .group(group(2))
            .separator(InputOtpSeparator::new())
            .group(group(4)),
    )
}

pub fn four_digits() -> impl IntoElement {
    centered(InputOtp::new("Four-digit PIN", 4))
}

pub fn alphanumeric() -> impl IntoElement {
    centered(split(
        InputOtp::new("Alphanumeric code", 6).pattern(InputOtpPattern::Alphanumeric),
    ))
}

pub fn form() -> impl IntoElement {
    let target = element_ref();
    let help = element_ref();
    let error = element_ref();
    let value = signal(String::new());
    let submitted = signal(false);
    let notice = signal("");
    let is_invalid = move || submitted.get() && value.with(|value| value != "123456");
    let verify = move || {
        submitted.set(true);
        if value.with(|value| value == "123456") {
            notice.set("Sample code verified.");
        } else {
            notice.set("");
            target.focus();
        }
    };
    let resend = move || {
        batch(|| {
            submitted.set(false);
            value.set(String::new());
            notice.set("Sample code reset. Enter 123456.");
        });
        target.focus();
    };
    let input = split(
        InputOtp::new("Verification code", 6)
            .value(value)
            .control_ref(target)
            .described_by(move || Some(if is_invalid() { error } else { help }))
            .invalid(is_invalid)
            .slot_size(px(44.0), px(48.0))
            .on_change(move |_| {
                if submitted.get() {
                    submitted.set(false);
                    notice.set("");
                }
            })
            .on_submit(move |_| verify()),
    );
    let fields = Field::new()
        .gap_2()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap_2()
                .child(FieldLabel::new("Verification code").for_control(target))
                .child(
                    Button::new("Resend Code")
                        .size(ButtonSize::ExtraSmall)
                        .variant(ButtonVariant::Outline)
                        .icon_start(text("⟳"))
                        .on_press(resend),
                ),
        )
        .child(input)
        .child(
            FieldDescription::new("Use 123456 to try verifying the sample code.")
                .element_ref(&help),
        )
        .child(when(is_invalid, move || {
            FieldError::new("Enter the sample verification code.").element_ref(&error)
        }))
        .child(when(
            move || !notice.get().is_empty(),
            move || muted(move || notice.get()),
        ));
    view().w_full().max_w(px(448.0)).child(
        Card::new()
            .header(
                CardHeader::new()
                    .title(CardTitle::new("Verify your login"))
                    .description(CardDescription::new(
                        "Try a sample verification form for m@example.com.",
                    )),
            )
            .content(CardContent::new(fields))
            .footer(CardFooter::new(
                view()
                    .gap_2()
                    .child(Button::new("Verify").surface().w_full().on_press(verify))
                    .child(
                        view()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_1()
                            .child(muted("Having trouble signing in?"))
                            .child(
                                Button::new("Contact support")
                                    .variant(ButtonVariant::Link)
                                    .size(ButtonSize::Small)
                                    .on_press(move || notice.set("Support selected in preview.")),
                            ),
                    ),
            )),
    )
}

pub fn rtl() -> impl IntoElement {
    let lang = signal(1_usize);
    let code = signal("123456".to_owned());
    let direction = move || {
        if lang.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    let languages = [
        ("English", "Verification code"),
        ("Arabic", "رمز التحقق"),
        ("Hebrew", "קוד אימות"),
    ];
    let controls = view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_2()
        .children(languages.into_iter().enumerate().map(|(index, (name, _))| {
            Button::new(name)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || lang.set(index))
        }));
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_4()
        .child(controls)
        .child(DirectionProvider::new(direction, move || {
            let target = element_ref();
            let label = move || languages[lang.get()].1;
            Field::new()
                .child(FieldLabel::new(label).for_control(target))
                .child(InputOtp::new(label, 6).value(code).control_ref(target))
        }))
}
