use nsh_core::prelude::*;
use nsh_foundation::calendar::{Calendar, local_today};

pub fn preview() -> impl IntoElement {
    Calendar::new().default_value(local_today())
}
