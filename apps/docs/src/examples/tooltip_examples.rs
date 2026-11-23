use nsh_core::{AccessibilityState, prelude::*};

use super::toggle_examples::icon;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    kbd::{Kbd, KbdTone},
    tooltip::{TooltipContent, TooltipSide},
};

fn anchor(label: &'static str) -> View {
    Button::new(label).variant(ButtonVariant::Outline).surface()
}

pub fn demo() -> View {
    TooltipContent::new("Add to library").attach(anchor("Hover"))
}

fn row() -> View {
    view().flex_row().flex_wrap().justify_center().gap_2()
}

pub fn sides() -> View {
    row().children(
        [
            ("Left", TooltipSide::Left),
            ("Top", TooltipSide::Top),
            ("Bottom", TooltipSide::Bottom),
            ("Right", TooltipSide::Right),
        ]
        .map(|(label, side)| {
            TooltipContent::new("Add to library")
                .side(side)
                .attach(anchor(label))
        }),
    )
}

pub fn keyboard() -> View {
    let save = icon("M3 2.5h8l2 2v9H3zm2 0v4h5v-4m-5 11V9h6v4.5", false);
    TooltipContent::render(
        view()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .child(text("Save Changes").text_xs())
            .child(Kbd::new("S").tone(KbdTone::Tooltip)),
    )
    .attach(
        Button::new("Save changes")
            .size(ButtonSize::IconSmall)
            .icon_start(save)
            .variant(ButtonVariant::Outline)
            .surface(),
    )
}

pub fn disabled() -> View {
    TooltipContent::new("This feature is currently unavailable").attach(
        view()
            .self_start()
            .focusable(true)
            .role(AccessibilityRole::Group)
            .label("Disabled feature")
            .child(
                Button::new("Disabled")
                    .variant(ButtonVariant::Outline)
                    .disabled(true),
            ),
    )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let translations = [
        (
            "English",
            Direction::Ltr,
            "Add to library",
            [
                "Left",
                "Top",
                "Bottom",
                "Right",
                "Inline Start",
                "Inline End",
            ],
        ),
        (
            "Arabic",
            Direction::Rtl,
            "إضافة إلى المكتبة",
            ["يسار", "أعلى", "أسفل", "يمين", "بداية السطر", "نهاية السطر"],
        ),
        (
            "Hebrew",
            Direction::Rtl,
            "הוסף לספרייה",
            ["שמאל", "למעלה", "למטה", "ימין", "תחילת השורה", "סוף השורה"],
        ),
    ];
    view()
        .w_full()
        .gap_4()
        .items_center()
        .child(row().children(translations.into_iter().enumerate().map(
            move |(index, (name, _, _, _))| {
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
        )))
        .child(dynamic(move || {
            let (_, direction, message, labels) = translations[language.get()];
            DirectionProvider::new(direction, move || {
                let sides = [
                    TooltipSide::Left,
                    TooltipSide::Top,
                    TooltipSide::Bottom,
                    TooltipSide::Right,
                    TooltipSide::InlineStart,
                    TooltipSide::InlineEnd,
                ];
                let item = move |index: usize| {
                    TooltipContent::new(message)
                        .side(sides[index])
                        .attach(anchor(labels[index]))
                };
                view()
                    .gap_4()
                    .child(row().children((0..4).map(item)))
                    .child(row().children((4..6).map(item)))
            })
        }))
}
