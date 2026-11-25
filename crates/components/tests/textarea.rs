use components::textarea::invalid_textarea;
use nsh_core::{platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn MessageForm() -> impl IntoElement {
    let value = signal(String::new());
    view()
        .child(
            invalid_textarea(move || value.with(|text| text.trim().is_empty()))
                .label("Message")
                .test_id("message")
                .value(value)
                .on_change(move |text: &str| value.set(text.to_owned())),
        )
        .child(text(move || {
            format!("{} lines", value.with(|text| text.lines().count()))
        }))
}

#[test]
fn multiline_input_keeps_newlines_and_updates_validation() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<MessageForm>());
    config.environment.os = Os::Linux;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let state = |app: &Headless| app.screen().accessibility_state("id:message").unwrap();
    assert!(state(&app).invalid);
    app.type_text("id:message", "First line\nSecond line")
        .unwrap();
    app.wait_for("2 lines").unwrap();
    assert!(!state(&app).invalid);
    app.clear("id:message").unwrap();
    app.wait_for("0 lines").unwrap();
    assert!(state(&app).invalid);
}
