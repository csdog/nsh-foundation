use nsh_core::prelude::*;
use nsh_foundation::calendar::Calendar;

pub fn preview() -> impl IntoElement {
    Calendar::new()
}
