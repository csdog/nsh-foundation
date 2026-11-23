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

#[test]
fn sides_attach_to_the_window_and_can_close_with_reduced_motion() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Phone;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#sheet").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:sheet-docs-scroll").unwrap();

    for side in ["Top", "Right", "Bottom", "Left"] {
        let trigger = format!("role:region[Side] >> role:button[{side}]");
        app.press(&trigger).unwrap();
        app.wait_for("id:sheet-panel").unwrap();
        let panel = app
            .screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("sheet-panel"))
            .unwrap()
            .bounds;
        let vertical = matches!(side, "Top" | "Bottom");
        if vertical {
            assert!((panel.size.width.0 - 390.0).abs() < 1.0, "{panel:?}");
            assert!(panel.size.height.0 <= 422.0, "{panel:?}");
        } else {
            assert!((panel.size.width.0 - 292.5).abs() < 1.0, "{panel:?}");
            assert!((panel.size.height.0 - 844.0).abs() < 1.0, "{panel:?}");
        }
        let edge = match side {
            "Top" => panel.origin.y.0,
            "Right" => 390.0 - panel.origin.x.0 - panel.size.width.0,
            "Bottom" => 844.0 - panel.origin.y.0 - panel.size.height.0,
            "Left" => panel.origin.x.0,
            _ => unreachable!(),
        };
        assert!(edge.abs() < 1.0, "{side}: {panel:?}");
        app.press_key("escape").unwrap();
        app.wait_for_gone("id:sheet-panel", Duration::from_secs(1))
            .unwrap();
    }
}
