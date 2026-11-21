use std::time::Duration;

use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonVariant},
    toast::{Toast, ToastKind, Toaster},
};

pub fn demo(toaster: Toaster) -> View {
    Button::new("Show Toast")
        .variant(ButtonVariant::Outline)
        .on_press(move || {
            toaster.show(
                Toast::new("Event created")
                    .description("Sunday, December 3 at 9:00 AM")
                    .action("Undo", || {}),
            );
        })
}

pub fn types(toaster: Toaster) -> View {
    view().flex_row().flex_wrap().gap_2().children(
        [
            ("Default", ToastKind::Default, "Event has been created."),
            ("Success", ToastKind::Success, "Event has been created."),
            (
                "Info",
                ToastKind::Info,
                "Arrive 10 minutes before the event.",
            ),
            (
                "Warning",
                ToastKind::Warning,
                "The event cannot start before 8:00 AM.",
            ),
            ("Error", ToastKind::Error, "The event could not be created."),
        ]
        .map(|(label, kind, description)| {
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .on_press(move || {
                    toaster.show(Toast::message(description).kind(kind));
                })
        }),
    )
}

pub fn promise(toaster: Toaster) -> View {
    Button::new("Create Event")
        .variant(ButtonVariant::Outline)
        .on_press(move || {
            toaster.promise(
                async {
                    timer(Duration::from_secs(2)).await;
                    Ok::<_, ()>("Event")
                },
                "Creating event…",
                |result| match result {
                    Ok(event) => Toast::new(format!("{event} created.")).kind(ToastKind::Success),
                    Err(()) => Toast::new("Could not create event.").kind(ToastKind::Error),
                },
            );
        })
}
