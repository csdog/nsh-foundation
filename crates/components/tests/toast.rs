use std::time::Duration;

use components::{
    button::Button,
    toast::{Toast, ToastKind, Toaster},
};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn Notifications() -> impl IntoElement {
    let toaster = Toaster::new();
    let undone = signal(false);
    view()
        .child(Button::new("Show").on_press(move || {
            toaster.show(Toast::new("Event created").action("Undo", move || undone.set(true)));
        }))
        .child(Button::new("Start").on_press(move || {
            toaster.promise(
                async {
                    timer(Duration::from_millis(600)).await;
                    Err::<(), _>("Could not create event")
                },
                "Creating event",
                |result| Toast::new(result.unwrap_err()).kind(ToastKind::Error),
            );
        }))
        .child(when(undone, || text("Undone")))
        .child(toaster.viewport())
}

#[test]
fn action_runs_and_the_async_failure_replaces_its_pending_toast() {
    let mut config = EngineConfig::new(Root::of::<Notifications>());
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    config.viewport = Size::new(px(390.0), px(844.0));
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let timeout = Duration::from_secs(3);
    app.press("role:button[Show]").unwrap();
    app.wait_for("Event created").unwrap();
    app.press("role:button[Undo]").unwrap();
    app.wait_for("Undone").unwrap();
    app.wait_for_gone("Event created", timeout).unwrap();

    app.press("role:button[Show]").unwrap();
    app.wait_for("Event created").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("Event created", timeout).unwrap();

    app.press("role:button[Start]").unwrap();
    app.wait_for("Creating event").unwrap();
    app.wait_for_within("Could not create event", timeout)
        .unwrap();
    assert!(!app.shows("Creating event"));
    assert_eq!(app.screen().count("id:toast-card"), Ok(1));

    app.press("role:button[Close toast]").unwrap();
    app.wait_for_gone("id:toast-card", timeout).unwrap();
}
