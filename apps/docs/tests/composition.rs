use docs::App;
use nsh_core::{element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[path = "../src/examples/composition/badge_rust.rs"]
mod badge_rust;
#[path = "../src/examples/composition/button_rust.rs"]
mod button_rust;
#[path = "../src/examples/composition/card_rust.rs"]
mod card_rust;
#[path = "../src/examples/composition/carousel_rust.rs"]
mod carousel_rust;

#[component]
fn RustExamples() -> impl IntoElement {
    view()
        .child(button_rust::example())
        .child(badge_rust::example())
        .child(card_rust::example())
        .child(carousel_rust::example())
}

#[test]
fn the_documented_rust_builders_render() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<RustExamples>())).unwrap();
    app.first_render().unwrap();
    app.press("role:button[Save]").unwrap();
    assert!(app.shows("Saved."));
    assert!(app.shows("In progress"));
    assert!(app.shows("Project settings"));
    assert_eq!(app.screen().count("role:region[Featured places]"), Ok(1));
}

#[test]
fn composition_tabs_copy_the_rsx_and_native_tsx_sources() {
    for (page, title, rsx) in [
        (
            "button",
            "Button",
            include_str!("../src/examples/composition/button_rsx.rs"),
        ),
        (
            "badge",
            "Badge",
            include_str!("../src/examples/composition/badge_rsx.rs"),
        ),
        (
            "card",
            "Card",
            include_str!("../src/examples/composition/card_rsx.rs"),
        ),
        (
            "carousel",
            "Carousel",
            include_str!("../src/examples/composition/carousel_rsx.rs"),
        ),
    ] {
        let mut app = Headless::start_at_page(
            EngineConfig::new(Root::of::<App>()),
            &format!("https://foundation.test/#{page}"),
        )
        .unwrap();
        app.first_render().unwrap();
        let region = format!("role:region[{title} composition]");
        if page == "button" {
            app.press(&format!("{region} >> role:button[Save]"))
                .unwrap();
            assert!(app.shows("Saved."));
        }
        let tsx = match page {
            "button" => Some(include_str!("../src/examples/composition/button.tsx")),
            "badge" => Some(include_str!("../src/examples/composition/badge.tsx")),
            _ => None,
        };
        if let Some(tsx) = tsx {
            app.press(&format!("{region} >> TSX")).unwrap();
            app.press(&format!("{region} >> Copy")).unwrap();
            assert_eq!(app.screen().clipboard().unwrap(), tsx);
        }
        app.press(&format!("{region} >> RSX")).unwrap();
        app.press(&format!("{region} >> Copy")).unwrap();
        assert_eq!(app.screen().clipboard().unwrap(), rsx);
    }
}
