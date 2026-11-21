use nsh_base::{
    Input, SelectEntry, SelectItem, SliderValue, input, radio, radio_group, select, slider, switch,
    theme,
};
use nsh_core::{AccessibilityState, prelude::*};

use crate::{
    button::{Button, ButtonVariant},
    checkbox::{Checkbox, CheckboxState},
    direction::{Direction, DirectionProvider},
    field::{
        Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
        FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle,
    },
    ui::muted,
};

fn narrow() -> View {
    view().w_full().max_w(px(320.0))
}

fn field_input(
    label: &'static str,
    placeholder: &'static str,
    value: Signal<String>,
    invalid: impl Fn() -> bool + Copy + 'static,
) -> Input {
    input()
        .label(label)
        .placeholder(placeholder)
        .value(value)
        .on_change(move |next: &str| value.set(next.to_owned()))
        .h(px(34.0))
        .w_full()
        .min_w(px(0.0))
        .px_3()
        .border(px(1.0))
        .rounded(px(12.0))
        .border_color(move || {
            if invalid() {
                theme().destructive
            } else {
                theme().border
            }
        })
        .bg(move || theme().input.opacity(0.25))
        .text_color(move || theme().foreground)
        .text_sm()
        .accessibility_state(move || AccessibilityState {
            invalid: invalid(),
            ..Default::default()
        })
        .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring.opacity(0.55)))
}

fn text_field(
    label: &'static str,
    placeholder: &'static str,
    value: Signal<String>,
    description: Option<&'static str>,
    masked: bool,
    multiline: bool,
) -> Field {
    let control = element_ref();
    let help = element_ref();
    Field::new()
        .child(FieldLabel::new(label).for_control(control))
        .child(
            field_input(label, placeholder, value, || false)
                .element_ref(&control)
                .accessibility_described_by(description.map(|_| help))
                .masked(masked)
                .multiline(multiline)
                .when(multiline, |input| input.h(px(96.0)).py_2()),
        )
        .child(description.map(|message| FieldDescription::new(message).element_ref(&help)))
}

pub fn input_example() -> impl IntoElement {
    narrow().child(
        FieldGroup::new()
            .child(text_field(
                "Username",
                "Max Leiter",
                signal(String::new()),
                Some("Choose a unique username for your account."),
                false,
                false,
            ))
            .child(text_field(
                "Password",
                "••••••••",
                signal(String::new()),
                Some("Use at least 8 characters."),
                true,
                false,
            )),
    )
}

pub fn textarea() -> impl IntoElement {
    narrow().child(text_field(
        "Feedback",
        "Your feedback helps us improve...",
        signal(String::new()),
        Some("Share your thoughts about our service."),
        false,
        true,
    ))
}

fn selection(
    label: &'static str,
    placeholder: &'static str,
    items: Vec<SelectItem>,
    state: Signal<Option<SharedString>>,
) -> impl IntoElement {
    let labels = items.clone();
    select()
        .accessibility_label(label)
        .placeholder(placeholder)
        .value(move || state.get())
        .on_change(move |value: &Option<SharedString>| state.set(value.clone()))
        .items(items)
        .trigger(
            Button::new(move || {
                let value = state.get();
                labels
                    .iter()
                    .find(|item| Some(&item.value) == value.as_ref())
                    .map_or_else(|| placeholder.into(), |item| item.title.clone())
            })
            .variant(ButtonVariant::Outline)
            .icon_end(text("⌄"))
            .surface()
            .w_full()
            .justify_between(),
        )
        .w_full()
        .min_w(px(0.0))
        .popup_class("p-1 bg-popover text-popover-foreground rounded-xl border shadow-md")
        .item_class("px-3 py-2 rounded-lg focus:bg-muted selected:bg-muted")
        .render_item(|entry: &SelectEntry| text(entry.title.clone()).text_sm().w_full())
}

pub fn select_example() -> impl IntoElement {
    let items = [
        "Engineering",
        "Design",
        "Marketing",
        "Sales",
        "Customer Support",
        "Human Resources",
        "Finance",
        "Operations",
    ];
    narrow().child(
        Field::new()
            .child(FieldLabel::new("Department"))
            .child(selection(
                "Department",
                "Choose department",
                items
                    .into_iter()
                    .map(|name| SelectItem::new(name, name))
                    .collect(),
                signal(None),
            ))
            .child(FieldDescription::new(
                "Select your department or area of work.",
            )),
    )
}

