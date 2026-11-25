use components::checkbox::{Checkbox, CheckboxState};
use docs::App;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

fn page(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#checkbox").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:checkbox-docs-scroll").unwrap();
    app
}

fn checked(app: &Headless, selector: &str) -> CheckboxState {
    app.screen()
        .accessibility_state(selector)
        .unwrap()
        .checked
        .unwrap()
}

#[test]
fn checkbox_gallery_exposes_description_invalid_and_disabled_states() {
    let mut app = page(390.0);
    for name in [
        "Checkbox demo",
        "Invalid State",
        "Basic",
        "Description",
        "Disabled",
        "Group",
        "Table",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let agree = "role:checkbox[Accept terms and conditions]";
    assert!(
        app.screen()
            .accessibility_state(&format!("role:region[Invalid State] >> {agree}"))
            .unwrap()
            .invalid
    );
    let described = format!("role:region[Description] >> {agree}");
    assert_eq!(checked(&app, &described), CheckboxState::True);
    assert_eq!(
        app.screen()
            .accessibility_description(&described)
            .unwrap()
            .as_deref(),
        Some("By clicking this checkbox, you agree to the terms.")
    );
    let disabled = "role:region[Disabled] >> role:checkbox[Enable notifications]";
    assert_eq!(app.screen().enabled(disabled), Ok(false));

    app.press("role:region[Group] >> text:Hard disks").unwrap();
    assert_eq!(
        checked(&app, "role:checkbox[Hard disks]"),
        CheckboxState::False
    );
    assert_eq!(
        checked(&app, "role:checkbox[External disks]"),
        CheckboxState::True
    );
    app.press("role:region[RTL] >> עברית").unwrap();
    assert!(app.shows("קבל תנאים והגבלות"));
    let mark_x = |app: &Headless| {
        let nodes = app.screen().layout_nodes();
        let control = nodes
            .iter()
            .find(|node| node.test_id.as_deref() == Some("localized-checkbox"))
            .unwrap();
        let mark = nodes
            .iter()
            .find(|node| {
                node.parent == control.id && node.test_id.as_deref() == Some("checkbox-mark")
            })
            .unwrap();
        mark.bounds.origin.x.0 - control.bounds.origin.x.0
    };
    assert!(mark_x(&app) > 100.0);
    app.press("role:region[RTL] >> English").unwrap();
    assert!(mark_x(&app).abs() < 1.0);
}

#[test]
fn table_header_tracks_mixed_checked_and_empty_row_selections() {
    let mut app = page(1280.0);
    let all = "role:checkbox[Select all rows]";
    let sarah = "role:checkbox[Select Sarah Chen]";
    let marcus = "role:checkbox[Select Marcus Rodriguez]";
    assert_eq!(checked(&app, all), CheckboxState::Mixed);
    assert_eq!(checked(&app, sarah), CheckboxState::True);
    assert_eq!(checked(&app, marcus), CheckboxState::False);
    app.press(all).unwrap();
    assert_eq!(checked(&app, all), CheckboxState::True);
    assert_eq!(checked(&app, marcus), CheckboxState::True);
    app.press(sarah).unwrap();
    assert_eq!(checked(&app, all), CheckboxState::Mixed);
    app.press(all).unwrap();
    app.press(all).unwrap();
    assert_eq!(checked(&app, all), CheckboxState::False);
    assert_eq!(checked(&app, marcus), CheckboxState::False);
}

#[component]
fn KeyboardExample() -> impl IntoElement {
    let accepted = signal(CheckboxState::Mixed);
    view().child(
        Checkbox::new("Include all")
            .checked(accepted)
            .on_change(move |next| {
                accepted.set(if next {
                    CheckboxState::True
                } else {
                    CheckboxState::False
                })
            }),
    )
}

#[test]
fn space_requests_checked_from_the_mixed_state() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<KeyboardExample>());
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    let control = "role:checkbox[Include all]";
    assert_eq!(checked(&app, control), CheckboxState::Mixed);
    app.press_key("tab").unwrap();
    app.press_key("space").unwrap();
    assert_eq!(checked(&app, control), CheckboxState::True);
    app.press_key("space").unwrap();
    assert_eq!(checked(&app, control), CheckboxState::False);
}
