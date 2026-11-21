//! Percentage-based split panes with local direction and accessible dividers.

use std::{collections::BTreeMap, rc::Rc};

use nsh_base::theme;
use nsh_core::{
    AccessibilityActionDef, AccessibilityValue, element::LayoutDirection, keymap::Keystroke,
    prelude::*,
};

use crate::direction::{Direction, use_direction};

const TOTAL: f32 = 100.0;
const STEP: f32 = 1.0;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    fn horizontal(self) -> bool {
        self == Self::Horizontal
    }
}

pub struct ResizablePanel {
    body: AnyElement,
    default_size: Option<f32>,
    min_size: f32,
    max_size: f32,
}

impl ResizablePanel {
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            body: body.into_element(),
            default_size: None,
            min_size: 0.0,
            max_size: TOTAL,
        }
    }

    /// The initial percentage of usable panel space.
    pub fn default_size(mut self, percent: f32) -> Self {
        self.default_size = Some(percent);
        self
    }

    /// The permitted percentages of usable panel space.
    pub fn size_range(mut self, min: f32, max: f32) -> Self {
        if min.is_finite() && max.is_finite() && min <= max {
            self.min_size = min.clamp(0.0, TOTAL);
            self.max_size = max.clamp(0.0, TOTAL);
        }
        self
    }
}

pub struct ResizableHandle {
    label: SharedString,
    with_handle: bool,
    disabled: Reactive<bool>,
}

impl Default for ResizableHandle {
    fn default() -> Self {
        Self {
            label: "Resize panels".into(),
            with_handle: false,
            disabled: false.into_reactive(),
        }
    }
}

