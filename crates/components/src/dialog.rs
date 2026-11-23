//! Composable Rhea dialog parts backed by Native Shell's modal.

use nsh_base::{dialog, theme};
use nsh_core::{element::LayoutDirection, keymap::When, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    ui::muted,
};

#[derive(Clone, Copy)]
pub struct Dialog {
    pub open: Signal<bool>,
}

impl Dialog {
    pub fn new(open: Signal<bool>) -> Self {
        Self { open }
    }

    pub fn close(self) {
        self.open.set(false);
    }
}

pub struct DialogTrigger(View);

impl DialogTrigger {
    pub fn new(dialog: Dialog, label: &'static str) -> Self {
        Self::render(
            dialog,
            Button::new(label).variant(ButtonVariant::Outline).surface(),
        )
    }

    pub fn render(dialog: Dialog, control: View) -> Self {
        Self(modal_trigger(control, dialog.open))
    }

    pub fn test_id(self, id: &'static str) -> Self {
        Self(self.0.test_id(id))
    }
}

pub(crate) fn modal_trigger(control: View, open: Signal<bool>) -> View {
    let trigger = element_ref();
    let opened_here = stored(false);
    effect(move || {
        if !open.get() && opened_here.replace(false) {
            spawn(async move {
                next_frame().await;
                if !open.get_untracked() {
                    trigger.focus();
                }
            });
        }
    });
    control
        .role(AccessibilityRole::Button)
        .element_ref(&trigger)
        .accessibility_state(move || AccessibilityState {
            expanded: Some(open.get()),
            has_popup: true,
            ..Default::default()
        })
        .on_press(move || {
            if !open.get_untracked() {
                opened_here.replace(true);
                open.set(true);
            }
        })
}

impl IntoElement for DialogTrigger {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct DialogClose(View);

impl DialogClose {
    pub fn new(dialog: Dialog, label: &'static str) -> Self {
        Self::render(
            dialog,
            Button::new(label).variant(ButtonVariant::Outline).surface(),
        )
    }

