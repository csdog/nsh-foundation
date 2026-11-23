use components::{
    direction::{Direction, DirectionProvider},
    scroll_area::{ScrollArea, ScrollAreaOrientation, ScrollBar},
};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn RightToLeftStrip() -> impl IntoElement {
    DirectionProvider::new(Direction::Rtl, || {
        let pane = |id| view().w(px(200.0)).h(px(80.0)).flex_shrink_0().test_id(id);
        ScrollArea::new(
            "Strip",
            view()
                .w(px(400.0))
                .flex_shrink_0()
                .flex_row()
                .child(pane("first"))
                .child(pane("last")),
        )
        .width(102.0)
        .height(82.0)
        .scroll_bar(ScrollBar::new(ScrollAreaOrientation::Horizontal))
    })
}

#[test]
fn horizontal_rtl_home_and_end_move_to_the_logical_boundaries() {
    let mut config = EngineConfig::new(Root::of::<RightToLeftStrip>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    config.viewport = Size::new(px(420.0), px(220.0));
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let left = |app: &Headless| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("first"))
            .unwrap()
            .bounds
            .origin
            .x
            .0
    };
    app.press_key("tab").unwrap();
    app.press_key("home").unwrap();
    let at_start = left(&app);
    app.press_key("end").unwrap();
    let at_end = left(&app);
    assert!((at_end - at_start - 300.0).abs() < 1.0);
    app.press_key("home").unwrap();
    assert!((left(&app) - at_start).abs() < 1.0);
}
