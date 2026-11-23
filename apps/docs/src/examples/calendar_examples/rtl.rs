use components::calendar::{Calendar, local_today};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Calendar::new().rtl(true).default_value(local_today())
}
