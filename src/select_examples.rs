use nsh_base::switch;
use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    field::{Field, FieldDescription, FieldError, FieldLabel, FieldOrientation},
    select::{Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue},
};

const FRUITS: [(&str, &str); 5] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("blueberry", "Blueberry"),
    ("grapes", "Grapes"),
    ("pineapple", "Pineapple"),
];
const VEGETABLES: [(&str, &str); 3] = [
    ("carrot", "Carrot"),
    ("broccoli", "Broccoli"),
    ("spinach", "Spinach"),
];

fn group(label: &'static str, items: &[(&'static str, &'static str)]) -> SelectGroup {
    items
        .iter()
        .fold(SelectGroup::new(label), |group, &(value, title)| {
            group.item(SelectItem::new(value, title))
        })
}

fn control(label: &'static str, placeholder: &'static str, content: SelectContent) -> Select {
    Select::new(
        label,
        SelectTrigger::new(SelectValue::new(placeholder)),
        content,
    )
}

fn frame(select: impl IntoElement, width: f32) -> View {
    view().w_full().max_w(px(width)).child(select)
}

pub fn demo() -> View {
    frame(
        control(
            "Fruit",
            "Select a fruit",
            SelectContent::new().group(group("Fruits", &FRUITS)),
        ),
        192.0,
    )
}

pub fn align_item() -> View {
    let cover = signal(true);
    frame(
        view().gap_4()
            .child(
                Field::new().orientation(FieldOrientation::Horizontal).items_center().justify_between()
                    .child(view().flex_1().gap_1()
                        .child(FieldLabel::new("Align Item"))
                        .child(FieldDescription::new("Toggle to align the item with the trigger.")))
                    .child(
                        switch()
                            .accessibility_label("Align Item")
                            .checked(cover)
                            .on_change(move |next: &bool| cover.set(*next))
                            .flex_shrink_0()
                            .track_class("w-9 h-5 rounded-full bg-input checked:bg-primary")
                            .thumb_class("size-4 rounded-full bg-background translate-x-0.5 checked:translate-x-4")
                            .focus_visible(|s| s.ring(px(2.0)).ring_color(nsh_base::theme().ring)),
                    )
            )
            .child(
                control("Aligned fruit", "Select a fruit",
                    FRUITS.into_iter().fold(SelectContent::new().align_item_with_trigger(cover), |content, (value,label)| {
                        content.item(SelectItem::new(value,label))
                    }),
                ).default_value("banana"),
            ),
        300.0,
    )
}

pub fn groups() -> View {
    frame(
        control(
            "Produce",
            "Select a fruit",
            SelectContent::new()
                .group(group("Fruits", &FRUITS[..3]))
                .group(group("Vegetables", &VEGETABLES)),
        ),
        192.0,
    )
}

const TIME_ZONES: [(&str, &[(&str, &str)]); 5] = [
    (
        "North America",
        &[
            ("est", "Eastern Standard Time"),
            ("cst", "Central Standard Time"),
            ("mst", "Mountain Standard Time"),
            ("pst", "Pacific Standard Time"),
            ("akst", "Alaska Standard Time"),
            ("hst", "Hawaii Standard Time"),
        ],
    ),
    (
        "Europe & Africa",
        &[
            ("gmt", "Greenwich Mean Time"),
            ("cet", "Central European Time"),
            ("eet", "Eastern European Time"),
            ("west", "Western European Summer Time"),
            ("cat", "Central Africa Time"),
            ("eat", "East Africa Time"),
        ],
    ),
    (
        "Asia",
        &[
            ("msk", "Moscow Time"),
            ("ist", "India Standard Time"),
            ("cst_china", "China Standard Time"),
            ("jst", "Japan Standard Time"),
            ("kst", "Korea Standard Time"),
            ("ist_indonesia", "Indonesia Central Standard Time"),
        ],
    ),
    (
        "Australia & Pacific",
        &[
            ("awst", "Australian Western Standard Time"),
            ("acst", "Australian Central Standard Time"),
            ("aest", "Australian Eastern Standard Time"),
            ("nzst", "New Zealand Standard Time"),
            ("fjt", "Fiji Time"),
        ],
    ),
    (
        "South America",
        &[
            ("art", "Argentina Time"),
            ("bot", "Bolivia Time"),
            ("brt", "Brasilia Time"),
            ("clt", "Chile Standard Time"),
        ],
    ),
];

pub fn scrollable() -> View {
    let content = TIME_ZONES.into_iter().fold(
        SelectContent::new().max_height(256.0),
        |content, (label, items)| content.group(group(label, items)),
    );
    frame(control("Time zone", "Select a timezone", content), 256.0)
}

pub fn disabled() -> View {
    let fruits = FRUITS
        .into_iter()
        .fold(SelectGroup::new("Fruits"), |group, (value, label)| {
            group.item(SelectItem::new(value, label).disabled(value == "grapes"))
        });
    frame(
        control(
            "Disabled fruit",
            "Select a fruit",
            SelectContent::new().group(fruits),
        )
        .disabled(true),
        192.0,
    )
}

pub fn invalid() -> View {
    let content = FRUITS[..3]
        .iter()
        .fold(SelectContent::new(), |content, &(value, label)| {
            content.item(SelectItem::new(value, label))
        });
    frame(
        Field::new()
            .invalid(true)
            .child(FieldLabel::new("Fruit"))
            .child(control("Fruit", "Select a fruit", content).invalid(true))
            .child(FieldError::new("Please select a fruit.")),
        192.0,
    )
}

struct Locale {
    direction: Direction,
    label: &'static str,
    placeholder: &'static str,
    fruit: &'static str,
    fruits: [&'static str; 5],
    vegetable: &'static str,
    vegetables: [&'static str; 3],
}

const LOCALES: [Locale; 3] = [
    Locale {
        direction: Direction::Ltr,
        label: "Produce",
        placeholder: "Select a fruit",
        fruit: "Fruits",
        fruits: ["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"],
        vegetable: "Vegetables",
        vegetables: ["Carrot", "Broccoli", "Spinach"],
    },
    Locale {
        direction: Direction::Rtl,
        label: "المنتجات",
        placeholder: "اختر فاكهة",
        fruit: "الفواكه",
        fruits: ["تفاح", "موز", "توت أزرق", "عنب", "أناناس"],
        vegetable: "الخضروات",
        vegetables: ["جزر", "بروكلي", "سبانخ"],
    },
    Locale {
        direction: Direction::Rtl,
        label: "תוצרת",
        placeholder: "בחר פרי",
        fruit: "פירות",
        fruits: ["תפוח", "בננה", "אוכמניה", "ענבים", "אננס"],
        vegetable: "ירקות",
        vegetables: ["גזר", "ברוקולי", "תרד"],
    },
];

fn translate<const N: usize>(
    group: &'static str,
    entries: [(&'static str, &'static str); N],
    labels: [&'static str; N],
) -> SelectGroup {
    entries
        .into_iter()
        .zip(labels)
        .fold(SelectGroup::new(group), |group, ((value, _), label)| {
            group.item(SelectItem::new(value, label))
        })
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let value = signal(None::<SharedString>);
    view()
        .w_full()
        .items_center()
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
                let content = SelectContent::new()
                    .group(translate(locale.fruit, FRUITS, locale.fruits))
                    .group(translate(locale.vegetable, VEGETABLES, locale.vegetables));
                frame(
                    control(locale.label, locale.placeholder, content).value(value),
                    144.0,
                )
            })
        }))
}
