use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonVariant},
    direction::{Direction, DirectionProvider},
    hover_card::{HoverCard, HoverCardContent, HoverCardSide, HoverCardTrigger},
    ui::muted,
};

const PHYSICAL_SIDES: [HoverCardSide; 4] = [
    HoverCardSide::Left,
    HoverCardSide::Top,
    HoverCardSide::Bottom,
    HoverCardSide::Right,
];

const LOGICAL_SIDES: [HoverCardSide; 2] = [HoverCardSide::InlineStart, HoverCardSide::InlineEnd];

fn row() -> View {
    view().flex_row().flex_wrap().justify_center().gap_2()
}

fn stage() -> View {
    view()
        .w_full()
        .min_h(px(280.0))
        .items_center()
        .justify_center()
}

pub fn demo() -> impl IntoElement {
    stage().child(HoverCard::new(
        HoverCardTrigger::new("Hover Here")
            .delay(10.0)
            .close_delay(100.0),
        HoverCardContent::new().width(256).child(
            view()
                .gap_1()
                .child(text("@nextjs").text_sm().font_weight(FontWeight::SEMIBOLD))
                .child(text("The React Framework – created and maintained by @vercel.").text_sm())
                .child(muted("Joined December 2021").text_xs().mt_1()),
        ),
    ))
}

pub fn sides() -> impl IntoElement {
    let labels = ["Left", "Top", "Bottom", "Right"];
    stage().child(
        row().children(PHYSICAL_SIDES.into_iter().zip(labels).map(|(side, label)| {
            HoverCard::new(
                HoverCardTrigger::render(
                    Button::new(label).variant(ButtonVariant::Outline).surface(),
                )
                .delay(100.0)
                .close_delay(100.0),
                HoverCardContent::new().side(side).child(
                    view()
                        .gap_1()
                        .child(text("Hover Card").text_sm().font_weight(FontWeight::MEDIUM))
                        .child(
                            text(format!(
                                "This hover card appears on the {} side of the trigger.",
                                label.to_lowercase()
                            ))
                            .text_sm(),
                        ),
                ),
            )
        })),
    )
}

struct Product {
    title: &'static str,
    price: &'static str,
    physical: [&'static str; 4],
    logical: [&'static str; 2],
}

const ENGLISH: Product = Product {
    title: "Wireless Headphones",
    price: "$99.99",
    physical: ["Left", "Top", "Bottom", "Right"],
    logical: ["Inline Start", "Inline End"],
};

const ARABIC: Product = Product {
    title: "سماعات لاسلكية",
    price: "٩٩.٩٩ $",
    physical: ["يسار", "أعلى", "أسفل", "يمين"],
    logical: ["بداية السطر", "نهاية السطر"],
};

const HEBREW: Product = Product {
    title: "אוזניות אלחוטיות",
    price: "99.99 $",
    physical: ["שמאל", "למעלה", "למטה", "ימין"],
    logical: ["תחילת השורה", "סוף השורה"],
};

fn product_preview(product: &Product, side: HoverCardSide, label: &'static str) -> HoverCard {
    HoverCard::new(
        HoverCardTrigger::render(Button::new(label).variant(ButtonVariant::Outline).surface())
            .delay(10.0)
            .close_delay(100.0),
        HoverCardContent::new().side(side).width(256).child(
            view()
                .gap_1()
                .child(
                    text(product.title)
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD),
                )
                .child(muted(product.price)),
        ),
    )
}

pub fn rtl() -> impl IntoElement {
    let selected = signal(1_usize);
    let direction = move || {
        if selected.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    stage()
        .gap_4()
        .child(
            row().children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                |(index, label)| {
                    Button::new(label)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || selected.set(index))
                },
            )),
        )
        .child(DirectionProvider::new(direction, move || {
            dynamic(move || {
                let product = [&ENGLISH, &ARABIC, &HEBREW][selected.get()];
                view()
                    .gap_3()
                    .items_center()
                    .child(
                        row().children(
                            PHYSICAL_SIDES
                                .into_iter()
                                .zip(product.physical)
                                .map(|(side, label)| product_preview(product, side, label)),
                        ),
                    )
                    .child(
                        row().children(
                            LOGICAL_SIDES
                                .into_iter()
                                .zip(product.logical)
                                .map(|(side, label)| product_preview(product, side, label)),
                        ),
                    )
            })
        }))
}
