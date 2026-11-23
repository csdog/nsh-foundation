use std::time::Duration;

use nsh_base::{SliderValue, slider};
use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    progress::{Progress, ProgressLabel, ProgressValue},
};

fn frame() -> View {
    view().w_full().max_w(px(384.0)).gap_4()
}

pub fn demo() -> View {
    let value = signal(13.0_f32);
    effect(move || {
        let timer = set_timeout(Duration::from_millis(500), move || value.set(66.0));
        cleanup(move || timer.cancel());
    });
    view()
        .w(percentage(0.6))
        .child(Progress::new(value).accessibility_label("Example progress"))
}

pub fn label() -> View {
    frame().child(
        Progress::new(56.0)
            .label(ProgressLabel::new("Upload progress"))
            .display(ProgressValue::new()),
    )
}

pub fn controlled() -> View {
    let value = signal(SliderValue::Single(50.0));
    frame()
        .child(Progress::new(move || value.get().start() as f32).accessibility_label("Controlled progress"))
        .child(
            slider()
                .w_full()
                .min(0.0)
                .max(100.0)
                .step(1.0)
                .value(move || Some(value.get()))
                .on_change(move |next: &SliderValue| value.set(*next))
                .accessibility_label("Set progress")
                .track_class("h-6 justify-center")
                .indicator_class("h-2 rounded-full bg-muted")
                .range_class("rounded-full bg-primary")
                .thumb_class("size-4 rounded-full bg-background border border-primary focus-visible:ring-2 focus-visible:ring-ring"),
        )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    frame()
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
            let index = language.get();
            let direction = if index == 0 {
                Direction::Ltr
            } else {
                Direction::Rtl
            };
            let label = ["Upload progress", "تقدم الرفع", "התקדמות העלאה"][index];
            DirectionProvider::new(direction, move || {
                let value = if index == 1 {
                    ProgressValue::new().format(|number| {
                        const DIGITS: [char; 10] =
                            ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
                        let mut localized: String = format!("{number:.0}")
                            .bytes()
                            .map(|digit| DIGITS[usize::from(digit - b'0')])
                            .collect();
                        localized.push('%');
                        localized
                    })
                } else {
                    ProgressValue::new()
                };
                Progress::new(56.0)
                    .label(ProgressLabel::new(label))
                    .display(value)
            })
        }))
}
