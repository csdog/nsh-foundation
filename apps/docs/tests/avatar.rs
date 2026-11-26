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

#[test]
fn avatar_demo_shows_and_copies_the_actual_rsx_component() {
    let mut app = Headless::start_at_page(
        EngineConfig::new(Root::of::<App>()),
        "https://foundation.test/#avatar",
    )
    .unwrap();
    app.first_render().unwrap();
    let region = "role:region[Avatar demo]";
    assert_eq!(
        app.screen()
            .count(&format!("{region} >> role:image[Ada Lovelace]")),
        Ok(1)
    );
    app.press(&format!("{region} >> RSX")).unwrap();
    app.press(&format!("{region} >> Copy")).unwrap();
    assert_eq!(
        app.screen().clipboard().unwrap(),
        include_str!("../src/examples/rsx/avatar.rs"),
    );
    app.press(&format!("{region} >> TSX")).unwrap();
    app.press(&format!("{region} >> Copy")).unwrap();
    assert_eq!(
        app.screen().clipboard().unwrap(),
        include_str!("../src/examples/rsx/avatar.tsx"),
    );
}

#[test]
fn avatar_variants_offer_their_own_runnable_rsx_and_native_tsx() {
    macro_rules! source {
        ($file:literal, $label:literal) => {
            (
                $label,
                include_str!(concat!(
                    "../src/examples/avatar_examples/rsx/",
                    $file,
                    ".rs"
                )),
                include_str!(concat!(
                    "../src/examples/avatar_examples/tsx/",
                    $file,
                    ".tsx"
                )),
            )
        };
    }

    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#avatar").unwrap();
    app.first_render().unwrap();

    for (label, rsx, tsx) in [
        source!("demo", "Avatar group preview"),
        source!("basic", "Basic"),
        source!("badge", "Badge"),
        source!("badge_icon", "Badge with Icon"),
        source!("group", "Avatar Group"),
        source!("group_count", "Avatar Group Count"),
        source!("group_count_icon", "Avatar Group with Icon"),
        source!("size", "Sizes"),
        source!("dropdown", "Dropdown"),
        source!("rtl", "RTL"),
    ] {
        let region = format!("role:region[{label}]");
        for (tab, code) in [("RSX", rsx), ("TSX", tsx)] {
            app.press(&format!("{region} >> {tab}")).unwrap();
            app.press(&format!("{region} >> Copy")).unwrap();
            assert_eq!(app.screen().clipboard().unwrap(), code, "{label} {tab}");
            if tab == "RSX" {
                app.press(&format!("{region} >> Run example")).unwrap();
                if label == "Dropdown" {
                    app.press(&format!("{region} >> role:button[Account menu]"))
                        .unwrap();
                    app.press("role:menuitem[Billing]").unwrap();
                    app.wait_for("Selected in this preview: Billing").unwrap();
                }
                app.press(&format!("{region} >> Hide example")).unwrap();
            }
        }
    }
}
