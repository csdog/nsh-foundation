use std::time::Duration;

use components::{button::Button, tooltip::TooltipContent};
use nsh_core::{element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn TooltipFixture() -> impl IntoElement {
    view().child(
        TooltipContent::new("Save the draft")
            .delay(Duration::from_secs(10))
            .attach(Button::new("Save").surface()),
    )
}

#[test]
fn keyboard_focus_describes_the_trigger_and_escape_hides_the_tip() {
    let mut config = EngineConfig::new(Root::of::<TooltipFixture>());
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let button = "role:button[Save]";
    let tooltip = "id:tooltip-content";
    let timeout = Duration::from_secs(2);
    assert!(!app.shows("Save the draft"));
    app.press_key("tab").unwrap();
    app.wait_for_within("Save the draft", timeout).unwrap();
    assert_eq!(
        app.screen().accessibility_described_by(button).unwrap(),
        Some(app.screen().id(tooltip).unwrap())
    );

    app.press_key("escape").unwrap();
    app.wait_for_gone("Save the draft", timeout).unwrap();
    assert_eq!(
        app.screen().accessibility_described_by(button).unwrap(),
        None
    );
}
