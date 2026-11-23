use std::time::Duration;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent},
    carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious},
};
use nsh_base::theme;
use nsh_core::{AccessibilityState, element::LayoutDirection, geometry::Axis, prelude::*};

const SLIDES: u32 = 5;

pub fn demo() -> impl IntoElement {
    frame(horizontal("Slide gallery", 16.0, || 1.0))
}

pub fn sizes() -> impl IntoElement {
    frame(horizontal("Responsive sizes", 16.0, responsive_basis))
}

pub fn spacing() -> impl IntoElement {
    frame(horizontal("Custom spacing", 4.0, responsive_basis))
}

pub fn orientation() -> impl IntoElement {
    frame(
        Carousel::new("Vertical carousel")
            .orientation(Axis::Vertical)
            .previous(CarouselPrevious::new())
            .content(numbered(Axis::Vertical, 8.0, || 0.5, false))
            .next(CarouselNext::new()),
    )
}

pub fn api() -> impl IntoElement {
    let index = signal(0);
    view()
        .w_full()
        .items_center()
        .gap_2()
        .child(frame(
            horizontal("Controlled slides", 4.0, || 1.0).index(index),
        ))
        .child(
            text(move || format!("Slide {} of {SLIDES}", index.get() + 1))
                .text_sm()
                .text_color(move || theme().muted_foreground),
        )
}

pub fn autoplay() -> impl IntoElement {
    frame(
        horizontal("Autoplay slides", 16.0, || 1.0)
            .looping(true)
            .autoplay(Duration::from_secs(2)),
    )
}

pub fn rtl() -> impl IntoElement {
    let selection = signal(1);
    let mut controls = view().flex_row().flex_wrap().justify_center().gap_1();
    for (index, name) in ["English", "العربية", "עברית"].into_iter().enumerate() {
        controls = controls.child(
            Button::new(name)
                .size(ButtonSize::Small)
                .variant(ButtonVariant::Outline)
                .on_press(move || selection.set(index))
                .accessibility_state(move || AccessibilityState {
                    selected: selection.get() == index,
                    ..Default::default()
                })
                .style_when(
                    move || selection.get() == index,
                    |style| style.bg(theme().muted),
                ),
        );
    }
    view()
        .w_full()
        .items_center()
        .gap_4()
        .child(controls)
        .child(dynamic(move || {
            let language = selection.get();
            let direction = if language == 0 {
                LayoutDirection::Ltr
            } else {
                LayoutDirection::Rtl
            };
            let (previous, next) = match language {
                1 => ("الشريحة السابقة", "الشريحة التالية"),
                2 => ("השקופית הקודמת", "השקופית הבאה"),
                _ => ("Previous slide", "Next slide"),
            };
            frame(
                Carousel::new("Localized slide gallery")
                    .direction(direction)
                    .previous(CarouselPrevious::new().label(previous))
                    .content(numbered(Axis::Horizontal, 16.0, || 1.0, language == 1))
                    .next(CarouselNext::new().label(next)),
            )
        }))
}

fn frame(carousel: Carousel) -> View {
    carousel.surface().max_w(move || {
        let width = window().width.0;
        px(if width >= 768.0 {
            472.0
        } else if width >= 640.0 {
            408.0
        } else {
            280.0
        })
    })
}

fn horizontal(label: &'static str, gap: f32, basis: impl Fn() -> f32 + Copy + 'static) -> Carousel {
    Carousel::new(label)
        .previous(CarouselPrevious::new())
        .content(numbered(Axis::Horizontal, gap, basis, false))
        .next(CarouselNext::new())
}

fn responsive_basis() -> f32 {
    if window().width.0 >= 1024.0 {
        1.0 / 3.0
    } else {
        0.5
    }
}

fn numbered(
    axis: Axis,
    gap: f32,
    basis: impl Fn() -> f32 + Copy + 'static,
    arabic: bool,
) -> CarouselContent {
    let mut content = CarouselContent::new().spacing(gap);
    for index in 1..=SLIDES {
        let numeral = if arabic {
            char::from_u32('٠' as u32 + index)
                .unwrap_or('٠')
                .to_string()
        } else {
            index.to_string()
        };
        let panel = view()
            .items_center()
            .justify_center()
            .when(axis == Axis::Horizontal, |panel| panel.aspect_ratio(1.0))
            .when(axis == Axis::Vertical, |panel| panel.h(px(88.0)))
            .child(text(numeral).text_3xl().font_weight(FontWeight::SEMIBOLD));
        let card = Card::new().spacing(0.0).content(CardContent::new(panel));
        content = content.item(CarouselItem::new(card).basis(basis));
    }
    content
}