    pub fn render(dialog: Dialog, control: View) -> Self {
        Self(
            control
                .role(AccessibilityRole::Button)
                .on_press(move || dialog.close()),
        )
    }
}

impl IntoElement for DialogClose {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct DialogTitle(&'static str);

impl DialogTitle {
    pub fn new(title: &'static str) -> Self {
        Self(title)
    }
}

pub struct DialogDescription(&'static str);

impl DialogDescription {
    pub fn new(description: &'static str) -> Self {
        Self(description)
    }
}

pub struct DialogHeader {
    title: DialogTitle,
    description: Option<DialogDescription>,
}

impl DialogHeader {
    pub fn new(title: DialogTitle) -> Self {
        Self {
            title,
            description: None,
        }
    }

    pub fn description(mut self, description: DialogDescription) -> Self {
        self.description = Some(description);
        self
    }

    fn render(&self, rtl: bool, corner_close: bool) -> View {
        view()
            .flex_shrink_0()
            .gap(px(6.0))
            .when(corner_close && rtl, |v| v.pl(px(28.0)))
            .when(corner_close && !rtl, |v| v.pr(px(28.0)))
            .child(
                text(self.title.0)
                    .text_size(px(16.0))
                    .line_height(px(20.0))
                    .font_weight(FontWeight::MEDIUM)
                    .when(rtl, |title| title.text_right()),
            )
            .child(self.description.as_ref().map(|description| {
                muted(description.0).when(rtl, |description| description.text_right())
            }))
    }
}

pub struct DialogFooter {
    start: bool,
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align_start(mut self) -> Self {
        self.start = true;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self {
            start: false,
            children: Vec::new(),
        }
    }
}

impl IntoElement for DialogFooter {
    fn into_element(self) -> AnyElement {
        let desktop = || window().width.0 >= 640.0;
        view()
            .flex_shrink_0()
            .flex_col_reverse()
            .gap_2()
            .style_when(desktop, |style| style.flex_row())
            .style_when(
                move || !self.start && desktop(),
                |style| style.justify_end(),
            )
            .test_id("dialog-footer")
            .children(self.children)
            .into_element()
    }
}

pub struct DialogOverlay(View);

impl DialogOverlay {
    pub fn new(dialog: Dialog, close_on_press: bool) -> Self {
        Self(
            view()
                .absolute()
                .inset_0()
                .bg(hsla(0.0, 0.0, 0.0, 0.30))
                .backdrop_blur(px(8.0))
                .focusable(false)
                .test_id("dialog-overlay")
                .when(close_on_press, |overlay| {
                    overlay.on_press(move || dialog.close())
                }),
        )
    }
}

impl IntoElement for DialogOverlay {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

/// The modal and its lazy content. Only the body scrolls, so the header
/// and footer stay in view while long content moves.
pub struct DialogContent {
    dialog: Dialog,
    header: DialogHeader,
    body: Option<Box<dyn Fn() -> View>>,
    footer: Option<Box<dyn Fn() -> DialogFooter>>,
    close_button: Option<&'static str>,
    outside_press: bool,
    escape: bool,
    direction: LayoutDirection,
    width: f32,
    long_body: bool,
}

impl DialogContent {
    pub fn new(dialog: Dialog, header: DialogHeader) -> Self {
        Self {
            dialog,
            header,
            body: None,
            footer: None,
            close_button: Some("Close"),
            outside_press: true,
            escape: true,
            direction: LayoutDirection::Inherit,
            width: 448.0,
            long_body: false,
        }
    }

    pub fn body(mut self, body: impl Fn() -> View + 'static) -> Self {
        self.body = Some(Box::new(body));
        self
    }

    pub fn scrollable(mut self, body: impl Fn() -> View + 'static) -> Self {
        self.long_body = true;
        self.body(body)
    }

    pub fn footer(mut self, footer: impl Fn() -> DialogFooter + 'static) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Sets the corner button's accessible label. None hides it.
    pub fn close_button(mut self, label: Option<&'static str>) -> Self {
        self.close_button = label;
        self
    }

    pub fn close_on_outside_press(mut self, enabled: bool) -> Self {
        self.outside_press = enabled;
        self
    }

    pub fn close_on_escape(mut self, enabled: bool) -> Self {
        self.escape = enabled;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn max_width(mut self, points: f32) -> Self {
        self.width = points.max(0.0);
        self
    }
}

impl IntoElement for DialogContent {
    fn into_element(self) -> AnyElement {
        let state = self.dialog;
        let rtl = self.direction == LayoutDirection::Rtl;
        when(state.open, move || {
            let width = self.width;
            let long_body = self.long_body;
            let panel_ref = element_ref();
            let close_ref = element_ref();
            let end_ref = element_ref();
            let placed = stored(false);
            let wrapping_back = stored(false);
            let corner_close = self.close_button.is_some();
            let has_content = self.body.is_some() || self.footer.is_some();
            if self.escape {
                // Native text fields handle Escape before a modal's back action.
                on_key_with(
                    "escape",
                    KeyOptions {
                        when: When::Always,
                        for_focus: true,
                        ..Default::default()
                    },
                    move |_| state.close(),
                );
            }
            let body = self.body.as_ref().map(|body| {
                scroll()
                    .min_h(px(0.0))
                    .flex_shrink(1.0)
                    .max_h(move || px(window().height.0 * if long_body { 0.5 } else { 0.8 }))
                    .scrollbar(false)
                    .test_id("dialog-body")
                    .child(body())
            });
            let corner = self.close_button.map(|label| {
                let control = Button::new(label)
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::IconSmall)
                    .icon_start(text("×").text_lg().accessibility_elements_hidden(true))
                    .surface()
                    .absolute()
                    .top_4()
                    .when(rtl, |button| button.left_4())
                    .when(!rtl, |button| button.right_4())
                    .element_ref(&close_ref)
                    .test_id("dialog-corner-close");
                DialogClose::render(state, control)
            });
            let panel = view()
                .direction(self.direction)
                .w(move || px((window().width.0 - 32.0).clamp(0.0, width)))
                .max_h(move || px((window().height.0 - 32.0).max(0.0)))
                .p_6()
                .gap_6()
                .rounded(px(24.0))
                .border(px(1.0))
                .border_color(move || theme().border)
                .bg(move || theme().popover)
                .text_color(move || theme().popover_foreground)
                .shadow_xl()
                .focusable(true)
                .element_ref(&panel_ref)
                .on_layout(move |_: &Bounds<Points>| {
                    if !placed.replace(true) && state.open.get_untracked() {
                        panel_ref.focus();
                    }
                })
                .test_id("dialog-panel")
                .child(self.header.render(rtl, corner_close))
                .child(body)
                .child(self.footer.as_ref().map(|footer| footer()))
                .child(corner);
            let before = focus_boundary().on_focus(move || {
                if corner_close {
                    close_ref.focus();
                } else if has_content {
                    wrapping_back.replace(true);
                    end_ref.focus();
                } else {
                    panel_ref.focus();
                }
            });
            let after = focus_boundary().element_ref(&end_ref).on_focus(move || {
                if !wrapping_back.replace(false) {
                    panel_ref.focus();
                }
            });
            let modal = dialog()
                .label(self.header.title.0)
                .animation(ModalAnimation::Fade)
                .close_on_escape(self.escape)
                .close_on_swipe(false)
                .close_on_backdrop_press(false)
                .when(self.escape, |modal| modal.on_cancel(move || state.close()));
            let modal = if let Some(description) = self.header.description.as_ref() {
                modal.accessibility_description(description.0)
            } else {
                modal
            };
            modal
                .child(DialogOverlay::new(state, self.outside_press))
                .child(before)
                .child(
                    view()
                        .absolute()
                        .inset_0()
                        .items_center()
                        .justify_center()
                        .pointer_events(PointerEvents::BoxNone)
                        .child(panel),
                )
                .child(after)
        })
        .into_element()
    }
}

pub(crate) fn focus_boundary() -> View {
    // Native Shell removes accessibility-hidden views from the tab order.
    view()
        .absolute()
        .size(px(1.0))
        .opacity(0.0)
        .pointer_events(PointerEvents::None)
        .focusable(true)
}
