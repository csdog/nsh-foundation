use components::calendar::Calendar;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Calendar::new()
}