pub fn slider_example() -> impl IntoElement {
    let value = signal(SliderValue::Range(200.0, 800.0));
    narrow().child(Field::new()
        .child(FieldTitle::new("Price Range"))
        .child(FieldDescription::new(move || format!("Set your budget range (${} – ${}).", value.get().start(), value.get().end())))
        .child(slider().w_full().min(0.0).max(1000.0).step(10.0)
            .value(move || Some(value.get()))
            .on_change(move |next: &SliderValue| value.set(*next))
            .start_accessibility_label("Minimum price")
            .accessibility_label("Maximum price")
            .track_class("h-6 justify-center")
            .indicator_class("h-1.5 rounded-full bg-muted")
            .range_class("rounded-full bg-primary")
            .thumb_class("size-4 rounded-full bg-background border border-primary focus-visible:ring-2 focus-visible:ring-ring")))
}

pub fn fieldset() -> impl IntoElement {
    narrow().child(
        FieldSet::new(FieldLegend::new("Address Information"))
            .child(FieldDescription::new("Enter your complete address below."))
            .child(text_field(
                "Street Address",
                "123 Main St",
                signal(String::new()),
                None,
                false,
                false,
            ))
            .child(
                view()
                    .flex_row()
                    .gap_3()
                    .child(
                        text_field(
                            "City",
                            "New York",
                            signal(String::new()),
                            None,
                            false,
                            false,
                        )
                        .flex_1(),
                    )
                    .child(
                        text_field(
                            "Postal Code",
                            "90502",
                            signal(String::new()),
                            None,
                            false,
                            false,
                        )
                        .flex_1(),
                    ),
            ),
    )
}

fn check(label: &'static str, checked: bool, disabled: bool) -> Field {
    Field::new()
        .orientation(FieldOrientation::Horizontal)
        .child(
            Checkbox::new(label)
                .default_checked(checked)
                .disabled(disabled),
        )
}

pub fn checkbox() -> impl IntoElement {
    narrow().child(FieldGroup::new()
        .child(FieldSet::new(FieldLegend::new("Show these items on the desktop").variant(FieldLegendVariant::Label))
            .child(FieldDescription::new("Select the items you want to show on the desktop."))
            .children(["Hard disks", "External disks", "CDs, DVDs, and iPods", "Connected servers"]
                .map(|label| check(label, false, false))))
        .child(FieldSeparator::new())
        .child(Field::new().orientation(FieldOrientation::Horizontal)
            .child(Checkbox::new("Sync Desktop & Documents folders")
                .description("Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices.")
                .default_checked(true))))
}

fn radio_option(
    label: &'static str,
    description: Option<&'static str>,
    index: usize,
    selected: Signal<usize>,
) -> impl IntoElement {
    let checked = move || selected.get() == index;
    let dot = view()
        .size(px(16.0))
        .flex_shrink_0()
        .rounded_full()
        .border(px(1.0))
        .border_color(move || {
            if checked() {
                theme().primary
            } else {
                theme().border
            }
        })
        .items_center()
        .justify_center()
        .child(when(checked, || {
            view()
                .size(px(6.0))
                .rounded_full()
                .bg(move || theme().primary)
        }));
    let title = FieldTitle::new(label);
    let body = match description {
        Some(detail) => Field::new()
            .orientation(FieldOrientation::Horizontal)
            .p_4()
            .items_start()
            .child(
                FieldContent::new()
                    .child(title)
                    .child(FieldDescription::new(detail)),
            )
            .child(dot),
        None => Field::new()
            .orientation(FieldOrientation::Horizontal)
            .child(dot)
            .child(title),
    };
    radio()
        .checked(checked)
        .accessibility_label(label)
        .on_change(move |_: &bool| selected.set(index))
        .w_full()
        .rounded(px(16.0))
        .when(description.is_some(), |row| {
            row.border(px(1.0))
                .border_color(move || theme().border)
                .style_when(checked, |s| s.bg(theme().input.opacity(0.3)))
        })
        .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
        .child(body)
}

pub fn radio_example() -> impl IntoElement {
    let plan = signal(0);
    narrow().child(
        FieldSet::new(FieldLegend::new("Subscription Plan").variant(FieldLegendVariant::Label))
            .child(FieldDescription::new("Choose a plan that fits your needs."))
            .child(
                radio_group()
                    .accessibility_label("Subscription Plan")
                    .gap_2()
                    .children(
                        [
                            "Monthly ($9.99/month)",
                            "Yearly ($99.99/year)",
                            "Lifetime ($299.99)",
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(i, name)| radio_option(name, None, i, plan)),
                    ),
            ),
    )
}

