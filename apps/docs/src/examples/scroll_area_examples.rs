use nsh_base::theme;
use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider, use_direction},
    scroll_area::{ScrollArea, ScrollAreaOrientation, ScrollBar},
};

fn tags(title: impl IntoReactive<SharedString>) -> ScrollArea {
    let title = title.into_reactive();
    let direction = use_direction();
    let list = view()
        .w_full()
        .p_4()
        .pe(px(15.0))
        .style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right(),
        )
        .child(
            text(title.clone())
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .mb_2(),
        )
        .children((1..=50).rev().map(|number| {
            view()
                .py_2()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(text(format!("v1.2.0-beta.{number}")).text_sm())
        }));
    ScrollArea::new(title, list)
}

pub fn demo() -> ScrollArea {
    tags("Tags")
}

pub fn horizontal() -> ScrollArea {
    let plates = [
        ("Ornella Binni", asset!("scroll-area-ornella.jpg")),
        ("Tom Byrom", asset!("scroll-area-tom.jpg")),
        ("Vladimir Malyavko", asset!("scroll-area-vladimir.jpg")),
    ];
    let content = view()
        .w(px(964.0))
        .flex_row()
        .flex_shrink_0()
        .gap_4()
        .p_4()
        .children(plates.into_iter().map(|(artist, photo)| {
            view()
                .w(px(300.0))
                .flex_shrink_0()
                .child(
                    image(photo)
                        .role(AccessibilityRole::Image)
                        .label(format!("Photo by {artist}"))
                        .w_full()
                        .h(px(400.0))
                        .object_fit(ResizeMode::Cover)
                        .rounded(px(6.0)),
                )
                .child(
                    text("Photo by ")
                        .pt_2()
                        .text_xs()
                        .text_color(move || theme().muted_foreground)
                        .child(
                            text(artist)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(move || theme().foreground),
                        ),
                )
        }));
    ScrollArea::new("Artwork gallery", content)
        .width(384.0)
        .height(460.0)
        .scroll_bar(ScrollBar::new(ScrollAreaOrientation::Horizontal))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    view()
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
        .child(DirectionProvider::new(
            move || {
                if language.get() == 0 {
                    Direction::Ltr
                } else {
                    Direction::Rtl
                }
            },
            move || tags(move || ["Tags", "العلامات", "תגיות"][language.get()]),
        ))
}
