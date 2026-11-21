//! Composable confirmation dialogs with Native Shell modal semantics.

use nsh_base::{dialog, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::ui::{Variant, button, muted};

#[derive(Clone, Copy)]
pub struct AlertDialog {
    pub open: Signal<bool>,
}

impl AlertDialog {
    pub fn new(open: Signal<bool>) -> Self {
        Self { open }
    }
}

pub struct AlertDialogTrigger(View);

impl AlertDialogTrigger {
    pub fn new(dialog: AlertDialog, label: &'static str) -> Self {
        Self(button(label, Variant::Outline, move || {
            dialog.open.set(true)
        }))
    }

    pub fn render(dialog: AlertDialog, trigger: View) -> Self {
        Self(trigger.on_press(move || dialog.open.set(true)))
    }
}

impl IntoElement for AlertDialogTrigger {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertDialogSize {
    #[default]
    Default,
    Small,
}

pub struct AlertDialogContent {
    dialog: AlertDialog,
    header: AlertDialogHeader,
    footer: AlertDialogFooter,
    size: AlertDialogSize,
    direction: LayoutDirection,
}

impl AlertDialogContent {
    pub fn new(dialog: AlertDialog, header: AlertDialogHeader, footer: AlertDialogFooter) -> Self {
        Self {
            dialog,
            header,
            footer,
            size: AlertDialogSize::Default,
            direction: LayoutDirection::Inherit,
        }
    }

    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl IntoElement for AlertDialogContent {
    fn into_element(self) -> AnyElement {
        let open = self.dialog.open;
        let size = self.size;
        let direction = self.direction;
        when(open, move || {
            let desktop = move || size == AlertDialogSize::Default && window().width.0 >= 640.0;
            let (cancel, action) = (self.footer.cancel, &self.footer.action);
            dialog()
                .role(AccessibilityRole::AlertDialog)
                .label(self.header.title.0)
                .accessibility_description(self.header.description.0)
                .animation(ModalAnimation::Fade)
                .close_on_backdrop_press(false)
                .close_on_swipe(false)
                .on_cancel(move || open.set(false))
                .child(
                    view()
                        .absolute()
                        .inset_0()
                        .bg(hsla(0.0, 0.0, 0.0, 0.30))
                        .backdrop_blur(px(8.0))
                        .test_id("alert-dialog-backdrop"),
                )
                .child(
                    view()
                        .absolute()
                        .inset_0()
                        .items_center()
                        .justify_center()
                        .pointer_events(PointerEvents::BoxNone)
                        .child(
                            view()
                                .direction(direction)
                                .w(move || {
                                    px((window().width.0 - 32.0)
                                        .clamp(0.0, if desktop() { 448.0 } else { 320.0 }))
                                })
                                .gap_6()
                                .rounded(px(24.0))
                                .p_6()
                                .border(px(1.0))
                                .border_color(move || theme().border)
                                .bg(move || theme().popover)
                                .text_color(move || theme().popover_foreground)
                                .shadow_lg()
                                .test_id("alert-dialog-panel")
                                .child(self.header.render(desktop, direction))
                                .child(
                                    view()
                                        .gap_2()
                                        .style_when(
                                            move || size == AlertDialogSize::Small,
                                            |s| s.flex_row(),
                                        )
                                        .style_when(desktop, |s| s.flex_row().justify_end())
                                        .style_when(
                                            move || size == AlertDialogSize::Default && !desktop(),
                                            |s| s.flex_col_reverse(),
                                        )
                                        .child(cancel.render(open, size))
                                        .child(action.render(open, size)),
                                ),
                        ),
                )
        })
        .into_element()
    }
}

pub struct AlertDialogHeader {
    title: AlertDialogTitle,
    description: AlertDialogDescription,
    media: Option<AlertDialogMedia>,
}

impl AlertDialogHeader {
    pub fn new(title: AlertDialogTitle, description: AlertDialogDescription) -> Self {
        Self {
            title,
            description,
            media: None,
        }
    }

    pub fn media(mut self, media: AlertDialogMedia) -> Self {
        self.media = Some(media);
        self
    }

    fn render(
        &self,
        desktop: impl Fn() -> bool + Copy + 'static,
        direction: LayoutDirection,
    ) -> View {
        let rtl = direction == LayoutDirection::Rtl;
        let mut layout = view()
            .gap_3()
            .style_when(move || !desktop(), |s| s.items_center())
            .style_when(desktop, |s| s.flex_row().items_start());
        if let Some(media) = self.media {
            layout = layout.child(media);
        }
        layout.child(
            view()
                .gap(px(6.0))
                .min_w(px(0.0))
                .child(
                    text(self.title.0)
                        .text_lg()
                        .font_weight(FontWeight::MEDIUM)
                        .style_when(move || !desktop(), |s| s.text_center())
                        .style_when(move || desktop() && rtl, |s| s.text_right()),
                )
                .child(
                    muted(self.description.0)
                        .style_when(move || !desktop(), |s| s.text_center())
                        .style_when(move || desktop() && rtl, |s| s.text_right()),
                ),
        )
    }
}

pub struct AlertDialogTitle(&'static str);

impl AlertDialogTitle {
    pub fn new(title: &'static str) -> Self {
        Self(title)
    }
}

pub struct AlertDialogDescription(&'static str);

impl AlertDialogDescription {
    pub fn new(description: &'static str) -> Self {
        Self(description)
    }
}

#[derive(Clone, Copy)]
pub struct AlertDialogMedia {
    symbol: &'static str,
    destructive: bool,
}

impl AlertDialogMedia {
    pub fn new(symbol: &'static str) -> Self {
        Self {
            symbol,
            destructive: false,
        }
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }
}

impl IntoElement for AlertDialogMedia {
    fn into_element(self) -> AnyElement {
        view()
            .w(px(64.0))
            .h(px(64.0))
            .flex_shrink_0()
            .rounded_full()
            .bg(move || {
                if self.destructive {
                    theme().destructive.opacity(0.1)
                } else {
                    theme().muted
                }
            })
            .items_center()
            .justify_center()
            .accessibility_elements_hidden(true)
            .child(text(self.symbol).text_size(px(28.0)).text_color(move || {
                if self.destructive {
                    theme().destructive
                } else {
                    theme().foreground
                }
            }))
            .into_element()
    }
}

pub struct AlertDialogFooter {
    cancel: AlertDialogCancel,
    action: AlertDialogAction,
}

impl AlertDialogFooter {
    pub fn new(cancel: AlertDialogCancel, action: AlertDialogAction) -> Self {
        Self { cancel, action }
    }
}

#[derive(Clone, Copy)]
pub struct AlertDialogCancel(&'static str);

impl AlertDialogCancel {
    pub fn new(label: &'static str) -> Self {
        Self(label)
    }

    fn render(self, open: Signal<bool>, size: AlertDialogSize) -> View {
        let cancel = element_ref();
        let focused = stored(false);
        button(self.0, Variant::Outline, move || open.set(false))
            .element_ref(&cancel)
            .on_layout(move |_: &Bounds<Points>| {
                if !focused.replace(true) {
                    cancel.focus();
                }
            })
            .when(size == AlertDialogSize::Small, |button| {
                button.flex_1().min_w(px(0.0))
            })
    }
}

pub struct AlertDialogAction {
    label: &'static str,
    destructive: bool,
    confirm: std::rc::Rc<dyn Fn()>,
}

impl AlertDialogAction {
    pub fn new(label: &'static str, confirm: impl Fn() + 'static) -> Self {
        Self {
            label,
            destructive: false,
            confirm: std::rc::Rc::new(confirm),
        }
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    fn render(&self, open: Signal<bool>, size: AlertDialogSize) -> View {
        let confirm = self.confirm.clone();
        let variant = if self.destructive {
            Variant::Destructive
        } else {
            Variant::Primary
        };
        button(self.label, variant, move || {
            if open.get_untracked() {
                open.set(false);
                confirm();
            }
        })
        .when(size == AlertDialogSize::Small, |button| {
            button.flex_1().min_w(px(0.0))
        })
    }
}
