use chrono::Days;
use nsh_core::prelude::*;
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    calendar::{Calendar, Date, date_this_year, local_today},
};

pub fn preview() -> impl IntoElement {
    let date = signal(Date::from(date_this_year(2, 12)));
    let today = local_today();
    let mut presets = view().max_w(px(280.0)).flex_row().flex_wrap().gap_2();
    for (label, offset) in [
        ("Today", 0),
        ("Tomorrow", 1),
        ("In 3 days", 3),
        ("In a week", 7),
        ("In 2 weeks", 14),
    ] {
        presets = presets.child(
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || {
                    if let Some(day) = today.checked_add_days(Days::new(offset)) {
                        date.set(day.into());
                    }
                }),
        );
    }
    view()
        .items_center()
        .gap_3()
        .child(Calendar::new().value(date))
        .child(presets)
        .child(
            text(move || {
                format!(
                    "Selected: {}",
                    date.get().format("%Y-%m-%d").unwrap_or_default()
                )
            })
            .text_xs(),
        )
}
