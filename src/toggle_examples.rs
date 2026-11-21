use nsh_core::{
    AccessibilityState, SvgPaint,
    canvas::{LineCap, LineJoin},
    prelude::*,
};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    toggle::{Toggle, ToggleSize, ToggleVariant},
};

const BOOKMARK: &str = "M3.5 13.5V3.3C3.5 2.6 4 2.1 4.7 2.1h6.6c.7 0 1.2.5 1.2 1.2v10.2L8 10.2z";
pub(crate) const ITALIC: &str = "M7 3.2h6M3 12.8h6m1-9.6L6 12.8";
pub(crate) const BOLD: &str = "M4.4 8h5.1a2.4 2.4 0 0 1 0 4.8H4.4V3.2h4.5a2.4 2.4 0 0 1 0 4.8";

pub(crate) fn icon(path: &'static str, filled: bool) -> CanvasView {
    let painter = Painter::new()
        .stroke_style_from_view(SvgPaint::CurrentColor)
        .fill_style_from_view(SvgPaint::CurrentColor)
        .line_width(1.35)
        .line_cap(LineCap::Round)
        .line_join(LineJoin::Round);
    let painter = if filled {
        painter.fill_svg(path)
    } else {
        painter
    };
    canvas(painter.stroke_svg(path)).size(px(16.0))
}

fn bookmark(label: &'static str) -> Toggle {
    Toggle::new(label)
        .label("Toggle bookmark")
        .variant(ToggleVariant::Outline)
        .size(ToggleSize::Small)
        .icon(|pressed| {
            dynamic(move || {
                icon(BOOKMARK, pressed.get()).test_id(if pressed.get() {
                    "bookmark-filled"
                } else {
                    "bookmark-outline"
                })
            })
        })
}

pub fn demo() -> Toggle {
    bookmark("Bookmark")
}

fn options() -> View {
    view().flex_row().flex_wrap().items_center().gap_2()
}

pub fn outline() -> View {
    options()
        .child(
            Toggle::new("Italic")
                .label("Toggle italic")
                .variant(ToggleVariant::Outline)
                .icon(|_| icon(ITALIC, false)),
        )
        .child(
            Toggle::new("Bold")
                .label("Toggle bold")
                .variant(ToggleVariant::Outline)
                .icon(|_| icon(BOLD, false)),
        )
}

pub fn with_text() -> Toggle {
    Toggle::new("Italic")
        .label("Toggle italic")
        .icon(|_| icon(ITALIC, false))
}

pub fn sizes() -> View {
    options().children(
        [
            (ToggleSize::Small, "Small", "Toggle small"),
            (ToggleSize::Default, "Default", "Toggle default"),
            (ToggleSize::Large, "Large", "Toggle large"),
        ]
        .map(|(size, text, label)| {
            Toggle::new(text)
                .label(label)
                .size(size)
                .variant(ToggleVariant::Outline)
        }),
    )
}

pub fn disabled() -> View {
    options()
        .child(
            Toggle::new("Disabled")
                .label("Toggle disabled")
                .disabled(true),
        )
        .child(
            Toggle::new("Disabled")
                .label("Toggle disabled outline")
                .variant(ToggleVariant::Outline)
                .disabled(true),
        )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let pressed = signal(false);
    let translations = [
        ("English", Direction::Ltr, "Bookmark"),
        ("Arabic", Direction::Rtl, "إشارة مرجعية"),
        ("Hebrew", Direction::Rtl, "סימנייה"),
    ];
    view()
        .gap_4()
        .items_center()
        .child(
            options()
                .justify_center()
                .children(translations.into_iter().enumerate().map(
                    move |(index, (name, _, _))| {
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
                )),
        )
        .child(dynamic(move || {
            let (_, direction, label) = translations[language.get()];
            DirectionProvider::new(direction, move || {
                bookmark(label)
                    .pressed(pressed)
                    .on_change(move |next| pressed.set(next))
            })
        }))
}
