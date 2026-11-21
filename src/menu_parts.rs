//! Shared Rhea menu rows for anchored and context menus.

use nsh_base::{menu_item, menu_separator, submenu, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::direction::use_direction;

pub struct MenuItem {
    label: &'static str,
    disabled: bool,
    destructive: bool,
    icon: Option<&'static str>,
    shortcut: Option<&'static str>,
    checked: Option<Box<dyn Fn() -> bool>>,
    radio: bool,
    direction: Option<LayoutDirection>,
    close_on_select: bool,
    on_select: Box<dyn Fn()>,
}

impl MenuItem {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            disabled: false,
            destructive: false,
            icon: None,
            shortcut: None,
            checked: None,
            radio: false,
            direction: None,
            close_on_select: true,
            on_select: Box::new(|| {}),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: &'static str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn checkbox(mut self, checked: impl Fn() -> bool + 'static) -> Self {
        self.checked = Some(Box::new(checked));
        self
    }

    pub fn radio(mut self, checked: impl Fn() -> bool + 'static) -> Self {
        self.radio = true;
        self.checkbox(checked)
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Keeps a menu open when used with false, such as for independent checkboxes.
    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }

    pub fn on_select(mut self, action: impl Fn() + 'static) -> Self {
        self.on_select = Box::new(action);
        self
    }
}

impl IntoElement for MenuItem {
    fn into_element(self) -> AnyElement {
        let Self {
            label,
            disabled,
            destructive,
            icon,
            shortcut,
            checked,
            radio,
            direction,
            close_on_select,
            on_select,
        } = self;
        let direction = direction.map_or_else(
            || use_direction().map(LayoutDirection::from),
            Reactive::Static,
        );
        let checked = stored(checked);
        let is_checked = move || checked.with(|value| value.as_ref().map(|read| read()));
        let highlight = move || {
            if destructive {
                theme().destructive.opacity(0.12)
            } else {
                theme().accent
            }
        };
        menu_item()
            .disabled(disabled)
            .role(radio.then_some(AccessibilityRole::MenuItemRadio))
            .checked(is_checked)
            .close_on_select(close_on_select)
            .accessibility_label(label)
            .min_h(px(32.0))
            .px_2()
            .py_1()
            .rounded(px(12.0))
            .opacity(if disabled { 0.5 } else { 1.0 })
            .hover(move |s| s.bg(highlight()))
            .focus(move |s| s.bg(highlight()))
            .on_select(on_select)
            .child(
                view()
                    .flex_row()
                    .direction(direction)
                    .items_center()
                    .gap_2()
                    .child(icon.map(|icon| {
                        text(icon)
                            .w(px(16.0))
                            .text_sm()
                            .accessibility_elements_hidden(true)
                    }))
                    .child(text(label).flex_1().text_sm().text_color(move || {
                        if destructive {
                            theme().destructive
                        } else {
                            theme().popover_foreground
                        }
                    }))
                    .child(shortcut.map(hint))
                    .child(checked.with(|value| {
                        value.as_ref().map(|_| {
                            text(move || {
                                if is_checked() == Some(true) {
                                    "✓"
                                } else {
                                    ""
                                }
                            })
                            .w(px(16.0))
                            .text_sm()
                            .accessibility_elements_hidden(true)
                        })
                    })),
            )
            .into_element()
    }
}

fn hint(shortcut: &'static str) -> impl IntoElement {
    text(shortcut)
        .text_xs()
        .text_color(move || theme().muted_foreground)
}

pub fn menu_label(label: &'static str, direction: LayoutDirection) -> impl IntoElement {
    view().direction(direction).px_2().py_1().child(
        text(label)
            .text_xs()
            .text_color(move || theme().muted_foreground),
    )
}

pub fn separator() -> impl IntoElement {
    menu_separator()
        .h(px(1.0))
        .my_1()
        .bg(move || theme().border.opacity(0.5))
}

pub fn sub_menu(
    label: &'static str,
    direction: LayoutDirection,
    children: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    MenuSubmenu::new(label)
        .direction(direction)
        .children(children)
}

pub struct MenuSubmenu {
    label: &'static str,
    direction: Option<LayoutDirection>,
    icon: Option<&'static str>,
    children: Vec<AnyElement>,
}

impl MenuSubmenu {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            direction: None,
            icon: None,
            children: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
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

impl IntoElement for MenuSubmenu {
    fn into_element(self) -> AnyElement {
        let direction = self.direction.map_or_else(
            || use_direction().map(LayoutDirection::from),
            Reactive::Static,
        );
        let arrow = direction.clone();
        submenu()
            .offset(px(4.0))
            .trigger(
                view()
                    .label(self.label)
                    .min_h(px(32.0))
                    .flex_row()
                    .direction(direction)
                    .items_center()
                    .gap_2()
                    .rounded(px(12.0))
                    .px_2()
                    .py_1()
                    .hover(|s| s.bg(theme().accent))
                    .focus(|s| s.bg(theme().accent))
                    .child(self.icon.map(|icon| {
                        text(icon)
                            .w(px(16.0))
                            .text_sm()
                            .accessibility_elements_hidden(true)
                    }))
                    .child(text(self.label).flex_1().text_sm())
                    .child(
                        text(move || {
                            if arrow.get() == LayoutDirection::Rtl {
                                "‹"
                            } else {
                                "›"
                            }
                        })
                        .text_xs()
                        .text_color(move || theme().muted_foreground)
                        .accessibility_elements_hidden(true),
                    ),
            )
            .children(self.children)
            .into_element()
    }
}
