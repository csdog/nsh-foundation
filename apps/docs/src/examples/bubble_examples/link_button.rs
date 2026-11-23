use components::bubble::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let selected = signal(None::<&'static str>);
    let mut choices = BubbleGroup::new();
    for choice in [
        "I forgot my password",
        "I need help with my subscription",
        "Something else. Talk to a human.",
    ] {
        choices = choices.child(
            Bubble::new(BubbleContent::button(choice, move || {
                selected.set(Some(choice))
            }))
            .variant(BubbleVariant::Tinted)
            .align(BubbleAlign::End),
        );
    }
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_6()
        .child(
            Bubble::new(BubbleContent::text("How can I help you today?"))
                .variant(BubbleVariant::Muted),
        )
        .child(choices)
        .child(when(
            move || selected.get().is_some(),
            move || {
                text(move || {
                    format!(
                        "Selected in this preview: {}",
                        selected.get().unwrap_or_default()
                    )
                })
                .text_sm()
            },
        ))
}
