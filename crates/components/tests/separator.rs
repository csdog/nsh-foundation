use components::separator::{Separator, SeparatorOrientation};
use nsh_core::{Size, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn Lines() -> impl IntoElement {
    view()
        .w(px(160.0))
        .gap_2()
        .child(
            Separator::new()
                .label("New section")
                .surface()
                .test_id("rule"),
        )
        .child(
            view()
                .h(px(24.0))
                .flex_row()
                .items_center()
                .child(text("First"))
                .child(
                    Separator::new()
                        .orientation(SeparatorOrientation::Vertical)
                        .surface()
                        .test_id("divider"),
                )
                .child(text("Second")),
        )
}

#[test]
fn named_and_decorative_lines_stretch_on_their_own_axes() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<Lines>());
    config.viewport = Size::new(px(400.0), px(220.0));
    config.environment.os = Os::Linux;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    assert_eq!(app.screen().count("role:text[New section]"), Ok(1));
    let size = |id| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
            .size
    };
    assert_eq!(size("rule"), Size::new(px(160.0), px(1.0)));
    assert_eq!(size("divider"), Size::new(px(1.0), px(24.0)));
}
