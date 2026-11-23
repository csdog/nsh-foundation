use components::calendar::{Calendar, Date, local_today};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let date = signal(Date::from(local_today()));
    view()
        .items_center()
        .gap_2()
        .child(Calendar::new().value(date))
        .child(
            text(move || {
                format!(
                    "Local date: {}",
                    date.get().format("%Y-%m-%d").unwrap_or_default()
                )
            })
            .text_xs(),
        )
}
