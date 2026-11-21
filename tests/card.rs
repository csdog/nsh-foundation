use nsh_core::{Bounds, Points, Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

fn start(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#card").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:card-docs-scroll").unwrap();
    app
}

fn bounds(app: &Headless, id: &str) -> Bounds<Points> {
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("missing layout for {id}"))
        .bounds
}

fn near(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 1.0, "{actual} vs {expected}");
}

#[test]
fn spacing_updates_in_place_and_the_login_form_validates() {
    let mut app = start(390.0);
    for name in [
        "Card demo",
        "Small",
        "Spacing",
        "Edge to Edge",
        "Image",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let demo = "role:region[Card demo]";
    app.press(&format!("{demo} >> role:button[Login]")).unwrap();
    assert!(app.shows("Enter a valid email and password."));
    app.type_text(&format!("{demo} >> label:Email"), "person@example.com")
        .unwrap();
    app.type_text(&format!("{demo} >> label:Password"), "example password")
        .unwrap();
    app.press(&format!("{demo} >> role:button[Login]")).unwrap();
    assert!(app.shows("Login form submitted."));

    let body_x = |app: &Headless| bounds(app, "card-spacing-body").origin.x.0;
    near(body_x(&app) - bounds(&app, "card-spacing").origin.x.0, 16.0);
    app.press("role:region[Spacing] >> 32px").unwrap();
    near(body_x(&app) - bounds(&app, "card-spacing").origin.x.0, 32.0);
    assert!(
        app.screen()
            .selected("role:region[Spacing] >> 32px")
            .unwrap()
    );
}

#[test]
fn the_cover_and_terms_are_full_width_and_the_header_action_mirrors() {
    let mut app = start(1280.0);
    near(
        bounds(&app, "card-small-body").origin.x.0 - bounds(&app, "card-small").origin.x.0,
        16.0,
    );
    let card = bounds(&app, "card-edge");
    let terms = bounds(&app, "card-terms");
    near(terms.origin.x.0, card.origin.x.0);
    near(terms.size.width.0, card.size.width.0);
    near(
        terms.bottom().0,
        bounds(&app, "card-terms-actions").origin.y.0,
    );
    app.press("role:region[Edge to Edge] >> Accept").unwrap();
    assert!(app.shows("Terms accepted."));

    let card = bounds(&app, "card-image");
    let cover = bounds(&app, "card-event-cover");
    near(cover.origin.y.0, card.origin.y.0);
    near(cover.size.width.0, card.size.width.0);
    near(cover.size.height.0, cover.size.width.0 * 9.0 / 16.0);

    let action_x = |app: &Headless| bounds(app, "card-rtl-sign-up").origin.x.0;
    let arabic_x = action_x(&app);
    near(arabic_x - bounds(&app, "card-rtl").origin.x.0, 20.0);
    app.press("role:region[RTL] >> English").unwrap();
    assert!(action_x(&app) > arabic_x + 100.0);
}
