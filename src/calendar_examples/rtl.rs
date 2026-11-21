use nsh_core::prelude::*;
use nsh_foundation::calendar::{Calendar, local_today};

pub fn preview() -> impl IntoElement {
    Calendar::new().rtl(true).default_value(local_today())
}
