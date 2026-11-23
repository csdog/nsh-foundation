use components::resizable::{
    ResizableHandle, ResizableOrientation, ResizablePanel, ResizablePanelGroup,
};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn ConstrainedSplit() -> impl IntoElement {
    let layout = signal(vec![40.0, 60.0]);
    let changed = signal(0_u32);
    view()
        .w(px(402.0))
        .h(px(180.0))
        .child(
            ResizablePanelGroup::new(ResizableOrientation::Horizontal)
                .layout(layout)
                .on_resize(move |_| changed.update(|count| *count += 1))
                .panel(ResizablePanel::new(text("First")).size_range(20.0, 60.0))
                .handle(ResizableHandle::new().label("Split"))
                .panel(ResizablePanel::new(text("Second"))),
        )
        .child(text(move || {
            format!("Layout {:.0}, changes {}", layout.get()[0], changed.get())
        }))
}

#[test]
fn keyboard_respects_panel_limits_and_only_reports_changed_layouts() {
    let mut config = EngineConfig::new(Root::of::<ConstrainedSplit>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    config.viewport = Size::new(px(500.0), px(320.0));
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    app.press_key("tab").unwrap();
    app.press_key("end").unwrap();
    app.wait_for("text:Layout 60, changes 1").unwrap();
    app.press_key("right").unwrap();
    assert!(app.shows("Layout 60, changes 1"));
    app.press_key("home").unwrap();
    app.wait_for("text:Layout 20, changes 2").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("text:Layout 21, changes 3").unwrap();
}
