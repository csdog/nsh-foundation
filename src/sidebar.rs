//! Composable navigation rails with independent desktop and mobile state.

use nsh_base::{theme, tooltip};
use nsh_core::{
    element::{Input, LayoutDirection, open_url},
    keymap::When,
    prelude::*,
    storage::Storage,
};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::use_direction,
    input::input,
    separator::Separator,
    sheet::{Sheet, SheetContent, SheetDescription, SheetHeader, SheetSide, SheetTitle},
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

#[derive(Clone, Copy)]
pub struct SidebarState {
    pub open: Signal<bool>,
    pub open_mobile: Signal<bool>,
    mobile: Sheet,
    trigger: Stored<Option<ElementRef>>,
}

impl SidebarState {
    pub fn is_mobile(self) -> bool {
        window().width.0 < 768.0
    }

    pub fn toggle(self) {
        if self.is_mobile() {
            if self.open_mobile.get_untracked() {
                self.mobile.close();
            } else if let Some(reference) = self.trigger.get() {
                self.mobile.open_from(reference);
            } else {
                self.open_mobile.set(true);
            }
        } else {
            self.open.update(|open| *open = !*open);
        }
    }
}

pub struct SidebarProvider<F> {
    content: F,
    default_open: bool,
    controlled: Option<Signal<bool>>,
    storage_key: Option<&'static str>,
    shortcut: bool,
}

impl<F> SidebarProvider<F> {
    pub fn new(content: F) -> Self {
        Self {
            content,
            default_open: true,
            controlled: None,
            storage_key: None,
            shortcut: true,
        }
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Bind the desktop state to a caller-owned signal.
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.controlled = Some(open);
        self
    }

    /// Store the uncontrolled desktop state in Native Shell storage under this key.
    pub fn persist(mut self, key: &'static str) -> Self {
        self.storage_key = Some(key);
        self
    }

    /// Disable the window's Mod+B binding when multiple sidebars share it.
    pub fn shortcut(mut self, enabled: bool) -> Self {
        self.shortcut = enabled;
        self
    }
}

impl<F, E> IntoElement for SidebarProvider<F>
where
    F: FnOnce(SidebarState) -> E,
    E: IntoElement,
{
    fn into_element(self) -> AnyElement {
        let open = self.controlled.unwrap_or_else(|| {
            let saved = self.storage_key.and_then(Storage::get_item);
            signal(
                saved
                    .as_deref()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(self.default_open),
            )
        });
        if let Some(key) = self.storage_key {
            effect(move || {
                let _ = Storage::set_item(key, open.get().to_string());
            });
        }
        let open_mobile = signal(false);
        let state = SidebarState {
            open,
            open_mobile,
            mobile: Sheet::new(open_mobile),
            trigger: stored(None),
        };
        effect(move || {
            if !state.is_mobile() {
                open_mobile.set(false);
            }
        });
        if self.shortcut {
            on_key_with(
                "mod+b",
                KeyOptions {
                    when: When::Always,
                    for_focus: true,
                    ..Default::default()
                },
                move |_| state.toggle(),
            );
        }
        (self.content)(state).into_element()
    }
}

#[derive(Clone, Copy)]
pub struct SidebarParts {
    icon_only: Memo<bool>,
    side: SidebarSide,
    direction: LayoutDirection,
}

impl SidebarParts {
    pub fn icon_only(self) -> bool {
        self.icon_only.get()
    }

    pub fn expanded_only(self, element: impl IntoElement) -> View {
        view()
            .style_when(move || self.icon_only(), |style| style.hidden())
            .accessibility_elements_hidden(move || self.icon_only())
            .child(element)
    }

    fn inward(self) -> Placement {
        match self.side {
            SidebarSide::Left => Placement::Right,
            SidebarSide::Right => Placement::Left,
        }
    }
}

pub struct Sidebar<F> {
    state: SidebarState,
    content: F,
    side: SidebarSide,
    variant: SidebarVariant,
    collapse: SidebarCollapsible,
    dir: Option<LayoutDirection>,
    widths: [f32; 3],
    rail: bool,
    label: &'static str,
}

