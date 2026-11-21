use nsh_core::{element::LayoutDirection, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider, use_direction},
    input::input,
    popover::{
        Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverSide, PopoverTitle,
        PopoverTrigger,
    },
};

const TITLE: &str = "Dimensions";
const DESCRIPTION: &str = "Set the dimensions for the layer.";

fn header(title: &'static str, description: &'static str) -> View {
    let direction = use_direction();
    PopoverHeader::new()
        .direction(direction.clone().map(LayoutDirection::from))
        .style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right(),
        )
        .child(PopoverTitle::new(title))
        .child(PopoverDescription::new(description))
}

fn field(
    label: &'static str,
    default: &'static str,
    width: f32,
    focus: Option<ElementRef>,
) -> View {
    view()
        .flex_row()
        .items_center()
        .gap_3()
        .child(
            text(label)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .w(px(width)),
        )
        .child(
            input()
                .label(label)
                .default_value(default)
                .min_w(px(0.0))
                .w(auto())
                .flex_1()
                .when_some(focus, |field, focus| field.element_ref(&focus)),
        )
}

pub fn demo() -> Popover {
    let first = element_ref();
    Popover::new(
        PopoverTrigger::new("Open popover"),
        PopoverContent::new()
            .label(TITLE)
            .width(320.0)
            .initial_focus(first)
            .child(header(TITLE, DESCRIPTION))
            .child(dynamic(move || {
                view()
                    .gap_2()
                    .child(field("Width", "100%", 86.0, Some(first)))
                    .child(field("Max. width", "300px", 86.0, None))
                    .child(field("Height", "25px", 86.0, None))
                    .child(field("Max. height", "none", 86.0, None))
            })),
    )
}

pub fn basic() -> Popover {
    Popover::new(
        PopoverTrigger::new("Open Popover"),
        PopoverContent::new()
            .label(TITLE)
            .align(Align::Start)
            .child(header(TITLE, DESCRIPTION)),
    )
}

pub fn alignments() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_6()
        .children(
            [
                ("Start", "Aligned to start", Align::Start),
                ("Center", "Aligned to center", Align::Center),
                ("End", "Aligned to end", Align::End),
            ]
            .into_iter()
            .map(|(button, label, align)| {
                let trigger = Button::new(button)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Small)
                    .surface();
                Popover::new(
                    PopoverTrigger::render(trigger),
                    PopoverContent::new()
                        .label(label)
                        .width(160.0)
                        .align(align)
                        .child(text(label).text_sm()),
                )
            }),
        )
}

pub fn form() -> Popover {
    let open = signal(false);
    let first = element_ref();
    Popover::new(
        PopoverTrigger::new("Open Popover"),
        PopoverContent::new()
            .label(TITLE)
            .width(256.0)
            .align(Align::Start)
            .initial_focus(first)
            .child(header(TITLE, DESCRIPTION))
            .child(dynamic(move || {
                view()
                    .gap_4()
                    .child(field("Width", "100%", 90.0, Some(first)))
                    .child(field("Height", "25px", 90.0, None))
            })),
    )
    .open(open)
}

struct Locale {
    direction: Direction,
    title: &'static str,
    description: &'static str,
    physical: [&'static str; 4],
    logical: [&'static str; 2],
}

const LOCALES: [Locale; 3] = [
    Locale {
        direction: Direction::Ltr,
        title: TITLE,
        description: DESCRIPTION,
        physical: ["Left", "Top", "Bottom", "Right"],
        logical: ["Inline Start", "Inline End"],
    },
    Locale {
        direction: Direction::Rtl,
        title: "الأبعاد",
        description: "تعيين الأبعاد للطبقة.",
        physical: ["يسار", "أعلى", "أسفل", "يمين"],
        logical: ["بداية السطر", "نهاية السطر"],
    },
    Locale {
        direction: Direction::Rtl,
        title: "מימדים",
        description: "הגדר את המימדים לשכבה.",
        physical: ["שמאל", "למעלה", "למטה", "ימין"],
        logical: ["תחילת השורה", "סוף השורה"],
    },
];

fn sides<const N: usize>(
    locale: &'static Locale,
    sides: [PopoverSide; N],
    labels: [&'static str; N],
) -> View {
    view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_2()
        .children(sides.into_iter().zip(labels).map(|(side, label)| {
            Popover::new(
                PopoverTrigger::new(label),
                PopoverContent::new()
                    .label(locale.title)
                    .side(side)
                    .child(header(locale.title, locale.description)),
            )
        }))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
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
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
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
                use PopoverSide::*;
                view()
                    .gap_4()
                    .child(sides(locale, [Left, Top, Bottom, Right], locale.physical))
                    .child(sides(locale, [InlineStart, InlineEnd], locale.logical))
            })
        }))
}
