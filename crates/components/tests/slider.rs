use components::{direction::Direction, slider::Slider};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn SliderFixture() -> impl IntoElement {
    let values = signal([10.0, 20.0, 70.0]);
    let commits = signal(0);
    view()
        .w(px(320.0))
        .gap_4()
        .child(
            Slider::new([10.0, 20.0, 70.0])
                .step(10.0)
                .value(values)
                .labels(["Start", "Middle", "End"])
                .on_value_commit(move |_| commits.update(|count| *count += 1)),
        )
        .child(text(move || {
            let [a, b, c] = values.get();
            format!("{a}, {b}, {c}")
        }))
        .child(text(move || format!("{} commits", commits.get())))
}

#[test]
fn drag_pushes_neighboring_thumbs_and_commits_once_on_release() {
    let mut cfg = EngineConfig::new(Root::of::<SliderFixture>());
    cfg.viewport = Size::new(px(380.0), px(240.0));
    cfg.environment.os = Os::Web;
    let mut app = Headless::start(cfg).unwrap();
    app.first_render().unwrap();
    app.drag("label:Start", point(px(216.0), px(0.0))).unwrap();
    app.wait_for("80, 80, 80").unwrap();
    assert!(app.shows("1 commits"));
    app.drag("label:End", point(px(60.0), px(0.0))).unwrap();
    app.wait_for("80, 80, 100").unwrap();
    assert!(app.shows("2 commits"));
}

#[component]
fn DecimalSlider() -> impl IntoElement {
    let value = signal([0.3]);
    view()
        .w(px(320.0))
        .child(
            Slider::new([0.3])
                .value(value)
                .range(0.0, 1.0)
                .step(0.1)
                .direction(Direction::Rtl),
        )
        .child(text(move || format!("Value {}", value.get()[0])))
}

#[test]
fn keyboard_mirrors_in_rtl_and_updates_the_bound_decimal_value() {
    let mut cfg = EngineConfig::new(Root::of::<DecimalSlider>());
    cfg.viewport = Size::new(px(380.0), px(240.0));
    cfg.environment.os = Os::Linux;
    let mut app = Headless::start(cfg).unwrap();
    app.first_render().unwrap();

    app.press_key("tab").unwrap();
    app.press_key("left").unwrap();
    app.wait_for("Value 0.4").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Value 0.3").unwrap();
    app.press_key("home").unwrap();
    app.wait_for("Value 0").unwrap();
}
