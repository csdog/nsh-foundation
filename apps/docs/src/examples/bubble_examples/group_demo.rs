use components::bubble::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_6()
        .child(
            Bubble::new(BubbleContent::text("Can you tell me what's the issue?"))
                .variant(BubbleVariant::Muted),
        )
        .child(
            BubbleGroup::new()
                .child(Bubble::new(BubbleContent::text("You tell me!")).align(BubbleAlign::End))
                .child(
                    Bubble::new(BubbleContent::text("It worked yesterday. You broke it!"))
                        .align(BubbleAlign::End),
                )
                .child(
                    Bubble::new(BubbleContent::text("Find the bug and fix it."))
                        .align(BubbleAlign::End)
                        .reactions(
                            BubbleReactions::new()
                                .align(BubbleAlign::Start)
                                .label("Reaction: eyes")
                                .child(text("👀").text_sm()),
                        ),
                ),
        )
        .child(
            Bubble::new(BubbleContent::text(
                "Want me to diff yesterday's you against today's you? It's a bit embarrassing.",
            ))
            .variant(BubbleVariant::Muted),
        )
}
