//! Composable Native Shell Drawer parts sharing one controlled Rust state.

use nsh_base::{dialog, theme};
use nsh_core::{TransformOrigin, element::ElNode, prelude::*};

use crate::ui::{Variant, button_surface, heading, muted};

const OPEN_MS: f32 = 450.0;
const CLOSE_MS: f32 = 400.0;
const PANEL_EASING: Easing = Easing::Bezier(0.22, 1.0, 0.36, 1.0);
const OVERLAY_EASING: Easing = Easing::Bezier(0.32, 0.72, 0.0, 1.0);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    #[default]
    Down,
    Left,
}

impl Direction {
    fn vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }

    fn sign(self) -> f32 {
        match self {
            Self::Up | Self::Left => -1.0,
            Self::Down | Self::Right => 1.0,
        }
    }

    fn movement(self, event: &PanEvent) -> (f32, f32) {
        let (distance, velocity) = if self.vertical() {
            (event.translation.y.0, event.velocity_y)
        } else {
            (event.translation.x.0, event.velocity_x)
        };
        (distance * self.sign(), velocity * self.sign())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapPoint {
    Fraction(f32),
    Pixels(f32),
    Rem(f32),
}

impl SnapPoint {
    fn resolve(self, viewport: f32) -> f32 {
        match self {
            Self::Fraction(fraction) => viewport * fraction,
            Self::Pixels(points) => points,
            Self::Rem(rem) => rem * 16.0,
        }
        .clamp(0.0, viewport)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Modality {
    #[default]
    Modal,
    NonModal,
}

#[derive(Debug, Clone, Copy)]
pub struct DrawerOptions {
    pub direction: Direction,
    pub modality: Modality,
    pub show_swipe_handle: bool,
    pub disable_outside_dismissal: bool,
    /// Vertical size override in points.
    pub height: Option<f32>,
    /// Horizontal size override in points.
    pub width: Option<f32>,
    /// Gap between the panel and viewport edges in points. Defaults to 8.
    pub inset: f32,
    pub snap_points: &'static [SnapPoint],
    /// Present this surface as a centered, non-draggable dialog.
    pub centered: bool,
}

impl Default for DrawerOptions {
    fn default() -> Self {
        Self {
            direction: Direction::default(),
            modality: Modality::default(),
            show_swipe_handle: false,
            disable_outside_dismissal: false,
            height: None,
            width: None,
            inset: 8.0,
            snap_points: &[],
            centered: false,
        }
    }
}

/// A Rust signal that a trigger opens and the Drawer updates on dismissal.
#[derive(Clone, Copy)]
pub struct Drawer {
    pub open: Signal<bool>,
    pub snap_index: Signal<usize>,
    showing_child: Signal<bool>,
    parent_stack: Option<Signal<bool>>,
}

impl Drawer {
    pub fn new(open: Signal<bool>) -> Self {
        Self {
            open,
            snap_index: signal(0),
            showing_child: signal(false),
            parent_stack: None,
        }
    }

    pub fn nested(parent: Drawer) -> Self {
        Self {
            parent_stack: Some(parent.showing_child),
            ..Self::new(signal(false))
        }
    }

    pub fn open(self) {
        self.snap_index.set(0);
        self.open.set(true);
    }

    pub fn close(self) {
        self.open.set(false);
    }

    #[deprecated(note = "Use DrawerContent::new(drawer, label, body).options(options)")]
    pub fn content(
        self,
        label: &'static str,
        options: impl Fn() -> DrawerOptions + 'static,
        body: impl Fn() -> View + 'static,
    ) -> impl IntoElement {
        DrawerContent::new(self, label, body).options(options)
    }
}

type DrawerBody = Box<dyn Fn() -> View>;
type DrawerConfiguration = Box<dyn Fn() -> DrawerOptions>;
type SwipeHandleStyle = Box<dyn Fn(DrawerSwipeHandle) -> DrawerSwipeHandle>;

/// The popup. Its body is built when the drawer opens and released after it exits.
pub struct DrawerContent {
    drawer: Drawer,
    label: &'static str,
    options: DrawerConfiguration,
    body: DrawerBody,
    handle_style: Option<SwipeHandleStyle>,
}

impl DrawerContent {
    pub fn new(drawer: Drawer, label: &'static str, body: impl Fn() -> View + 'static) -> Self {
        Self {
            drawer,
            label,
            options: Box::new(DrawerOptions::default),
            body: Box::new(body),
            handle_style: None,
        }
    }

    /// Supplies options each time the content opens.
    pub fn options(mut self, options: impl Fn() -> DrawerOptions + 'static) -> Self {
        self.options = Box::new(options);
        self
    }

    /// Styles the interactive handle supplied by `show_swipe_handle`.
    pub fn swipe_handle(
        mut self,
        style: impl Fn(DrawerSwipeHandle) -> DrawerSwipeHandle + 'static,
    ) -> Self {
        self.handle_style = Some(Box::new(style));
        self
    }
}

impl IntoElement for DrawerContent {
    fn into_element(self) -> AnyElement {
        let drawer = self.drawer;
        let mounted = signal(false);
        effect(move || {
            if drawer.open.get() {
                mounted.set(true);
            }
        });
        if let Some(parent) = drawer.parent_stack {
            effect(move || parent.set(mounted.get()));
        }
        when(mounted, move || {
            let options = (self.options)();
            let ready = signal(options.centered);
            let close_ms = stored(CLOSE_MS);
            let surface = drawer.surface(
                options,
                self.label,
                (self.body)(),
                self.handle_style.as_deref(),
                ready,
                close_ms,
                move || mounted.set(false),
            );
            DrawerPortal::new(drawer, self.label, options.modality)
                .child(DrawerOverlay::with_motion(drawer, options, ready, close_ms))
                .child(surface)
        })
        .into_element()
    }
}

/// The window layer used by DrawerContent. Mount it only while its contents are visible.
pub struct DrawerPortal {
    drawer: Drawer,
    label: &'static str,
    modality: Modality,
    children: Vec<AnyElement>,
}

impl DrawerPortal {
    pub fn new(drawer: Drawer, label: &'static str, modality: Modality) -> Self {
        Self {
            drawer,
            label,
            modality,
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }
}

impl IntoElement for DrawerPortal {
    fn into_element(self) -> AnyElement {
        let drawer = self.drawer;
        match self.modality {
            Modality::Modal => dialog()
                .animation(ModalAnimation::None)
                .close_on_escape(false)
                .close_on_backdrop_press(false)
                .close_on_swipe(false)
                .label(self.label)
                .on_cancel(move || drawer.close())
                .children(self.children)
                .into_element(),
            Modality::NonModal => {
                on_key("escape", move |_| drawer.close());
                view()
                    .absolute()
                    .inset_0()
                    .pointer_events(PointerEvents::BoxNone)
                    .children(self.children)
                    .into_element()
            }
        }
    }
}

impl DrawerOverlay {
    pub fn new(drawer: Drawer, options: DrawerOptions) -> Self {
        Self::with_motion(drawer, options, signal(true), stored(CLOSE_MS))
    }

    fn with_motion(
        drawer: Drawer,
        options: DrawerOptions,
        ready: Signal<bool>,
        close_ms: Stored<f32>,
    ) -> Self {
        let modal = options.modality == Modality::Modal;
        let fade = animated_value(0.0);
        effect(move || {
            if !ready.get() {
                return;
            }
            let open = drawer.open.get();
            let target = if modal && open { 1.0 } else { 0.0 };
            if reduce_motion() {
                fade.set(target);
            } else {
                let duration = if open { OPEN_MS } else { close_ms.get() };
                let _ = fade.timing(target, duration, OVERLAY_EASING);
            }
        });
        let layer = view()
            .absolute()
            .inset_0()
            .focusable(false)
            .pointer_events(move || {
                if drawer.open.get() && (modal || !options.disable_outside_dismissal) {
                    PointerEvents::Auto
                } else {
                    PointerEvents::None
                }
            })
            .when(modal, |layer| {
                // shadcn/ui's Rhea drawer overlay uses bg-black/30 and backdrop-blur-sm.
                layer.bg(hsla(0.0, 0.0, 0.0, 0.30)).backdrop_blur(px(8.0))
            })
            .bind_opacity(&fade)
            .test_id("drawer-backdrop");
        let layer = if !options.disable_outside_dismissal {
            layer.on_press(move || drawer.close())
        } else {
            layer
        };
        Self(layer)
    }
}

impl Drawer {
    fn surface(
        self,
        options: DrawerOptions,
        title: &'static str,
        children: View,
        handle_style: Option<&dyn Fn(DrawerSwipeHandle) -> DrawerSwipeHandle>,
        ready: Signal<bool>,
        close_ms: Stored<f32>,
        on_exit: impl Fn() + Copy + 'static,
    ) -> View {
        let direction = options.direction;
        let vertical = direction.vertical();
        let viewport = untracked(window);
        let viewport_extent = if vertical {
            viewport.height.0
        } else {
            viewport.width.0
        };
        let offset = animated_value(viewport_extent * direction.sign());
        let measured_extent = signal(None::<f32>);
        let started = stored(false);
        let moving = stored(false);
        let exit = stored(None::<Spawned<()>>);
        let snapped = vertical && !options.snap_points.is_empty() && !options.centered;
        let extent = move || {
            let available = (window().height.0 - 2.0 * options.inset).max(0.0);
            let maximum = options
                .snap_points
                .iter()
                .map(|point| point.resolve(available))
                .fold(0.0_f32, f32::max);
            if snapped { maximum } else { available }
        };
        let position = move |index: usize| {
            if snapped {
                let viewport = (window().height.0 - 2.0 * options.inset).max(0.0);
                let index = index.min(options.snap_points.len() - 1);
                (extent() - options.snap_points[index].resolve(viewport)) * direction.sign()
            } else {
                0.0
            }
        };

        let fade = animated_value(0.0);
        let scale = animated_value(1.0);
        effect(move || {
            if let Some(task) = exit.replace(None) {
                task.cancel();
            }
            let open = self.open.get();
            if open {
                close_ms.replace(CLOSE_MS);
            }
            if !open && (reduce_motion() || !ready.get_untracked()) {
                on_exit();
                return;
            }
            let duration = if open { OPEN_MS } else { close_ms.get() };
            let animation = if options.centered {
                let target = if open { 1.0 } else { 0.0 };
                if reduce_motion() {
                    fade.set(target);
                    return;
                }
                fade.timing(target, duration, OVERLAY_EASING)
            } else {
                let closed_position =
                    |length: f32| (length + options.inset + 2.0) * direction.sign();
                if !started.get() {
                    let Some(length) = measured_extent.get() else {
                        return;
                    };
                    started.replace(true);
                    offset.set(closed_position(length));
                    ready.set(true);
                }
                if open && moving.get() {
                    return;
                }
                let behind = self.showing_child.get();
                let scale_target = if behind { 0.95 } else { 1.0 };
                let target = if open {
                    position(self.snap_index.get())
                        - if behind { 16.0 * direction.sign() } else { 0.0 }
                } else {
                    closed_position(measured_extent.get_untracked().unwrap_or(viewport_extent))
                };
                if reduce_motion() {
                    scale.set(scale_target);
                    offset.set(target);
                    return;
                }
                let _ = scale.timing(scale_target, duration, PANEL_EASING);
                offset.timing(target, duration, PANEL_EASING)
            };
            if !open {
                exit.replace(Some(spawn(async move {
                    if animation.await && !self.open.get_untracked() {
                        on_exit();
                    }
                })));
            }
        });

        let radius = px(24.0);
        let inset = px(options.inset);
        let mut panel = view()
            .absolute()
            .pointer_events(PointerEvents::Auto)
            .bg(move || theme().popover)
            .text_color(move || theme().popover_foreground)
            .border(px(1.0))
            .border_color(move || theme().border)
            .shadow_xl()
            .overflow_hidden()
            .bind_scale(&scale)
            .test_id("drawer-panel");

        if options.modality == Modality::NonModal {
            panel = panel.role(AccessibilityRole::Dialog).label(title);
        }

        if options.centered {
            let width = || (window().width.0 - 32.0).clamp(0.0, 425.0);
            return container_centered(
                panel
                    .w(move || px(width()))
                    .rounded(radius)
                    .bind_opacity(&fade)
                    .child(children),
            );
        }

        let origin = match direction {
            Direction::Up => "center top",
            Direction::Down => "center bottom",
            Direction::Left => "left center",
            Direction::Right => "right center",
        };
        panel = panel.transform_origin(TransformOrigin::parse(origin).unwrap());
        panel = panel.on_layout(move |bounds: &Bounds<Points>| {
            let length = if vertical {
                bounds.size.height.0
            } else {
                bounds.size.width.0
            };
            if length > 0.0 {
                measured_extent.set(Some(length));
            }
        });

        panel = match direction {
            Direction::Up => panel.top(inset).left(inset).right(inset),
            Direction::Down => panel.bottom(inset).left(inset).right(inset),
            Direction::Left => panel.left(inset).top(inset).bottom(inset),
            Direction::Right => panel.right(inset).top(inset).bottom(inset),
        };

        panel = panel.rounded(radius);

        if vertical {
            panel = panel.max_h(move || {
                px(if snapped || options.height.is_some() {
                    extent()
                } else {
                    (extent() - 96.0).max(0.0)
                })
            });
            if snapped || options.height.is_some() {
                panel = panel.h(move || px(options.height.unwrap_or_else(extent)));
            }
            panel = panel.bind_translate_y(&offset);
        } else {
            panel = panel
                .w(move || {
                    let viewport = window().width.0;
                    let default_width = if viewport >= 640.0 {
                        384.0
                    } else {
                        viewport * 0.75
                    };
                    px(options
                        .width
                        .unwrap_or(default_width)
                        .min((viewport - options.inset * 2.0).max(0.0)))
                })
                .bind_translate_x(&offset);
        }

        let drag = |surface: View| {
            surface
                .pan_enabled(move || self.open.get())
                .pan_values(
                    if vertical { None } else { Some(&offset) },
                    if vertical { Some(&offset) } else { None },
                )
                .pan_scroll_handoff(direction == Direction::Down)
                .on_pan_start(move |_: &PanEvent| {
                    moving.replace(true);
                })
                .on_pan_end(move |event: &PanEvent| {
                    moving.replace(false);
                    if !self.open.get_untracked() {
                        return;
                    }
                    let current = self.snap_index.get();
                    let (distance, velocity) = direction.movement(event);
                    let next = if event.cancelled {
                        Some(current)
                    } else if snapped {
                        let viewport = (window().height.0 - 2.0 * options.inset).max(0.0);
                        settle_snap(options.snap_points, viewport, current, distance, velocity)
                    } else if distance > 96.0 || (distance > 12.0 && velocity > 750.0) {
                        None
                    } else {
                        Some(current)
                    };
                    match next {
                        Some(index) if index == current => {
                            let target = position(index);
                            if reduce_motion() {
                                offset.set(target);
                            } else {
                                let _ = offset.timing(target, OPEN_MS, PANEL_EASING);
                            }
                        }
                        Some(index) => self.snap_index.set(index),
                        None => {
                            let length = measured_extent.get_untracked().unwrap_or(viewport_extent);
                            let remaining = (length + options.inset + 2.0
                                - position(current).abs()
                                - distance.max(0.0))
                            .max(0.0);
                            close_ms.replace(
                                (remaining / velocity.max(1.0) * 1000.0).clamp(160.0, CLOSE_MS),
                            );
                            self.close();
                        }
                    }
                })
        };

        if options.show_swipe_handle {
            let handle = DrawerSwipeHandle(drag(swipe_handle(vertical, move || {
                if snapped {
                    self.snap_index
                        .update(|index| *index = (*index + 1) % options.snap_points.len());
                }
            })));
            let handle = match handle_style {
                Some(style) => style(handle),
                None => handle,
            };
            if vertical {
                panel.child(handle).child(children)
            } else {
                panel.child(
                    view()
                        .flex_1()
                        .flex_row()
                        .child(handle)
                        .child(children.flex_1()),
                )
            }
        } else {
            drag(panel).child(children)
        }
    }
}

fn container_centered(panel: View) -> View {
    // The modal fills the root window and lays out its child in the center.
    view()
        .absolute()
        .inset_0()
        .pointer_events(PointerEvents::BoxNone)
        .items_center()
        .justify_center()
        .child(panel.relative())
}

fn settle_snap(
    points: &[SnapPoint],
    viewport: f32,
    index: usize,
    distance: f32,
    velocity: f32,
) -> Option<usize> {
    let current = points
        .get(index)
        .or_else(|| points.first())?
        .resolve(viewport);
    let moved = current - distance;
    let lowest = points
        .iter()
        .map(|point| point.resolve(viewport))
        .fold(viewport, f32::min);
    if moved < lowest && (lowest - moved > lowest * 0.27 || velocity > 750.0) {
        return None;
    }
    let projected = moved - velocity.clamp(-1800.0, 1800.0) * 0.14;
    points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (a.resolve(viewport) - projected)
                .abs()
                .total_cmp(&(b.resolve(viewport) - projected).abs())
        })
        .map(|(index, _)| index)
}

// These parts use the underlying Native Shell node directly, so composing
// them adds no wrappers and keeps the standard Rust style methods available.
macro_rules! drawer_part {
    ($( $(#[$meta:meta])* $name:ident($node:ty); )+) => {
        $(
            $(#[$meta])*
            pub struct $name($node);

            impl IntoElement for $name {
                fn into_element(self) -> AnyElement {
                    self.0.into_element()
                }
            }

            impl Element for $name {
                fn node(&mut self) -> &mut ElNode {
                    self.0.node()
                }
            }
        )+
    };
}

drawer_part! {
    /// A button that opens its associated drawer.
    DrawerTrigger(View);
    /// A button that closes its associated drawer.
    DrawerClose(View);
    /// Layout for the title and optional description.
    DrawerHeader(View);
    /// The heading displayed in the drawer.
    DrawerTitle(Text);
    /// Supporting text displayed below the title.
    DrawerDescription(Text);
    /// Layout for drawer actions.
    DrawerFooter(View);
    /// The backdrop normally created by DrawerContent.
    DrawerOverlay(View);
    /// The interactive drag target supplied to DrawerContent's handle styling hook.
    DrawerSwipeHandle(View);
}

impl DrawerTrigger {
    pub fn new(drawer: Drawer, label: &'static str) -> Self {
        Self::render(drawer, button_surface(label, Variant::Outline))
    }

    /// Uses the supplied control as the trigger, with no extra node.
    pub fn render(drawer: Drawer, control: View) -> Self {
        Self(
            control
                .role(AccessibilityRole::Button)
                .accessibility_state(move || AccessibilityState {
                    expanded: Some(drawer.open.get()),
                    ..Default::default()
                })
                .on_press(move || drawer.open()),
        )
    }

    pub fn test_id(self, id: &str) -> Self {
        Self(self.0.test_id(id))
    }
}

impl DrawerClose {
    pub fn new(drawer: Drawer, label: &'static str) -> Self {
        Self::render(drawer, button_surface(label, Variant::Primary))
    }

    /// Uses the supplied control as the close button, with no extra node.
    pub fn render(drawer: Drawer, control: View) -> Self {
        Self(
            control
                .role(AccessibilityRole::Button)
                .on_press(move || drawer.close()),
        )
    }
}

impl DrawerHeader {
    pub fn new() -> Self {
        Self(
            view()
                .flex_shrink_0()
                .px_4()
                .pt_4()
                .gap(move || px(if window().width.0 >= 768.0 { 6.0 } else { 2.0 })),
        )
    }
}

impl Default for DrawerHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DrawerHeader {}

impl DrawerTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(heading(title))
    }
}

impl DrawerDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self(muted(description))
    }
}

impl DrawerFooter {
    pub fn new() -> Self {
        Self(view().flex_shrink_0().px_4().pb_4().gap_2())
    }
}

impl Default for DrawerFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DrawerFooter {}

#[deprecated(note = "Use DrawerTrigger::new")]
pub fn trigger(drawer: Drawer, label: &'static str) -> View {
    DrawerTrigger::new(drawer, label).0
}

#[deprecated(note = "Use DrawerClose::new")]
pub fn close(drawer: Drawer, label: &'static str) -> View {
    DrawerClose::new(drawer, label).0
}

#[deprecated(note = "Compose DrawerHeader with DrawerTitle and DrawerDescription")]
pub fn header(title: &'static str, description: &'static str) -> View {
    DrawerHeader::new()
        .child(DrawerTitle::new(title))
        .child((!description.is_empty()).then(|| DrawerDescription::new(description)))
        .0
}

#[deprecated(note = "Use DrawerFooter::new")]
pub fn footer() -> View {
    DrawerFooter::new().0
}

fn swipe_handle(vertical: bool, on_press: impl Fn() + 'static) -> View {
    let (width, height, mark_width, mark_height) = if vertical {
        (80.0, 28.0, 40.0, 4.0)
    } else {
        (22.0, 80.0, 4.0, 40.0)
    };
    view()
        .w(px(width))
        .h(px(height))
        .self_center()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .role(AccessibilityRole::Button)
        .label("Drawer swipe handle")
        .test_id("drawer-swipe-handle")
        .on_press(on_press)
        .child(
            view()
                .w(px(mark_width))
                .h(px(mark_height))
                .rounded_full()
                .bg(move || theme().muted_foreground),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settles_between_peek_and_full_height() {
        let points = [SnapPoint::Rem(31.0), SnapPoint::Fraction(1.0)];
        assert_eq!(settle_snap(&points, 800.0, 0, -40.0, 0.0), Some(0));
        assert_eq!(settle_snap(&points, 800.0, 0, -170.0, 0.0), Some(1));
        assert_eq!(settle_snap(&points, 800.0, 1, 180.0, 0.0), Some(0));
        assert_eq!(settle_snap(&points, 800.0, 0, 180.0, 0.0), None);
    }

    #[test]
    fn snap_units_clamp_to_available_height() {
        assert_eq!(SnapPoint::Rem(31.0).resolve(450.0), 450.0);
        assert_eq!(SnapPoint::Fraction(0.25).resolve(800.0), 200.0);
        assert_eq!(SnapPoint::Pixels(300.0).resolve(800.0), 300.0);
    }
}
