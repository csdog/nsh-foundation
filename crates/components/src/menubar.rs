//! Application menus with a shared open state and Native Shell menu and toolbar behavior.

use nsh_base::{MenuEvent, menu, theme, toolbar};
use nsh_core::{element::LayoutDirection, keymap::When, prelude::*};

use crate::direction::{Direction, use_direction};

pub use crate::menu_parts::{
    MenuItem as MenubarItem, MenuSubmenu as MenubarSub, menu_label as menubar_label,
    separator as menubar_separator,
};

const POPUP: &str = "min-w-36 p-1 rounded-2xl bg-popover text-popover-foreground border border-border shadow-lg transition-opacity duration-100";

pub struct Menubar {
    label: &'static str,
    menus: Vec<MenubarMenu>,
}

impl Menubar {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            menus: Vec::new(),
        }
    }

    pub fn menu(mut self, menu: MenubarMenu) -> Self {
        self.menus.push(menu);
        self
    }
}

impl IntoElement for Menubar {
    fn into_element(self) -> AnyElement {
        let count = self.menus.len();
        let direction = untracked(|| use_direction().get());
        let active = signal(None::<usize>);
        let focused = signal(None::<usize>);
        if count > 1 {
            effect(move || {
                let Some(index) = active.get() else { return };
                let previous = (index + count - 1) % count;
                let next = (index + 1) % count;
                let (left, right) = match direction {
                    Direction::Ltr => (previous, next),
                    Direction::Rtl => (next, previous),
                };
                let bind = move || {
                    for (key, target) in [("left", left), ("right", right)] {
                        on_key_with(
                            key,
                            KeyOptions {
                                when: When::NotEditing,
                                ..Default::default()
                            },
                            move |_| active.set(Some(target)),
                        );
                    }
                };
                bind();
                if focused.get().is_some() {
                    // Closing the old popup can restore its trigger's focus.
                    // Keep the bar's open-menu keys ahead of toolbar navigation.
                    let frame = request_animation_frame(move |_| bind());
                    cleanup(move || frame.cancel());
                }
            });
        }

        // The toolbar's directional keys follow the ambient platform, so
        // register children in that order when the bar overrides it locally.
        let mut menus: Vec<_> = self.menus.into_iter().enumerate().collect();
        let rtl = direction == Direction::Rtl;
        let ambient_rtl = nsh_core::element::layout_rtl();
        if rtl != ambient_rtl {
            menus.reverse();
        }
        view()
            .role(AccessibilityRole::Menubar)
            .label(self.label)
            .direction(LayoutDirection::from(direction))
            .w_full()
            .max_w(px(288.0))
            .min_h(px(32.0))
            .border(px(1.0))
            .border_color(|| theme().border)
            .rounded(px(16.0))
            .p(px(3.0))
            .child(
                toolbar()
                    .direction(if ambient_rtl {
                        LayoutDirection::Rtl
                    } else {
                        LayoutDirection::Ltr
                    })
                    .when(rtl != ambient_rtl, |bar| bar.justify_end())
                    .items_center()
                    .children(
                        menus
                            .into_iter()
                            .map(|(index, menu)| menu.render(index, active, focused, direction)),
                    ),
            )
            .into_element()
    }
}

pub struct MenubarMenu {
    trigger: MenubarTrigger,
    content: MenubarContent,
}

impl MenubarMenu {
    pub fn new(trigger: MenubarTrigger, content: MenubarContent) -> Self {
        Self { trigger, content }
    }

    fn render(
        self,
        index: usize,
        active: Signal<Option<usize>>,
        focused: Signal<Option<usize>>,
        direction: Direction,
    ) -> impl IntoElement {
        let open = move || active.get() == Some(index);
        let popup = move || {
            Some(SharedString::from(match self.content.width {
                Some(width) if is_desktop() => format!("{POPUP} w-[{width}px]"),
                _ => POPUP.to_owned(),
            }))
        };
        menu()
            .direction(LayoutDirection::from(direction))
            .open(move || Some(open()))
            .on_open_change(move |event: &MenuEvent| {
                if let MenuEvent::OpenChange(next, _) = event {
                    if *next {
                        active.set(Some(index));
                    } else if active.get_untracked() == Some(index) {
                        active.set(None);
                    }
                }
            })
            .align(Align::Start)
            .offset(px(8.0))
            .margin(px(8.0))
            .popup_class(popup)
            .popup_starting_class("opacity-0")
            .popup_ending_class("opacity-0")
            .trigger(
                view()
                    .role(AccessibilityRole::Menuitem)
                    .label(self.trigger.0)
                    .on_focus(move || focused.set(Some(index)))
                    .on_blur(move || {
                        if focused.get_untracked() == Some(index) {
                            focused.set(None);
                        }
                    })
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(12.0))
                    .items_center()
                    .justify_center()
                    .hover(|style| style.bg(theme().muted))
                    .style_when(open, |style| style.bg(theme().muted))
                    .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                    .child(
                        text(self.trigger.0)
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .selectable(false),
                    ),
            )
            .children(self.content.items)
    }
}

pub struct MenubarTrigger(&'static str);

impl MenubarTrigger {
    pub fn new(label: &'static str) -> Self {
        Self(label)
    }
}

#[derive(Default)]
pub struct MenubarContent {
    width: Option<u16>,
    items: Vec<AnyElement>,
}

impl MenubarContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width.max(144));
        self
    }

    pub fn child(mut self, item: impl IntoElement) -> Self {
        self.items.push(item.into_element());
        self
    }

    pub fn children(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.items
            .extend(items.into_iter().map(IntoElement::into_element));
        self
    }
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

    #[component]
    fn ApplicationMenu() -> impl IntoElement {
        let chosen = signal(false);
        view()
            .child(
                Menubar::new("Application")
                    .menu(MenubarMenu::new(
                        MenubarTrigger::new("File"),
                        MenubarContent::new().child(MenubarItem::new("New Tab")),
                    ))
                    .menu(MenubarMenu::new(
                        MenubarTrigger::new("Edit"),
                        MenubarContent::new()
                            .child(MenubarItem::new("Undo").on_select(move || chosen.set(true))),
                    )),
            )
            .child(text(move || {
                if chosen.get() {
                    "Undo selected"
                } else {
                    "Ready"
                }
            }))
    }

    #[test]
    fn arrows_move_between_menus_and_only_one_is_open() {
        let mut config = EngineConfig::new(Root::of::<ApplicationMenu>());
        config.viewport = Size::new(px(1280.0), px(800.0));
        config.environment.os = Os::Web;
        config.environment.idiom = Idiom::Desktop;
        config.environment.reduce_motion = true;
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();

        app.press_key("tab").unwrap();
        app.press_key("right").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for("role:menuitem[Undo]").unwrap();
        app.press_key("left").unwrap();
        app.wait_for("role:menuitem[New Tab]").unwrap();
        app.wait_for_gone("role:menuitem[Undo]", Duration::from_secs(2))
            .unwrap();
        app.press_key("right").unwrap();
        app.wait_for("role:menuitem[Undo]").unwrap();
        app.press_key("down").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for("Undo selected").unwrap();
    }
}
