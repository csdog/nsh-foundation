//! Button-triggered menus using Native Shell positioning and keyboard navigation.

use nsh_base::{MenuEvent, menu};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::{
    button::{Button, ButtonVariant},
    direction::use_direction,
};

pub use crate::menu_parts::{
    MenuItem as DropdownMenuItem, MenuSubmenu as DropdownMenuSubmenu,
    menu_label as dropdown_menu_label, separator as dropdown_menu_separator,
};

const POPUP: &str = "min-w-32 p-1 rounded-2xl bg-popover text-popover-foreground border border-border shadow-lg transition-opacity duration-100";

pub struct DropdownMenuTrigger(View);

impl DropdownMenuTrigger {
    pub fn new(label: &'static str) -> Self {
        Self::render(Button::new(label).variant(ButtonVariant::Outline).surface())
    }

    pub fn render(view: View) -> Self {
        Self(view)
    }

    pub fn test_id(self, id: &'static str) -> Self {
        Self(self.0.test_id(id))
    }
}

pub struct DropdownMenuContent {
    placement: Placement,
    align: Align,
    width: Option<u16>,
    direction: Option<LayoutDirection>,
    children: Vec<AnyElement>,
}

impl DropdownMenuContent {
    pub fn new() -> Self {
        Self {
            placement: Placement::Bottom,
            align: Align::Start,
            width: None,
            direction: None,
            children: Vec::new(),
        }
    }

    pub fn placement(mut self, side: Placement) -> Self {
        self.placement = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, points: u16) -> Self {
        self.width = Some(points.max(128));
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_element));
        self
    }
}

impl Default for DropdownMenuContent {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DropdownMenu {
    trigger: DropdownMenuTrigger,
    content: DropdownMenuContent,
    open: Option<Signal<bool>>,
}

impl DropdownMenu {
    pub fn new(trigger: DropdownMenuTrigger, content: DropdownMenuContent) -> Self {
        Self {
            trigger,
            content,
            open: None,
        }
    }

    /// Controls the open state with a Rust signal.
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.open = Some(open);
        self
    }
}

impl IntoElement for DropdownMenu {
    fn into_element(self) -> AnyElement {
        let DropdownMenuContent {
            placement,
            align,
            width,
            direction,
            children,
        } = self.content;
        let open = self.open;
        let direction = direction.map_or_else(
            || use_direction().map(LayoutDirection::from),
            Reactive::Static,
        );
        let class = move || {
            // Native Shell's touch sheet uses the available viewport width.
            Some(SharedString::from(
                width.filter(|_| is_desktop()).map_or_else(
                    || POPUP.to_owned(),
                    |width| format!("{POPUP} w-[{width}px]"),
                ),
            ))
        };
        menu()
            .direction(direction)
            .open(move || open.map(|open| open.get()))
            .on_open_change(move |event: &MenuEvent| {
                if let (Some(open), MenuEvent::OpenChange(next, _)) = (open, event) {
                    open.set(*next);
                }
            })
            .placement(placement)
            .align(align)
            .margin(px(8.0))
            .offset(px(4.0))
            .popup_class(class)
            .popup_starting_class("opacity-0")
            .popup_ending_class("opacity-0")
            .trigger(self.trigger.0)
            .children(children)
            .into_element()
    }
}

pub fn dropdown_menu_group(
    label: &'static str,
    direction: LayoutDirection,
    children: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    view()
        .direction(direction)
        .role(AccessibilityRole::Group)
        .label(label)
        .child(dropdown_menu_label(label, direction))
        .children(children)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use nsh_core::{
        Size,
        element::Root,
        platform::{Idiom, Os},
    };
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;
    use crate::App;

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
}
