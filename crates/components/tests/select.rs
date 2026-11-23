use components::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};
use nsh_core::{
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn BoundSelect() -> impl IntoElement {
    let value = signal(Some(SharedString::from("alpha")));
    let open = signal(false);
    view()
        .child(
            Select::new(
                "Choice",
                SelectTrigger::new(SelectValue::new("Choose one")),
                SelectContent::new()
                    .group(
                        SelectGroup::new("Available")
                            .item(SelectItem::new("alpha", "Alpha"))
                            .item(SelectItem::new("beta", "Beta").disabled(true)),
                    )
                    .group(SelectGroup::new("Other").item(SelectItem::new("gamma", "Gamma"))),
            )
            .open(open)
            .value(value),
        )
        .child(text(move || {
            format!("Selected {}", value.get().unwrap_or_default())
        }))
        .child(text(move || {
            if open.get() {
                "List opened"
            } else {
                "List closed"
            }
        }))
}

#[test]
fn keyboard_skips_the_disabled_item_and_updates_the_bound_signals() {
    let mut config = EngineConfig::new(Root::of::<BoundSelect>());
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("text:List opened").unwrap();
    assert_eq!(app.screen().enabled("role:option[Beta]"), Ok(false));
    app.press_key("down").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("text:Selected gamma").unwrap();
    app.wait_for("text:List closed").unwrap();
}
