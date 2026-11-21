use nsh_base::{menu, menu_item, menu_separator, submenu, theme};
use nsh_core::prelude::*;
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    let status = signal("");
    let label = signal("Personal");
    let outline = |name| Button::new(name).variant(ButtonVariant::Outline);
    let item = |name: &'static str| {
        menu_item()
            .accessibility_label(name)
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || status.set(name))
            .child(text(name).text_sm().text_color(move || {
                if name == "Trash" {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            }))
    };
    let radio = |name: &'static str| {
        menu_item()
            .accessibility_label(name)
            .role(AccessibilityRole::MenuItemRadio)
            .checked(move || Some(label.get() == name))
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || {
                label.set(name);
                status.set(name);
            })
            .child(text(name).text_sm())
    };
    let more = menu()
        .align(Align::End)
        .offset(px(5.0))
        .margin(px(8.0))
        .popup_class("w-40 p-1 bg-popover text-popover-foreground rounded-xl border shadow-md")
        .trigger(
            outline("More Options")
                .size(ButtonSize::Icon)
                .icon_start(text("…"))
                .surface()
                .rounded_l(px(0.0))
                .ml(px(-1.0)),
        )
        .child(item("Mark as Read"))
        .child(item("Archive"))
        .child(separator())
        .child(item("Snooze"))
        .child(item("Add to Calendar"))
        .child(item("Add to List"))
        .child(
            submenu()
                .trigger(view().px_2().py_1().child(text("Label As…").text_sm()))
                .child(radio("Personal"))
                .child(radio("Work"))
                .child(radio("Other")),
        )
        .child(separator())
        .child(item("Trash"));

    view()
        .items_center()
        .gap_2()
        .child(
            view()
                .role(AccessibilityRole::Group)
                .label("Message actions")
                .flex_row()
                .flex_wrap()
                .gap_2()
                .child(when(
                    || window().width.0 >= 640.0,
                    move || {
                        outline("Go Back")
                            .size(ButtonSize::Icon)
                            .icon_start(text("←"))
                            .on_press(move || status.set("Go Back"))
                    },
                ))
                .child(
                    view()
                        .flex_row()
                        .child(
                            outline("Archive")
                                .on_press(move || status.set("Archive"))
                                .rounded_r(px(0.0)),
                        )
                        .child(
                            outline("Report")
                                .on_press(move || status.set("Report"))
                                .rounded_l(px(0.0))
                                .ml(px(-1.0)),
                        ),
                )
                .child(
                    view()
                        .flex_row()
                        .child(
                            outline("Snooze")
                                .on_press(move || status.set("Snooze"))
                                .rounded_r(px(0.0)),
                        )
                        .child(more),
                ),
        )
        .child(when(
            move || !status.get().is_empty(),
            move || text(move || format!("Selected in this preview: {}", status.get())).text_xs(),
        ))
}

fn separator() -> impl IntoElement {
    menu_separator()
        .h(px(1.0))
        .my_1()
        .bg(move || theme().border)
}
