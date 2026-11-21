use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Variant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

pub(crate) fn button(label: &'static str, variant: Variant, press: impl Fn() + 'static) -> View {
    view()
        .role(AccessibilityRole::Button)
        .label(label)
        .cursor_pointer()
        .h(px(36.0))
        .px_3()
        .flex_shrink_0()
        .justify_center()
        .items_center()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || match variant {
            Variant::Outline => theme().border,
            _ => hsla(0.0, 0.0, 0.0, 0.0).into(),
        })
        .bg(move || match variant {
            Variant::Primary => theme().primary,
            Variant::Secondary => theme().secondary,
            _ => hsla(0.0, 0.0, 0.0, 0.0).into(),
        })
        .hover(|style| style.opacity(0.78))
        .active(|style| style.opacity(0.62))
        .focus_visible(|style| style.outline(px(2.0)).outline_color(rgb(0x71717a)))
        .on_press(press)
        .child(
            text(label)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || match variant {
                    Variant::Primary => theme().primary_foreground,
                    _ => theme().foreground,
                }),
        )
}

pub(crate) fn muted(label: impl IntoReactive<SharedString>) -> Text {
    text(label)
        .text_sm()
        .text_color(move || theme().muted_foreground)
}

pub(crate) fn heading(label: &'static str) -> Text {
    text(label)
        .role(AccessibilityRole::Header)
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
}
