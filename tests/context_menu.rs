use std::time::Duration;

use nsh_core::{
    Checked, Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_foundation::{
    App,
    context_menu::{ContextMenu, ContextMenuItem},
};
use nsh_testing::Headless;

fn start(root: Root, width: f32) -> Headless {
    let mut config = EngineConfig::new(root);
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#context-menu").unwrap();
    app.first_render().unwrap();
    app
}

#[component]
fn KeyboardExample() -> impl IntoElement {
    let choice = signal("No selection");
    view()
        .child(
            ContextMenu::new(view().w(px(240.0)).h(px(100.0)).child(text("Document")))
                .child(ContextMenuItem::new("Forward").disabled(true))
                .child(ContextMenuItem::new("Back").on_select(move || choice.set("Went back"))),
        )
        .child(text(move || choice.get()))
}

#[test]
fn keyboard_opens_at_the_first_enabled_item_and_escape_restores_focus() {
    let mut app = start(Root::of::<KeyboardExample>(), 920.0);
    app.press_key("tab").unwrap();
    app.press_key("shift+f10").unwrap();
    app.wait_for("role:menuitem[Back]").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("role:menuitem[Back]", Duration::from_secs(2))
        .unwrap();
    app.press_key("shift+f10").unwrap();
    app.wait_for("role:menuitem[Back]").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Went back").unwrap();
}

#[test]
fn gallery_right_clicks_open_the_submenu_and_keep_the_selected_state() {
    let mut app = start(Root::of::<App>(), 1280.0);
    app.wait_for("id:context-menu-docs-scroll").unwrap();
    for name in [
        "Context Menu demo",
        "Basic",
        "Submenu",
        "Shortcuts",
        "Groups",
        "Icons",
        "Checkboxes",
        "Radio",
        "Destructive",
        "Sides",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }

    let trigger = "id:context-menu-demo";
    app.right_click(trigger).unwrap();
    app.wait_for("role:menuitem[More Tools]").unwrap();
    assert_eq!(app.screen().enabled("role:menuitem[Forward]"), Ok(false));
    app.press("role:menuitem[More Tools]").unwrap();
    app.wait_for("role:menuitem[Save Page...]").unwrap();
    app.press("role:menuitem[Save Page...]").unwrap();
    app.wait_for("Selected in this preview: Save Page...")
        .unwrap();

    app.right_click(trigger).unwrap();
    let urls = "role:menuitemcheckbox[Show Full URLs]";
    assert_eq!(
        app.screen().accessibility_state(urls).unwrap().checked,
        Some(Checked::False)
    );
    app.press(urls).unwrap();
    app.right_click(trigger).unwrap();
    assert_eq!(
        app.screen().accessibility_state(urls).unwrap().checked,
        Some(Checked::True)
    );
    let person = "role:menuitemradio[Colm Tuite]";
    app.press(person).unwrap();
    app.right_click(trigger).unwrap();
    assert_eq!(
        app.screen().accessibility_state(person).unwrap().checked,
        Some(Checked::True)
    );
}
