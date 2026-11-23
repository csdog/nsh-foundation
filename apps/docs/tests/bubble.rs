use docs::App;
use std::time::Duration;

use components::bubble::{Bubble, BubbleAlign, BubbleContent};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn bubble_examples_support_actions_disclosure_and_popovers() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#bubble").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:bubble-docs-scroll").unwrap();
    for name in [
        "Bubble demo",
        "Variants",
        "Alignment",
        "Bubble Group",
        "Links and Buttons",
        "Reactions",
        "Show More / Collapsible",
        "Tooltip",
        "Popover",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    app.press("role:button[I forgot my password]").unwrap();
    assert!(app.shows("Selected in this preview: I forgot my password"));
    app.press("role:button[Yes, run it]").unwrap();
    assert!(app.shows("Confirmed in this preview."));
    app.press("role:button[Show more]").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Show less]")
            .unwrap()
            .expanded,
        Some(true)
    );
    app.press("role:button[Show less]").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Show more]")
            .unwrap()
            .expanded,
        Some(false)
    );
    app.scroll("id:bubble-docs-scroll", px(10_000.0)).unwrap();
    app.press("role:button[Read receipt]").unwrap();
    app.wait_for("Read on Jan 5, 2026 at 4:32 PM").unwrap();
    app.press("role:button[Show error details]").unwrap();
    app.wait_for("Command failed with exit code 1").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("Command failed with exit code 1", Duration::from_secs(1))
        .unwrap();
}

#[component]
fn AlignmentExample() -> impl IntoElement {
    view()
        .w(px(310.0))
        .child(Bubble::new(BubbleContent::new(
            view().h(px(20.0)).w(px(80.0)).test_id("start"),
        )))
        .child(
            Bubble::new(BubbleContent::new(
                view().h(px(20.0)).w(px(80.0)).test_id("end"),
            ))
            .align(BubbleAlign::End),
        )
}

#[test]
fn the_same_width_bubble_aligns_to_either_edge() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<AlignmentExample>())).unwrap();
    app.first_render().unwrap();
    let layout = app.screen().layout_nodes();
    let position = |id| {
        layout
            .iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
            .origin
            .x
            .0
    };
    assert!(position("end") - position("start") > 150.0);
}
