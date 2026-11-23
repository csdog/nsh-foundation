use docs::App;
use std::time::Duration;

use components::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage, AvatarSize};
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn FailedImage() -> impl IntoElement {
    Avatar::new("Casey Nguyen")
        .image(AvatarImage::new("https://foundation.test/headshot.png"))
        .fallback(AvatarFallback::new("CN"))
        .badge(AvatarBadge::new("Online"))
        .size(AvatarSize::Large)
}

#[test]
fn a_failed_image_keeps_the_initials_and_status_label() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<FailedImage>())).unwrap();
    app.first_render().unwrap();
    app.fail_images("https://foundation.test/headshot.png")
        .unwrap();
    assert!(app.shows("CN"));
    assert_eq!(
        app.screen().count("role:image[Casey Nguyen, Online]"),
        Ok(1)
    );
}

#[test]
fn avatar_gallery_has_all_examples_and_dropdown_is_operable() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#avatar").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:avatar-docs-scroll").unwrap();
    for example in [
        "Avatar demo",
        "Basic",
        "Badge",
        "Badge with Icon",
        "Avatar Group",
        "Avatar Group Count",
        "Avatar Group with Icon",
        "Sizes",
        "Dropdown",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }
    assert!(app.shows("+٣"));
    app.press("role:region[Dropdown] >> role:button[Account menu]")
        .unwrap();
    app.press("role:menuitem[Billing]").unwrap();
    app.wait_for("Selected in this preview: Billing").unwrap();
    app.wait_for_gone("role:menuitem[Billing]", Duration::from_secs(1))
        .unwrap();
    app.press("role:region[Dropdown] >> role:button[Account menu]")
        .unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("role:menuitem[Billing]", Duration::from_secs(1))
        .unwrap();
}
