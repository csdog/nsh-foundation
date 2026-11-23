use components::{
    toggle::ToggleVariant,
    toggle_group::{ToggleGroup, ToggleGroupItem},
};
use nsh_core::{Checked, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn CallFilter() -> impl IntoElement {
    ToggleGroup::new("Call filter")
        .variant(ToggleVariant::Outline)
        .spacing(0.0)
        .default_value(["all", "missed"])
        .items([
            ToggleGroupItem::new("all", "All"),
            ToggleGroupItem::new("received", "Received").disabled(true),
            ToggleGroupItem::new("missed", "Missed"),
        ])
}

#[test]
fn arrows_skip_disabled_items_and_activation_replaces_the_single_selection() {
    let mut config = EngineConfig::new(Root::of::<CallFilter>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let state = |app: &Headless, label| {
        app.screen()
            .accessibility_state(&format!("role:button[{label}]"))
            .unwrap()
    };
    assert_eq!(state(&app, "All").pressed, Some(Checked::True));
    assert_eq!(state(&app, "Missed").pressed, Some(Checked::False));
    assert!(state(&app, "Received").disabled);
    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    assert_eq!(state(&app, "All").pressed, Some(Checked::True));
    app.press_key("space").unwrap();
    assert_eq!(state(&app, "All").pressed, Some(Checked::False));
    assert_eq!(state(&app, "Missed").pressed, Some(Checked::True));
}
