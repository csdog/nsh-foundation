use std::time::Duration;

use docs::App;
use nsh_core::{Size, ViewKind, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn page(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::MacOs;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#command").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:command-docs-scroll").unwrap();
    app
}

const PALETTE: &str = "role:dialog[Command Palette]";
const SEARCH: &str = "role:dialog[Command Palette] >> label:Command search";

#[test]
fn command_searches_keywords_skips_disabled_items_and_selects_with_keyboard() {
    let mut app = page(390.0);
    for name in [
        "Command demo",
        "Dialog",
        "Basic",
        "Shortcuts",
        "Groups",
        "Scrollable",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let inline = "role:region[Command demo]";
    assert_eq!(
        app.screen()
            .enabled(&format!("{inline} >> role:option[Calculator]")),
        Ok(false)
    );
    let input = format!("{inline} >> label:Command search");
    let inputs = app
        .screen()
        .layout_nodes()
        .into_iter()
        .filter(|node| node.kind == ViewKind::Input && !node.hidden)
        .collect::<Vec<_>>();
    assert!(!inputs.is_empty());
    assert!(inputs.iter().all(|node| node.bounds.size.height.0 >= 24.0));
    app.type_text(&input, "smile").unwrap();
    app.wait_for(&format!("{inline} >> role:option[Search Emoji]"))
        .unwrap();
    assert_eq!(
        app.screen()
            .count(&format!("{inline} >> role:option[Calendar]")),
        Ok(0)
    );
    app.press_key("enter").unwrap();
    app.wait_for("Selected: emoji").unwrap();
}

#[test]
fn shortcut_opens_dialog_and_escape_clears_then_dismisses() {
    let mut app = page(1280.0);
    app.press_key("mod+j").unwrap();
    app.wait_for(PALETTE).unwrap();
    app.type_text(SEARCH, "not a command").unwrap();
    app.wait_for(&format!("{PALETTE} >> text:No results found."))
        .unwrap();
    app.press_key("escape").unwrap();
    app.wait_for(&format!("{PALETTE} >> role:option[Calendar]"))
        .unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone(PALETTE, Duration::from_secs(1)).unwrap();

    app.press_key("mod+j").unwrap();
    app.wait_for(PALETTE).unwrap();
    app.type_text(SEARCH, "billing").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for_gone(PALETTE, Duration::from_secs(1)).unwrap();
    app.wait_for("Selected: billing").unwrap();
    app.press_key("mod+j").unwrap();
    app.wait_for(PALETTE).unwrap();
    app.press_key("mod+j").unwrap();
    app.wait_for_gone(PALETTE, Duration::from_secs(1)).unwrap();
}

#[test]
fn offscreen_commands_are_searchable_and_rtl_uses_localized_labels() {
    let mut app = page(1280.0);
    app.press("role:region[Scrollable] >> Open Menu").unwrap();
    app.scroll(&format!("{PALETTE} >> role:listbox"), px(440.0))
        .unwrap();
    app.type_text(SEARCH, "code editor").unwrap();
    app.press(&format!("{PALETTE} >> role:option[Code Editor]"))
        .unwrap();
    app.wait_for("Selected: code-editor").unwrap();

    app.press("role:region[RTL] >> עברית").unwrap();
    app.type_text("role:region[RTL] >> label:Command search", "calendar")
        .unwrap();
    app.press("role:region[RTL] >> role:option[לוח שנה]")
        .unwrap();
    app.wait_for("Selected: calendar").unwrap();
}
