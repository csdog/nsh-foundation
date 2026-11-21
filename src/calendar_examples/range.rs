use chrono::Days;
use nsh_core::prelude::*;
use nsh_foundation::calendar::{Calendar, Date, date_this_year};

pub fn preview() -> impl IntoElement {
    let start = date_this_year(1, 12);
    let value = signal(Date::Range(
        Some(start),
        start.checked_add_days(Days::new(30)),
    ));
    view()
        .items_center()
        .gap_2()
        .child(Calendar::new().range(true).months(2).value(value))
        .child(
            text(move || {
                value
                    .get()
                    .format("%Y-%m-%d")
                    .unwrap_or_else(|| "Select a start and end date".into())
            })
            .text_xs(),
        )
}