impl ResizableHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_handle(mut self, visible: bool) -> Self {
        self.with_handle = visible;
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    fn render(
        self,
        state: Rc<SplitState>,
        boundary: usize,
        orientation: ResizableOrientation,
        direction: Reactive<Direction>,
    ) -> View {
        let horizontal = orientation.horizontal();
        let disabled = computed(move || self.disabled.get());
        let drag_start = stored(None::<Vec<f32>>);
        let origin_direction = direction.clone();
        let accessibility_state = state.clone();
        let keyboard = state.clone();
        let actions = state.clone();
        let drag = state.clone();
        let keys = if horizontal {
            ["left", "right", "home", "end"]
        } else {
            ["up", "down", "home", "end"]
        };
        let keys = KeyCapture::keys(
            keys.into_iter()
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        let hit = px(if is_desktop() { 14.0 } else { 24.0 });
        let displacement = move |event: &ResponderEvent| {
            let delta = event.gesture.delta;
            if horizontal {
                delta.x.0
                    * if origin_direction.get() == Direction::Rtl {
                        -1.0
                    } else {
                        1.0
                    }
            } else {
                delta.y.0
            }
        };
        let target = view()
            .absolute()
            .z_index(1)
            .when(horizontal, |target| {
                target
                    .top(px(0.0))
                    .bottom(px(0.0))
                    .left(px(0.5 - hit.0 / 2.0))
                    .w(hit)
            })
            .when(!horizontal, |target| {
                target
                    .left(px(0.0))
                    .right(px(0.0))
                    .top(px(0.5 - hit.0 / 2.0))
                    .h(hit)
            })
            .items_center()
            .justify_center()
            .cursor(if horizontal {
                Cursor::ColResize
            } else {
                Cursor::RowResize
            })
            .style_when(disabled, |style| style.cursor(Cursor::Default))
            .role(AccessibilityRole::Adjustable)
            .label(self.label)
            .test_id("resizable-handle")
            .disabled(disabled)
            .focusable(move || !disabled.get())
            .focus_visible(|style| style.ring_inset().ring(px(2.0)).ring_color(theme().ring))
            .accessibility_value(move || {
                let sizes = accessibility_state.sizes.get();
                let now = sizes[..boundary].iter().sum::<f32>();
                let (back, forward) = capacity(&sizes, &accessibility_state.limits, boundary);
                Some(AccessibilityValue {
                    min: Some((now - back).max(0.0)),
                    max: Some((now + forward).min(TOTAL)),
                    now: Some(now),
                    text: Some(format!("{now:.0}%").into()),
                })
            })
            .accessibility_actions(vec![
                AccessibilityActionDef {
                    name: "increment".into(),
                    label: None,
                },
                AccessibilityActionDef {
                    name: "decrement".into(),
                    label: None,
                },
            ])
            .on_accessibility_action(move |action: &str| {
                if !disabled.get() {
                    match action {
                        "increment" => actions.step(boundary, STEP),
                        "decrement" => actions.step(boundary, -STEP),
                        _ => {}
                    }
                }
            })
            .capture_keys(keys)
            .on_key_down(move |event: &KeyEvent| {
                if disabled.get() || !event.default_prevented {
                    return;
                }
                let rtl = direction.get() == Direction::Rtl;
                let delta = match event.key.as_ref() {
                    "ArrowLeft" if horizontal => {
                        if rtl {
                            STEP
                        } else {
                            -STEP
                        }
                    }
                    "ArrowRight" if horizontal => {
                        if rtl {
                            -STEP
                        } else {
                            STEP
                        }
                    }
                    "ArrowUp" if !horizontal => -STEP,
                    "ArrowDown" if !horizontal => STEP,
                    "Home" => -TOTAL,
                    "End" => TOTAL,
                    _ => return,
                };
                keyboard.step(boundary, delta);
            })
            .claim_touches(TouchClaim::on_start().ask(move |request: &ClaimRequest| {
                if !disabled.get() {
                    request.claim();
                }
            }))
            .keep_touches(true)
            .on_responder_grant({
                let state = state.clone();
                move |_: &ResponderEvent| {
                    drag_start.replace(Some(state.sizes.get()));
                }
            })
            .on_responder_move(move |event: &ResponderEvent| {
                if let Some(start) = drag_start.get() {
                    let extent = drag.extent.get();
                    if !disabled.get() && extent > 0.0 {
                        drag.resize(&start, boundary, displacement(event) * TOTAL / extent);
                    }
                }
            })
            .on_responder_release({
                let state = state.clone();
                move |_: &ResponderEvent| state.finish_drag(drag_start.replace(None))
            })
            .on_responder_terminate(move |_: &ResponderEvent| {
                state.finish_drag(drag_start.replace(None))
            })
            .child(self.with_handle.then(|| {
                view()
                    .test_id("resizable-grip")
                    .when(horizontal, |grip| grip.w(px(12.0)).h(px(16.0)))
                    .when(!horizontal, |grip| grip.w(px(16.0)).h(px(12.0)))
                    .rounded(px(3.0))
                    .bg(move || theme().border)
                    .pointer_events(PointerEvents::None)
                    .accessibility_elements_hidden(true)
            }));

        view()
            .relative()
            .z_index(1)
            .flex_shrink_0()
            .when(horizontal, |line| line.w(px(1.0)).h_full())
            .when(!horizontal, |line| line.h(px(1.0)).w_full())
            .bg(move || theme().border)
            .child(target)
    }
}

type OnResize = dyn Fn(Vec<f32>);

pub struct ResizablePanelGroup {
    orientation: ResizableOrientation,
    panels: Vec<ResizablePanel>,
    handles: BTreeMap<usize, ResizableHandle>,
    layout: Option<Signal<Vec<f32>>>,
    on_resize: Option<Rc<OnResize>>,
}

impl ResizablePanelGroup {
    pub fn new(orientation: ResizableOrientation) -> Self {
        Self {
            orientation,
            panels: Vec::new(),
            handles: BTreeMap::new(),
            layout: None,
            on_resize: None,
        }
    }

    pub fn panel(mut self, panel: ResizablePanel) -> Self {
        self.panels.push(panel);
        self
    }

    /// Configure the divider after the last added panel.
    pub fn handle(mut self, handle: ResizableHandle) -> Self {
        self.handles.insert(self.panels.len(), handle);
        self
    }

