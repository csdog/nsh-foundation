use nsh_core::prelude::*;
use nsh_foundation::bubble::*;

pub fn preview() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_6()
        .child(
            Bubble::new(BubbleContent::text(
                "This bubble is aligned to the start. This is the default alignment.",
            ))
            .variant(BubbleVariant::Muted),
        )
        .child(
            Bubble::new(BubbleContent::text(
                "This bubble is aligned to the end. Use this for user messages.",
            ))
            .align(BubbleAlign::End),
        )
}
