use components::calendar::Calendar;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Calendar::new()
        .persian(true)
        .default_value(chrono::NaiveDate::from_ymd_opt(2025, 6, 12).unwrap())
}
