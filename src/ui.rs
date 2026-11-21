use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Variant {
    Primary,
    Destructive,
    Secondary,
    Outline,
    Ghost,
}

pub(crate) fn button(label: &'static str, variant: Variant, press: impl Fn() + 'static) -> View {
    button_surface(label, variant).on_press(press)
}

pub(crate) fn button_surface(label: &'static str, variant: Variant) -> View {
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
            Variant::Destructive => theme().destructive,
            Variant::Secondary => theme().secondary,
            _ => hsla(0.0, 0.0, 0.0, 0.0).into(),
        })
        .hover(|style| style.opacity(0.78))
        .active(|style| style.opacity(0.62))
        .focus_visible(|style| style.outline(px(2.0)).outline_color(rgb(0x71717a)))
        .child(
            text(label)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || match variant {
                    Variant::Primary => theme().primary_foreground,
                    Variant::Destructive => rgb(0xffffff).into(),
                    _ => theme().foreground,
                }),
        )
}

pub(crate) fn muted(label: impl IntoReactive<SharedString>) -> Text {
    text(label)
        .text_sm()
        .text_color(move || theme().muted_foreground)
}

pub(crate) fn heading(label: impl IntoReactive<SharedString>) -> Text {
    text(label)
        .role(AccessibilityRole::Header)
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
}
