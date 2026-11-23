use components::bubble::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view().w_full().max_w(px(384.0)).gap_6()
        .child(Bubble::new(BubbleContent::text("Hey there! what's up?")).align(BubbleAlign::End))
        .child(BubbleGroup::new()
            .child(Bubble::new(BubbleContent::text("Hey! Want to see chat bubbles?")).variant(BubbleVariant::Muted))
            .child(Bubble::new(BubbleContent::text("I can group messages, switch sides, and keep the whole thread easy to scan."))
                .variant(BubbleVariant::Muted)
                .reactions(BubbleReactions::new().label("Reaction: thumbs up").child(text("👍").text_sm()))))
        .child(Bubble::new(BubbleContent::text("Sure. Hit me with your best demo.")).align(BubbleAlign::End))
        .child(Bubble::new(BubbleContent::text("Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand."))
            .variant(BubbleVariant::Muted)
            .reactions(BubbleReactions::new().label("Reactions: thumbs up, fire, eyes, and two more")
                .child(text("👍").text_sm()).child(text("🔥").text_sm()).child(text("👀").text_sm()).child(text("+2").text_sm())))
}
