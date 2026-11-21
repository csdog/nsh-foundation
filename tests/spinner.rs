use std::time::Duration;

use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

#[test]
fn standalone_spinner_has_an_accessible_name_and_cancel_removes_the_empty_indicator() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#spinner").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:spinner-docs-scroll").unwrap();

    assert_eq!(
        app.screen()
            .count("role:region[Spinner demo] >> role:progressbar[Loading]"),
        Ok(1)
    );
    assert_eq!(
        app.screen()
            .count("role:region[Button] >> role:progressbar"),
        Ok(0)
    );

    let empty = "role:region[Empty] >> id:spinner";
    assert_eq!(app.screen().count(empty), Ok(1));
    app.press("role:region[Empty] >> role:button[Cancel]")
        .unwrap();
    app.wait_for_gone(empty, Duration::from_secs(1)).unwrap();
    assert!(app.shows("Request canceled"));
}
