use nsh_core::prelude::*;
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::Direction,
    slider::{Slider, SliderOrientation},
};

fn horizontal(content: impl IntoElement) -> View {
    view().w_full().max_w(px(320.0)).child(content)
}

pub fn demo() -> View {
    horizontal(Slider::new([75.0]).labels(["Volume"]))
}

pub fn range() -> View {
    horizontal(
        Slider::new([25.0, 50.0])
            .step(5.0)
            .labels(["Minimum", "Maximum"]),
    )
}

pub fn multiple() -> View {
    horizontal(
        Slider::new([10.0, 20.0, 70.0])
            .step(10.0)
            .labels(["Start", "Middle", "End"]),
    )
}

pub fn vertical() -> View {
    horizontal(
        view()
            .flex_row()
            .justify_center()
            .gap_6()
            .child(
                Slider::new([50.0])
                    .orientation(SliderOrientation::Vertical)
                    .labels(["First vertical slider"]),
            )
            .child(
                Slider::new([25.0])
                    .orientation(SliderOrientation::Vertical)
                    .labels(["Second vertical slider"]),
            ),
    )
}

pub fn controlled() -> View {
    let value = signal([0.3, 0.7]);
    horizontal(
        view()
            .gap_3()
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .child(
                        text("Temperature")
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM),
                    )
                    .child(
                        text(move || {
                            let [low, high] = value.get();
                            format!("{low}, {high}")
                        })
                        .text_sm()
                        .test_id("slider-temperature"),
                    ),
            )
            .child(
                Slider::new([0.3, 0.7])
                    .value(value)
                    .range(0.0, 1.0)
                    .step(0.1)
                    .labels(["Minimum temperature", "Maximum temperature"]),
            ),
    )
}

pub fn disabled() -> View {
    horizontal(
        Slider::new([50.0])
            .disabled(true)
            .labels(["Disabled slider"]),
    )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    horizontal(
        view()
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
                                .surface()
                                .accessibility_state(move || AccessibilityState {
                                    selected: language.get() == index,
                                    ..Default::default()
                                })
                                .on_press(move || language.set(index))
                        },
                    )),
            )
            .child(Slider::new([75.0]).labels(["Volume"]).direction(move || {
                if language.get() == 0 {
                    Direction::Ltr
                } else {
                    Direction::Rtl
                }
            })),
    )
}
