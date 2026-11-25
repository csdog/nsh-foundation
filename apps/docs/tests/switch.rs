use components::switch::Switch;
use docs::App;
use nsh_core::{Checked, Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn UncontrolledSwitch() -> impl IntoElement {
    Switch::new("Notifications").default_checked(true)
}

#[test]
fn default_state_toggles_with_space() {
    let mut cfg = EngineConfig::new(nsh_testing::windowed_root::<UncontrolledSwitch>());
    cfg.environment.os = Os::Linux;
    cfg.environment.reduce_motion = true;
    let mut app = Headless::start(cfg).unwrap();
    app.first_render().unwrap();
    let selector = "role:switch[Notifications]";
    let checked = |app: &Headless| app.screen().accessibility_state(selector).unwrap().checked;
    assert_eq!(checked(&app), Some(Checked::True));
    app.press_key("tab").unwrap();
    app.press_key("space").unwrap();
    assert_eq!(checked(&app), Some(Checked::False));
}

#[test]
fn labels_and_choice_cards_toggle_the_bound_switch_once() {
    let mut cfg = EngineConfig::new(Root::of::<App>());
    cfg.environment.os = Os::Web;
    cfg.environment.reduce_motion = true;
    cfg.viewport = Size::new(px(390.0), px(844.0));
    let mut app = Headless::start_at_page(cfg, "https://foundation.test/#switch").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:switch-docs-scroll").unwrap();

    let checked =
        |app: &Headless, target| app.screen().accessibility_state(target).unwrap().checked;
    let main = "role:region[Switch demo] >> role:switch[Airplane Mode]";
    assert_eq!(checked(&app, main), Some(Checked::False));
    app.press("role:region[Switch demo] >> id:switch-label")
        .unwrap();
    assert_eq!(checked(&app, main), Some(Checked::True));

    let notifications = "role:region[Choice Card] >> role:switch[Enable notifications]";
    assert_eq!(checked(&app, notifications), Some(Checked::True));
    app.press("Receive notifications when focus mode is enabled or disabled.")
        .unwrap();
    assert_eq!(checked(&app, notifications), Some(Checked::False));
    app.press(notifications).unwrap();
    assert_eq!(checked(&app, notifications), Some(Checked::True));
}
