//! A controlled Drawer composed of a container, surface, and reusable content slots.

use std::time::Duration;

use nsh_base::{dialog, theme};
use nsh_core::prelude::*;

use crate::ui::{Variant, button, heading, muted};

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

#[derive(Debug, Clone, Copy, Default)]
pub struct DrawerOptions {
    pub direction: Direction,
    pub modality: Modality,
    pub show_swipe_handle: bool,
    pub disable_outside_dismissal: bool,
    /// Vertical size override in points.
    pub height: Option<f32>,
    /// Horizontal size override in points.
    pub width: Option<f32>,
    pub inset: f32,
    pub snap_points: &'static [SnapPoint],
    /// Present this surface as a centered, non-draggable dialog.
    pub centered: bool,
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

    pub fn content(
        self,
        title: &'static str,
        options: impl Fn() -> DrawerOptions + 'static,
        children: impl Fn() -> View + 'static,
    ) -> impl IntoElement {
        let mounted = signal(false);
        let exit = stored(None::<Spawned<()>>);
        effect(move || {
            if let Some(task) = exit.replace(None) {
                task.cancel();
            }
            if self.open.get() {
                mounted.set(true);
            } else if !mounted.get_untracked() || reduce_motion() {
                mounted.set(false);
            } else {
                exit.replace(Some(set_timeout(Duration::from_millis(240), move || {
                    mounted.set(false)
                })));
            }
        });
        if let Some(parent) = self.parent_stack {
            effect(move || parent.set(mounted.get()));
        }
        when(mounted, move || {
            let options = options();
            let surface = self.surface(options, title, children());
            match options.modality {
                Modality::Modal => {
                    let layer = dialog()
                        .animation(ModalAnimation::None)
                        .close_on_escape(false)
                        .close_on_backdrop_press(false)
                        .close_on_swipe(false)
                        .label(title)
                        .on_cancel(move || self.close());
                    let backdrop = self.backdrop(options, true);
                    layer.child(backdrop).child(surface).into_element()
                }
                Modality::NonModal => {
                    on_key("escape", move |_| self.close());
                    view()
                        .absolute()
                        .inset_0()
                        .pointer_events(PointerEvents::BoxNone)
                        .child(self.backdrop(options, false))
                        .child(surface)
                        .into_element()
                }
            }
        })
    }

    fn backdrop(self, options: DrawerOptions, visible: bool) -> View {
        let fade = animated_value(0.0);
        effect(move || {
            let target = if visible && self.open.get() { 1.0 } else { 0.0 };
            if reduce_motion() {
                fade.set(target);
            } else {
                let _ = fade.timing(target, 200.0, Easing::EaseOut);
            }
        });
        let layer = view()
            .absolute()
            .inset_0()
            .focusable(false)
            .bg(hsla(0.0, 0.0, 0.0, if visible { 0.5 } else { 0.0 }))
            .bind_opacity(&fade)
            .test_id("drawer-backdrop");
        if options.disable_outside_dismissal {
            if visible {
                layer
            } else {
                layer.pointer_events(PointerEvents::None)
            }
        } else {
            layer.on_press(move || self.close())
        }
    }

    fn surface(self, options: DrawerOptions, title: &'static str, children: View) -> View {
        let direction = options.direction;
        let vertical = direction.vertical();
        let offset = animated_value(0.0);
        let moving = stored(false);
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

        let offscreen = move || {
            let size = window();
            (if vertical {
                size.height.0
            } else {
                size.width.0
            }) * direction.sign()
        };
        offset.set(untracked(offscreen));
        let fade = animated_value(0.0);
        let scale = animated_value(1.0);
        effect(move || {
            let open = self.open.get();
            let behind = self.showing_child.get();
            let fade_target = if open { 1.0 } else { 0.0 };
            let scale_target = if behind { 0.98 } else { 1.0 };
            let target = if open {
                position(self.snap_index.get()) - if behind { 22.0 * direction.sign() } else { 0.0 }
            } else {
                offscreen()
            };
            if reduce_motion() {
                fade.set(fade_target);
                scale.set(scale_target);
                offset.set(target);
            } else {
                let _ = fade.timing(fade_target, 200.0, Easing::EaseOut);
                let _ = scale.timing(scale_target, 220.0, Easing::EaseOut);
                if !moving.get() || !open {
                    let _ = offset.timing(target, 220.0, Easing::EaseOut);
                }
            }
        });

        let radius = px(16.0);
        let inset = px(options.inset);
        let mut panel = view()
            .absolute()
            .pointer_events(PointerEvents::Auto)
            .bg(move || theme().popover)
            .text_color(move || theme().popover_foreground)
            .border(px(1.0))
            .border_color(move || theme().border)
            .shadow_lg()
            .overflow_hidden()
            .bind_opacity(&fade)
            .bind_scale(&scale)
            .test_id("drawer-panel");

        if options.modality == Modality::NonModal {
            panel = panel.role(AccessibilityRole::Dialog).label(title);
        }

        if options.centered {
            let width = || (window().width.0 - 32.0).clamp(0.0, 425.0);
            return container_centered(
                panel.w(move || px(width())).rounded(radius).child(children),
            );
        }

        panel = match direction {
            Direction::Up => panel.top(inset).left(inset).right(inset).rounded_b(radius),
            Direction::Down => panel
                .bottom(inset)
                .left(inset)
                .right(inset)
                .rounded_t(radius),
            Direction::Left => panel.left(inset).top(inset).bottom(inset).rounded_r(radius),
            Direction::Right => panel
                .right(inset)
                .top(inset)
                .bottom(inset)
                .rounded_l(radius),
        };

        if options.inset > 0.0 {
            panel = panel.rounded(radius);
        }

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
                    let default_width = 384.0_f32.min(viewport * 0.75);
                    px(options
                        .width
                        .unwrap_or(default_width)
                        .min((viewport - options.inset * 2.0).max(0.0)))
                })
                .bind_translate_x(&offset);
        }

        let drag = |surface: View| {
            surface
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
                        Some(index) => {
                            self.snap_index.set(index);
                            let target = position(index);
                            if reduce_motion() {
                                offset.set(target);
                            } else {
                                let _ = offset.timing(target, 220.0, Easing::EaseOut);
                            }
                        }
                        None => self.close(),
                    }
                })
        };

        if options.show_swipe_handle {
            let handle = drag(swipe_handle(vertical, move || {
                if snapped {
                    self.snap_index
                        .update(|index| *index = (*index + 1) % options.snap_points.len());
                }
            }));
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

pub fn header(title: &'static str, description: &'static str) -> View {
    view()
        .flex_shrink_0()
        .px_4()
        .pt_4()
        .pb_2()
        .gap_1()
        .child(heading(title))
        .child(when(
            move || !description.is_empty(),
            move || muted(description),
        ))
}

pub fn footer() -> View {
    view().flex_shrink_0().p_4().gap_2()
}

pub fn trigger(drawer: Drawer, label: &'static str) -> View {
    button(label, Variant::Outline, move || drawer.open())
}

pub fn close(drawer: Drawer, label: &'static str) -> View {
    button(label, Variant::Primary, move || drawer.close())
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
