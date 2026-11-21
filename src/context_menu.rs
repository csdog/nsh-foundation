//! Composable context menus built on Native Shell's menu, focus and dismissal behavior.

use std::time::Duration;

use nsh_base::{MenuEvent, context_menu, menu, menu_item, menu_separator, submenu, theme};
use nsh_core::{TouchEvent, TouchPhase, element::LayoutDirection, prelude::*};

const POPUP: &str = "min-w-36 p-1 rounded-2xl bg-popover text-popover-foreground border border-border shadow-lg transition-opacity duration-100";

pub struct ContextMenu {
    trigger: View,
    placement: Option<Placement>,
    children: Vec<AnyElement>,
}

impl ContextMenu {
    pub fn new(trigger: View) -> Self {
        Self {
            trigger,
            placement: None,
            children: Vec::new(),
        }
    }

    /// Sets the preferred side of the pointer. The menu can flip to stay on screen.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = Some(placement);
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

impl IntoElement for ContextMenu {
    fn into_element(self) -> AnyElement {
        let trigger = self
            .trigger
            .focusable(true)
            .focus(|s| s.border_color(theme().ring));
        match self.placement {
            None => context_menu()
                .offset(px(4.0))
                .margin(px(8.0))
                .popup_class(POPUP)
                .popup_starting_class("opacity-0")
                .popup_ending_class("opacity-0")
                .trigger(trigger)
                .children(self.children)
                .into_element(),
            Some(placement) => positioned_menu(trigger, placement, self.children).into_element(),
        }
    }
}

fn positioned_menu(trigger: View, placement: Placement, children: Vec<AnyElement>) -> View {
    // Native ContextMenu fixes its placement to Bottom. A controlled Menu
    // can use the same native collision and keyboard behavior at a point anchor.
    let at = signal(Point::<Points>::default());
    let open = signal(false);
    let hold = stored(None::<(Spawned<()>, Point<Points>)>);
    let trigger_ref = element_ref();
    let show = move |position| {
        at.set(position);
        open.set(true);
    };
    trigger
        .relative()
        .role(AccessibilityRole::Button)
        .accessibility_state(move || AccessibilityState {
            expanded: Some(open.get()),
            has_popup: true,
            ..Default::default()
        })
        .element_ref(&trigger_ref)
        .on_context_menu(move |e: &ContextMenuEvent| show(e.position))
        .on_touch(move |e: &TouchEvent| {
            if let [finger] = e.points.as_slice() {
                if e.phase == TouchPhase::Start && finger.pointer_type != PointerType::Mouse {
                    let point = finger.location;
                    let task = set_timeout(Duration::from_millis(400), move || {
                        hold.replace(None);
                        trigger_ref.cancel_press();
                        show(point);
                    });
                    if let Some((previous, _)) = hold.replace(Some((task, point))) {
                        previous.cancel();
                    }
                    return;
                }
                if e.phase == TouchPhase::Move
                    && hold.with(|pending| {
                        pending.as_ref().is_some_and(|(_, start)| {
                            (start.x.0 - finger.location.x.0).hypot(start.y.0 - finger.location.y.0)
                                <= 10.0
                        })
                    })
                {
                    return;
                }
            }
            if let Some((task, _)) = hold.replace(None) {
                task.cancel();
            }
        })
        .child(
            view()
                .absolute()
                .left(move || at.get().x)
                .top(move || at.get().y)
                .size(px(1.0))
                .pointer_events(PointerEvents::None)
                .child(
                    menu()
                        .open(move || open.get())
                        .placement(placement)
                        .offset(px(4.0))
                        .margin(px(8.0))
                        .popup_class(POPUP)
                        .popup_starting_class("opacity-0")
                        .popup_ending_class("opacity-0")
                        .on_open_change(move |event: &MenuEvent| {
                            if let MenuEvent::OpenChange(false, _) = event {
                                open.set(false);
                            }
                        })
                        .on_close(move || trigger_ref.focus())
                        .trigger(view().size(px(1.0)).focusable(false))
                        .children(children),
                ),
        )
}

pub struct ContextMenuItem {
    label: &'static str,
    disabled: bool,
    destructive: bool,
    icon: Option<&'static str>,
    shortcut: Option<&'static str>,
    checked: Option<Box<dyn Fn() -> bool>>,
    radio: bool,
    direction: LayoutDirection,
    on_select: Box<dyn Fn()>,
}

impl ContextMenuItem {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            disabled: false,
            destructive: false,
            icon: None,
            shortcut: None,
            checked: None,
            radio: false,
            direction: LayoutDirection::Ltr,
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
        self.direction = direction;
        self
    }

    pub fn on_select(mut self, action: impl Fn() + 'static) -> Self {
        self.on_select = Box::new(action);
        self
    }
}

impl IntoElement for ContextMenuItem {
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
            on_select,
        } = self;
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

pub fn context_menu_label(label: &'static str, direction: LayoutDirection) -> impl IntoElement {
    view().direction(direction).px_2().py_1().child(
        text(label)
            .text_xs()
            .text_color(move || theme().muted_foreground),
    )
}

pub fn context_menu_separator() -> impl IntoElement {
    menu_separator()
        .h(px(1.0))
        .my_1()
        .bg(move || theme().border.opacity(0.5))
}

pub fn context_menu_submenu(
    label: &'static str,
    direction: LayoutDirection,
    children: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    submenu()
        .offset(px(4.0))
        .trigger(
            view()
                .label(label)
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
                .child(text(label).flex_1().text_sm())
                .child(hint(if direction == LayoutDirection::Rtl {
                    "‹"
                } else {
                    "›"
                })),
        )
        .children(children)
}
