use components::combobox::{Combobox, ComboboxItem};
use docs::App;
use nsh_core::{
    Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn config(root: Root, width: f32) -> EngineConfig {
    let mut config = EngineConfig::new(root);
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::MacOs;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    config
}

fn page(width: f32) -> Headless {
    let mut app = Headless::start_at_page(
        config(Root::of::<App>(), width),
        "https://foundation.test/#combobox",
    )
    .unwrap();
    app.first_render().unwrap();
    app.wait_for("id:combobox-docs-scroll").unwrap();
    app
}

#[component]
fn KeyboardChoices() -> impl IntoElement {
    let value = signal(Vec::new());
    let choices = [
        ComboboxItem::new("ruby", "Ruby").disabled(true),
        ComboboxItem::new("rust", "Rust"),
        ComboboxItem::new("typescript", "TypeScript"),
    ];
    view()
        .child(
            Combobox::new("Language", choices)
                .value(value)
                .show_clear(true),
        )
        .child(text(move || {
            value
                .get()
                .first()
                .map_or(SharedString::from("Nothing selected"), |v| v.clone())
        }))
}

#[test]
fn typing_highlights_the_first_enabled_match_and_clear_resets_the_value() {
    let mut app = Headless::start(config(Root::of::<KeyboardChoices>(), 390.0)).unwrap();
    app.first_render().unwrap();
    app.type_text("role:combobox[Language]", "ru").unwrap();
    app.wait_for("role:option[Rust]").unwrap();
    assert_eq!(app.screen().enabled("role:option[Ruby]"), Ok(false));
    app.press_key("enter").unwrap();
    app.wait_for("text:rust").unwrap();
    app.press("role:button[Clear selection]").unwrap();
    app.wait_for("text:Nothing selected").unwrap();
    app.type_text("role:combobox[Language]", "Zebra").unwrap();
    app.wait_for("No items found.").unwrap();
    app.press_key("escape").unwrap();
}

#[test]
fn gallery_supports_multiple_chips_and_searchable_popup() {
    let mut app = page(1280.0);
    for name in [
        "Combobox demo",
        "Basic",
        "Multiple",
        "Clear Button",
        "Groups",
        "Custom Items",
        "Invalid",
        "Disabled",
        "Auto Highlight",
        "Popup",
        "Input Group",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    assert!(
        app.screen()
            .accessibility_state("id:invalid-combobox")
            .unwrap()
            .invalid
    );
    assert_eq!(
        app.screen()
            .enabled("role:region[Disabled] >> role:combobox[Framework]"),
        Ok(false)
    );

    let multiple = "role:region[Multiple]";
    app.type_text(&format!("{multiple} >> role:combobox[Framework]"), "rem")
        .unwrap();
    app.press_key("enter").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for(&format!("{multiple} >> role:button[Remove Remix]"))
        .unwrap();
    app.press(&format!("{multiple} >> role:button[Remove Next.js]"))
        .unwrap();
    assert_eq!(
        app.screen()
            .count(&format!("{multiple} >> role:button[Remove Next.js]")),
        Ok(0)
    );

    app.press("role:region[Popup] >> label:Select country")
        .unwrap();
    app.type_text("label:Search options", "japan").unwrap();
    app.press("role:option[Japan]").unwrap();
    app.wait_for("role:region[Popup] >> text:Japan").unwrap();
}

#[test]
fn groups_are_filtered_and_the_rtl_selection_survives_locale_changes() {
    let mut app = page(390.0);
    app.type_text("role:region[Groups] >> role:combobox[Timezone]", "tokyo")
        .unwrap();
    app.wait_for("role:option[(GMT+9) Tokyo]").unwrap();
    assert_eq!(app.screen().count("role:option[(GMT-5) New York]"), Ok(0));
    app.press_key("escape").unwrap();

    let rtl = "role:region[RTL]";
    app.wait_for(&format!("{rtl} >> role:button[Remove التكنولوجيا]"))
        .unwrap();
    app.press(&format!("{rtl} >> עברית")).unwrap();
    app.wait_for(&format!("{rtl} >> role:button[Remove טכנולוגיה]"))
        .unwrap();
    app.press(&format!("{rtl} >> English")).unwrap();
    app.wait_for(&format!("{rtl} >> role:button[Remove Technology]"))
        .unwrap();
}
