use std::time::Duration;

use docs::App;
use nsh_core::{
    Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn start(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::Web;
    config.environment.idiom = if width < 768.0 {
        Idiom::Phone
    } else {
        Idiom::Desktop
    };
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#sidebar").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:sidebar-docs-scroll").unwrap();
    app
}

#[test]
fn desktop_collapses_to_icons_and_preserves_the_open_submenu() {
    let mut app = start(1280.0);
    let trigger = "role:region[Sidebar demo] >> role:button[Toggle Sidebar]";
    let history = "role:region[Sidebar demo] >> role:button[History]";
    let widths = |app: &Headless| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .filter(|node| node.test_id.as_deref() == Some("sidebar-gap"))
            .map(|node| node.bounds.size.width.0)
            .collect::<Vec<_>>()
    };

    assert_eq!(app.screen().count(history), Ok(1));
    assert!(widths(&app).iter().all(|width| *width == 256.0));
    app.press(trigger).unwrap();
    app.wait_for_gone(history, Duration::from_secs(1)).unwrap();
    assert!(widths(&app).contains(&48.0));
    app.press(trigger).unwrap();
    app.wait_for(history).unwrap();
    assert!(!widths(&app).contains(&48.0));

    let rtl_trigger = "role:region[RTL] >> role:button[تبديل الشريط الجانبي]";
    app.press(rtl_trigger).unwrap();
    assert!(widths(&app).contains(&48.0));
    app.press(rtl_trigger).unwrap();
    assert!(!widths(&app).contains(&48.0));
}

#[test]
fn mobile_navigation_uses_the_sheet_and_closes_after_selection() {
    let mut app = start(390.0);
    let trigger = "role:region[Sidebar demo] >> role:button[Toggle Sidebar]";
    assert_eq!(app.screen().count("id:sidebar-panel"), Ok(0));
    app.press(trigger).unwrap();
    app.wait_for("role:dialog[Workspace sidebar]").unwrap();
    assert_eq!(app.screen().count("role:button[History]"), Ok(1));
    app.press("role:button[History]").unwrap();
    app.wait_for_gone("id:sheet-panel", Duration::from_secs(1))
        .unwrap();
    assert!(app.shows("History"));
}
