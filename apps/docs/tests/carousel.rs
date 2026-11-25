use docs::App;
use std::time::{Duration, UNIX_EPOCH};

use components::carousel::{
    Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious,
};
use nsh_core::{Size, element::Root, geometry::Axis, platform::Os, prelude::*};
use nsh_engine::{Clock, EngineConfig};
use nsh_testing::Headless;

#[test]
fn gallery_allows_next_previous_and_touch_scroll() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::MacOs;
    config.clock = Clock::manual(UNIX_EPOCH);
    let mut app = Headless::start_at_page(config, "https://foundation.test/#carousel").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:carousel-docs-scroll").unwrap();
    for name in [
        "Carousel demo",
        "Sizes",
        "Spacing",
        "Orientation",
        "API",
        "Autoplay",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let previous = "role:region[API] >> role:button[Previous slide]";
    let next = "role:region[API] >> role:button[Next slide]";
    assert_eq!(app.screen().enabled(previous), Ok(false));
    app.press(next).unwrap();
    app.wait_for("text:Slide 2 of 5").unwrap();
    app.scroll(
        "role:region[Controlled slides] >> role:group[Slide 2 of 5]",
        px(192.0),
    )
    .unwrap();
    app.wait_for("text:Slide 3 of 5").unwrap();
    app.press(previous).unwrap();
    app.wait_for("text:Slide 2 of 5").unwrap();
    for _ in 0..3 {
        app.press(next).unwrap();
    }
    app.wait_for("text:Slide 5 of 5").unwrap();
    assert_eq!(app.screen().enabled(next), Ok(false));

    let last = "role:region[Controlled slides] >> text:5";
    app.drag(last, point(px(10.0), px(0.0)), Default::default())
        .unwrap();
    app.wait_for("text:Slide 5 of 5").unwrap();
    app.drag(last, point(px(110.0), px(0.0)), Default::default())
        .unwrap();
    app.wait_for("text:Slide 4 of 5").unwrap();
}

#[component]
fn NestedCarousel() -> impl IntoElement {
    let index = signal(0);
    let offset = signal(0);
    let mut slides = CarouselContent::new().height(140.0);
    for n in 1..=4 {
        slides = slides.item(CarouselItem::new(text(format!("Card {n}"))).basis(0.5));
    }
    view()
        .child(text(move || {
            format!("Active {} / Page {}", index.get() + 1, offset.get())
        }))
        .child(
            scroll()
                .h(px(300.0))
                .w(px(300.0))
                .on_scroll(move |event: &ScrollEvent| offset.set(event.offset.y.0.round() as i32))
                .child(
                    Carousel::new("Nested slides")
                        .orientation(Axis::Vertical)
                        .index(index)
                        .content(slides),
                )
                .child(view().test_id("outside-track").h(px(100.0)))
                .child(view().h(px(500.0))),
        )
}

#[test]
fn vertical_wheel_stays_in_the_carousel_at_both_ends() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<NestedCarousel>());
    config.environment.os = Os::MacOs;
    config.clock = Clock::manual(UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app.wait_for("text:Active 1 / Page 0").unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    let wheel = |app: &mut Headless, slide, distance| {
        app.wheel(
            &format!("role:group[Slide {slide} of 4]"),
            point(px(0.0), px(distance)),
        )
        .unwrap();
        app.run_for(Duration::from_millis(400)).unwrap();
    };

    wheel(&mut app, 1, 70.0);
    app.wait_for("text:Active 2 / Page 0").unwrap();
    wheel(&mut app, 2, 70.0);
    assert!(app.shows("Active 3 / Page 0"));
    wheel(&mut app, 3, 70.0);
    assert!(app.shows("Active 3 / Page 0"));
    app.drag("text:Card 3", point(px(0.0), px(75.0)), Default::default())
        .unwrap();
    app.wait_for("text:Active 2 / Page 0").unwrap();

    app.wheel("id:outside-track", point(px(0.0), px(45.0)))
        .unwrap();
    app.run_for(Duration::from_millis(400)).unwrap();
    assert!(app.shows("Active 2 / Page 45"));
    wheel(&mut app, 3, -70.0);
    assert!(app.shows("Active 1 / Page 45"));
    // The first slide is partly clipped by the page. Slide 2 is fully visible.
    wheel(&mut app, 2, -70.0);
    assert!(app.shows("Active 1 / Page 45"));
}

