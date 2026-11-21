use nsh_core::{AccessibilityState, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    field::{Field, FieldDescription, FieldLabel},
    toggle::{ToggleSize, ToggleVariant},
    toggle_examples::{BOLD, ITALIC, icon},
    toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation},
};

fn format_items(last_id: &'static str, last_label: &'static str) -> [ToggleGroupItem; 3] {
    [
        ("bold", "Toggle bold", BOLD),
        ("italic", "Toggle italic", ITALIC),
        (last_id, last_label, "M4 3.2v4a4 4 0 0 0 8 0v-4M3 13h10"),
    ]
    .map(|(id, label, path)| {
        ToggleGroupItem::new(id, label).content(
            view()
                .accessibility_elements_hidden(true)
                .child(icon(path, false)),
        )
    })
}

pub fn demo() -> ToggleGroup {
    ToggleGroup::new("Text formatting")
        .variant(ToggleVariant::Outline)
        .multiple(true)
        .items(format_items("strikethrough", "Toggle strikethrough"))
}

pub fn outline() -> ToggleGroup {
    ToggleGroup::new("Call filter")
        .variant(ToggleVariant::Outline)
        .default_value(["all"])
        .items([
            ToggleGroupItem::new("all", "All").label("Toggle all"),
            ToggleGroupItem::new("missed", "Missed").label("Toggle missed"),
        ])
}

fn positions(label: &'static str, size: ToggleSize) -> ToggleGroup {
    ToggleGroup::new(label)
        .size(size)
        .variant(ToggleVariant::Outline)
        .default_value(["top"])
        .items(
            [
                ("top", "Top", "Toggle top"),
                ("bottom", "Bottom", "Toggle bottom"),
                ("left", "Left", "Toggle left"),
                ("right", "Right", "Toggle right"),
            ]
            .map(|(id, text, label)| ToggleGroupItem::new(id, text).label(label)),
        )
}

pub fn sizes() -> View {
    view()
        .gap_4()
        .child(positions("Small positions", ToggleSize::Small))
        .child(positions("Default positions", ToggleSize::Default))
}

pub fn spacing() -> ToggleGroup {
    positions("Spaced positions", ToggleSize::Small).spacing(8.0)
}

pub fn vertical() -> ToggleGroup {
    ToggleGroup::new("Vertical formatting")
        .multiple(true)
        .orientation(ToggleGroupOrientation::Vertical)
        .spacing(4.0)
        .default_value(["bold", "italic"])
        .items(format_items("underline", "Toggle underline"))
}

pub fn disabled() -> ToggleGroup {
    ToggleGroup::new("Disabled formatting")
        .disabled(true)
        .items(format_items("strikethrough", "Toggle strikethrough"))
}

pub fn custom() -> View {
    let value = signal(vec!["normal"]);
    let weights = [
        ("light", "Light", FontWeight::LIGHT),
        ("normal", "Normal", FontWeight::NORMAL),
        ("medium", "Medium", FontWeight::MEDIUM),
        ("bold", "Bold", FontWeight::BOLD),
    ];
    view().w_full().max_w(px(320.0)).child(
        Field::new()
            .child(FieldLabel::new("Font Weight"))
            .child(
                ToggleGroup::new("Font weight")
                    .variant(ToggleVariant::Outline)
                    .size(ToggleSize::Large)
                    .value(value)
                    .on_change(move |next| {
                        value.set(if next.is_empty() {
                            vec!["normal"]
                        } else {
                            next
                        });
                    })
                    .items(weights.map(|(id, label, weight)| {
                        ToggleGroupItem::new(id, label)
                            .content(
                                view()
                                    .items_center()
                                    .child(
                                        text("Aa")
                                            .text_2xl()
                                            .line_height(px(24.0))
                                            .font_weight(weight),
                                    )
                                    .child(text(label).text_xs()),
                            )
                            .style(|item| {
                                item.h(px(64.0))
                                    .w(|| px(if window().width.0 < 360.0 { 52.0 } else { 64.0 }))
                                    .px_0()
                                    .rounded_xl()
                            })
                    })),
            )
            .child(FieldDescription::new(move || {
                SharedString::from(
                    value
                        .with(|current| format!("Use font-{} to set the font weight.", current[0])),
                )
            })),
    )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let value = signal(vec!["list"]);
    let translations = [
        ("English", Direction::Ltr, ["List", "Grid", "Cards"]),
        ("Arabic", Direction::Rtl, ["قائمة", "شبكة", "بطاقات"]),
        ("Hebrew", Direction::Rtl, ["רשימה", "רשת", "כרטיסים"]),
    ];
    view()
        .gap_4()
        .items_center()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(translations.into_iter().enumerate().map(
                    move |(index, (name, _, _))| {
                        Button::new(name)
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || language.set(index))
                    },
                )),
        )
        .child(dynamic(move || {
            let (_, direction, labels) = translations[language.get()];
            DirectionProvider::new(direction, move || {
                ToggleGroup::new("View mode")
                    .variant(ToggleVariant::Outline)
                    .value(value)
                    .on_change(move |next| value.set(next))
                    .items(
                        ["list", "grid", "cards"]
                            .into_iter()
                            .zip(labels)
                            .map(|(id, label)| ToggleGroupItem::new(id, label)),
                    )
            })
        }))
}
