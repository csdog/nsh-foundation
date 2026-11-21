use nsh_core::prelude::*;
use nsh_foundation::calendar::Calendar;

pub fn preview() -> impl IntoElement {
    view()
        .items_center()
        .gap_2()
        .child(Calendar::new())
        .child(text("Select the month or year in the header to jump to a date.").text_xs())
}
