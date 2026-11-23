use std::time::Duration;

use nsh_core::{
    Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

use docs::App;

fn start(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(844.0));
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#dialog").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:dialog-docs-scroll").unwrap();
    app
}

fn wait_closed(app: &mut Headless) {
    app.wait_for_gone("id:dialog-panel", Duration::from_secs(2))
        .unwrap();
}

#[test]
fn profile_changes_are_saved_on_submit_and_drafts_are_discarded_on_cancel() {
    let mut app = start(1280.0);
    let trigger = "role:region[Dialog demo] >> role:button[Open Dialog]";
    app.press(trigger).unwrap();
    app.clear("label:Name").unwrap();
    app.type_text("label:Name", "Jane Doe").unwrap();
    app.press("role:button[Save changes]").unwrap();
    wait_closed(&mut app);
    assert!(app.shows("Save changes · Jane Doe"));

    app.press(trigger).unwrap();
    app.clear("label:Name").unwrap();
    app.type_text("label:Name", "Discard me").unwrap();
    app.press("role:button[Cancel]").unwrap();
    wait_closed(&mut app);
    app.press(trigger).unwrap();
    app.submit("label:Name").unwrap();
    wait_closed(&mut app);
    assert!(app.shows("Save changes · Jane Doe"));
}

#[test]
fn hiding_the_corner_leaves_escape_and_outside_press_available() {
    let mut app = start(390.0);
    let trigger = "role:region[No Close Button] >> role:button[No Close Button]";
    app.press(trigger).unwrap();
    assert_eq!(app.screen().count("id:dialog-corner-close"), Ok(0));
    let panel = app
        .screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some("dialog-panel"))
        .unwrap();
    assert_eq!(panel.bounds.size.width.0, 358.0);
    app.press_key("escape").unwrap();
    wait_closed(&mut app);
    app.press(trigger).unwrap();
    app.press_at("id:dialog-overlay", point(px(6.0), px(120.0)))
        .unwrap();
    wait_closed(&mut app);
}
