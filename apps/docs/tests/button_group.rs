use docs::App;
use std::time::Duration;

use components::{
    button::Button,
    button_group::{ButtonGroup, ButtonGroupOrientation},
};
use nsh_core::{
    Size,
    element::{LayoutDirection, Root},
    platform::Os,
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn button_group_examples_compose_native_inputs_selects_and_popovers() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#button-group").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:button-group-docs-scroll").unwrap();
    for label in [
        "Button Group demo",
        "Orientation",
        "Size",
        "Nested",
        "Separator",
        "Split",
        "Input",
        "Input Group",
        "Dropdown Menu",
        "Select",
        "Popover",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{label}]")), Ok(1));
    }
    app.press("role:button[Increase volume]").unwrap();
    assert!(app.shows("Volume 6"));

    app.type_text("Search query", "Native Shell").unwrap();
    app.press("role:button[Search]").unwrap();
    assert!(app.shows("Searched for Native Shell"));
    app.press("role:button[Toggle voice mode]").unwrap();
    assert_eq!(app.screen().enabled("Voice or text message"), Ok(false));
    app.press("role:button[Toggle voice mode]").unwrap();
    assert_eq!(app.screen().enabled("Voice or text message"), Ok(true));

    app.press("Currency").unwrap();
    app.press("role:option[Euro]").unwrap();
    app.type_text("Amount", "12.50").unwrap();
    app.press("role:button[Send amount]").unwrap();
    assert!(app.shows("Preview amount: €12.50"));

    app.press("role:button[Open Popover]").unwrap();
    app.wait_for("Start a new task with Copilot").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("Start a new task with Copilot", Duration::from_secs(1))
        .unwrap();
}

#[component]
fn Groups() -> impl IntoElement {
    let count = signal(0);
    let button = |label: &'static str| {
        Button::new(label)
            .on_press(move || count.update(|n| *n += 1))
            .test_id(label)
    };
    view()
        .child(
            ButtonGroup::new("Horizontal")
                .child(button("first"))
                .child(button("second")),
        )
        .child(
            ButtonGroup::new("Vertical")
                .orientation(ButtonGroupOrientation::Vertical)
                .child(button("top"))
                .child(button("bottom")),
        )
        .child(
            ButtonGroup::new("RTL")
                .direction(LayoutDirection::Rtl)
                .child(button("right"))
                .child(button("left")),
        )
        .child(text(move || format!("Actions: {}", count.get())))
}

#[test]
fn orientations_preserve_layout_order_and_tab_activation() {
    let mut config = EngineConfig::new(Root::of::<Groups>());
    config.environment.os = Os::Linux;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let layout = app.screen().layout_nodes();
    let bounds = |id| {
        layout
            .iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
    };
    let first = bounds("first");
    let second = bounds("second");
    assert!((first.origin.x.0 + first.size.width.0 - second.origin.x.0).abs() < 1.0);
    assert!(bounds("top").origin.y.0 < bounds("bottom").origin.y.0);
    assert!(bounds("right").origin.x.0 > bounds("left").origin.x.0);
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    app.press_key("tab").unwrap();
    app.press_key("space").unwrap();
    assert!(app.shows("Actions: 2"));
}