pub fn switch_example() -> impl IntoElement {
    let enabled = signal(false);
    narrow().child(
        Field::new()
            .orientation(FieldOrientation::Horizontal)
            .justify_between()
            .child(FieldLabel::new("Multi-factor authentication").flex_shrink(1.0))
            .child(
                switch()
                    .accessibility_label("Multi-factor authentication")
                    .checked(enabled)
                    .on_change(move |next: &bool| enabled.set(*next))
                    .flex_shrink_0()
                    .track_class("w-9 h-5 rounded-full bg-input checked:bg-primary")
                    .thumb_class(
                        "size-4 rounded-full bg-background translate-x-0.5 checked:translate-x-4",
                    )
                    .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring)),
            ),
    )
}

pub fn choice_card() -> impl IntoElement {
    let selected = signal(0);
    narrow().child(
        FieldSet::new(FieldLegend::new("Compute Environment").variant(FieldLegendVariant::Label))
            .child(FieldDescription::new(
                "Select the compute environment for your cluster.",
            ))
            .child(
                radio_group()
                    .accessibility_label("Compute Environment")
                    .gap_2()
                    .child(radio_option(
                        "Kubernetes",
                        Some("Run GPU workloads on a Kubernetes cluster."),
                        0,
                        selected,
                    ))
                    .child(radio_option(
                        "Virtual Machine",
                        Some("Access a VM configured for GPU workloads."),
                        1,
                        selected,
                    )),
            ),
    )
}

pub fn group() -> impl IntoElement {
    narrow().child(FieldGroup::new()
        .child(FieldSet::new(FieldLegend::new("Responses").variant(FieldLegendVariant::Label))
            .child(FieldDescription::new("Get notified when ChatGPT responds to requests that take time, like research or image generation."))
            .child(check("Push notifications", true, true)))
        .child(FieldSeparator::new())
        .child(FieldSet::new(FieldLegend::new("Tasks").variant(FieldLegendVariant::Label))
            .child(FieldDescription::new("Get notified when tasks you've created have updates."))
            .child(Button::new("Manage tasks").variant(ButtonVariant::Link).href("https://help.openai.com/en/articles/10291617-tasks-in-chatgpt"))
            .child(check("Push notifications", false, false))
            .child(check("Email notifications", false, false))))
}

struct CheckoutCopy {
    payment: &'static str,
    preview: &'static str,
    name: &'static str,
    number: &'static str,
    number_hint: &'static str,
    month: &'static str,
    year: &'static str,
    billing: &'static str,
    billing_hint: &'static str,
    same: &'static str,
    comments: &'static str,
    comments_hint: &'static str,
    submit: &'static str,
    cancel: &'static str,
    required: &'static str,
    number_error: &'static str,
    code_error: &'static str,
    saved: &'static str,
}

const ENGLISH: CheckoutCopy = CheckoutCopy {
    payment: "Payment Method",
    preview: "Use sample values to preview form validation.",
    name: "Name on Card",
    number: "Card Number",
    number_hint: "Enter a 16-digit sample card number.",
    month: "Month",
    year: "Year",
    billing: "Billing Address",
    billing_hint: "The billing address associated with your payment method.",
    same: "Same as shipping address",
    comments: "Comments",
    comments_hint: "Add any additional comments",
    submit: "Submit",
    cancel: "Cancel",
    required: "Enter a name.",
    number_error: "Enter a 16-digit sample card number.",
    code_error: "Enter a 3-digit sample code.",
    saved: "Sample form validated.",
};

const ARABIC: CheckoutCopy = CheckoutCopy {
    payment: "طريقة الدفع",
    preview: "أدخل بيانات تجريبية للتحقق من النموذج.",
    name: "الاسم على البطاقة",
    number: "رقم البطاقة",
    number_hint: "أدخل رقم بطاقة تجريبيًا من 16 رقمًا.",
    month: "الشهر",
    year: "السنة",
    billing: "عنوان الفوترة",
    billing_hint: "عنوان الفوترة المرتبط بطريقة الدفع.",
    same: "نفس عنوان الشحن",
    comments: "تعليقات",
    comments_hint: "أضف أي تعليقات إضافية",
    submit: "إرسال",
    cancel: "إلغاء",
    required: "أدخل الاسم.",
    number_error: "أدخل رقم بطاقة تجريبيًا من 16 رقمًا.",
    code_error: "أدخل رمزًا تجريبيًا من 3 أرقام.",
    saved: "تم التحقق من النموذج التجريبي.",
};

