use components::{
    button::Button,
    radio_group::{RadioGroup, RadioGroupItem},
};
use nsh_core::{Checked, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn ControlledRadioGroup() -> impl IntoElement {
    let value = signal(None::<SharedString>);
    let calls = signal(0);
    view()
        .child(
            RadioGroup::new("Choice")
                .value(value)
                .on_change(move |_| calls.update(|count| *count += 1))
                .item(RadioGroupItem::new("a", "A"))
                .item(RadioGroupItem::new("b", "B").disabled(true))
                .item(RadioGroupItem::new("c", "C")),
        )
        .child(Button::new("Select A externally").on_press(move || value.set(Some("a".into()))))
        .child(text(move || format!("Changes: {}", calls.get())))
}

#[test]
fn radio_keyboard_skips_disabled_items_and_bound_state_can_be_set_externally() {
    let mut config = EngineConfig::new(Root::of::<ControlledRadioGroup>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("text:Changes: 1").unwrap();
    let checked = |app: &Headless, name| {
        app.screen()
            .accessibility_state(&format!("role:radio[{name}]"))
            .unwrap()
            .checked
    };
    assert_eq!(checked(&app, "C"), Some(Checked::True));
    assert_eq!(checked(&app, "B"), Some(Checked::False));
    app.press_key("space").unwrap();
    assert!(app.shows("Changes: 1"));

    app.press("role:button[Select A externally]").unwrap();
    assert_eq!(checked(&app, "A"), Some(Checked::True));
    assert_eq!(checked(&app, "C"), Some(Checked::False));
    assert!(app.shows("Changes: 1"));
}
