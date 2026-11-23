use std::f32::consts::TAU;

use components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    input_group::{
        InputGroup, InputGroupAddon, InputGroupAlign, input_group_button, input_group_input,
        input_group_text, input_group_textarea,
    },
    item::{Item, ItemContent, ItemMedia, ItemTitle, ItemVariant},
    spinner::Spinner,
};
use nsh_core::{SvgPaint, canvas::LineCap, element::announce_for_accessibility, prelude::*};

fn payment(title: &'static str, amount: &'static str, loading: &'static str) -> View {
    view().w_full().max_w(px(320.0)).child(
        Item::new()
            .variant(ItemVariant::Muted)
            .media(ItemMedia::new(Spinner::new().label(loading)))
            .content(ItemContent::new().title(ItemTitle::new(title)))
            .content(ItemContent::new().child(text(amount).text_sm())),
    )
}

pub fn demo() -> View {
    payment("Processing payment...", "$100.00", "Loading")
}

pub fn custom() -> impl IntoElement {
    let mut painter = Painter::new()
        .stroke_style_from_view(SvgPaint::CurrentColor)
        .line_width(16.0 / 12.0)
        .line_cap(LineCap::Round)
        .begin_path();
    for index in 0..8 {
        let angle = index as f32 * TAU / 8.0;
        let (y, x) = angle.sin_cos();
        painter = painter
            .move_to(8.0 + x * 8.0 / 3.0, 8.0 + y * 8.0 / 3.0)
            .line_to(8.0 + x * 20.0 / 3.0, 8.0 + y * 20.0 / 3.0);
    }
    Spinner::new().icon(canvas(painter.stroke()).size(px(16.0)))
}

pub fn size() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .items_center()
        .gap_6()
        .children([12.0, 16.0, 24.0, 32.0].map(|size| Spinner::new().size(size)))
}

pub fn button() -> View {
    view().items_center().gap_4().children(
        [
            ("Loading...", ButtonVariant::Default),
            ("Please wait", ButtonVariant::Outline),
            ("Processing", ButtonVariant::Secondary),
        ]
        .map(|(label, variant)| {
            Button::new(label)
                .variant(variant)
                .disabled(true)
                .size(ButtonSize::Small)
                .icon_start(Spinner::new().decorative())
        }),
    )
}

pub fn badge() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .items_center()
        .gap_4()
        .children(
            [
                ("Syncing", BadgeVariant::Default),
                ("Updating", BadgeVariant::Secondary),
                ("Processing", BadgeVariant::Outline),
            ]
            .map(|(label, variant)| {
                Badge::new(label)
                    .variant(variant)
                    .icon_start(Spinner::new().size(12.0).decorative())
            }),
        )
}

pub fn input_group() -> View {
    let sent = signal(false);
    view()
        .w_full()
        .max_w(px(448.0))
        .gap_4()
        .child(
            InputGroup::new(
                input_group_input()
                    .label("Message")
                    .placeholder("Send a message...")
                    .disabled(true),
            )
            .addon(InputGroupAddon::new(InputGroupAlign::InlineEnd).child(Spinner::new())),
        )
        .child(
            InputGroup::new(
                input_group_textarea()
                    .label("Message body")
                    .placeholder("Send a message...")
                    .disabled(true),
            )
            .addon(
                InputGroupAddon::new(InputGroupAlign::BlockEnd)
                    .child(when(move || !sent.get(), || Spinner::new().decorative()))
                    .child(input_group_text(move || {
                        if sent.get() {
                            "Sent in preview"
                        } else {
                            "Validating..."
                        }
                    }))
                    .action(
                        input_group_button("Send")
                            .variant(ButtonVariant::Default)
                            .size(ButtonSize::IconExtraSmall)
                            .disabled(sent)
                            .icon_start(text("↑").text_sm().accessibility_elements_hidden(true))
                            .on_press(move || {
                                sent.set(true);
                                announce_for_accessibility("Sent in preview");
                            })
                            .ml(auto()),
                    ),
            ),
        )
}

pub fn empty() -> View {
    let active = signal(true);
    view().w_full().child(dynamic(move || {
        if active.get() {
            Empty::new()
                .child(
                    EmptyHeader::new()
                        .child(
                            EmptyMedia::new()
                                .variant(EmptyMediaVariant::Icon)
                                .child(Spinner::new().decorative()),
                        )
                        .child(EmptyTitle::new("Processing your request"))
                        .child(EmptyDescription::new(
                            "Please wait while we process your request. Do not refresh the page.",
                        )),
                )
                .child(
                    EmptyContent::new().child(
                        Button::new("Cancel")
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
                            .on_press(move || {
                                active.set(false);
                                announce_for_accessibility("Request canceled");
                            }),
                    ),
                )
        } else {
            Empty::new().child(EmptyHeader::new().child(EmptyTitle::new("Request canceled")))
        }
    }))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    view()
        .w_full()
        .max_w(px(320.0))
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
        .child(dynamic(move || {
            let (direction, title, amount, loading) = match language.get() {
                0 => (
                    Direction::Ltr,
                    "Processing payment...",
                    "$100.00",
                    "Loading",
                ),
                1 => (
                    Direction::Rtl,
                    "جاري معالجة الدفع...",
                    "١٠٠.٠٠ دولار",
                    "جارٍ التحميل",
                ),
                _ => (Direction::Rtl, "מעבד תשלום...", "$100.00", "טוען"),
            };
            DirectionProvider::new(direction, move || payment(title, amount, loading))
        }))
}
