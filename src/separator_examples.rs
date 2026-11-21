use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider, use_direction},
    separator::{Separator, SeparatorOrientation},
    ui::muted,
};

const SUBTITLE: &str = "The Foundation for your Design System";
const DESCRIPTION: &str =
    "A set of beautifully designed components that you can customize, extend, and build on.";

fn horizontal() -> View {
    Separator::new().surface().test_id("separator-horizontal")
}

fn line() -> View {
    Separator::new()
        .orientation(SeparatorOrientation::Vertical)
        .surface()
        .test_id("separator-vertical")
}

fn introduction(subtitle: &'static str, description: &'static str) -> View {
    let direction = use_direction();
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right(),
        )
        .child(
            view()
                .gap(px(6.0))
                .child(text("shadcn/ui").text_sm().font_weight(FontWeight::MEDIUM))
                .child(muted(subtitle)),
        )
        .child(horizontal())
        .child(text(description).text_sm())
}

pub fn demo() -> View {
    introduction(SUBTITLE, DESCRIPTION)
}

pub fn vertical() -> View {
    view()
        .h(px(20.0))
        .flex_row()
        .items_center()
        .gap_4()
        .child(text("Blog").text_sm())
        .child(line())
        .child(text("Docs").text_sm())
        .child(line())
        .child(text("Source").text_sm())
}

pub fn menu() -> View {
    let item = |title, description| {
        view()
            .min_w(px(0.0))
            .gap_1()
            .child(text(title).text_sm().font_weight(FontWeight::MEDIUM))
            .child(muted(description).text_xs())
    };
    let large = || window().width.0 >= 768.0;
    view()
        .flex_row()
        .items_center()
        .max_w(percentage(1.0))
        .gap(move || px(if large() { 16.0 } else { 8.0 }))
        .child(item("Settings", "Manage preferences"))
        .child(line())
        .child(item("Account", "Profile & security"))
        .child(when(large, line))
        .child(when(large, move || item("Help", "Support & docs")))
}

pub fn list() -> View {
    let mut list = view()
        .w_full()
        .max_w(px(384.0))
        .gap_2()
        .role(AccessibilityRole::Group)
        .label("Items and values");
    for index in 1..=3 {
        if index > 1 {
            list = list.child(horizontal());
        }
        list = list.child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_3()
                .child(text(format!("Item {index}")).text_sm())
                .child(muted(format!("Value {index}"))),
        );
    }
    list
}

const LOCALES: [(Direction, &str, &str); 3] = [
    (Direction::Ltr, SUBTITLE, DESCRIPTION),
    (
        Direction::Rtl,
        "الأساس لنظام التصميم الخاص بك",
        "مجموعة من المكونات المصممة بشكل جميل يمكنك تخصيصها وتوسيعها والبناء عليها.",
    ),
    (
        Direction::Rtl,
        "הבסיס למערכת העיצוב שלך",
        "סט של רכיבים מעוצבים בצורה יפה שאתה יכול להתאים אישית, להרחיב ולבנות עליהם.",
    ),
];

pub fn rtl() -> View {
    let language = signal(1_usize);
    view()
        .w_full()
        .max_w(px(384.0))
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
            let (direction, subtitle, description) = LOCALES[language.get()];
            DirectionProvider::new(direction, move || introduction(subtitle, description))
        }))
}
