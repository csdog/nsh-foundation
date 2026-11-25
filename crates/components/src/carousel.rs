//! Native scrolling, snapping and keyboard navigation with Rhea carousel parts.

use std::{rc::Rc, time::Duration};

use nsh_base::{carousel, carousel_content, carousel_item, carousel_next, carousel_previous};
use nsh_core::{
    PanActivation,
    element::{LayoutDirection, StyleRefinement, layout_rtl},
    geometry::Axis,
    prelude::*,
    time::now,
};

const WHEEL_INTERVAL: Duration = Duration::from_millis(180);
const DRAG_DISTANCE: f32 = 36.0;

pub struct Carousel {
    label: &'static str,
    content: CarouselContent,
    index: Option<Signal<u32>>,
    on_change: Option<Box<dyn Fn(u32)>>,
    axis: Axis,
    direction: LayoutDirection,
    looping: bool,
    autoplay: Option<Duration>,
    previous: Option<CarouselPrevious>,
    next: Option<CarouselNext>,
}

impl Carousel {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            content: CarouselContent::new(),
            index: None,
            on_change: None,
            axis: Axis::Horizontal,
            direction: LayoutDirection::Inherit,
            looping: false,
            autoplay: None,
            previous: None,
            next: None,
        }
    }

    pub fn content(mut self, content: CarouselContent) -> Self {
        self.content = content;
        self
    }

    pub fn previous(mut self, control: CarouselPrevious) -> Self {
        self.previous = Some(control);
        self
    }

    pub fn next(mut self, control: CarouselNext) -> Self {
        self.next = Some(control);
        self
    }

    pub fn orientation(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Bind the active slide. Set the signal to navigate programmatically.
    pub fn index(mut self, index: Signal<u32>) -> Self {
        self.index = Some(index);
        self
    }

    /// Receives selections from gestures, the wheel, controls and keyboard navigation.
    pub fn on_change(mut self, handler: impl Fn(u32) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Advances after the delay. Pauses on hover, focus, drag and reduced motion.
    pub fn autoplay(mut self, delay: Duration) -> Self {
        self.autoplay = Some(delay.max(Duration::from_millis(100)));
        self
    }

    pub fn surface(self) -> View {
        let axis = self.axis;
        let rtl = match self.direction {
            LayoutDirection::Rtl => true,
            LayoutDirection::Ltr => false,
            _ => layout_rtl(),
        };
        let index = self.index.unwrap_or_else(|| signal(0));
        let count = self.content.slides.len() as u32;
        let mirrored = rtl && axis == Axis::Horizontal;
        // Native scroll offsets increase from left to right. In a mirrored
        // track the first slide and its previous/next controls run from the right.
        let track_index = move |value: u32| {
            let last = count.saturating_sub(1);
            let value = value.min(last);
            if mirrored { last - value } else { value }
        };
        let select = move |current| {
            index.set(current);
            if let Some(handler) = &self.on_change {
                handler(current);
            }
        };
        let select = Rc::new(select);
        let on_change = select.clone();
        let hovered = signal(false);
        let focused = signal(false);
        let dragging = signal(false);
        let panning = signal(false);
        let paused = signal(false);
        if let Some(delay) = self.autoplay.filter(|_| count > 1) {
            effect(move || {
                let current = index.get();
                if hovered.get()
                    || focused.get()
                    || dragging.get()
                    || panning.get()
                    || paused.get()
                    || reduce_motion()
                {
                    return;
                }
                let advance = set_timeout(delay, move || {
                    index.set((current.min(count - 1) + 1) % count)
                });
                cleanup(move || advance.cancel());
            });
        }

        let looping = self.looping;
        let mut bases: Vec<_> = self
            .content
            .slides
            .iter()
            .map(|slide| slide.basis.clone())
            .collect();
        if mirrored {
            bases.reverse();
        }
        let last = computed(move || last_snap_index(&bases, looping));
        let step = move |forward: bool| {
            let last = last.get_untracked();
            let current = track_index(index.get_untracked()).min(last);
            let target = match forward {
                true if current < last => current + 1,
                false if current > 0 => current - 1,
                true if looping => 0,
                false if looping => last,
                _ => current,
            };
            if target != current {
                select(track_index(target));
            }
        };

        let last_wheel = stored(None);
        let wheel_step = step.clone();
        let on_wheel = move |event: &WheelEvent| {
            let (mut x, mut y) = event.delta.xy();
            if event.modifiers.shift && x == 0.0 {
                (x, y) = (y, 0.0);
            }
            let delta = if axis == Axis::Horizontal { x } else { y };
            let across = if axis == Axis::Horizontal { y } else { x };
            if event.modifiers.ctrl || delta == 0.0 || delta.abs() < across.abs() {
                return;
            }
            let now = now();
            if last_wheel
                .get()
                .is_some_and(|last| now.duration_since(last) < WHEEL_INTERVAL)
            {
                return;
            }
            last_wheel.replace(Some(now));
            wheel_step(delta > 0.0);
        };
        let on_pan_end = move |event: &PanEvent| {
            panning.set(false);
            let distance = match axis {
                Axis::Horizontal => event.translation.x.0,
                Axis::Vertical => event.translation.y.0,
            };
            if !event.cancelled && distance.abs() >= DRAG_DISTANCE {
                step(distance < 0.0);
            }
        };
        let layout = view()
            .items_center()
            .gap_4()
            .when(axis == Axis::Horizontal, |layout| layout.flex_row())
            .child(self.previous.map(|control| control.surface(axis, rtl)))
            .child(self.content.surface(axis, on_wheel, panning, on_pan_end))
            .child(self.next.map(|control| control.surface(axis, rtl)));
        let rotation_control = self.autoplay.filter(|_| count > 1).map(|_| {
            crate::button::Button::new(move || {
                SharedString::from(if paused.get() {
                    "Play carousel"
                } else {
                    "Pause carousel"
                })
            })
            .variant(crate::button::ButtonVariant::Outline)
            .size(crate::button::ButtonSize::Small)
            .on_press(move || paused.update(|paused| *paused = !*paused))
            .style_when(reduce_motion, |style| style.hidden())
        });
        view()
            .direction(self.direction)
            .w_full()
            .min_w(px(0.0))
            .gap_2()
            .on_mouse_enter(move |_: &MouseEvent| hovered.set(true))
            .on_mouse_leave(move |_: &MouseEvent| hovered.set(false))
            .child(rotation_control.map(|control| view().items_center().child(control)))
            .child(
                carousel()
                    .accessibility_label(self.label)
                    .axis(axis)
                    .looping(self.looping)
                    .index(move || Some(track_index(index.get())))
                    .on_change(move |current: &u32| on_change(track_index(*current)))
                    .on_focus(move || focused.set(true))
                    .on_blur(move || focused.set(false))
                    .on_drag_start(move || dragging.set(true))
                    .on_drag_end(move || dragging.set(false))
                    .child(layout),
            )
    }
}

fn slide_fraction(value: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        value.min(1.0)
    } else {
        1.0
    }
}

