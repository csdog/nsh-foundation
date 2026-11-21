use nsh_core::{Bounds, Points, Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::{App, aspect_ratio::AspectRatio};
use nsh_testing::Headless;

fn bounds(app: &Headless, id: &str) -> Bounds<Points> {
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some(id))
        .unwrap()
        .bounds
}

#[test]
fn example_frames_keep_their_ratios_at_mobile_and_desktop_widths() {
    for width in [390.0, 1280.0] {
        let mut config = EngineConfig::new(Root::of::<App>());
        config.viewport = Size::new(px(width), px(844.0));
        let mut app =
            Headless::start_at_page(config, "https://foundation.test/#aspect-ratio").unwrap();
        app.first_render().unwrap();
        app.wait_for("id:aspect-ratio-docs-scroll").unwrap();
        for (id, ratio, max_width) in [
            ("aspect-demo", 16.0 / 9.0, 384.0),
            ("aspect-square", 1.0, 192.0),
            ("aspect-portrait", 9.0 / 16.0, 160.0),
            ("aspect-rtl", 16.0 / 9.0, 384.0),
        ] {
            let frame = bounds(&app, id);
            assert!(frame.size.width.0 > 0.0 && frame.size.width.0 <= max_width);
            assert!(
                (frame.size.height.0 - frame.size.width.0 / ratio).abs() < 1.0,
                "{id} at {width}: {frame:?}"
            );
        }
        assert!(app.shows("منظر طبيعي جميل"));
    }
}

#[component]
fn Resizable() -> impl IntoElement {
    let ratio = signal(2.0_f32);
    view()
        .w(px(200.0))
        .child(AspectRatio::new(ratio, text("Content")).test_id("frame"))
        .child(
            view()
                .on_press(move || ratio.set(f32::NAN))
                .child(text("Change ratio")),
        )
}

#[test]
fn changing_the_ratio_resizes_the_frame_and_invalid_ratios_fall_back() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Resizable>())).unwrap();
    app.first_render().unwrap();
    assert_eq!(bounds(&app, "frame").size.height.0, 100.0);
    app.press("Change ratio").unwrap();
    assert_eq!(bounds(&app, "frame").size.height.0, 200.0);
}
