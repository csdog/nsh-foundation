use components::calendar::{Calendar, local_today};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Calendar::new().default_value(local_today())
}
