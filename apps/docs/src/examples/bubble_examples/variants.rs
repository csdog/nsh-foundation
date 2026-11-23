use components::bubble::*;
use nsh_base::text_view;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    use BubbleVariant::*;
    let mut stack = view().w_full().max_w(px(384.0)).gap_5();
    for (variant, align, message, reaction) in [
        (
            Default,
            BubbleAlign::Start,
            "This is the default primary bubble.",
            None,
        ),
        (
            Secondary,
            BubbleAlign::End,
            "This is the secondary variant.",
            None,
        ),
        (
            Muted,
            BubbleAlign::Start,
            "This one is muted. It uses a lower emphasis color for the chat bubble.",
            Some(("Reaction: thumbs up", "👍")),
        ),
        (
            Tinted,
            BubbleAlign::End,
            "This one is tinted. The tint is a softer color derived from the primary color.",
            None,
        ),
        (
            Outline,
            BubbleAlign::Start,
            "We can also use an outlined variant.",
            None,
        ),
        (
            Destructive,
            BubbleAlign::End,
            "Or a destructive variant with a reaction.",
            Some(("Reaction: fire", "🔥")),
        ),
    ] {
        let mut bubble = Bubble::new(BubbleContent::text(message))
            .variant(variant)
            .align(align);
        if let Some((label, icon)) = reaction {
            bubble = bubble.reactions(
                BubbleReactions::new()
                    .label(label)
                    .child(text(icon).text_sm()),
            );
        }
        stack = stack.child(bubble);
    }
    stack.child(Bubble::new(BubbleContent::new(
        text_view().markdown("Ghost bubbles work for assistant text, **Markdown**, and inline `code`.\n\nThey can take the full width of the container.")
            .text_sm().line_height(px(20.0)),
    )).variant(Ghost))
}
