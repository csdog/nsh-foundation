use components::toggle::Toggle;
use nsh_core::{Checked, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn Formatting() -> impl IntoElement {
    let shared = signal(false);
    view()
        .child(Toggle::new("Local").default_pressed(true))
        .child(
            Toggle::new("Shared")
                .pressed(shared)
                .on_change(move |next| shared.set(next)),
        )
        .child(Toggle::new("Disabled").pressed(shared).disabled(true))
}

#[test]
fn keyboard_toggles_local_and_bound_state_and_skips_the_disabled_control() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<Formatting>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let state = |app: &Headless, label| {
        app.screen()
            .accessibility_state(&format!("role:button[{label}]"))
            .unwrap()
    };
    assert_eq!(state(&app, "Local").pressed, Some(Checked::True));
    assert_eq!(state(&app, "Shared").pressed, Some(Checked::False));

    app.press_key("tab").unwrap();
    app.press_key("space").unwrap();
    assert_eq!(state(&app, "Local").pressed, Some(Checked::False));
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    assert_eq!(state(&app, "Shared").pressed, Some(Checked::True));
    assert_eq!(state(&app, "Disabled").pressed, Some(Checked::True));
    assert!(state(&app, "Disabled").disabled);
}
