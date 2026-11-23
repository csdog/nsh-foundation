use components::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use nsh_core::{element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn ControlledTabs() -> impl IntoElement {
    let value = signal("home");
    Tabs::new(TabsList::new("Sections").activate_on_focus(true).triggers([
        TabsTrigger::new("home", "Home"),
        TabsTrigger::new("disabled", "Disabled").disabled(true),
        TabsTrigger::new("reports", "Reports"),
    ]))
    .value(value)
    .content(TabsContent::new("home", || text("Home content")))
    .content(TabsContent::new("reports", || text("Report content")))
}

#[test]
fn automatic_activation_skips_disabled_tabs_and_updates_the_visible_panel() {
    let mut config = EngineConfig::new(Root::of::<ControlledTabs>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    assert!(app.shows("Home content"));
    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("role:region[Reports]").unwrap();
    assert!(app.shows("Report content"));
    assert!(!app.shows("Home content"));
    assert!(
        app.screen()
            .accessibility_state("role:tab[Reports]")
            .unwrap()
            .selected
    );
    assert!(
        app.screen()
            .accessibility_state("role:tab[Disabled]")
            .unwrap()
            .disabled
    );
}
