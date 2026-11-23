//! Composable context menus built on Native Shell's menu, focus and dismissal behavior.

use std::time::Duration;

use nsh_base::{MenuEvent, context_menu, menu, theme};
use nsh_core::{TouchEvent, TouchPhase, prelude::*};

pub use crate::menu_parts::{
    MenuItem as ContextMenuItem, menu_label as context_menu_label,
    separator as context_menu_separator, sub_menu as context_menu_submenu,
};

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
