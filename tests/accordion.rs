use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::{
    App,
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
};
use nsh_testing::Headless;

#[component]
fn KeyboardExample() -> impl IntoElement {
    let entry = |id, disabled| {
        AccordionItem::new(
            id,
            AccordionTrigger::new(id),
            AccordionContent::new(text(format!("{id} details"))),
        )
        .disabled(disabled)
    };
    Accordion::new(&["First"])
        .child(entry("First", false))
        .child(entry("Unavailable", true))
        .child(entry("Last", false))
}

#[test]
fn keyboard_navigation_skips_disabled_items_and_activates_focused_trigger() {
    let mut config = EngineConfig::new(Root::of::<KeyboardExample>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    assert!(app.shows("First details"));
    app.press_key("tab").unwrap();
    app.press_key("down").unwrap();
    app.press_key("enter").unwrap();
    assert!(app.shows("Last details"));
    assert!(!app.shows("First details"));
    app.press_key("home").unwrap();
    app.press_key("space").unwrap();
    assert!(app.shows("First details"));
    assert!(!app.shows("Last details"));
    assert!(!app.shows("Unavailable details"));
}

#[test]
fn mobile_docs_link_to_accordion_and_back_to_drawer() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app.press("id:open-accordion-docs").unwrap();
    assert!(app.shows("What are your shipping options?"));
    app.press("role:region[Multiple] >> Privacy & Security")
        .unwrap();
    let notifications = app
        .screen()
        .accessibility_state("role:button[Notification Settings]")
        .unwrap();
    let privacy = app
        .screen()
        .accessibility_state("role:button[Privacy & Security]")
        .unwrap();
    assert_eq!(notifications.expanded, Some(true));
    assert_eq!(privacy.expanded, Some(true));
    app.press("role:region[Basic] >> TSX").unwrap();
    app.press("role:region[Basic] >> Copy").unwrap();
    assert_eq!(
        app.screen().clipboard().unwrap(),
        include_str!("../references/accordion-tsx/accordion-basic.tsx")
    );
    app.press("role:button[Drawer]").unwrap();
    assert!(app.shows("Drawer"));
    assert_eq!(app.screen().count("id:drawer-docs-scroll"), Ok(1));
}

#[test]
fn accordion_deep_link_and_history_restore_the_correct_page() {
    let config = EngineConfig::new(Root::of::<App>());
    let mut app = Headless::start_at_page(config, "https://foundation.test/#accordion").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:accordion-docs-scroll").unwrap();
    app.press("role:button[Drawer]").unwrap();
    assert_eq!(app.browser().url(), Some("https://foundation.test/#drawer"));
    app.browser().back().unwrap();
    assert_eq!(app.screen().count("id:accordion-docs-scroll"), Ok(1));
}
