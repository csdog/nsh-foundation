use components::calendar::{Calendar, date_this_year};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Calendar::new()
        .week_numbers(true)
        .default_value(date_this_year(1, 12))
}
