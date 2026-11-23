use components::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
};
use nsh_base::{SelectEntry, SelectItem, select, theme};
use nsh_core::{element::input, prelude::*};

pub fn preview() -> impl IntoElement {
    let currency = signal(Some(SharedString::from("$")));
    let amount = signal(String::new());
    let status = signal(String::new());
    view().items_center().gap_2().child(
        ButtonGroup::new("Payment controls")
            .separate(ButtonGroup::new("Currency and amount")
                .control(move |join| {
                    select().accessibility_label("Currency")
                        .items(vec![SelectItem::new("$", "US Dollar"), SelectItem::new("€", "Euro"), SelectItem::new("£", "British Pound")])
                        .value(move || currency.get())
                        .on_change(move |value: &Option<SharedString>| currency.set(value.clone()))
                        .trigger(join.apply(Button::new(move || currency.get().unwrap_or_default())
                            .variant(ButtonVariant::Outline).icon_end(text("⌄")).surface()))
                        .popup_class("w-36 p-1 bg-popover text-popover-foreground rounded-xl border shadow-md")
                        .item_class("px-2 py-1 rounded-md focus:bg-muted text-sm")
                        .render_item(|entry: &SelectEntry| text(entry.title.clone()).text_sm())
                })
                .child(input().label("Amount").placeholder("10.00").value(amount)
                    .w(px(104.0)).min_w(px(40.0)).h(px(32.0)).px_3().rounded(px(16.0))
                    .text_sm().border(px(1.0)).border_color(move || theme().border)
                    .bg(move || theme().input.opacity(0.5)).text_color(move || theme().foreground)
                    .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                    .on_change(move |value: &str| amount.set(value.to_owned()))))
            .separate(Button::new("Send amount").variant(ButtonVariant::Outline)
                .size(ButtonSize::Icon).icon_start(text("↑")).on_press(move || {
                    let amount = amount.get();
                    if let Ok(value) = amount.trim().parse::<f64>() {
                        if value.is_finite() && value >= 0.0 {
                            status.set(format!("Preview amount: {}{value:.2}", currency.get().unwrap_or_default()));
                            return;
                        }
                    }
                    status.set("Enter a valid amount".to_owned());
                })),
    ).child(text(move || status.get()).text_xs())
}
