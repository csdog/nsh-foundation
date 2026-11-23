use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    direction::{Direction, DirectionProvider, use_direction},
};
use nsh_base::{input, theme};
use nsh_core::{AccessibilityState, KeyboardType, element::LayoutDirection, prelude::*};

struct Labels {
    title: &'static str,
    description: &'static str,
    sign_up: &'static str,
    email: &'static str,
    password: &'static str,
    forgot: &'static str,
    login: &'static str,
    google: &'static str,
    required: &'static str,
    submitted: &'static str,
    preview: &'static str,
}

const ENGLISH: Labels = Labels {
    title: "Login to your account",
    description: "Enter your email below to login to your account",
    sign_up: "Sign Up",
    email: "Email",
    password: "Password",
    forgot: "Forgot your password?",
    login: "Login",
    google: "Login with Google",
    required: "Enter a valid email and password.",
    submitted: "Login form submitted.",
    preview: "This action is available in the preview only.",
};

const ARABIC: Labels = Labels {
    title: "تسجيل الدخول إلى حسابك",
    description: "أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك",
    sign_up: "إنشاء حساب",
    email: "البريد الإلكتروني",
    password: "كلمة المرور",
    forgot: "نسيت كلمة المرور؟",
    login: "تسجيل الدخول",
    google: "تسجيل الدخول باستخدام Google",
    required: "أدخل بريدًا إلكترونيًا صالحًا وكلمة مرور.",
    submitted: "تم إرسال نموذج تسجيل الدخول.",
    preview: "هذا الإجراء للمعاينة فقط.",
};

const HEBREW: Labels = Labels {
    title: "התחבר לחשבון שלך",
    description: "הזן את האימייל שלך למטה כדי להתחבר לחשבון שלך",
    sign_up: "הירשם",
    email: "אימייל",
    password: "סיסמה",
    forgot: "שכחת את הסיסמה?",
    login: "התחבר",
    google: "התחבר עם Google",
    required: "יש להזין כתובת אימייל תקינה וסיסמה.",
    submitted: "טופס הכניסה נשלח.",
    preview: "הפעולה זמינה בתצוגה המקדימה בלבד.",
};

pub fn preview() -> impl IntoElement {
    login(&ENGLISH, LayoutDirection::Ltr, 20.0, "card-demo")
}

pub fn spacing() -> impl IntoElement {
    let spacing = signal(16.0);
    let mut choices = view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_1()
        .role(AccessibilityRole::Group)
        .label("Card spacing");
    for (value, label) in [
        (16.0, "16px"),
        (20.0, "20px"),
        (24.0, "24px"),
        (32.0, "32px"),
    ] {
        choices = choices.child(
            Button::new(label)
                .size(ButtonSize::Small)
                .variant(ButtonVariant::Outline)
                .on_press(move || spacing.set(value))
                .accessibility_state(move || AccessibilityState {
                    selected: spacing.get() == value,
                    ..Default::default()
                })
                .style_when(
                    move || spacing.get() == value,
                    |style| style.bg(theme().muted),
                ),
        );
    }
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(choices)
        .child(login(
            &ENGLISH,
            LayoutDirection::Ltr,
            spacing,
            "card-spacing",
        ))
}

pub fn rtl() -> impl IntoElement {
    rtl_preview(false)
}

pub fn direction() -> impl IntoElement {
    rtl_preview(true)
}