fn last_snap_index(bases: &[Reactive<f32>], looping: bool) -> u32 {
    const EPSILON: f32 = 0.0001;
    let fraction = |basis: &Reactive<f32>| slide_fraction(basis.get());
    let total: f32 = bases.iter().map(fraction).sum();
    if looping && total + EPSILON >= 1.0 + bases.iter().map(fraction).fold(0.0, f32::max) {
        return bases.len().saturating_sub(1) as u32;
    }
    // Slides whose starts fall beyond the available scroll range share the last stop.
    let end = (total - 1.0).max(0.0);
    let mut start = 0.0;
    for (index, basis) in bases.iter().enumerate() {
        if start + EPSILON >= end {
            return index as u32;
        }
        start += fraction(basis);
    }
    bases.len().saturating_sub(1) as u32
}

impl IntoElement for Carousel {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct CarouselContent {
    slides: Vec<CarouselItem>,
    spacing: f32,
    height: f32,
}

impl Default for CarouselContent {
    fn default() -> Self {
        Self {
            slides: Vec::new(),
            spacing: 16.0,
            height: 270.0,
        }
    }
}

impl CarouselContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacing(mut self, points: f32) -> Self {
        self.spacing = points.max(0.0);
        self
    }

    /// Sets the viewport height for a vertical carousel.
    pub fn height(mut self, points: f32) -> Self {
        self.height = points.max(0.0);
        self
    }

