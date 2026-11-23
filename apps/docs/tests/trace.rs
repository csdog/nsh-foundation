use std::time::Duration;

use docs::App;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn app() -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    config.viewport = Size::new(px(390.0), px(844.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#trace").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:trace-docs-scroll").unwrap();
    app
}

#[test]
fn controlled_disclosure_synchronizes_the_trigger_and_external_button() {
    let mut app = app();
    let link = "role:region[Open State] >> role:button[Ran pnpm --version]";
    let controls = |app: &Headless| app.screen().accessibility_state(link).unwrap().expanded;
    assert_eq!(controls(&app), Some(false));
    app.press("role:region[Open State] >> role:button[Show the pnpm output]")
        .unwrap();
    app.wait_for("10.33.4").unwrap();
    assert_eq!(controls(&app), Some(true));
    app.press(link).unwrap();
    app.wait_for_gone("10.33.4", Duration::from_secs(2))
        .unwrap();
    assert_eq!(controls(&app), Some(false));
}

#[test]
fn tool_replay_advances_from_running_to_results_and_resets() {
    let mut app = app();
    let tool = "role:region[Tool]";
    app.press(&format!("{tool} >> role:button[Send message]"))
        .unwrap();
    app.wait_for("Running node --version").unwrap();
    app.wait_for_within(
        &format!("{tool} >> role:button[Failed to run pnpm --version]"),
        Duration::from_secs(6),
    )
    .unwrap();
    app.press(&format!(
        "{tool} >> role:button[Failed to run pnpm --version]"
    ))
    .unwrap();
    app.wait_for(&format!("{tool} >> text:command not found: pnpm"))
        .unwrap();
    app.press(&format!("{tool} >> role:button[Reset]")).unwrap();
    app.wait_for(&format!("{tool} >> role:button[Send message]"))
        .unwrap();
    assert_eq!(
        app.screen().count(&format!(
            "{tool} >> role:button[Failed to run pnpm --version]"
        )),
        Ok(0)
    );
}
