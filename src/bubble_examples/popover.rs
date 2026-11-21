use nsh_base::{popover, theme};
use nsh_core::prelude::*;
use nsh_foundation::bubble::*;

pub fn preview() -> impl IntoElement {
    let details = popover()
        .offset(px(6.0))
        .margin(px(8.0))
        .popup_class("w-64 p-3 bg-popover text-popover-foreground rounded-xl border shadow-md")
        .trigger(
            view()
                .role(AccessibilityRole::Button)
                .label("Show error details")
                .cursor_pointer()
                .w(px(24.0))
                .h(px(24.0))
                .items_center()
                .justify_center()
                .rounded_full()
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                .child(text("ⓘ").text_xs()),
        )
        .child(
            view()
                .gap_2()
                .child(
                    text("Command failed with exit code 1")
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM),
                )
                .child(text("ENOENT: no such file or directory, open pnpm-lock.yaml").text_sm()),
        );
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(Bubble::new(BubbleContent::text("Run the build script.")).align(BubbleAlign::End))
        .child(
            Bubble::new(BubbleContent::text("Failed to run the command."))
                .variant(BubbleVariant::Destructive)
                .reactions(BubbleReactions::new().child(details)),
        )
}
