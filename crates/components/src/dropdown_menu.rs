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