fn rtl_preview(show_direction: bool) -> impl IntoElement {
    let language = signal(1);
    let choices = [
        ("English", &ENGLISH),
        ("العربية", &ARABIC),
        ("עברית", &HEBREW),
    ];
    let mut controls = view().flex_row().flex_wrap().justify_center().gap_1();
    for (index, (name, _)) in choices.iter().enumerate() {
        controls = controls.child(
            Button::new(*name)
                .size(ButtonSize::Small)
                .variant(ButtonVariant::Outline)
                .on_press(move || language.set(index))
                .accessibility_state(move || AccessibilityState {
                    selected: language.get() == index,
                    ..Default::default()
                })
                .style_when(
                    move || language.get() == index,
                    |style| style.bg(theme().muted),
                ),
        );
    }
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(controls)
        .child(DirectionProvider::new(
            move || {
                if language.get() == 0 {
                    Direction::Ltr
                } else {
                    Direction::Rtl
                }
            },
            move || {
                let direction = use_direction();
                let status = direction.clone();
                view()
                    .gap_2()
                    .child(show_direction.then(|| {
                        text(move || format!("Current direction: {}", status.get().as_str()))
                            .test_id("direction-value")
                            .text_sm()
                            .text_color(move || theme().muted_foreground)
                    }))
                    .child(dynamic(move || {
                        let copy = choices[language.get()].1;
                        login(copy, direction.get().into(), 20.0, "card-rtl")
                    }))
            },
        ))
}

fn login(
    copy: &'static Labels,
    direction: LayoutDirection,
    spacing: impl IntoReactive<f32>,
    id: &'static str,
) -> View {
    let email = signal(String::new());
    let password = signal(String::new());
    let feedback = signal(None::<&'static str>);
    let submit = move || {
        let address = email.get();
        let valid_email = address.trim().split_once('@').is_some_and(|(user, host)| {
            !user.is_empty()
                && !host.is_empty()
                && !host.contains('@')
                && !address.trim().contains(char::is_whitespace)
        });
        feedback.set(Some(if valid_email && !password.get().is_empty() {
            copy.submitted
        } else {
            copy.required
        }));
    };
    let preview_action = move || feedback.set(Some(copy.preview));
    let field = |label, placeholder, masked, value: Signal<String>| {
        input()
            .label(label)
            .placeholder(placeholder)
            .keyboard_type(if masked {
                KeyboardType::Default
            } else {
                KeyboardType::Email
            })
            .masked(masked)
            .value(move || value.get())
            .on_change(move |contents: &str| value.set(contents.to_owned()))
            .on_submit(move |_: &str| submit())
            .h(px(32.0))
            .px_3()
            .rounded(px(12.0))
            .border(px(1.0))
            .border_color(move || theme().input)
            .text_color(move || theme().foreground)
    };
    let label = |title| {
        text(title)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .when(direction == LayoutDirection::Rtl, |label| {
                label.text_right()
            })
    };
    let form = view()
        .test_id(&format!("{id}-body"))
        .gap_6()
        .child(view().gap_2().child(label(copy.email)).child(field(
            copy.email,
            "m@example.com",
            false,
            email,
        )))
        .child(
            view()
                .gap_2()
                .child(
                    view()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .justify_between()
                        .child(label(copy.password))
                        .child(
                            Button::new(copy.forgot)
                                .variant(ButtonVariant::Link)
                                .size(ButtonSize::Small)
                                .on_press(preview_action),
                        ),
                )
                .child(field(copy.password, "", true, password)),
        );
    let actions = view()
        .gap_2()
        .child(Button::new(copy.login).on_press(submit))
        .child(
            Button::new(copy.google)
                .variant(ButtonVariant::Outline)
                .on_press(preview_action),
        )
        .child(when(
            move || feedback.get().is_some(),
            move || {
                text(move || SharedString::from(feedback.get().unwrap_or("")))
                    .text_sm()
                    .when(direction == LayoutDirection::Rtl, |label| {
                        label.text_right()
                    })
            },
        ));
    Card::new()
        .spacing(spacing)
        .direction(direction)
        .header(
            CardHeader::new()
                .title(CardTitle::new(copy.title))
                .description(CardDescription::new(copy.description))
                .action(CardAction::new(
                    Button::new(copy.sign_up)
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Link)
                        .on_press(preview_action)
                        .test_id(&format!("{id}-sign-up")),
                )),
        )
        .content(CardContent::new(form))
        .footer(CardFooter::new(actions))
        .surface()
        .max_w(px(384.0))
        .test_id(id)
}
