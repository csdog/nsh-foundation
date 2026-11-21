use nsh_core::prelude::*;
use nsh_foundation::calendar::{Calendar, date_this_year};

pub fn preview() -> impl IntoElement {
    Calendar::new()
        .week_numbers(true)
        .default_value(date_this_year(1, 12))
}
