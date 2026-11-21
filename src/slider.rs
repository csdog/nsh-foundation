//! Rhea sliders with any fixed number of independently focusable thumbs.

use std::rc::Rc;

use nsh_base::theme;
use nsh_core::{AccessibilityActionDef, AccessibilityValue, keymap::Keystroke, prelude::*};

use crate::direction::{Direction, use_direction};

const THUMB: f32 = 16.0;
const HALF_THUMB: f32 = THUMB / 2.0;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct Slider<const N: usize> {
    initial: [f64; N],
    value: Option<Signal<[f64; N]>>,
    min: f64,
    max: f64,
    step: f64,
    large_step: f64,
    orientation: SliderOrientation,
    direction: Option<Reactive<Direction>>,
    disabled: Reactive<bool>,
    labels: Option<[SharedString; N]>,
    on_change: Option<ValueHandler<N>>,
    on_commit: Option<ValueHandler<N>>,
}

type ValueHandler<const N: usize> = Rc<dyn Fn([f64; N])>;

impl<const N: usize> Slider<N> {
    pub fn new(values: [f64; N]) -> Self {
        Self {
            initial: values,
            value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            large_step: 10.0,
            orientation: SliderOrientation::Horizontal,
            direction: None,
            disabled: false.into_reactive(),
            labels: None,
            on_change: None,
            on_commit: None,
        }
    }

