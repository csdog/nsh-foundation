use components::bubble::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let confirmed = signal(false);
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_7()
        .child(
            Bubble::new(BubbleContent::text(
                "I don't need tests, I know my code works.",
            ))
            .variant(BubbleVariant::Muted)
            .align(BubbleAlign::End)
            .reactions(
                BubbleReactions::new()
                    .align(BubbleAlign::Start)
                    .label("Reactions: thumbs up and surprised")
                    .child(text("👍"))
                    .child(text("😮")),
            ),
        )
        .child(
            Bubble::new(BubbleContent::text(
                "Bold. Fine I'll add some tests. I'll let you know when they're done.",
            ))
            .variant(BubbleVariant::Muted)
            .reactions(
                BubbleReactions::new()
                    .label("Reactions: eyes, rocket, and two more")
                    .child(text("👀"))
                    .child(text("🚀"))
                    .child(text("+2").text_sm()),
            ),
        )
        .child(
            Bubble::new(BubbleContent::text(
                "Tests passed on the first try. All 142 of them. Looking good!",
            ))
            .align(BubbleAlign::End)
            .reactions(
                BubbleReactions::new()
                    .side(ReactionSide::Top)
                    .align(BubbleAlign::Start)
                    .label("Reactions: party popper and clapping hands")
                    .child(text("🎉"))
                    .child(text("👏")),
            ),
        )
        .child(
            Bubble::new(BubbleContent::text("Are you sure I can run this command?"))
                .variant(BubbleVariant::Destructive)
                .reactions(
                    BubbleReactions::new().child(
                        view()
                            .role(AccessibilityRole::Button)
                            .label("Yes, run it")
                            .cursor_pointer()
                            .px_2()
                            .py_1()
                            .rounded_full()
                            .on_press(move || confirmed.set(true))
                            .focus_visible(|s| s.ring(px(2.0)).ring_color(nsh_base::theme().ring))
                            .child(text("Yes, run it").text_xs()),
                    ),
                ),
        )
        .child(when(confirmed, || {
            text("Confirmed in this preview.").text_sm()
        }))
}