    /// Share panel percentages with the host.
    pub fn layout(mut self, layout: Signal<Vec<f32>>) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Receive a completed resize as percentages in panel declaration order.
    pub fn on_resize(mut self, on_resize: impl Fn(Vec<f32>) + 'static) -> Self {
        self.on_resize = Some(Rc::new(on_resize));
        self
    }
}

impl IntoElement for ResizablePanelGroup {
    fn into_element(mut self) -> AnyElement {
        let count = self.panels.len();
        let mut limits: Vec<_> = self
            .panels
            .iter()
            .map(|panel| (panel.min_size, panel.max_size))
            .collect();
        make_feasible(&mut limits);
        let fixed = self
            .panels
            .iter()
            .filter_map(|panel| panel.default_size)
            .filter(|size| size.is_finite())
            .map(|size| size.clamp(0.0, TOTAL))
            .sum::<f32>();
        let unspecified = self
            .panels
            .iter()
            .filter(|panel| panel.default_size.is_none_or(|size| !size.is_finite()))
            .count();
        let share = if unspecified == 0 {
            0.0
        } else {
            (TOTAL - fixed).max(0.0) / unspecified as f32
        };
        let defaults = self
            .panels
            .iter()
            .map(|panel| {
                panel
                    .default_size
                    .filter(|size| size.is_finite())
                    .unwrap_or(share)
            })
            .collect::<Vec<_>>();
        let layout = self.layout.unwrap_or_else(|| signal(defaults.clone()));
        let limits = Rc::new(limits);
        let sizes = computed({
            let limits = limits.clone();
            move || {
                layout.with(|sizes| {
                    normalize(
                        if sizes.len() == count {
                            sizes
                        } else {
                            &defaults
                        },
                        &limits,
                    )
                })
            }
        });
        let extent = signal(0.0_f32);
        let horizontal = self.orientation.horizontal();
        let state = Rc::new(SplitState {
            layout,
            sizes,
            extent,
            limits,
            on_resize: self.on_resize,
        });
        let direction = use_direction();
        let mut group = view()
            .size_full()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .direction(direction.clone().map(LayoutDirection::from))
            .when(horizontal, |group| group.flex_row())
            .test_id("resizable-group")
            .on_layout(move |bounds: &Bounds<Points>| {
                let size = if horizontal {
                    bounds.size.width.0
                } else {
                    bounds.size.height.0
                };
                extent.set((size - count.saturating_sub(1) as f32).max(0.0));
            });
        for (index, panel) in self.panels.into_iter().enumerate() {
            if index > 0 {
                group = group.child(self.handles.remove(&index).unwrap_or_default().render(
                    state.clone(),
                    index,
                    self.orientation,
                    direction.clone(),
                ));
            }
            group = group.child(
                view()
                    .test_id("resizable-panel")
                    .basis(px(0.0))
                    .flex_grow(move || sizes.with(|sizes| sizes[index]))
                    .min_h(px(0.0))
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .child(panel.body),
            );
        }
        group.into_element()
    }
}

struct SplitState {
    layout: Signal<Vec<f32>>,
    sizes: Memo<Vec<f32>>,
    extent: Signal<f32>,
    limits: Rc<Vec<(f32, f32)>>,
    on_resize: Option<Rc<OnResize>>,
}

impl SplitState {
    fn resize(&self, start: &[f32], boundary: usize, delta: f32) {
        self.layout
            .set(adjust(start, &self.limits, boundary, delta));
    }

    fn step(&self, boundary: usize, delta: f32) {
        let start = self.sizes.get();
        self.resize(&start, boundary, delta);
        self.finish_drag(Some(start));
    }

