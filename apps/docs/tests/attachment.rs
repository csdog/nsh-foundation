use docs::App;
use std::time::Duration;

use components::attachment::AttachmentGroup;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn actions_and_card_trigger_are_independently_operable() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let url = "https://foundation.test/#attachment";
    let mut app = Headless::start_at_page(config, url).unwrap();
    app.first_render().unwrap();
    app.wait_for("id:attachment-docs-scroll").unwrap();
    for example in [
        "Attachment demo",
        "Image",
        "States",
        "Sizes",
        "Group",
        "Trigger",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    app.press("role:button[Copy link]").unwrap();
    app.wait_for("Preview link copied.").unwrap();
    assert_eq!(app.screen().clipboard().as_deref(), Some(url));
    assert_eq!(app.screen().count("id:attachment-preview-dialog"), Ok(0));

    app.press("role:button[Preview research-summary.pdf]")
        .unwrap();
    app.wait_for("id:attachment-preview-dialog").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("id:attachment-preview-dialog", Duration::from_secs(1))
        .unwrap();
    app.press("role:button[Remove research-summary.pdf]")
        .unwrap();
    assert_eq!(
        app.screen()
            .count("role:button[Preview research-summary.pdf]"),
        Ok(0)
    );

    app.press("role:region[States] >> role:button[Cancel upload]")
        .unwrap();
    assert!(!app.shows("design-system.zip"));
    app.press("role:button[Retry upload]").unwrap();
    app.wait_for("Uploaded · Sample file").unwrap();
    assert!(!app.shows("Upload failed · File exceeds 25 MB"));
}

#[component]
fn ScrollExample() -> impl IntoElement {
    view().w(px(200.0)).child(
        AttachmentGroup::new("Files", 180.0)
            .child(view().test_id("first-file").child(text("First")))
            .child(view().test_id("second-file").child(text("Second")))
            .child(view().test_id("third-file").child(text("Third"))),
    )
}

#[test]
fn focused_group_scrolls_horizontally_with_arrow_keys() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<ScrollExample>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let x = |app: &Headless| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("second-file"))
            .unwrap()
            .bounds
            .origin
            .x
            .0
    };
    let initial = x(&app);
    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    assert!(x(&app) < initial);
    app.press_key("left").unwrap();
    assert_eq!(x(&app), initial);
}
