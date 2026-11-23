use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_base::{popover, theme};
use nsh_core::{element::input, prelude::*};

pub fn preview() -> impl IntoElement {
    let task = signal(String::new());
    let open = signal(false);
    let status = signal(String::new());
    view().items_center().gap_2()
        .child(ButtonGroup::new("Copilot controls")
            .child(Button::new("Copilot").variant(ButtonVariant::Outline)
                .icon_start(text("◇")).on_press(move || open.set(true)))
            .control(move |join| {
                popover().align(Align::End).offset(px(6.0)).margin(px(8.0))
                    .open(move || Some(open.get()))
                    .on_open_change(move |event: &nsh_base::PopoverEvent| {
                        if let nsh_base::PopoverEvent::OpenChange(next, _) = event { open.set(*next); }
                    })
                    .popup_class("w-64 p-3 bg-popover text-popover-foreground rounded-xl border shadow-md")
                    .trigger(join.apply(Button::new("Open Popover").variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon).icon_start(text("⌄")).surface()))
                    .child(view().gap_2()
                        .child(text("Start a new task with Copilot").text_sm().font_weight(FontWeight::MEDIUM))
                        .child(text("Describe your task in natural language.").text_sm())
                        .child(input().label("Task Description").placeholder("I need to…").multiline(true)
                            .value(task).h(px(64.0)).p_2().rounded(px(8.0)).text_sm()
                            .border(px(1.0)).border_color(move || theme().border)
                            .text_color(move || theme().foreground)
                            .on_change(move |value: &str| task.set(value.to_owned())))
                        .child(Button::new("Save sample task").variant(ButtonVariant::Secondary).on_press(move || {
                            let value = task.get();
                            if !value.trim().is_empty() {
                                status.set(format!("Draft saved in this preview: {}", value.trim()));
                                open.set(false);
                            }
                        })))
            }))
        .child(text(move || status.get()).text_xs())
}