    /// Bind values in ascending order to a caller-owned signal.
    pub fn value(mut self, value: Signal<[f64; N]>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// The distance between ticks. Zero allows continuous input.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Amount used by Page Up, Page Down, and Shift plus an arrow key.
    pub fn large_step(mut self, step: f64) -> Self {
        self.large_step = step;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: impl IntoReactive<Direction>) -> Self {
        self.direction = Some(direction.into_reactive());
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    /// Accessible names, in the same order as the values.
    pub fn labels(mut self, labels: [impl Into<SharedString>; N]) -> Self {
        self.labels = Some(labels.map(Into::into));
        self
    }

    pub fn on_value_change(mut self, handler: impl Fn([f64; N]) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Receive one notification after a changed drag, key, or accessibility action.
    pub fn on_value_commit(mut self, handler: impl Fn([f64; N]) + 'static) -> Self {
        self.on_commit = Some(Rc::new(handler));
        self
    }

    pub fn surface(self) -> View {
        let horizontal = self.orientation == SliderOrientation::Horizontal;
        let root = if horizontal {
            view().w_full().min_w(px(THUMB)).h(px(28.0))
        } else {
            view().w(px(28.0)).h(px(160.0)).min_h(px(160.0))
        };
        if N == 0 {
            return root;
        }
        let bounds = Range::new(self.min, self.max, self.step);
        let value = self
            .value
            .unwrap_or_else(|| signal(bounds.ordered(self.initial)));
        let values = computed(move || bounds.ordered(value.get()));
        let disabled = computed(move || self.disabled.get());
        let direction = self.direction.unwrap_or_else(use_direction);
        let reverse = computed(move || !horizontal || direction.get() == Direction::Rtl);
        let length = signal(0.0);
        let track = Track {
            horizontal,
            reverse,
            length,
        };
        let state = Rc::new(State {
            range: bounds,
            large_step: if self.large_step.is_finite() {
                self.large_step.max(0.0)
            } else {
                10.0
            },
            values,
            value,
            disabled,
            track,
            drag: stored(None),
            refs: std::array::from_fn(|_| element_ref()),
            on_change: self.on_change,
            on_commit: self.on_commit,
        });

        let fill = view()
            .absolute()
            .bg(|| theme().primary)
            .style_with(move |style| {
                let values = values.get();
                let start = if N > 1 {
                    track.position(bounds.fraction(values[0]))
                } else if track.reverse.get() {
                    track.length.get()
                } else {
                    0.0
                };
                let end = track.position(bounds.fraction(values[N - 1]));
                let edge = start.min(end);
                let size = (end - start).abs();
                if horizontal {
                    style.left(px(edge)).w(px(size)).h_full()
                } else {
                    style.top(px(edge)).h(px(size)).w_full()
                }
            })
            .test_id("slider-fill");
        let rail = view()
            .absolute()
            .rounded(px(16.0))
            .overflow_hidden()
            .bg(|| theme().input.opacity(0.9))
            .when(horizontal, |rail| {
                rail.left_0().right_0().top(px(12.0)).h(px(4.0))
            })
            .when(!horizontal, |rail| {
                rail.top_0().bottom_0().left(px(12.0)).w(px(4.0))
            })
            .pointer_events(PointerEvents::None)
            .test_id("slider-track")
            .child(fill);
        let labels = self.labels.unwrap_or_else(|| {
            std::array::from_fn(|index| {
                if N == 1 {
                    "Value".into()
                } else {
                    format!("Value {}", index + 1).into()
                }
            })
        });
        let thumbs = labels
            .into_iter()
            .enumerate()
            .map(|(index, label)| thumb(state.clone(), index, label));
        responder(root, state.clone(), None)
            .on_layout(move |frame: &Bounds<Points>| {
                length.set(if horizontal {
                    frame.size.width.0
                } else {
                    frame.size.height.0
                });
            })
            .style_when(disabled, |style| style.opacity(0.5))
            .test_id("slider-control")
            .child(rail)
            .children(thumbs)
    }
}

impl<const N: usize> IntoElement for Slider<N> {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

#[derive(Clone, Copy)]
struct Range {
    min: f64,
    max: f64,
    step: f64,
    precision: f64,
}

impl Range {
    fn new(min: f64, max: f64, step: f64) -> Self {
        let min = if min.is_finite() { min } else { 0.0 };
        let max = if max.is_finite() {
            max.max(min)
        } else {
            100.0_f64.max(min)
        };
        let step = if step.is_finite() { step.max(0.0) } else { 1.0 };
        let decimal_places = |value: f64| {
            value
                .to_string()
                .split_once('.')
                .map_or(0, |(_, decimals)| decimals.len())
        };
        let precision = 10.0_f64.powi(decimal_places(step).max(decimal_places(min)) as i32);
        Self {
            min,
            max,
            step,
            precision,
        }
    }

    fn ordered<const N: usize>(self, values: [f64; N]) -> [f64; N] {
        let mut values = values.map(|value| {
            if value.is_nan() {
                self.min
            } else {
                value.clamp(self.min, self.max)
            }
        });
        values.sort_by(f64::total_cmp);
        values
    }

    fn snap(self, value: f64) -> f64 {
        if self.step == 0.0 || value <= self.min || value >= self.max {
            return value.clamp(self.min, self.max);
        }
        let index = ((value - self.min) / self.step).round();
        let snapped = self.min + index * self.step;
        let scaled = snapped * self.precision;
        let rounded = if scaled.is_finite() {
            scaled.round() / self.precision
        } else {
            snapped
        };
        if rounded.is_finite() {
            rounded.clamp(self.min, self.max)
        } else {
            value
        }
    }

    fn fraction(self, value: f64) -> f32 {
        let span = self.max - self.min;
        if span == 0.0 {
            0.0
        } else if span.is_finite() {
            ((value - self.min) / span) as f32
        } else {
            ((value / 2.0 - self.min / 2.0) / (self.max / 2.0 - self.min / 2.0)) as f32
        }
    }

    fn at(self, fraction: f32) -> f64 {
        let fraction = f64::from(fraction.clamp(0.0, 1.0));
        self.snap((1.0 - fraction) * self.min + fraction * self.max)
    }

    fn step_from(self, value: f64, steps: i32) -> f64 {
        if self.step == 0.0 {
            return self.snap(value + (self.max / 100.0 - self.min / 100.0) * f64::from(steps));
        }
        let position = (value - self.min) / self.step;
        let nearest = position.round();
        let exact = (nearest - position).abs() < 1e-8;
        let index = if exact {
            nearest
        } else if steps > 0 {
            position.floor()
        } else {
            position.ceil()
        };
        self.snap(self.min + (index + f64::from(steps)) * self.step)
    }
}

#[derive(Clone, Copy)]
struct Track {
    horizontal: bool,
    reverse: Memo<bool>,
    length: Signal<f32>,
}

impl Track {
    fn travel(self) -> f32 {
        (self.length.get() - THUMB).max(0.0)
    }

    fn position(self, fraction: f32) -> f32 {
        let fraction = if self.reverse.get() {
            1.0 - fraction
        } else {
            fraction
        };
        (self.length.get() / 2.0).min(HALF_THUMB) + fraction * self.travel()
    }

    fn offset(self, point: Point<Points>) -> f32 {
        let axis = if self.horizontal {
            point.x.0
        } else {
            point.y.0
        };
        if self.reverse.get() { -axis } else { axis }
    }

    fn from_press(self, location: Point<Points>) -> f32 {
        self.offset(location)
            + if self.reverse.get() {
                self.length.get()
            } else {
                0.0
            }
            - HALF_THUMB
    }
}

#[derive(Clone, Copy)]
struct Drag {
    index: usize,
    origin: f32,
    start: f64,
    changed: bool,
    direct: bool,
}

struct State<const N: usize> {
    range: Range,
    large_step: f64,
    values: Memo<[f64; N]>,
    value: Signal<[f64; N]>,
    disabled: Memo<bool>,
    track: Track,
    drag: Stored<Option<Drag>>,
    refs: [ElementRef; N],
    on_change: Option<ValueHandler<N>>,
    on_commit: Option<ValueHandler<N>>,
}

impl<const N: usize> State<N> {
    fn bounds(&self, values: &[f64; N], index: usize) -> (f64, f64) {
        let minimum = if index == 0 {
            self.range.min
        } else {
            values[index - 1]
        };
        let maximum = values.get(index + 1).copied().unwrap_or(self.range.max);
        (minimum, maximum)
    }

    fn change(&self, index: usize, target: f64, push: bool) -> bool {
        if self.disabled.get_untracked() {
            return false;
        }
        let mut values = self.values.get_untracked();
        let next = if push {
            target.clamp(self.range.min, self.range.max)
        } else {
            let (min, max) = self.bounds(&values, index);
            target.clamp(min, max)
        };
        if values[index] == next {
            return false;
        }

        values[index] = next;
        if push {
            for value in &mut values[..index] {
                *value = value.min(next);
            }
            for value in &mut values[index + 1..] {
                *value = value.max(next);
            }
        }
        self.value.set(values);
        if let Some(handler) = &self.on_change {
            handler(values);
        }
        true
    }

    fn commit(&self) {
        if let Some(handler) = &self.on_commit {
            handler(self.values.get_untracked());
        }
    }

    fn drag_start(&self, event: &ResponderEvent, thumb: Option<usize>) {
        self.drag.replace(None);
        let travel = self.track.travel();
        if travel <= 0.0 || self.disabled.get_untracked() {
            return;
        }
        let values = self.values.get_untracked();
        let at = self.track.from_press(event.location);
        let target = self.range.at(at / travel);
        let index = thumb.unwrap_or_else(|| {
            (0..N)
                .min_by(|a, b| {
                    (values[*a] - target)
                        .abs()
                        .total_cmp(&(values[*b] - target).abs())
                })
                .unwrap_or(0)
        });
        let start = values[index];
        let origin = if thumb.is_some() {
            self.range.fraction(start) * travel
        } else {
            at
        };
        self.refs[index].focus();
        self.drag.replace(Some(Drag {
            index,
            origin,
            start,
            changed: false,
            direct: thumb.is_some(),
        }));
        if thumb.is_none() {
            self.drag_to(0.0);
        }
    }

    fn drag_to(&self, delta: f32) {
        let Some(mut drag) = self.drag.get() else {
            return;
        };
        let travel = self.track.travel();
        if travel <= 0.0 {
            return;
        }
        let target = self.range.at((drag.origin + delta) / travel);
        if !drag.changed && drag.direct && target != drag.start {
            // When two handles share a value, choose the handle that can move in this direction.
            let values = self.values.get_untracked();
            let index = if target < drag.start {
                values.iter().position(|value| *value == drag.start)
            } else {
                values.iter().rposition(|value| *value == drag.start)
            };
            if let Some(index) = index.filter(|&index| index != drag.index) {
                drag.index = index;
                self.refs[index].focus();
            }
        }
        drag.changed |= self.change(drag.index, target, true);
        self.drag.replace(Some(drag));
    }

    fn drag_end(&self, released: bool) {
        let Some(drag) = self.drag.replace(None) else {
            return;
        };
        if drag.changed {
            self.commit();
        }
        if released {
            let (reference, disabled, active_drag) =
                (self.refs[drag.index], self.disabled, self.drag);
            // A touch can reset focus as it lifts from the track. Restore it after the event.
            spawn(async move {
                next_frame().await;
                if active_drag.get().is_none() && !disabled.get_untracked() {
                    reference.focus();
                }
            });
        }
    }

    fn key(&self, index: usize, key: &str, shift: bool) {
        let value = self.values.get_untracked()[index];
        let mirrored = self.track.horizontal && self.track.reverse.get_untracked();
        let large = |direction: i32| {
            self.range
                .snap(value + self.large_step * f64::from(direction))
        };
        let arrow = |direction: i32| {
            if shift {
                large(direction)
            } else {
                self.range.step_from(value, direction)
            }
        };
        let destination = match key {
            "Home" => self.range.min,
            "End" => self.range.max,
            "ArrowUp" => arrow(1),
            "ArrowDown" => arrow(-1),
            "ArrowRight" => arrow(if mirrored { -1 } else { 1 }),
            "ArrowLeft" => arrow(if mirrored { 1 } else { -1 }),
            "PageUp" => large(1),
            "PageDown" => large(-1),
            _ => return,
        };
        if self.change(index, destination, false) {
            self.commit();
        }
    }
}

fn responder<const N: usize>(surface: View, state: Rc<State<N>>, thumb: Option<usize>) -> View {
    let disabled = state.disabled;
    let press = state.clone();
    let movement = state.clone();
    let release = state.clone();
    surface
        .claim_touches(TouchClaim::on_start().ask(move |request: &ClaimRequest| {
            if !disabled.get_untracked() {
                request.claim();
            }
        }))
        .keep_touches(true)
        .on_responder_grant(move |event: &ResponderEvent| press.drag_start(event, thumb))
        .on_responder_move(move |event: &ResponderEvent| {
            movement.drag_to(movement.track.offset(event.gesture.delta))
        })
        .on_responder_release(move |_: &ResponderEvent| release.drag_end(true))
        .on_responder_terminate(move |_: &ResponderEvent| state.drag_end(false))
}

fn thumb<const N: usize>(state: Rc<State<N>>, index: usize, label: SharedString) -> View {
    let disabled = state.disabled;
    let track = state.track;
    let range = state.range;
    let values = state.values;
    let keyboard = state.clone();
    let accessibility = state.clone();
    let actions = state.clone();
    let keys = KeyCapture::keys(
        [
            "up",
            "down",
            "left",
            "right",
            "home",
            "end",
            "pageup",
            "pagedown",
            "shift+up",
            "shift+down",
            "shift+left",
            "shift+right",
        ]
        .into_iter()
        .filter_map(|key| Keystroke::parse(key, os()).ok()),
    );
    let surface = view()
        .absolute()
        .size(px(THUMB))
        .rounded(px(16.0))
        .bg(hsla(0.0, 0.0, 1.0, 1.0))
        .ring(px(1.0))
        .ring_color(hsla(0.0, 0.0, 0.0, 0.10))
        .shadow_md()
        .cursor_pointer()
        .transition_shadow()
        .duration(200.0)
        .hover(|style| style.ring(px(4.0)).ring_color(|| theme().ring.opacity(0.3)))
        .focus_visible(|style| style.ring(px(4.0)).ring_color(|| theme().ring.opacity(0.3)))
        .style_with(move |style| {
            let position = track.position(range.fraction(values.get()[index])) - HALF_THUMB;
            if track.horizontal {
                style.left(px(position)).top(px(6.0))
            } else {
                style.top(px(position)).left(px(6.0))
            }
        })
        .accessible(true)
        .role(AccessibilityRole::Adjustable)
        .label(label)
        .disabled(disabled)
        .focusable(move || !disabled.get())
        .element_ref(&state.refs[index])
        .on_press(|| {})
        .accessibility_value(move || {
            let values = values.get();
            let (min, max) = accessibility.bounds(&values, index);
            Some(AccessibilityValue {
                min: Some(min as f32),
                max: Some(max as f32),
                now: Some(values[index] as f32),
                text: None,
            })
        })
        .accessibility_actions(
            ["increment", "decrement"]
                .map(|name| AccessibilityActionDef {
                    name: name.into(),
                    label: None,
                })
                .to_vec(),
        )
        .on_accessibility_action(move |action: &str| match action {
            "increment" => actions.key(index, "ArrowUp", false),
            "decrement" => actions.key(index, "ArrowDown", false),
            _ => {}
        })
        .capture_keys(keys)
        .on_key_down(move |event: &KeyEvent| {
            if event.default_prevented {
                keyboard.key(index, event.key.as_ref(), event.modifiers.shift);
            }
        })
        .test_id("slider-thumb");
    responder(surface, state, Some(index))
}
