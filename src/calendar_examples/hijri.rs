use nsh_core::prelude::*;
use nsh_foundation::calendar::Calendar;

pub fn preview() -> impl IntoElement {
    Calendar::new()
        .persian(true)
        .default_value(chrono::NaiveDate::from_ymd_opt(2025, 6, 12).unwrap())
}
