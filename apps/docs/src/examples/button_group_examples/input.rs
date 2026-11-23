use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_base::theme;
use nsh_core::{element::input, prelude::*};

pub fn preview() -> impl IntoElement {
    let query = signal(String::new());
    let result = signal(String::new());
    let search = move || {
        let query = query.get();
        if !query.trim().is_empty() {
            result.set(format!("Searched for {}", query.trim()));
        }
    };
    view()
        .items_center()
        .gap_2()
        .child(
            ButtonGroup::new("Search controls")
                .child(
                    input()
                        .label("Search query")
                        .placeholder("Search…")
                        .value(query)
                        .w(px(176.0))
                        .min_w(px(40.0))
                        .h(px(32.0))
                        .px_3()
                        .rounded(px(16.0))
                        .text_sm()
                        .border(px(1.0))
                        .border_color(move || theme().border)
                        .bg(move || theme().input.opacity(0.5))
                        .text_color(move || theme().foreground)
                        .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                        .on_change(move |value: &str| query.set(value.to_owned()))
                        .on_submit(move |_: &str| search()),
                )
                .child(
                    Button::new("Search")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .icon_start(text("⌕"))
                        .on_press(search),
                ),
        )
        .child(text(move || result.get()).text_xs())
}