    pub fn item(mut self, item: CarouselItem) -> Self {
        self.slides.push(item);
        self
    }

    fn surface(
        self,
        axis: Axis,
        on_wheel: impl Fn(&WheelEvent) + Clone + 'static,
        panning: Signal<bool>,
        on_pan_end: impl Fn(&PanEvent) + Clone + 'static,
    ) -> impl IntoElement {
        let mut viewport = carousel_content()
            .when(axis == Axis::Horizontal, |track| {
                track.flex_1().min_w(px(0.0))
            })
            .when(axis == Axis::Vertical, |track| {
                track.w_full().h(px(self.height))
            });
        let count = self.slides.len();
        let threshold = PanActivation::MIN_DISTANCE;
        let activation = match axis {
            Axis::Horizontal => PanActivation::horizontal(threshold, threshold),
            Axis::Vertical => PanActivation::vertical(threshold, threshold),
        };
        for (index, slide) in self.slides.into_iter().enumerate() {
            viewport = viewport.child(slide.surface(axis, self.spacing, index, count, |body| {
                body
                    // A listener inside the native viewport owns wheel input on this axis,
                    // including at the ends, where the scroll view would hand it to the page.
                    .pan_activation(activation)
                    .on_wheel(on_wheel.clone())
                    .on_pan_start(move |_: &PanEvent| panning.set(true))
                    .on_pan_end(on_pan_end.clone())
            }));
        }
        viewport
    }
}

pub struct CarouselItem {
    body: AnyElement,
    basis: Reactive<f32>,
}

impl CarouselItem {
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            body: body.into_element(),
            basis: 1.0.into_reactive(),
        }
    }

    /// Fraction of the viewport occupied by this slide, from 0 to 1.
    pub fn basis(mut self, fraction: impl IntoReactive<f32>) -> Self {
        self.basis = fraction.into_reactive();
        self
    }

    fn surface(
        self,
        axis: Axis,
        spacing: f32,
        index: usize,
        count: usize,
        input: impl FnOnce(View) -> View,
    ) -> impl IntoElement {
        let gap = px(spacing * 0.5);
        let basis = self.basis;
        let body = view()
            .style_with(StyleRefinement::flex_1)
            .when(axis == Axis::Horizontal, |body| body.px(gap).py_1())
            .when(axis == Axis::Vertical, |body| body.py(gap).px_1())
            .child(self.body);
        carousel_item()
            .accessibility_label(format!("Slide {} of {count}", index + 1))
            .flex_shrink_0()
            .basis(move || percentage(slide_fraction(basis.get())))
            .child(input(body))
    }
}

pub struct CarouselPrevious {
    label: &'static str,
}

impl CarouselPrevious {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    fn surface(self, axis: Axis, rtl: bool) -> AnyElement {
        control(self.label, axis, rtl, false)
    }
}

impl Default for CarouselPrevious {
    fn default() -> Self {
        Self {
            label: "Previous slide",
        }
    }
}

pub struct CarouselNext {
    label: &'static str,
}

impl CarouselNext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    fn surface(self, axis: Axis, rtl: bool) -> AnyElement {
        control(self.label, axis, rtl, true)
    }
}

impl Default for CarouselNext {
    fn default() -> Self {
        Self {
            label: "Next slide",
        }
    }
}

const CONTROL: &str = "size-7 shrink-0 items-center justify-center rounded-2xl border border-border bg-background hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring disabled:opacity-40";

fn control(label: &'static str, axis: Axis, rtl: bool, next: bool) -> AnyElement {
    let icon = arrow(axis, rtl, next);
    if next != (axis == Axis::Horizontal && rtl) {
        carousel_next()
            .accessibility_label(label)
            .app_class(CONTROL)
            .child(icon)
            .into_element()
    } else {
        carousel_previous()
            .accessibility_label(label)
            .app_class(CONTROL)
            .child(icon)
            .into_element()
    }
}

fn arrow(axis: Axis, rtl: bool, next: bool) -> Text {
    let glyph = if axis == Axis::Vertical {
        if next { "⌄" } else { "⌃" }
    } else if next != rtl {
        "›"
    } else {
        "‹"
    };
    text(glyph).text_base().font_weight(FontWeight::SEMIBOLD)
}
