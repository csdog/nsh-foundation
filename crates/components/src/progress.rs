//! Composable, accessible linear progress with a reactive value.

use nsh_base::theme;
use nsh_core::{AccessibilityValue, element::LayoutDirection, prelude::*};

use crate::direction::{Direction, use_direction};

pub struct Progress {
    value: Reactive<f32>,
    indeterminate: Reactive<bool>,
    range: (f32, f32),
    label: Option<ProgressLabel>,
    accessibility_label: Option<Reactive<SharedString>>,
    display: Option<ProgressValue>,
    track: ProgressTrack,
}

impl Progress {
    pub fn new(value: impl IntoReactive<f32>) -> Self {
        Self {
            value: value.into_reactive(),
            indeterminate: false.into_reactive(),
            range: (0.0, 100.0),
            label: None,
            accessibility_label: None,
            display: None,
            track: ProgressTrack::default(),
        }
    }

    /// Use a finite, increasing range. The default is 0 to 100.
    pub fn range(mut self, min: f32, max: f32) -> Self {
        if min.is_finite() && max.is_finite() && min < max {
            self.range = (min, max);
        }
        self
    }

    /// Omit the current value while the extent of the task is unknown.
    pub fn indeterminate(mut self, indeterminate: impl IntoReactive<bool>) -> Self {
        self.indeterminate = indeterminate.into_reactive();
        self
    }

    pub fn label(mut self, label: ProgressLabel) -> Self {
        self.label = Some(label);
        self
    }

    pub fn accessibility_label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.accessibility_label = Some(label.into_reactive());
        self
    }

    pub fn display(mut self, value: ProgressValue) -> Self {
        self.display = Some(value);
        self
    }

    pub fn track(mut self, track: ProgressTrack) -> Self {
        self.track = track;
        self
    }
}

impl IntoElement for Progress {
    fn into_element(self) -> AnyElement {
        let Self {
            value,
            indeterminate,
            range: (min, max),
            label,
            accessibility_label,
            display,
            track,
        } = self;
        let direction = use_direction();
        let now = computed(move || {
            if indeterminate.get() {
                None
            } else {
                let value = value.get();
                (!value.is_nan()).then(|| value.clamp(min, max))
            }
        });
        let fraction = computed(move || {
            now.get().map_or(0.0, |value| {
                ((f64::from(value) - f64::from(min)) / (f64::from(max) - f64::from(min))) as f32
            })
        });
        let formatted = display.map(|display| {
            computed(move || {
                now.get().map(|value| {
                    display.format.as_ref().map_or_else(
                        || format!("{:.0}%", fraction.get() * 100.0),
                        |format| format(value),
                    )
                })
            })
        });
        let accessible_name =
            accessibility_label.or_else(|| label.as_ref().map(|label| label.0.clone()));
        let has_header = label.is_some() || formatted.is_some();
        let header = view()
            .flex_row()
            .items_center()
            .justify_between()
            .gap_3()
            .accessibility_elements_hidden(true)
            .child(label.map(ProgressLabel::render))
            .child(formatted.map(|formatted| {
                text(move || formatted.get().unwrap_or_else(|| "—".to_owned()))
                    .text_sm()
                    .tabular_nums()
                    .ms(auto())
                    .text_color(move || theme().muted_foreground)
            }));
        view()
            .role(AccessibilityRole::Progressbar)
            .accessible(true)
            .when_some(accessible_name, |progress, label| progress.label(label))
            .accessibility_value(move || {
                Some(AccessibilityValue {
                    min: Some(min),
                    max: Some(max),
                    now: now.get(),
                    text: formatted.and_then(|formatted| formatted.get().map(Into::into)),
                })
            })
            .direction(direction.map(LayoutDirection::from))
            .w_full()
            .gap_3()
            .child(has_header.then_some(header))
            .child(track.render(move || fraction.get()))
            .into_element()
    }
}

pub struct ProgressLabel(Reactive<SharedString>);

impl ProgressLabel {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self(label.into_reactive())
    }

    fn render(self) -> Text {
        text(self.0).text_sm().font_weight(FontWeight::MEDIUM)
    }
}

#[derive(Default)]
pub struct ProgressValue {
    format: Option<Box<dyn Fn(f32) -> String>>,
}

impl ProgressValue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Format the clamped value for both the display and assistive technology.
    pub fn format(mut self, format: impl Fn(f32) -> String + 'static) -> Self {
        self.format = Some(Box::new(format));
        self
    }
}

pub struct ProgressTrack {
    height: f32,
    indicator: ProgressIndicator,
}

impl ProgressTrack {
    pub fn new(indicator: ProgressIndicator) -> Self {
        Self {
            height: 8.0,
            indicator,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    fn render(self, fraction: impl Fn() -> f32 + 'static) -> View {
        view()
            .w_full()
            .h(px(self.height))
            .flex_row()
            .overflow_hidden()
            .rounded(px(16.0))
            .bg(move || theme().muted)
            .accessibility_elements_hidden(true)
            .test_id("progress-track")
            .child(self.indicator.render(fraction))
    }
}

impl Default for ProgressTrack {
    fn default() -> Self {
        Self::new(ProgressIndicator::default())
    }
}

#[derive(Default)]
pub struct ProgressIndicator {
    color: Option<AdaptiveColor>,
}

impl ProgressIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: AdaptiveColor) -> Self {
        self.color = Some(color);
        self
    }

    fn render(self, fraction: impl Fn() -> f32 + 'static) -> View {
        let fraction = computed(fraction);
        let direction = use_direction();
        // Native Shell animates transforms separately from layout dimensions.
        view()
            .h_full()
            .bg(move || self.color.unwrap_or_else(|| theme().primary))
            .test_id("progress-indicator")
            .w(move || percentage(if reduce_motion() { fraction.get() } else { 1.0 }))
            .origin_left()
            .style_when(
                move || direction.get() == Direction::Rtl,
                |style| style.origin_right(),
            )
            .scale_x(move || if reduce_motion() { 1.0 } else { fraction.get() })
            .style_when(
                || !reduce_motion(),
                |style| style.transition_transform().duration(200.0),
            )
    }
}
