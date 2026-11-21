use std::time::Duration;

use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

fn start(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(844.0));
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#alert-dialog").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:alert-dialog-docs-scroll").unwrap();
    app
}

fn panel_width(app: &Headless) -> f32 {
    app.screen()
        .layout_nodes()
        .iter()
        .find(|node| node.test_id.as_deref() == Some("alert-dialog-panel"))
        .unwrap()
        .bounds
        .size
        .width
        .0
}

#[test]
fn dialog_confirmation_runs_only_for_the_confirm_action() {
    let mut app = start(390.0);
    let demo = "role:region[Alert Dialog demo] >> role:button[Show Dialog]";
    app.press(demo).unwrap();
    assert_eq!(panel_width(&app), 320.0);
    app.press_key("enter").unwrap();
    app.wait_for_gone("id:alert-dialog-panel", Duration::from_secs(1))
        .unwrap();
    assert!(!app.shows("Confirmed in this preview."));
    app.press(demo).unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("id:alert-dialog-panel", Duration::from_secs(1))
        .unwrap();
    assert!(!app.shows("Confirmed in this preview."));
    app.press(demo).unwrap();
    app.press("role:button[Continue]").unwrap();
    app.wait_for_gone("id:alert-dialog-panel", Duration::from_secs(1))
        .unwrap();
    assert!(app.shows("Confirmed in this preview."));
}

#[test]
fn desktop_and_compact_dialogs_use_their_own_widths() {
    let mut app = start(1280.0);
    for example in [
        "Alert Dialog demo",
        "Basic",
        "Small",
        "Media",
        "Small with Media",
        "Destructive",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    app.press("role:region[Basic] >> role:button[Show Dialog]")
        .unwrap();
    assert_eq!(panel_width(&app), 448.0);
    app.press("role:button[Cancel]").unwrap();
    app.wait_for_gone("id:alert-dialog-panel", Duration::from_secs(1))
        .unwrap();
    app.press("role:region[Small] >> role:button[Show Dialog]")
        .unwrap();
    assert_eq!(panel_width(&app), 320.0);
}
