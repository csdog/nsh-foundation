use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    field::{FieldDescription, FieldLegend, FieldLegendVariant, FieldSet},
    radio_group::{RadioGroup, RadioGroupItem as Item},
};

const DENSITIES: [&str; 3] = ["default", "comfortable", "compact"];
const ENGLISH: [(&str, &str); 3] = [
    ("Default", "Standard spacing for most use cases."),
    ("Comfortable", "More space between elements."),
    ("Compact", "Minimal spacing for dense layouts."),
];

fn compact(group: RadioGroup) -> View {
    view().w_full().max_w(px(256.0)).child(group)
}

fn density(
    label: &'static str,
    descriptions: Option<[(&'static str, &'static str); 3]>,
) -> RadioGroup {
    DENSITIES
        .into_iter()
        .zip(descriptions.unwrap_or(ENGLISH))
        .fold(
            RadioGroup::new(label).default_value("comfortable"),
            |group, (value, (label, description))| {
                let item = Item::new(value, label);
                group.item(if descriptions.is_some() {
                    item.description(description)
                } else {
                    item
                })
            },
        )
}

pub fn demo() -> View {
    compact(density("Display density", None))
}

pub fn description() -> View {
    compact(density("Display density with descriptions", Some(ENGLISH)))
}

pub fn choice_card() -> View {
    let plan = signal(Some(SharedString::from("plus")));
    view().w_full().max_w(px(384.0)).child(
        RadioGroup::new("Plan options")
            .value(plan)
            .item(
                Item::new("plus", "Plus")
                    .description("For individuals and small teams.")
                    .card(),
            )
            .item(
                Item::new("pro", "Pro")
                    .description("For growing businesses.")
                    .card(),
            )
            .item(
                Item::new("enterprise", "Enterprise")
                    .description("For large teams and enterprises.")
                    .card(),
            ),
    )
}

fn field(title: &'static str, description: &'static str, group: RadioGroup) -> View {
    view().w_full().max_w(px(320.0)).child(
        FieldSet::new(FieldLegend::new(title).variant(FieldLegendVariant::Label))
            .child(FieldDescription::new(description))
            .child(group),
    )
}

pub fn fieldset() -> View {
    const TITLE: &str = "Subscription Plan";
    field(
        TITLE,
        "Yearly and lifetime plans offer significant savings.",
        RadioGroup::new(TITLE)
            .default_value("monthly")
            .item(Item::new("monthly", "Monthly ($9.99/month)"))
            .item(Item::new("yearly", "Yearly ($99.99/year)"))
            .item(Item::new("lifetime", "Lifetime ($299.99)")),
    )
}

pub fn disabled() -> View {
    compact(
        RadioGroup::new("Available options")
            .default_value("option2")
            .item(Item::new("option1", "Disabled").disabled(true))
            .item(Item::new("option2", "Option 2"))
            .item(Item::new("option3", "Option 3")),
    )
}

pub fn invalid() -> View {
    const TITLE: &str = "Notification Preferences";
    field(
        TITLE,
        "Choose how you want to receive notifications.",
        RadioGroup::new(TITLE)
            .default_value("email")
            .invalid(true)
            .item(Item::new("email", "Email only"))
            .item(Item::new("sms", "SMS only"))
            .item(Item::new("both", "Both Email & SMS")),
    )
}

struct Locale {
    direction: Direction,
    label: &'static str,
    options: [(&'static str, &'static str); 3],
}

const LOCALES: [Locale; 3] = [
    Locale {
        direction: Direction::Ltr,
        label: "Display density",
        options: ENGLISH,
    },
    Locale {
        direction: Direction::Rtl,
        label: "كثافة العرض",
        options: [
            ("افتراضي", "تباعد قياسي لمعظم حالات الاستخدام."),
            ("مريح", "مساحة أكبر بين العناصر."),
            ("مضغوط", "تباعد أدنى للتخطيطات الكثيفة."),
        ],
    },
    Locale {
        direction: Direction::Rtl,
        label: "צפיפות תצוגה",
        options: [
            ("ברירת מחדל", "ריווח סטנדרטי לרוב מקרי השימוש."),
            ("נוח", "יותר מקום בין האלמנטים."),
            ("קומפקטי", "ריווח מינימלי לפריסות צפופות."),
        ],
    },
];

pub fn rtl() -> View {
    let language = signal(1_usize);
    let selected = signal(Some(SharedString::from("comfortable")));
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || language.set(index))
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                    },
                )),
        )
        .child(dynamic(move || {
            let locale = &LOCALES[language.get()];
            DirectionProvider::new(locale.direction, move || {
                density(locale.label, Some(locale.options)).value(selected)
            })
        }))
}
