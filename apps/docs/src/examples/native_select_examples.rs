use nsh_base::theme;
use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    native_select::{NativeSelect, NativeSelectOptGroup as Group, NativeSelectOption as OptionRow},
};

struct Locale {
    label: &'static str,
    options: [&'static str; 5],
    direction: Direction,
}

const ENGLISH: Locale = Locale {
    label: "Status",
    options: ["Select status", "Todo", "In Progress", "Done", "Cancelled"],
    direction: Direction::Ltr,
};
const ARABIC: Locale = Locale {
    label: "الحالة",
    options: ["اختر الحالة", "مهام", "قيد التنفيذ", "منجز", "ملغي"],
    direction: Direction::Rtl,
};
const HEBREW: Locale = Locale {
    label: "סטטוס",
    options: ["בחר סטטוס", "לעשות", "בתהליך", "הושלם", "בוטל"],
    direction: Direction::Rtl,
};

fn stage(content: NativeSelect, width: f32) -> View {
    view().w(px(width)).max_w(percentage(1.0)).child(content)
}

fn status(locale: &Locale) -> NativeSelect {
    ["", "todo", "in-progress", "done", "cancelled"]
        .into_iter()
        .zip(locale.options)
        .fold(NativeSelect::new(locale.label), |select, (value, label)| {
            select.option(OptionRow::new(value, label))
        })
}

pub fn demo() -> View {
    stage(status(&ENGLISH), 160.0)
}

pub fn groups() -> View {
    stage(
        NativeSelect::new("Department")
            .option(OptionRow::new("", "Select department"))
            .group(
                Group::new("Engineering")
                    .option(OptionRow::new("frontend", "Frontend"))
                    .option(OptionRow::new("backend", "Backend"))
                    .option(OptionRow::new("devops", "DevOps")),
            )
            .group(
                Group::new("Sales")
                    .option(OptionRow::new("sales-rep", "Sales Rep"))
                    .option(OptionRow::new("account-manager", "Account Manager"))
                    .option(OptionRow::new("sales-director", "Sales Director")),
            )
            .group(
                Group::new("Operations")
                    .option(OptionRow::new("support", "Customer Support"))
                    .option(OptionRow::new("product-manager", "Product Manager"))
                    .option(OptionRow::new("ops-manager", "Operations Manager")),
            ),
        192.0,
    )
}

fn fruit(label: &'static str, prompt: &'static str) -> NativeSelect {
    NativeSelect::new(label)
        .option(OptionRow::new("", prompt))
        .option(OptionRow::new("apple", "Apple"))
        .option(OptionRow::new("banana", "Banana"))
        .option(OptionRow::new("blueberry", "Blueberry"))
}

pub fn disabled() -> View {
    stage(fruit("Disabled fruit", "Disabled").disabled(true), 144.0)
}

pub fn invalid() -> View {
    stage(fruit("Fruit", "Error state").invalid(true), 144.0)
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let value = signal(Some(SharedString::from("")));
    view()
        .gap_3()
        .items_center()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    |(index, name)| {
                        Button::new(name)
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
                            )
                    },
                )),
        )
        .child(dynamic(move || {
            let locale = [&ENGLISH, &ARABIC, &HEBREW][language.get()];
            DirectionProvider::new(locale.direction, move || {
                stage(status(locale).value(value), 160.0)
            })
        }))
}
