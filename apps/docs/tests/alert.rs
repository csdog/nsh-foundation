use docs::App;
use nsh_core::{Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn alert_docs_expose_all_examples_and_the_action_updates_its_message() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#alert").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:alert-docs-scroll").unwrap();

    for example in [
        "Alert demo",
        "Basic",
        "Destructive",
        "Action",
        "Custom Colors",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    assert!(app.shows("تم الدفع بنجاح"));
    assert_eq!(
        app.screen().count("role:region[Destructive] >> role:alert"),
        Ok(1)
    );

    app.press("role:region[Action] >> role:button[Enable]")
        .unwrap();
    assert!(app.shows("Preference saved for this example."));
    app.press("role:region[Action] >> shadcn/ui").unwrap();
    app.press("role:region[Action] >> Copy").unwrap();
    assert_eq!(
        app.screen().clipboard().unwrap(),
        include_str!("../../../references/alert-tsx/alert-action.tsx")
    );
    app.press("id:docs-menu-trigger").unwrap();
    app.press("role:dialog[Menu] >> Accordion").unwrap();
    assert_eq!(
        app.browser().url(),
        Some("https://foundation.test/#accordion")
    );
    app.browser().back().unwrap();
    assert_eq!(app.screen().count("id:alert-docs-scroll"), Ok(1));
}