impl<F> Sidebar<F>
where
    F: Fn(SidebarParts) -> View + 'static,
{
    pub fn new(state: SidebarState, content: F) -> Self {
        Self {
            state,
            content,
            side: SidebarSide::Left,
            variant: SidebarVariant::Sidebar,
            collapse: SidebarCollapsible::Offcanvas,
            dir: None,
            widths: [256.0, 288.0, 48.0],
            rail: false,
            label: "Sidebar",
        }
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn collapsible(mut self, collapse: SidebarCollapsible) -> Self {
        self.collapse = collapse;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.dir = Some(direction);
        self
    }

    /// Widths for the desktop panel, mobile panel, and icon rail in points.
    pub fn widths(mut self, desktop: f32, mobile: f32, icons: f32) -> Self {
        self.widths = [
            desktop.max(1.0),
            mobile.max(1.0),
            icons.max(1.0).min(desktop.max(1.0)),
        ];
        self
    }

    pub fn rail(mut self, enabled: bool) -> Self {
        self.rail = enabled;
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    /// Place the sidebar beside the main content within the available height.
    pub fn layout(self, main: View) -> View {
        let state = self.state;
        let side = self.side;
        let collapse = self.collapse;
        let variant = self.variant;
        let label = self.label;
        let direction = self.dir.unwrap_or_else(|| use_direction().get().into());
        let [desktop, mobile, icons] = self.widths;
        let static_panel = collapse == SidebarCollapsible::None;
        let mobile_sheet = move || !static_panel && state.is_mobile();
        let icon_only = computed(move || {
            !mobile_sheet() && collapse == SidebarCollapsible::Icon && !state.open.get()
        });
        let desktop_hidden = move || collapse == SidebarCollapsible::Offcanvas && !state.open.get();
        let panel_inset = variant != SidebarVariant::Sidebar;
        let width = move || {
            if desktop_hidden() {
                0.0
            } else if icon_only.get() {
                icons + if panel_inset { 16.0 } else { 0.0 }
            } else {
                desktop
            }
        };
        let displayed = animate_width(width);
        let content = std::rc::Rc::new(self.content);
        let mobile_content = content.clone();
        let parts = SidebarParts {
            icon_only,
            side,
            direction,
        };
        let desktop_part = move || {
            let panel = surface(label, parts, content(parts))
                .when(variant == SidebarVariant::Floating, |panel| {
                    panel
                        .rounded(px(16.0))
                        .border(px(1.0))
                        .border_color(|| theme().sidebar_border)
                        .shadow_sm()
                })
                .when(!panel_inset && side == SidebarSide::Left, |panel| {
                    panel
                        .border_r(px(1.0))
                        .border_color(|| theme().sidebar_border)
                })
                .when(!panel_inset && side == SidebarSide::Right, |panel| {
                    panel
                        .border_l(px(1.0))
                        .border_color(|| theme().sidebar_border)
                });
            let inner = view()
                .absolute()
                .top_0()
                .bottom_0()
                .when(side == SidebarSide::Left, |frame| frame.right_0())
                .when(side == SidebarSide::Right, |frame| frame.left_0())
                .w(move || {
                    px(if collapse == SidebarCollapsible::Icon {
                        displayed.get()
                    } else {
                        desktop
                    })
                })
                .when(panel_inset, |frame| frame.p_2())
                .child(panel);
            view()
                .h_full()
                .flex_shrink_0()
                .overflow_hidden()
                .w(move || px(displayed.get()))
                .test_id("sidebar-gap")
                .accessibility_elements_hidden(desktop_hidden)
                .pointer_events(move || {
                    if desktop_hidden() {
                        PointerEvents::None
                    } else {
                        PointerEvents::Auto
                    }
                })
                .child(inner)
        };
        view()
            .direction(LayoutDirection::Ltr)
            .flex_row()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .size_full()
            .when(side == SidebarSide::Right, |row| row.flex_row_reverse())
            .style_when(
                move || !mobile_sheet() && variant == SidebarVariant::Inset,
                |style| style.bg(|| theme().sidebar),
            )
            .child(when(move || !mobile_sheet(), desktop_part))
            .child(main.direction(direction).style_when(
                move || !mobile_sheet() && variant == SidebarVariant::Inset,
                |style| style.m_2().rounded(px(16.0)).overflow_hidden().shadow_sm(),
            ))
            .child(when(
                move || self.rail && !static_panel && !mobile_sheet(),
                move || {
                    sidebar_rail(state)
                        .when(side == SidebarSide::Left, |rail| {
                            rail.left(move || px((displayed.get() - 4.0).max(0.0)))
                        })
                        .when(side == SidebarSide::Right, |rail| {
                            rail.right(move || px((displayed.get() - 4.0).max(0.0)))
                        })
                },
            ))
            .child(when(mobile_sheet, move || {
                SheetContent::new(
                    state.mobile,
                    SheetHeader::new(SheetTitle::new(label))
                        .description(SheetDescription::new("Displays the mobile sidebar.")),
                )
                .side(match side {
                    SidebarSide::Left => SheetSide::Left,
                    SidebarSide::Right => SheetSide::Right,
                })
                .direction(direction)
                .close_button(None)
                .full_bleed(mobile, {
                    let content = mobile_content.clone();
                    move || surface(label, parts, content(parts))
                })
            }))
    }
}

fn surface(label: &'static str, parts: SidebarParts, body: View) -> View {
    view()
        .role(AccessibilityRole::Navigation)
        .label(label)
        .direction(parts.direction)
        .size_full()
        .min_h(px(0.0))
        .bg(|| theme().sidebar)
        .text_color(|| theme().sidebar_foreground)
        .test_id("sidebar-panel")
        .child(body)
}

fn animate_width(target: impl Fn() -> f32 + Copy + 'static) -> Signal<f32> {
    let visible = signal(untracked(target));
    let running = stored(None::<Spawned<()>>);
    effect(move || {
        if let Some(task) = running.replace(None) {
            task.cancel();
        }
        let end = target();
        let start = visible.get_untracked();
        if reduce_motion() || start == end {
            visible.set(end);
            return;
        }
        running.replace(Some(spawn(async move {
            let first = next_frame().await;
            loop {
                let now = next_frame().await;
                let progress = ((now - first).as_secs_f32() / 0.2).min(1.0);
                visible.set(start + (end - start) * progress);
                if progress >= 1.0 {
                    break;
                }
            }
        })));
    });
    visible
}

pub struct SidebarTrigger(View);

impl SidebarTrigger {
    pub fn new(state: SidebarState) -> Self {
        Self::labelled(state, "Toggle Sidebar")
    }

    pub fn labelled(state: SidebarState, label: &'static str) -> Self {
        Self::directional(state, label, use_direction().get().into())
    }

    pub fn directional(
        state: SidebarState,
        label: &'static str,
        direction: LayoutDirection,
    ) -> Self {
        let rtl = direction == LayoutDirection::Rtl;
        let icon = view()
            .size(px(14.0))
            .h(px(12.0))
            .rounded(px(2.0))
            .border(px(1.0))
            .border_color(|| theme().foreground)
            .accessibility_elements_hidden(true)
            .child(
                view()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .w(px(1.0))
                    .bg(|| theme().foreground)
                    .when(rtl, |line| line.right(px(4.0)))
                    .when(!rtl, |line| line.left(px(4.0))),
            );
        let reference = element_ref();
        state.trigger.replace(Some(reference));
        Self(
            Button::new(label)
                .size(ButtonSize::IconSmall)
                .variant(ButtonVariant::Ghost)
                .icon_start(icon)
                .surface()
                .element_ref(&reference)
                .test_id("sidebar-trigger")
                .accessibility_state(move || AccessibilityState {
                    expanded: Some(if state.is_mobile() {
                        state.open_mobile.get()
                    } else {
                        state.open.get()
                    }),
                    has_popup: state.is_mobile(),
                    ..Default::default()
                })
                .on_press(move || {
                    state.trigger.replace(Some(reference));
                    state.toggle();
                }),
        )
    }
}

impl IntoElement for SidebarTrigger {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub fn sidebar_rail(state: SidebarState) -> View {
    view()
        .absolute()
        .top_0()
        .bottom_0()
        .w(px(8.0))
        .z_index(1)
        .items_center()
        .cursor(Cursor::ColResize)
        .role(AccessibilityRole::Button)
        .label("Toggle Sidebar from rail")
        .focusable(false)
        .hover(|style| style.bg(|| theme().sidebar_border.opacity(0.6)))
        .test_id("sidebar-rail")
        .on_press(move || state.toggle())
}

pub fn sidebar_inset() -> View {
    view().flex_1().min_w(px(0.0)).bg(|| theme().background)
}

pub fn sidebar_header() -> View {
    view().p_2().gap_2().flex_shrink_0()
}

pub fn sidebar_footer() -> View {
    view().p_2().gap_2().flex_shrink_0()
}

pub fn sidebar_content(child: impl IntoElement) -> Scroll {
    scroll()
        .flex_1()
        .min_h(px(0.0))
        .scrollbar(false)
        .test_id("sidebar-content")
        .child(child)
}

pub fn sidebar_input() -> Input {
    input()
}

pub fn sidebar_separator() -> View {
    view()
        .px_2()
        .child(Separator::new().surface().bg(|| theme().sidebar_border))
}

pub fn sidebar_group() -> View {
    view().p_2().min_w(px(0.0))
}

pub fn sidebar_group_label(parts: SidebarParts, label: &'static str) -> View {
    parts.expanded_only(
        view().h(px(32.0)).px_3().justify_center().child(
            text(label)
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(|| theme().sidebar_foreground.opacity(0.7)),
        ),
    )
}

pub fn sidebar_group_content() -> View {
    view().w_full().min_w(px(0.0)).text_sm()
}

pub fn sidebar_group_action(label: &'static str, icon: &'static str) -> View {
    sidebar_menu_action(label, icon, false)
}

pub fn sidebar_menu() -> View {
    view().w_full().min_w(px(0.0)).gap(px(2.0))
}

pub fn sidebar_menu_item() -> View {
    view().w_full().min_w(px(0.0)).group()
}

pub fn sidebar_menu_sub(parts: SidebarParts) -> View {
    view()
        .mx(px(14.0))
        .px(px(10.0))
        .py(px(2.0))
        .gap_1()
        .when(parts.direction == LayoutDirection::Rtl, |sub| {
            sub.border_r(px(1.0))
        })
        .when(parts.direction != LayoutDirection::Rtl, |sub| {
            sub.border_l(px(1.0))
        })
        .border_color(|| theme().sidebar_border)
        .style_when(move || parts.icon_only(), |style| style.hidden())
        .accessibility_elements_hidden(move || parts.icon_only())
}

pub fn sidebar_menu_sub_item() -> View {
    view().min_w(px(0.0))
}

pub fn sidebar_menu_action(label: &'static str, icon: &'static str, show_on_hover: bool) -> View {
    Button::new(label)
        .size(ButtonSize::IconSmall)
        .variant(ButtonVariant::Ghost)
        .icon_start(text(icon).text_base().accessibility_elements_hidden(true))
        .surface()
        .when(show_on_hover, |action| {
            action
                .style_when(
                    move || window().width.0 >= 768.0,
                    |style| style.opacity(0.0),
                )
                .group_hover(|style| style.opacity(1.0))
                .focus_visible(|style| style.opacity(1.0))
        })
}

pub fn sidebar_menu_badge(label: impl IntoReactive<SharedString>) -> View {
    view()
        .min_w(px(20.0))
        .h(px(20.0))
        .px_1()
        .items_center()
        .justify_center()
        .pointer_events(PointerEvents::None)
        .child(text(label).text_xs().font_weight(FontWeight::MEDIUM))
}

pub fn sidebar_menu_skeleton(parts: SidebarParts, icon: bool) -> View {
    let placeholder = || theme().sidebar_accent;
    view()
        .h(px(32.0))
        .px_2()
        .gap_2()
        .flex_row()
        .items_center()
        .accessibility_elements_hidden(true)
        .child(icon.then(|| view().size(px(16.0)).rounded(px(8.0)).bg(placeholder)))
        .child(
            parts.expanded_only(
                view()
                    .h(px(16.0))
                    .w(percentage(0.65))
                    .rounded(px(8.0))
                    .bg(placeholder),
            ),
        )
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarMenuSize {
    Small,
    #[default]
    Default,
    Large,
}

pub struct SidebarMenuButton {
    parts: SidebarParts,
    label: Reactive<SharedString>,
    icon: Option<AnyElement>,
    detail: Option<Reactive<SharedString>>,
    suffix: Option<AnyElement>,
    active: Reactive<bool>,
    size: SidebarMenuSize,
    outline: bool,
}

impl SidebarMenuButton {
    pub fn new(parts: SidebarParts, label: impl IntoReactive<SharedString>) -> Self {
        Self {
            parts,
            label: label.into_reactive(),
            icon: None,
            detail: None,
            suffix: None,
            active: false.into_reactive(),
            size: SidebarMenuSize::Default,
            outline: false,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    pub fn detail(mut self, detail: impl IntoReactive<SharedString>) -> Self {
        self.detail = Some(detail.into_reactive());
        self
    }

    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_element());
        self
    }

    pub fn active(mut self, active: impl IntoReactive<bool>) -> Self {
        self.active = active.into_reactive();
        self
    }

    pub fn size(mut self, size: SidebarMenuSize) -> Self {
        self.size = size;
        self
    }

    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    pub fn surface(self) -> View {
        let parts = self.parts;
        let active = computed(move || self.active.get());
        let (height, font) = match self.size {
            SidebarMenuSize::Small => (28.0, 12.0),
            SidebarMenuSize::Default => (32.0, 14.0),
            SidebarMenuSize::Large => (48.0, 14.0),
        };
        let label = self.label;
        let rtl = parts.direction == LayoutDirection::Rtl;
        let labels = view()
            .flex_1()
            .min_w(px(0.0))
            .child(
                text(label.clone())
                    .text_size(px(font))
                    .line_height(px(font + 2.0))
                    .truncate()
                    .when(rtl, |caption| caption.text_right())
                    .style_when(active, |style| style.font_weight(FontWeight::MEDIUM)),
            )
            .child(self.detail.map(|detail| {
                text(detail)
                    .text_xs()
                    .line_height(px(14.0))
                    .truncate()
                    .when(rtl, |caption| caption.text_right())
            }));
        view()
            .w_full()
            .min_w(px(0.0))
            .h(px(height))
            .gap_2()
            .px(px(10.0))
            .flex_row()
            .items_center()
            .rounded(px(12.0))
            .cursor_pointer()
            .role(AccessibilityRole::Button)
            .focusable(true)
            .label(label.clone())
            .accessibility_state(move || AccessibilityState {
                selected: active.get(),
                ..Default::default()
            })
            .when(self.outline, |row| {
                row.bg(|| theme().background)
                    .border(px(1.0))
                    .border_color(|| theme().sidebar_border)
            })
            .style_when(active, |style| {
                style
                    .bg(|| theme().sidebar_accent)
                    .text_color(|| theme().sidebar_accent_foreground)
            })
            .hover(|style| {
                style
                    .bg(|| theme().sidebar_accent)
                    .text_color(|| theme().sidebar_accent_foreground)
            })
            .focus_visible(|style| style.ring(px(3.0)).ring_color(|| theme().sidebar_ring))
            .style_when(
                move || parts.icon_only(),
                |style| style.size(px(32.0)).px_0().gap_0().justify_center(),
            )
            .tooltip(
                tooltip()
                    .disabled(move || !parts.icon_only())
                    .placement(parts.inward())
                    .app_class("rounded-lg px-2 py-1 bg-foreground text-background text-xs")
                    .child(text(label).text_xs()),
            )
            .child(self.icon.map(|icon| {
                view()
                    .size(px(if height == 48.0 { 32.0 } else { 16.0 }))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .accessibility_elements_hidden(true)
                    .child(icon)
            }))
            .child(labels.style_when(move || parts.icon_only(), |style| style.hidden()))
            .child(self.suffix.map(|suffix| parts.expanded_only(suffix)))
    }

    pub fn link(self, url: &'static str) -> View {
        self.surface()
            .role(AccessibilityRole::Link)
            .on_press(move || {
                let _ = open_url(url);
            })
    }
}

pub fn sidebar_menu_sub_button(parts: SidebarParts, label: &'static str) -> SidebarMenuButton {
    SidebarMenuButton::new(parts, label).size(SidebarMenuSize::Small)
}