#[test]
fn a_touch_drag_switches_the_vertical_carousel_on_mobile() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<NestedCarousel>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Ios;
    config.clock = Clock::manual(UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app.wait_for("text:Active 1 / Page 0").unwrap();

    app.drag("text:Card 2", point(px(0.0), px(-65.0)), Default::default())
        .unwrap();
    app.wait_for("text:Active 2 / Page 0").unwrap();
}

#[component]
fn AutoplayDemo() -> impl IntoElement {
    let index = signal(0_u32);
    let mut content = CarouselContent::new().height(140.0);
    for n in 1..=3 {
        content = content.item(CarouselItem::new(text(format!("Item {n}"))));
    }
    view()
        .child(
            view()
                .test_id("outside-carousel")
                .h(px(30.0))
                .child(text("Outside")),
        )
        .child(
            Carousel::new("Vertical autoplay")
                .orientation(Axis::Vertical)
                .autoplay(Duration::from_secs(2))
                .previous(CarouselPrevious::new())
                .content(content)
                .next(CarouselNext::new())
                .index(index)
                .surface()
                .test_id("autoplay-carousel")
                .w(px(300.0)),
        )
        .child(text(move || format!("Current {}", index.get() + 1)))
}

#[test]
fn autoplay_pauses_for_hover_and_keyboard_navigates_vertically() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<AutoplayDemo>());
    config.viewport = Size::new(px(900.0), px(700.0));
    config.environment.os = Os::MacOs;
    config.clock = Clock::manual(UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    app.wait_for("text:Current 1").unwrap();
    app.run_for(Duration::from_secs(2)).unwrap();
    app.wait_for("text:Current 2").unwrap();
    app.hover("id:autoplay-carousel").unwrap();
    app.run_for(Duration::from_secs(3)).unwrap();
    assert!(app.shows("Current 2"));
    app.hover("id:outside-carousel").unwrap();
    app.run_for(Duration::from_secs(2)).unwrap();
    app.wait_for("text:Current 3").unwrap();

    app.press_key("tab").unwrap();
    app.press_key("tab").unwrap();
    app.press_key("up").unwrap();
    app.wait_for("text:Current 2").unwrap();
    app.press_key("down").unwrap();
    app.wait_for("text:Current 3").unwrap();
}

#[component]
fn LocalRtl() -> impl IntoElement {
    use nsh_core::element::LayoutDirection;

    let current = signal(0);
    let content = CarouselContent::new()
        .item(CarouselItem::new(text("First")))
        .item(CarouselItem::new(text("Second")))
        .item(CarouselItem::new(text("Third")));
    view()
        .child(
            Carousel::new("RTL carousel")
                .direction(LayoutDirection::Rtl)
                .content(content)
                .previous(CarouselPrevious::new())
                .next(CarouselNext::new())
                .index(current)
                .surface()
                .w(px(300.0)),
        )
        .child(text(move || format!("RTL position {}", current.get() + 1)))
}

#[test]
fn locally_mirrored_carousel_starts_at_the_right_and_left_advances() {
    use nsh_core::{Command, MountItem};
    use std::sync::{Arc, Mutex};

    let mut config = EngineConfig::new(nsh_testing::windowed_root::<LocalRtl>());
    config.environment.os = Os::MacOs;
    config.environment.rtl = false;
    config.clock = Clock::manual(UNIX_EPOCH);
    let mut app = Headless::start(config).unwrap();
    let offsets = Arc::new(Mutex::new(Vec::new()));
    let observed = offsets.clone();
    app.observe(move |_, batch| {
        observed
            .lock()
            .unwrap()
            .extend(batch.items.iter().filter_map(|item| match item {
                MountItem::Command {
                    command: Command::ScrollTo { x, .. },
                    ..
                } => Some(x.0),
                _ => None,
            }));
    });
    app.first_render().unwrap();
    app.wait_for("text:RTL position 1").unwrap();
    assert_eq!(app.screen().enabled("Previous slide"), Ok(false));
    let start = *offsets.lock().unwrap().last().expect("initial scroll");
    assert!(start > 0.0);
    app.press_key("tab").unwrap();
    app.press_key("left").unwrap();
    app.wait_for("text:RTL position 2").unwrap();
    let next = *offsets.lock().unwrap().last().expect("step scroll");
    assert!(next < start);
    app.press("Next slide").unwrap();
    app.wait_for("text:RTL position 3").unwrap();
    assert_eq!(app.screen().enabled("Next slide"), Ok(false));
    app.run_for(Duration::from_millis(400)).unwrap();
    app.wheel("text:Third", point(px(70.0), px(0.0))).unwrap();
    app.wait_for("text:RTL position 2").unwrap();
}
