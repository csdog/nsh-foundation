use components::button::{Button, ButtonSize};
use docs::App;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn ButtonStates() -> impl IntoElement {
    let loading = signal(true);
    let count = signal(0);
    view()
        .child(
            Button::new("Submit")
                .size(ButtonSize::Icon)
                .icon_start(text("↑"))
                .loading(loading)
                .on_press(move || count.update(|n| *n += 1))
                .test_id("submit"),
        )
        .child(Button::new("Finish loading").on_press(move || loading.set(false)))
        .child(
            Button::new("Unavailable")
                .disabled(true)
                .on_press(move || count.update(|n| *n += 1)),
        )
        .child(text(move || format!("Submitted: {}", count.get())))
}

#[test]
fn loading_and_disabled_buttons_do_not_activate() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<ButtonStates>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let button = "role:button[Submit]";
    assert!(app.screen().accessibility_state(button).unwrap().busy);
    assert_eq!(app.screen().enabled(button), Ok(false));
    assert_eq!(app.screen().enabled("role:button[Unavailable]"), Ok(false));
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    assert!(app.shows("Submitted: 0"));
    assert_eq!(app.screen().enabled(button), Ok(true));
    assert!(!app.screen().accessibility_state(button).unwrap().busy);
    app.press_key("shift+tab").unwrap();
    app.press_key("space").unwrap();
    assert!(app.shows("Submitted: 1"));
}

#[test]
fn button_docs_cover_sizes_link_and_group_actions() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#button").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:button-docs-scroll").unwrap();
    for name in [
        "Button demo",
        "Size",
        "Default",
        "Outline",
        "Secondary",
        "Ghost",
        "Destructive",
        "Link",
        "Icon",
        "With Icon",
        "Rounded",
        "Spinner",
        "Button Group",
        "As Link",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    assert_eq!(app.screen().enabled("role:button[Generating]"), Ok(false));
    app.press("role:link[Login]").unwrap();
    assert_eq!(
        app.screen().opened_urls(),
        &["https://ui.shadcn.com/docs/components/base/button#as-link"]
    );
    app.press("role:button[More Options]").unwrap();
    app.press("role:menuitem[Mark as Read]").unwrap();
    assert!(app.shows("Selected in this preview: Mark as Read"));
}
