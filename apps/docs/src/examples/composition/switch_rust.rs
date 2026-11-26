use components::switch::switch;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let notifications = signal(false);
    view()
        .gap_2()
        .child(text("Notifications"))
        .child(
            switch()
                .accessibility_label("Notifications")
                .checked(notifications)
                .on_change(move |checked| notifications.set(checked)),
        )
        .child(text(move || if notifications.get() { "On" } else { "Off" }))
}
