//! Rhea edge-aligned modal panels built on Native Shell's dialog host.

use nsh_base::{dialog, theme};
use nsh_core::{element::LayoutDirection, keymap::When, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    dialog::focus_boundary,
    direction::use_direction,
    ui::muted,
};

const PANEL_MS: f32 = 200.0;
const OVERLAY_MS: f32 = 150.0;
const TRANSITION: Easing = Easing::Bezier(0.42, 0.0, 0.58, 1.0);
const OVERLAY_TRANSITION: Easing = Easing::Bezier(0.4, 0.0, 0.2, 1.0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// Shared state for a sheet and its triggers.
#[derive(Clone, Copy)]
pub struct Sheet {
    pub open: Signal<bool>,
    opener: Stored<Option<ElementRef>>,
}

impl Sheet {
    pub fn new(open: Signal<bool>) -> Self {
        Self {
            open,
            opener: stored(None),
        }
    }

    pub fn close(self) {
        self.open.set(false);
    }

    pub(crate) fn open_from(self, reference: ElementRef) {
        self.opener.replace(Some(reference));
        self.open.set(true);
    }
}

pub struct SheetTrigger(View);

impl SheetTrigger {
    pub fn new(sheet: Sheet, label: &'static str) -> Self {
        Self::render(
            sheet,
            Button::new(label).variant(ButtonVariant::Outline).surface(),
        )
    }

    pub fn render(sheet: Sheet, control: View) -> Self {
        let reference = element_ref();
        Self(
            control
                .role(AccessibilityRole::Button)
                .element_ref(&reference)
                .accessibility_state(move || AccessibilityState {
                    expanded: Some(sheet.open.get()),
                    has_popup: true,
                    ..Default::default()
                })
                .on_press(move || sheet.open_from(reference)),
        )
    }

    pub fn test_id(self, id: &'static str) -> Self {
        Self(self.0.test_id(id))
    }
}

impl IntoElement for SheetTrigger {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct SheetClose(View);

impl SheetClose {
    pub fn new(sheet: Sheet, label: &'static str) -> Self {
        Self::render(
            sheet,
            Button::new(label).variant(ButtonVariant::Outline).surface(),
        )
    }

    pub fn render(sheet: Sheet, control: View) -> Self {
        Self(
            control
                .role(AccessibilityRole::Button)
                .on_press(move || sheet.close()),
        )
    }
}

impl IntoElement for SheetClose {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct SheetTitle(&'static str);

impl SheetTitle {
    pub fn new(title: &'static str) -> Self {
        Self(title)
    }
}

pub struct SheetDescription(&'static str);

impl SheetDescription {
    pub fn new(description: &'static str) -> Self {
        Self(description)
    }
}

pub struct SheetHeader {
    title: SheetTitle,
    description: Option<SheetDescription>,
}

impl SheetHeader {
    pub fn new(title: SheetTitle) -> Self {
        Self {
            title,
            description: None,
        }
    }

    pub fn description(mut self, description: SheetDescription) -> Self {
        self.description = Some(description);
        self
    }

    fn render(&self, rtl: bool, corner: bool) -> View {
        view()
            .flex_shrink_0()
            .p_6()
            .gap(px(6.0))
            .child(
                text(self.title.0)
                    .text_base()
                    .font_weight(FontWeight::MEDIUM)
                    .role(AccessibilityRole::Header)
                    .when(corner && rtl, |title| title.pl(px(28.0)))
                    .when(corner && !rtl, |title| title.pr(px(28.0)))
                    .when(rtl, |title| title.text_right()),
            )
            .child(self.description.as_ref().map(|description| {
                muted(description.0).when(rtl, |description| description.text_right())
            }))
    }
}

pub struct SheetFooter(View);

impl SheetFooter {
    pub fn new() -> Self {
        Self(view().mt(auto()).flex_shrink_0().gap_2().p_6())
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.0 = self.0.child(child);
        self
    }
}

impl Default for SheetFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for SheetFooter {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct SheetContent {
    sheet: Sheet,
    header: SheetHeader,
    body: Option<Box<dyn Fn() -> View>>,
    footer: Option<Box<dyn Fn() -> SheetFooter>>,
    side: SheetSide,
    direction: LayoutDirection,
    close_label: Option<&'static str>,
    max_height_fraction: f32,
    flush_width: Option<f32>,
}

impl SheetContent {
    pub fn new(sheet: Sheet, header: SheetHeader) -> Self {
        Self {
            sheet,
            header,
            body: None,
            footer: None,
            side: SheetSide::Right,
            direction: LayoutDirection::Inherit,
            close_label: Some("Close"),
            max_height_fraction: 1.0,
            flush_width: None,
        }
    }

    /// Physical edge of the viewport, independent of the text direction.
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the accessible label for the corner button. None hides the button.
    pub fn close_button(mut self, label: Option<&'static str>) -> Self {
        self.close_label = label;
        self
    }

    /// Caps a top or bottom panel to a fraction of the viewport height.
    pub fn max_height_fraction(mut self, fraction: f32) -> Self {
        if fraction.is_finite() {
            self.max_height_fraction = fraction.clamp(0.0, 1.0);
        }
        self
    }

    pub fn body(mut self, body: impl Fn() -> View + 'static) -> Self {
        self.body = Some(Box::new(body));
        self
    }

    /// Side panels with their own full-height layout keep their intrinsic chrome.
    pub(crate) fn full_bleed(mut self, width: f32, body: impl Fn() -> View + 'static) -> Self {
        self.flush_width = Some(width);
        self.body(body)
    }

    pub fn footer(mut self, footer: impl Fn() -> SheetFooter + 'static) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    fn modal(&self, mounted: Signal<bool>, direction: LayoutDirection) -> AnyElement {
        let sheet = self.sheet;
        let rtl = direction == LayoutDirection::Rtl;
        let side = self.side;
        let hidden_offset = match side {
            SheetSide::Top | SheetSide::Left => -40.0,
            SheetSide::Right | SheetSide::Bottom => 40.0,
        };
        let offset = animated_value(hidden_offset);
        let panel_fade = animated_value(0.0);
        let overlay_fade = animated_value(0.0);
        let pending_exit = stored(None::<Spawned<()>>);
        effect(move || {
            if let Some(task) = pending_exit.replace(None) {
                task.cancel();
            }
            let open = sheet.open.get();
            let translation = if open { 0.0 } else { hidden_offset };
            let opacity = if open { 1.0 } else { 0.0 };
            if reduce_motion() {
                offset.set(translation);
                panel_fade.set(opacity);
                overlay_fade.set(opacity);
                if !open {
                    mounted.set(false);
                }
                return;
            }

            let _ = offset.timing(translation, PANEL_MS, TRANSITION);
            let _ = overlay_fade.timing(opacity, OVERLAY_MS, OVERLAY_TRANSITION);
            let animation = panel_fade.timing(opacity, PANEL_MS, TRANSITION);
            if !open {
                pending_exit.replace(Some(spawn(async move {
                    if animation.await && !sheet.open.get_untracked() {
                        mounted.set(false);
                    }
                })));
            }
        });

        // The native input can handle Escape before the host's back action.
        on_key_with(
            "escape",
            KeyOptions {
                when: When::Always,
                for_focus: true,
                ..Default::default()
            },
            move |_| sheet.close(),
        );
        let overlay = view()
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.30))
            .backdrop_blur(px(8.0))
            .bind_opacity(&overlay_fade)
            .focusable(false)
            .on_press(move || sheet.close())
            .test_id("sheet-overlay");

        let panel_ref = element_ref();
        let close_ref = element_ref();
        let end_ref = element_ref();
        let placed = stored(false);
        let wrapping_back = stored(false);
        let has_corner = self.close_label.is_some();
        let has_content = self.body.is_some() || self.footer.is_some();
        let before = focus_boundary().on_focus(move || {
            if has_corner {
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
        let corner = self.close_label.map(|label| {
            SheetClose::render(
                sheet,
                Button::new(label)
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::IconSmall)
                    .icon_start(text("×").text_lg().accessibility_elements_hidden(true))
                    .surface()
                    .absolute()
                    .top_4()
                    .when(rtl, |button| button.left_4())
                    .when(!rtl, |button| button.right_4())
                    .element_ref(&close_ref)
                    .test_id("sheet-corner-close"),
            )
        });
        let flush_width = self.flush_width;
        let body = self.body.as_ref().map(|body| {
            if flush_width.is_some() {
                body().into_element()
            } else {
                scroll()
                    .min_h(px(0.0))
                    .flex_shrink(1.0)
                    .scrollbar(false)
                    .test_id("sheet-body")
                    .child(body().px_4())
                    .into_element()
            }
        });
        let fraction = self.max_height_fraction;
        let panel = view()
            .direction(direction)
            .bg(move || theme().popover)
            .text_color(move || theme().popover_foreground)
            .border_color(move || theme().border)
            .shadow_xl()
            .focusable(true)
            .element_ref(&panel_ref)
            .on_layout(move |_: &Bounds<Points>| {
                if !placed.replace(true) && sheet.open.get_untracked() {
                    panel_ref.focus();
                }
            })
            .bind_opacity(&panel_fade)
            .test_id("sheet-panel")
            .child(
                flush_width
                    .is_none()
                    .then(|| self.header.render(rtl, has_corner)),
            )
            .child(body)
            .child(self.footer.as_ref().map(|footer| footer()))
            .child(corner);
        let panel = match side {
            SheetSide::Top | SheetSide::Bottom => panel
                .w_full()
                .max_h(move || px(window().height.0 * fraction))
                .bind_translate_y(&offset)
                .when(side == SheetSide::Top, |panel| panel.border_b(px(1.0)))
                .when(side == SheetSide::Bottom, |panel| panel.border_t(px(1.0))),
            SheetSide::Left | SheetSide::Right => panel
                .h_full()
                .w(move || {
                    let width = window().width.0;
                    px(flush_width.map_or_else(
                        || {
                            if width >= 640.0 {
                                (width * 0.75).min(384.0)
                            } else {
                                width * 0.75
                            }
                        },
                        |requested| requested.min(width),
                    ))
                })
                .bind_translate_x(&offset)
                .when(flush_width.is_none() && side == SheetSide::Left, |panel| {
                    panel.border_r(px(1.0))
                })
                .when(flush_width.is_none() && side == SheetSide::Right, |panel| {
                    panel.border_l(px(1.0))
                }),
        };
        let position = view()
            .direction(LayoutDirection::Ltr)
            .absolute()
            .inset_0()
            .pointer_events(PointerEvents::BoxNone);
        let position = match side {
            SheetSide::Top => position.justify_start(),
            SheetSide::Right => position.items_end(),
            SheetSide::Bottom => position.justify_end(),
            SheetSide::Left => position.items_start(),
        };
        let modal = dialog()
            .label(self.header.title.0)
            .animation(ModalAnimation::None)
            .close_on_escape(false)
            .close_on_swipe(false)
            .close_on_backdrop_press(false)
            .on_cancel(move || sheet.close());
        let modal = match self.header.description.as_ref() {
            Some(description) => modal.accessibility_description(description.0),
            None => modal,
        };
        modal
            .child(overlay)
            .child(before)
            .child(position.child(panel))
            .child(after)
            .into_element()
    }
}

impl IntoElement for SheetContent {
    fn into_element(self) -> AnyElement {
        let mounted = signal(false);
        let sheet = self.sheet;
        effect(move || {
            if sheet.open.get() {
                mounted.set(true);
            }
        });
        effect(move || {
            if !mounted.get() {
                if let Some(opener) = sheet.opener.replace(None) {
                    spawn(async move {
                        next_frame().await;
                        if !sheet.open.get_untracked() {
                            opener.focus();
                        }
                    });
                }
            }
        });
        let inherited = use_direction();
        when(mounted, move || {
            let direction = match self.direction {
                LayoutDirection::Inherit => inherited.get().into(),
                direction => direction,
            };
            self.modal(mounted, direction)
        })
        .into_element()
    }
}
