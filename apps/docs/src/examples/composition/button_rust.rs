use components::button::button;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let saved = signal(false);
    view()
        .gap_2()
        .child(
            button()
                .style_with(|style| style.px(px(16.0)))
                .child(text("Save"))
                .on_press(move || saved.set(true)),
        )
        .child(text(move || if saved.get() { "Saved." } else { "" }))
}