const HEBREW: CheckoutCopy = CheckoutCopy {
    payment: "אמצעי תשלום",
    preview: "הזינו ערכים לדוגמה כדי לבדוק את הטופס.",
    name: "שם על הכרטיס",
    number: "מספר כרטיס",
    number_hint: "הזינו מספר כרטיס לדוגמה בן 16 ספרות.",
    month: "חודש",
    year: "שנה",
    billing: "כתובת חיוב",
    billing_hint: "כתובת החיוב המשויכת לאמצעי התשלום.",
    same: "זהה לכתובת המשלוח",
    comments: "הערות",
    comments_hint: "הוסיפו הערות נוספות",
    submit: "שליחה",
    cancel: "ביטול",
    required: "הזינו שם.",
    number_error: "הזינו מספר כרטיס לדוגמה בן 16 ספרות.",
    code_error: "הזינו קוד לדוגמה בן 3 ספרות.",
    saved: "הטופס לדוגמה אומת.",
};

#[derive(Clone, Copy)]
struct Checkout {
    name: Signal<String>,
    number: Signal<String>,
    code: Signal<String>,
    comments: Signal<String>,
    shipping: Signal<bool>,
    month: Signal<Option<SharedString>>,
    year: Signal<Option<SharedString>>,
    submitted: Signal<bool>,
}

impl Checkout {
    fn new() -> Self {
        Self {
            name: signal(String::new()),
            number: signal(String::new()),
            code: signal(String::new()),
            comments: signal(String::new()),
            shipping: signal(true),
            month: signal(None),
            year: signal(None),
            submitted: signal(false),
        }
    }

    fn reset(self) {
        batch(|| {
            for value in [self.name, self.number, self.code, self.comments] {
                value.set(String::new());
            }
            self.month.set(None);
            self.year.set(None);
            self.shipping.set(true);
            self.submitted.set(false);
        });
    }
}

fn digits(value: &str, length: usize) -> bool {
    let mut digits = value.bytes().filter(|byte| !byte.is_ascii_whitespace());
    (0..length).all(|_| digits.next().is_some_and(|byte| byte.is_ascii_digit()))
        && digits.next().is_none()
}

fn validated_field(
    label: &'static str,
    placeholder: &'static str,
    value: Signal<String>,
    invalid: impl Fn() -> bool + Copy + 'static,
    help: Option<&'static str>,
    error: &'static str,
    submit: impl Fn() + Copy + 'static,
) -> Field {
    let control = element_ref();
    let detail = element_ref();
    let error_ref = element_ref();
    Field::new()
        .invalid(invalid)
        .child(FieldLabel::new(label).for_control(control))
        .child(
            field_input(label, placeholder, value, invalid)
                .element_ref(&control)
                .accessibility_described_by(move || {
                    if invalid() {
                        Some(error_ref)
                    } else {
                        help.map(|_| detail)
                    }
                })
                .on_submit(move |_: &str| submit()),
        )
        .child(help.map(|help| FieldDescription::new(help).element_ref(&detail)))
        .child(when(invalid, move || {
            FieldError::new(error).element_ref(&error_ref)
        }))
}

