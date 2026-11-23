use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_base::{menu, menu_item, menu_separator, theme};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let following = signal(false);
    let status = signal("");
    let item = move |label: &'static str| {
        menu_item()
            .accessibility_label(label)
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || status.set(label))
            .child(text(label).text_sm().text_color(move || {
                if label == "Delete Conversation" {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            }))
    };
    view().items_center().gap_2()
        .child(ButtonGroup::new("Conversation controls")
            .child(Button::new(move || if following.get() { "Following" } else { "Follow" })
                .variant(ButtonVariant::Outline).on_press(move || following.update(|value| *value = !*value)))
            .control(move |join| {
                menu().align(Align::End).offset(px(5.0)).margin(px(8.0))
                    .popup_class("w-44 p-1 bg-popover text-popover-foreground rounded-xl border shadow-md")
                    .trigger(join.apply(Button::new("Conversation options").variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon).icon_start(text("⌄")).surface()))
                    .child(item("Mute Conversation"))
                    .child(item("Mark as Read"))
                    .child(item("Report Conversation"))
                    .child(item("Block User"))
                    .child(item("Share Conversation"))
                    .child(item("Copy Conversation"))
                    .child(menu_separator().h(px(1.0)).my_1().bg(move || theme().border))
                    .child(item("Delete Conversation"))
            }))
        .child(when(move || !status.get().is_empty(), move || text(move || format!("Selected in this preview: {}", status.get())).text_xs()))
}
