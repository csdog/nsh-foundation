use nsh_core::{element::LayoutDirection, prelude::*};
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent},
    skeleton::Skeleton,
};

fn block() -> View {
    Skeleton::new().surface().test_id("skeleton-placeholder")
}

fn line() -> View {
    block().h_4()
}

fn person(
    diameter: f32,
    long: f32,
    short: f32,
    direction: impl IntoReactive<LayoutDirection>,
) -> View {
    view()
        .direction(direction)
        .w_full()
        .max_w(px(diameter + 16.0 + long))
        .min_w(px(0.0))
        .flex_row()
        .items_center()
        .gap_4()
        .child(block().size(px(diameter)).rounded_full().flex_shrink_0())
        .child(
            view()
                .gap_2()
                .flex_1()
                .min_w(px(0.0))
                .child(line().w_full())
                .child(line().w(percentage(short / long))),
        )
}

pub fn demo() -> View {
    person(48.0, 250.0, 200.0, LayoutDirection::Ltr)
}

pub fn avatar() -> View {
    person(40.0, 150.0, 100.0, LayoutDirection::Ltr)
}

pub fn card() -> View {
    Card::new()
        .content(CardContent::new(
            view()
                .gap(px(20.0))
                .child(
                    view()
                        .gap(px(6.0))
                        .child(line().w(percentage(2.0 / 3.0)))
                        .child(line().w(percentage(0.5))),
                )
                .child(block().w_full().aspect_ratio(16.0 / 9.0)),
        ))
        .surface()
        .max_w(px(320.0))
}

pub fn text_lines() -> View {
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_2()
        .child(line().w_full())
        .child(line().w_full())
        .child(line().w(percentage(0.75)))
}

pub fn form() -> View {
    let field = |label_width| {
        view()
            .gap_3()
            .child(line().w(px(label_width)))
            .child(block().h_8().w_full())
    };
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_7()
        .child(field(80.0))
        .child(field(96.0))
        .child(block().h_8().w(px(96.0)))
}

pub fn table() -> View {
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_2()
        .children((0..5).map(|_| {
            view()
                .flex_row()
                .gap_4()
                .child(line().flex_1())
                .child(line().w(px(96.0)).flex_shrink_0())
                .child(line().w(px(80.0)).flex_shrink_0())
        }))
}

pub fn rtl() -> View {
    let locale = signal(1_usize);
    view()
        .w_full()
        .max_w(px(314.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_1()
                .justify_center()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, name)| {
                        Button::new(name)
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: locale.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || locale.set(index))
                    },
                )),
        )
        .child(person(48.0, 250.0, 200.0, move || {
            if locale.get() == 0 {
                LayoutDirection::Ltr
            } else {
                LayoutDirection::Rtl
            }
        }))
}
