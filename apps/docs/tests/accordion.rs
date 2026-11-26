use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use docs::App;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
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
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<KeyboardExample>());
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
    app.press("id:docs-menu-trigger").unwrap();
    assert_eq!(app.screen().count("id:docs-menu-panel"), Ok(1));
    app.press("role:dialog[Menu] >> Accordion").unwrap();
    assert_eq!(app.screen().count("id:docs-menu-panel"), Ok(0));
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
    assert_eq!(
        app.screen()
            .count("role:region[Basic] >> role:tab[shadcn/ui]"),
        Ok(0)
    );
    app.press("id:docs-menu-trigger").unwrap();
    app.scroll("id:docs-menu-panel", px(940.0)).unwrap();
    app.press("role:dialog[Menu] >> Drawer").unwrap();
    assert!(app.shows("Drawer"));
    assert_eq!(app.screen().count("id:drawer-docs-scroll"), Ok(1));
}

#[test]
fn accordion_deep_link_and_history_restore_the_correct_page() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(1280.0), px(900.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#accordion").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:accordion-docs-scroll").unwrap();
    app.press("role:button[Components]").unwrap();
    assert_eq!(app.browser().url(), Some("https://foundation.test/#drawer"));
    app.browser().back().unwrap();
    assert_eq!(app.screen().count("id:accordion-docs-scroll"), Ok(1));
}

fn sidebar_row_y(app: &Headless, label: &str) -> f32 {
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| {
            node.bounds.origin.x.0 < 216.0
                && node
                    .text
                    .as_ref()
                    .is_some_and(|text| &*text.text.text == label)
        })
        .unwrap()
        .bounds
        .origin
        .y
        .0
}

#[test]
fn desktop_sidebar_keeps_its_offset_and_reveals_the_current_page() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(1280.0), px(900.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#typography").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:typography-docs-scroll").unwrap();
    assert!((58.0..900.0).contains(&sidebar_row_y(&app, "Typography")));
    let sidebar_id = |app: &Headless| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("docs-sidebar"))
            .unwrap()
            .id
    };
    let sidebar = sidebar_id(&app);

    app.scroll("id:docs-sidebar", px(-180.0)).unwrap();
    let switch = sidebar_row_y(&app, "Switch");
    assert!((58.0..900.0).contains(&switch));
    app.press("id:docs-sidebar >> role:button[Switch]").unwrap();
    app.wait_for("id:switch-docs-scroll").unwrap();
    assert_eq!(sidebar_id(&app), sidebar);
    assert!((sidebar_row_y(&app, "Switch") - switch).abs() < 1.0);
    let current = app
        .screen()
        .accessibility_state("id:docs-sidebar >> role:button[Switch]")
        .unwrap();
    assert!(current.selected);

    app.browser().back().unwrap();
    app.wait_for("id:typography-docs-scroll").unwrap();
    assert!((58.0..900.0).contains(&sidebar_row_y(&app, "Typography")));
    app.browser().forward().unwrap();
    app.wait_for("id:switch-docs-scroll").unwrap();
    assert_eq!(sidebar_id(&app), sidebar);
    assert!((58.0..900.0).contains(&sidebar_row_y(&app, "Switch")));
}