fn checkout(copy: &'static CheckoutCopy, state: Checkout) -> impl IntoElement {
    let missing_name = move || state.name.with(|value| value.trim().is_empty());
    let bad_number = move || !state.number.with(|value| digits(value, 16));
    let bad_code = move || !state.code.with(|value| digits(value, 3));
    let submit = move || state.submitted.set(true);
    let options = |range: std::ops::RangeInclusive<u16>| {
        range
            .map(|n| {
                let label = format!("{n:02}");
                SelectItem::new(label.clone(), label)
            })
            .collect::<Vec<_>>()
    };
    view().w_full().max_w(px(448.0)).child(
        FieldGroup::new()
            .child(
                FieldSet::new(FieldLegend::new(copy.payment))
                    .child(FieldDescription::new(copy.preview))
                    .child(validated_field(
                        copy.name,
                        "Evil Rabbit",
                        state.name,
                        move || state.submitted.get() && missing_name(),
                        None,
                        copy.required,
                        submit,
                    ))
                    .child(validated_field(
                        copy.number,
                        "1234 5678 9012 3456",
                        state.number,
                        move || state.submitted.get() && bad_number(),
                        Some(copy.number_hint),
                        copy.number_error,
                        submit,
                    ))
                    .child(
                        view()
                            .flex_row()
                            .gap_2()
                            .child(
                                Field::new()
                                    .flex_1()
                                    .child(FieldLabel::new(copy.month))
                                    .child(selection(
                                        copy.month,
                                        "MM",
                                        options(1..=12),
                                        state.month,
                                    )),
                            )
                            .child(
                                Field::new()
                                    .flex_1()
                                    .child(FieldLabel::new(copy.year))
                                    .child(selection(
                                        copy.year,
                                        "YYYY",
                                        options(2026..=2035),
                                        state.year,
                                    )),
                            )
                            .child(
                                validated_field(
                                    "CVV",
                                    "123",
                                    state.code,
                                    move || state.submitted.get() && bad_code(),
                                    None,
                                    copy.code_error,
                                    submit,
                                )
                                .flex_1(),
                            ),
                    ),
            )
            .child(FieldSeparator::new())
            .child(
                FieldSet::new(FieldLegend::new(copy.billing))
                    .child(FieldDescription::new(copy.billing_hint))
                    .child(
                        Field::new()
                            .orientation(FieldOrientation::Horizontal)
                            .child(
                                Checkbox::new(copy.same)
                                    .checked(move || {
                                        if state.shipping.get() {
                                            CheckboxState::True
                                        } else {
                                            CheckboxState::False
                                        }
                                    })
                                    .on_change(move |checked| state.shipping.set(checked)),
                            ),
                    ),
            )
            .child(text_field(
                copy.comments,
                copy.comments_hint,
                state.comments,
                None,
                false,
                true,
            ))
            .child(
                Field::new()
                    .orientation(FieldOrientation::Horizontal)
                    .flex_wrap()
                    .child(Button::new(copy.submit).on_press(submit))
                    .child(
                        Button::new(copy.cancel)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || state.reset()),
                    ),
            )
            .child(when(
                move || state.submitted.get() && !missing_name() && !bad_number() && !bad_code(),
                move || muted(copy.saved),
            )),
    )
}

pub fn demo() -> impl IntoElement {
    checkout(&ENGLISH, Checkout::new())
}

pub fn rtl() -> impl IntoElement {
    let lang = signal(1_usize);
    let state = Checkout::new();
    let direction = move || {
        if lang.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    let languages = ["English", "Arabic", "Hebrew"];
    view()
        .w_full()
        .items_center()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(languages.into_iter().enumerate().map(|(i, label)| {
                    Button::new(label)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || lang.set(i))
                })),
        )
        .child(DirectionProvider::new(direction, move || {
            dynamic(move || checkout([&ENGLISH, &ARABIC, &HEBREW][lang.get()], state))
        }))
}

pub fn responsive() -> impl IntoElement {
    let name = signal(String::new());
    let submitted = signal(false);
    let invalid = move || submitted.get() && name.with(|value| value.trim().is_empty());
    let control = element_ref();
    let message = element_ref();
    let help = element_ref();
    let submit = move || submitted.set(true);
    view().w_full().max_w(px(512.0)).child(
        FieldSet::new(FieldLegend::new("Profile"))
            .child(FieldDescription::new("Fill in your profile information."))
            .child(
                FieldGroup::new()
                    .child(
                        Field::new()
                            .orientation(FieldOrientation::Responsive)
                            .invalid(invalid)
                            .child(
                                FieldContent::new()
                                    .child(FieldLabel::new("Name").for_control(control))
                                    .child(
                                        FieldDescription::new(
                                            "Provide your full name for identification.",
                                        )
                                        .element_ref(&help),
                                    )
                                    .child(when(invalid, move || {
                                        FieldError::new("Enter your name.").element_ref(&message)
                                    })),
                            )
                            .child(
                                view().flex_1().min_w(px(0.0)).child(
                                    field_input("Name", "Evil Rabbit", name, invalid)
                                        .element_ref(&control)
                                        .accessibility_described_by(move || {
                                            Some(if invalid() { message } else { help })
                                        })
                                        .on_submit(move |_: &str| submit()),
                                ),
                            ),
                    )
                    .child(
                        Field::new()
                            .orientation(FieldOrientation::Responsive)
                            .child(Button::new("Submit").on_press(submit))
                            .child(
                                Button::new("Cancel")
                                    .variant(ButtonVariant::Outline)
                                    .on_press(move || {
                                        name.set(String::new());
                                        submitted.set(false);
                                    }),
                            ),
                    )
                    .child(when(
                        move || submitted.get() && !invalid(),
                        || muted("Profile form validated."),
                    )),
            ),
    )
}
