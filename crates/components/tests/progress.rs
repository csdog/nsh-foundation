use components::{
    button::Button,
    progress::{Progress, ProgressLabel, ProgressValue},
};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn RangedProgress() -> impl IntoElement {
    let value = signal(30.0_f32);
    let pending = signal(false);
    view()
        .child(
            Progress::new(value)
                .range(20.0, 60.0)
                .indeterminate(pending)
                .label(ProgressLabel::new("Upload progress"))
                .display(ProgressValue::new()),
        )
        .child(Button::new("Above range").on_press(move || value.set(90.0)))
        .child(Button::new("Below range").on_press(move || value.set(10.0)))
        .child(Button::new("Waiting").on_press(move || pending.set(true)))
}

#[test]
fn nonzero_range_clamps_both_ends_and_unknown_progress_has_no_fill() {
    let mut config = EngineConfig::new(Root::of::<RangedProgress>());
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    config.viewport = Size::new(px(480.0), px(360.0));
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let width = |app: &Headless, id: &str| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
            .size
            .width
            .0
    };
    let track = width(&app, "progress-track");
    assert!(track > 0.0);
    let ratio = |app: &Headless| width(app, "progress-indicator") / track;
    assert!((ratio(&app) - 0.25).abs() < 0.01);

    app.press("role:button[Above range]").unwrap();
    app.wait_for("text:100%").unwrap();
    assert!((ratio(&app) - 1.0).abs() < 0.01);

    app.press("role:button[Below range]").unwrap();
    app.wait_for("text:0%").unwrap();
    assert_eq!(ratio(&app), 0.0);

    app.press("role:button[Waiting]").unwrap();
    app.wait_for("text:—").unwrap();
    assert_eq!(ratio(&app), 0.0);
}

#[test]
fn progress_reacts_to_a_runtime_reduced_motion_change() {
    let mut config = EngineConfig::new(Root::of::<RangedProgress>());
    config.environment.os = Os::Web;
    config.viewport = Size::new(px(480.0), px(360.0));
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let width = |app: &Headless, id: &str| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
            .size
            .width
            .0
    };
    assert_eq!(
        width(&app, "progress-indicator"),
        width(&app, "progress-track")
    );
    app.set_environment("Enable reduced motion", |env| env.reduce_motion = true)
        .unwrap();
    assert!(
        (width(&app, "progress-indicator") / width(&app, "progress-track") - 0.25).abs() < 0.01
    );
    app.set_environment("Restore motion", |env| env.reduce_motion = false)
        .unwrap();
    assert_eq!(
        width(&app, "progress-indicator"),
        width(&app, "progress-track")
    );
}
