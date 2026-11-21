use nsh_core::{
    Size,
    element::{LayoutDirection, Root},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_foundation::{App, badge::Badge};
use nsh_testing::Headless;

#[test]
fn badge_gallery_exposes_all_examples_and_opens_the_link() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#badge").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:badge-docs-scroll").unwrap();
    for example in [
        "Badge demo",
        "Variants",
        "With Icon",
        "With Spinner",
        "Link",
        "Custom Colors",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    assert!(app.shows("إشارة مرجعية"));
    assert!(app.shows("Generating"));
    app.press("role:link[Open Link]").unwrap();
    assert_eq!(
        app.screen().opened_urls(),
        &["https://ui.shadcn.com/docs/components/base/badge#link"]
    );
}

#[component]
fn IconOrder() -> impl IntoElement {
    let badge = |start, end| {
        Badge::new("Status")
            .icon_start(view().w(px(8.0)).h(px(8.0)).test_id(start))
            .icon_end(view().w(px(8.0)).h(px(8.0)).test_id(end))
    };
    view()
        .child(
            view()
                .direction(LayoutDirection::Ltr)
                .child(badge("ltr-start", "ltr-end")),
        )
        .child(
            view()
                .direction(LayoutDirection::Rtl)
                .child(badge("rtl-start", "rtl-end")),
        )
}

#[test]
fn inline_icon_positions_follow_layout_direction() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<IconOrder>())).unwrap();
    app.first_render().unwrap();
    let layout = app.screen().layout_nodes();
    let x = |id| {
        layout
            .iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
            .origin
            .x
            .0
    };
    assert!(x("ltr-start") < x("ltr-end"));
    assert!(x("rtl-start") > x("rtl-end"));
}
