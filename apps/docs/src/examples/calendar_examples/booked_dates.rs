use chrono::Days;
use components::calendar::{Calendar, Matcher, date_this_year};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let first = date_this_year(1, 12);
    let last = first.checked_add_days(Days::new(5));
    view()
        .items_center()
        .gap_2()
        .child(
            Calendar::new()
                .default_value(date_this_year(1, 6))
                .disabled(Matcher::range(Some(first), last)),
        )
        .child(text("January 12–17 is booked and cannot be selected.").text_xs())
}
