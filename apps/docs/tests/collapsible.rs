use docs::App;
use std::time::Duration;

use components::{
    button::Button,
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn gallery(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#collapsible").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:collapsible-docs-scroll").unwrap();
    app
}

fn expanded(app: &Headless, name: &str) -> Option<bool> {
    app.screen().accessibility_state(name).unwrap().expanded
}

#[test]
fn order_and_settings_expand_in_the_mobile_gallery() {
    let mut app = gallery(390.0);
    for name in [
        "Collapsible demo",
        "Basic",
        "Settings Panel",
        "File Tree",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let trigger = "role:region[Collapsible demo] >> role:button[Toggle details]";
    assert_eq!(expanded(&app, trigger), Some(false));
    assert!(!app.shows("Shipping address"));
    app.press(trigger).unwrap();
    assert_eq!(expanded(&app, trigger), Some(true));
    app.wait_for("Shipping address").unwrap();
    assert!(
        app.screen()
            .accessibility_controls(trigger)
            .unwrap()
            .is_some()
    );
    app.press(trigger).unwrap();
    app.wait_for_gone("Shipping address", Duration::from_secs(1))
        .unwrap();

    let settings = "role:button[More corner radii]";
    app.press(settings).unwrap();
    assert_eq!(expanded(&app, settings), Some(true));
    app.type_text("Bottom left radius", "12").unwrap();
    app.press(settings).unwrap();
    assert_eq!(expanded(&app, settings), Some(false));

    app.press("role:region[RTL] >> עברית").unwrap();
    app.press("role:region[RTL] >> role:button[הצג פרטים]")
        .unwrap();
    app.wait_for("כתובת משלוח").unwrap();
}

#[test]
fn file_tree_preserves_nested_open_state_and_selection() {
    let mut app = gallery(1280.0);
    let root = "role:region[File Tree] >> role:button[components]";
    let folder = "role:region[File Tree] >> role:button[ui]";
    let file = "role:region[File Tree] >> role:button[button.tsx]";
    assert_eq!(expanded(&app, root), Some(false));
    app.press(root).unwrap();
    app.press(folder).unwrap();
    app.press(file).unwrap();
    assert_eq!(app.screen().selected(file), Ok(true));
    app.press(root).unwrap();
    app.wait_for_gone(file, Duration::from_secs(1)).unwrap();
    app.press(root).unwrap();
    assert_eq!(expanded(&app, folder), Some(true));
    assert_eq!(app.screen().selected(file), Ok(true));
}

#[component]
fn KeyboardDisclosure() -> impl IntoElement {
    view()
        .gap_4()
        .child(Collapsible::new(
            CollapsibleTrigger::new("Details"),
            CollapsibleContent::new(text("Additional details")),
        ))
        .child(
            Collapsible::new(
                CollapsibleTrigger::new("Unavailable"),
                CollapsibleContent::new(Button::new("Hidden action")),
            )
            .disabled(true),
        )
}

#[test]
fn keyboard_toggles_the_disclosure_and_a_disabled_trigger_cannot_open() {
    let mut config = EngineConfig::new(Root::of::<KeyboardDisclosure>());
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    assert_eq!(app.screen().enabled("Unavailable"), Ok(false));
    app.press_key("tab").unwrap();
    app.press_key("space").unwrap();
    assert_eq!(expanded(&app, "Details"), Some(true));
    app.wait_for("Additional details").unwrap();
    app.press_key("enter").unwrap();
    assert_eq!(expanded(&app, "Details"), Some(false));
    app.wait_for_gone("Additional details", Duration::from_secs(1))
        .unwrap();
}
