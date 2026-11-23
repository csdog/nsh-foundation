use components::bubble::*;
use nsh_core::prelude::*;

const MORE: &str = "The dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.";

pub fn preview() -> impl IntoElement {
    let open = signal(false);
    let report = view().gap_2()
        .child(text(move || if open.get() {
            "The accessibility review found two focus states that were visually too subtle in dark mode. I checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface."
        } else {
            "The accessibility review found two focus states that were visually too subtle in dark mode. I checked the dialog, menu, and drawer paths…"
        }).text_sm())
        .child(when(open, || text(MORE).text_sm()))
        .child(when(open, || {
            text("I also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.").text_sm()
        }))
        .child(view().role(AccessibilityRole::Button)
            .label(move || if open.get() { "Show less" } else { "Show more" })
            .self_start().cursor_pointer().py_1().rounded(px(4.0))
            .accessibility_state(move || AccessibilityState { expanded: Some(open.get()), ..Default::default() })
            .focus_visible(|s| s.ring(px(2.0)).ring_color(nsh_base::theme().ring))
            .on_press(move || open.update(|value| *value = !*value))
            .child(text(move || if open.get() { "Show less ⌃" } else { "Show more ⌄" }).text_xs()));
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_6()
        .child(
            Bubble::new(BubbleContent::text("How can I help you today?"))
                .variant(BubbleVariant::Muted),
        )
        .child(
            Bubble::new(BubbleContent::new(report))
                .variant(BubbleVariant::Muted)
                .align(BubbleAlign::End),
        )
}