    fn finish_drag(&self, start: Option<Vec<f32>>) {
        if let (Some(start), Some(on_resize)) = (start, &self.on_resize) {
            let sizes = self.sizes.get();
            if start
                .iter()
                .zip(&sizes)
                .any(|(previous, current)| (previous - current).abs() > 0.001)
            {
                on_resize(sizes);
            }
        }
    }
}

fn make_feasible(limits: &mut [(f32, f32)]) {
    let low = limits.iter().map(|(min, _)| min).sum::<f32>();
    let high = limits.iter().map(|(_, max)| max).sum::<f32>();
    let count = limits.len() as f32;
    for (min, max) in limits {
        if low > TOTAL {
            *min *= TOTAL / low;
        }
        if high < TOTAL {
            *max += (TOTAL - high) * (TOTAL - *max) / (TOTAL * count - high);
        }
    }
}

fn normalize(values: &[f32], limits: &[(f32, f32)]) -> Vec<f32> {
    if values.is_empty() {
        return Vec::new();
    }
    let valid = |size: f32| {
        if size.is_finite() {
            size.clamp(0.0, TOTAL)
        } else {
            0.0
        }
    };
    let sum = values.iter().copied().map(valid).sum::<f32>();
    let equal = TOTAL / values.len() as f32;
    let mut output: Vec<_> = values
        .iter()
        .zip(limits)
        .map(|(&value, &(min, max))| {
            let percent = if sum > 0.0 {
                valid(value) * TOTAL / sum
            } else {
                equal
            };
            percent.clamp(min, max)
        })
        .collect();
    let remainder = TOTAL - output.iter().sum::<f32>();
    let available: Vec<_> = output
        .iter()
        .zip(limits)
        .map(|(&size, &(min, max))| {
            if remainder >= 0.0 {
                max - size
            } else {
                size - min
            }
        })
        .collect();
    let room = available.iter().sum::<f32>();
    if room > 0.0 {
        for (size, space) in output.iter_mut().zip(available) {
            *size += remainder * space / room;
        }
    }
    output
}

fn capacity(sizes: &[f32], limits: &[(f32, f32)], boundary: usize) -> (f32, f32) {
    let room = |indices: std::ops::Range<usize>, grow: bool| {
        indices
            .map(|i| {
                if grow {
                    limits[i].1 - sizes[i]
                } else {
                    sizes[i] - limits[i].0
                }
            })
            .sum::<f32>()
    };
    let end = sizes.len();
    (
        room(0..boundary, false)
            .min(room(boundary..end, true))
            .max(0.0),
        room(0..boundary, true)
            .min(room(boundary..end, false))
            .max(0.0),
    )
}

fn adjust(sizes: &[f32], limits: &[(f32, f32)], boundary: usize, delta: f32) -> Vec<f32> {
    let (back, forward) = capacity(sizes, limits, boundary);
    let delta = delta.clamp(-back, forward);
    let mut result = sizes.to_vec();
    let mut shift = |indices: &mut dyn Iterator<Item = usize>, delta: f32| {
        let mut remaining = delta.abs();
        for index in indices {
            let size = &mut result[index];
            let room = if delta >= 0.0 {
                limits[index].1 - *size
            } else {
                *size - limits[index].0
            };
            let change = remaining.min(room.max(0.0));
            *size += change.copysign(delta);
            remaining -= change;
        }
    };
    shift(&mut (0..boundary).rev(), delta);
    shift(&mut (boundary..sizes.len()), -delta);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_constraints_ripple_through_neighbors_at_a_boundary() {
        let start = [30.0, 30.0, 40.0];
        let limits = [(10.0, 70.0), (15.0, 70.0), (15.0, 90.0)];
        assert_eq!(adjust(&start, &limits, 1, 80.0), [70.0, 15.0, 15.0]);
        assert_eq!(adjust(&start, &limits, 2, -50.0), [10.0, 15.0, 75.0]);
        assert_eq!(adjust(&start, &limits, 1, -12.0), [18.0, 42.0, 40.0]);
    }

    #[test]
    fn defaults_and_conflicting_ranges_fit_the_available_space() {
        let mut limits = [(80.0, 100.0), (60.0, 90.0)];
        make_feasible(&mut limits);
        let sizes = normalize(&[10.0, 90.0], &limits);
        assert!((sizes.iter().sum::<f32>() - TOTAL).abs() < 0.001);
        assert!((sizes[0] - 400.0 / 7.0).abs() < 0.001);

        let mut limits = [(0.0, 15.0), (0.0, 25.0)];
        make_feasible(&mut limits);
        let sizes = normalize(&[75.0, 25.0], &limits);
        assert!((sizes.iter().sum::<f32>() - TOTAL).abs() < 0.001);
        assert_eq!(sizes, [limits[0].1, limits[1].1]);
    }
}
