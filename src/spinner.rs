//! Loading indicators drawn and animated with Native Shell.

use std::f32::consts::TAU;

use nsh_core::{SvgPaint, canvas::LineCap, prelude::*};

pub struct Spinner {
    size: f32,
    label: Option<Reactive<SharedString>>,
    color: Option<Reactive<AdaptiveColor>>,
    icon: Option<AnyElement>,
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, points: f32) -> Self {
        if points.is_finite() && points > 0.0 {
            self.size = points;
        }
        self
    }

    pub fn label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.label = Some(label.into_reactive());
        self
    }

    /// Hide the icon when a surrounding control already describes the status.
    pub fn decorative(mut self) -> Self {
        self.label = None;
        self
    }

    pub fn color(mut self, color: impl IntoReactive<AdaptiveColor>) -> Self {
        self.color = Some(color.into_reactive());
        self
    }

    /// Use a custom icon sized for this spinner.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    pub fn surface(self) -> View {
        let size = self.size;
        let icon = self.icon.unwrap_or_else(|| {
            let stroke = Painter::new()
                .stroke_style_from_view(SvgPaint::CurrentColor)
                .line_width(size / 12.0)
                .line_cap(LineCap::Round)
                .begin_path()
                .arc(size / 2.0, size / 2.0, size * 0.375, 0.0, TAU * 0.75)
                .stroke();
            canvas(stroke).size(px(size)).into_element()
        });
        let mut surface = view()
            .size(px(size))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .pointer_events(PointerEvents::None)
            .focusable(false)
            .test_id("spinner")
            .child(view().accessibility_elements_hidden(true).child(icon));
        if let Some(color) = self.color {
            surface = surface.text_color(color);
        }
        surface = if let Some(label) = self.label {
            surface
                .role(AccessibilityRole::Progressbar)
                .accessible(true)
                .label(label)
        } else {
            surface.accessibility_elements_hidden(true)
        };
        spin(surface, 1000.0)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: 16.0,
            label: Some("Loading".into_reactive()),
            color: None,
            icon: None,
        }
    }
}

impl IntoElement for Spinner {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

/// The compact ring already used in Button, Badge and Input Group.
pub(crate) fn ring(size: f32, color: impl Fn() -> AdaptiveColor + Clone + 'static) -> View {
    let muted = color.clone();
    spin(
        view()
            .size(px(size))
            .rounded_full()
            .border(px(1.0))
            .border_color(move || muted().opacity(0.30))
            .border_t_color(color)
            .accessibility_elements_hidden(true),
        800.0,
    )
}

fn spin(surface: View, duration: f32) -> View {
    let rotation = animated_value(0.0);
    let motion =
        Animation::timing(&rotation, 360.0, Timing::new(duration, Easing::Linear)).looped(None);
    effect({
        let motion = motion.clone();
        move || {
            if reduce_motion() {
                motion.stop();
                rotation.set(0.0);
            } else {
                let _ = motion.start();
            }
        }
    });
    cleanup(move || motion.stop());
    surface.bind_rotate(&rotation)
}
