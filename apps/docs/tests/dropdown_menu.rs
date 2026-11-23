use std::time::Duration;

use nsh_core::{
    Size,
    element::Root,
    platform::{Idiom, Os},
};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

use components::dropdown_menu::*;
use docs::App;
use nsh_core::prelude::*;

fn start(root: Root) -> Headless {
    let mut config = EngineConfig::new(root);
    config.viewport = Size::new(px(1280.0), px(844.0));
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    let mut app =
        Headless::start_at_page(config, "https://foundation.test/#dropdown-menu").unwrap();
    app.first_render().unwrap();
    app
}

#[component]
fn KeyboardMenu() -> impl IntoElement {
    let open = signal(false);
    let chosen = signal("");
    view()
        .child(
            DropdownMenu::new(
                DropdownMenuTrigger::new("Open menu"),
                DropdownMenuContent::new()
                    .child(DropdownMenuItem::new("Profile"))
                    .child(DropdownMenuItem::new("Disabled").disabled(true))
                    .child(
                        DropdownMenuItem::new("Sign out")
                            .on_select(move || chosen.set("Signed out")),
                    ),
            )
            .open(open),
        )
        .child(text(move || chosen.get()))
}

#[test]
fn controlled_open_and_keyboard_navigation_skip_disabled_rows() {
    let mut app = start(Root::of::<KeyboardMenu>());
    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("role:menuitem[Profile]").unwrap();
    app.press_key("down").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Signed out").unwrap();
    app.wait_for_gone("role:menu", Duration::from_secs(2))
        .unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("role:menuitem[Profile]").unwrap();
}

#[test]
fn gallery_nested_actions_and_checked_items_preserve_state() {
    let mut app = start(Root::of::<App>());
    app.wait_for("id:dropdown-menu-docs-scroll").unwrap();
    for example in [
        "Dropdown Menu demo",
        "Basic",
        "Submenu",
        "Shortcuts",
        "Icons",
        "Checkboxes",
        "Checkboxes Icons",
        "Radio Group",
        "Radio Icons",
        "Destructive",
        "Avatar",
        "Complex",
        "RTL",
    ] {
        assert_eq!(
            app.screen().count(&format!("role:region[{example}]")),
            Ok(1)
        );
    }

    app.press("role:region[Complex] >> role:button[Complex Menu]")
        .unwrap();
    app.press("role:menuitem[Open Recent]").unwrap();
    app.press("role:menuitem[More Projects]").unwrap();
    app.press("role:menuitem[Project Gamma]").unwrap();
    app.wait_for("Selected in this preview: Project Gamma")
        .unwrap();

    app.press("role:region[Checkboxes] >> role:button[Open]")
        .unwrap();
    let panel = "role:menuitemcheckbox[Panel]";
    assert_eq!(
        app.screen().accessibility_state(panel).unwrap().checked,
        Some(nsh_core::Checked::False)
    );
    app.press(panel).unwrap();
    assert_eq!(
        app.screen().accessibility_state(panel).unwrap().checked,
        Some(nsh_core::Checked::True)
    );
    assert_eq!(
        app.screen().enabled("role:menuitemcheckbox[Activity Bar]"),
        Ok(false)
    );
    app.press_key("escape").unwrap();
    app.wait_for_gone(panel, Duration::from_secs(2)).unwrap();
    app.press("role:region[Checkboxes] >> role:button[Open]")
        .unwrap();
    assert_eq!(
        app.screen().accessibility_state(panel).unwrap().checked,
        Some(nsh_core::Checked::True)
    );
    app.press_key("escape").unwrap();

    app.press("role:region[Radio Group] >> role:button[Open]")
        .unwrap();
    app.press("role:menuitemradio[Right]").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("role:menuitemradio[Right]")
            .unwrap()
            .checked,
        Some(nsh_core::Checked::True)
    );
    assert_eq!(
        app.screen()
            .accessibility_state("role:menuitemradio[Bottom]")
            .unwrap()
            .checked,
        Some(nsh_core::Checked::False)
    );
}
