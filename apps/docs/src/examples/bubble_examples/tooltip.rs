use components::bubble::*;
use nsh_base::{TooltipEvent, theme, tooltip};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let open = signal(false);
    let checked = view()
        .role(AccessibilityRole::Button)
        .label("Read receipt")
        .focusable(true)
        .cursor_pointer()
        .w(px(24.0))
        .h(px(24.0))
        .items_center()
        .justify_center()
        .rounded_full()
        .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
        .on_press(move || open.set(true))
        .child(text("✓").text_xs())
        .tooltip(
            tooltip()
                .open(move || Some(open.get()))
                .on_open_change(move |event: &TooltipEvent| {
                    if let TooltipEvent::OpenChange(next, _) = event {
                        open.set(*next);
                    }
                })
                .delay(250.0)
                .px_2()
                .py_1()
                .rounded(px(8.0))
                .bg(move || theme().foreground)
                .text_color(move || theme().background)
                .child(text("Read on Jan 5, 2026 at 4:32 PM").text_xs()),
        );
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(
            Bubble::new(BubbleContent::text("Did you remove the stale route?"))
                .variant(BubbleVariant::Secondary),
        )
        .child(
            Bubble::new(BubbleContent::text("Yes, removed it from the registry."))
                .align(BubbleAlign::End)
                .reactions(BubbleReactions::new().child(checked)),
        )
}
