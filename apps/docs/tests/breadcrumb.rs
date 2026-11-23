use std::time::Duration;

use docs::App;
use nsh_core::{AccessibilityCurrent, Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn breadcrumb_examples_expose_current_page_menu_and_links() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#breadcrumb").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:breadcrumb-docs-scroll").unwrap();

    for example in [
        "Breadcrumb demo",
        "Basic",
        "Custom separator",
        "Dropdown",
        "Collapsed",
        "Link component",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    let current = app
        .screen()
        .accessibility_state("role:region[Basic] >> role:text[Breadcrumb]")
        .unwrap();
    assert_eq!(current.current, Some(AccessibilityCurrent::Page));
    assert_eq!(
        app.screen()
            .count("role:region[Basic] >> role:link[Breadcrumb]"),
        Ok(0)
    );
    assert!(app.shows("مسار التنقل"));

    let trigger = "role:button[Toggle breadcrumb menu]";
    app.press(trigger).unwrap();
    app.press("role:menuitem[Themes]").unwrap();
    assert_eq!(
        app.screen().opened_urls(),
        &["https://ui.shadcn.com/themes"]
    );
    app.wait_for_gone("role:menuitem[Themes]", Duration::from_secs(1))
        .unwrap();
    app.press(trigger).unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("role:menuitem[Themes]", Duration::from_secs(1))
        .unwrap();
}
