use chrono::Days;
use components::calendar::{Calendar, Date, date_this_year};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let start = date_this_year(12, 8);
    let value = signal(Date::Range(
        Some(start),
        start.checked_add_days(Days::new(10)),
    ));
    view()
        .items_center()
        .gap_2()
        .child(
            Calendar::new()
                .range(true)
                .months(1)
                .value(value)
                .cell_size(|| {
                    if window().width.0 >= 768.0 {
                        48.0_f32
                    } else {
                        44.0_f32
                    }
                }),
        )
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
