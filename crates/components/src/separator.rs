//! One-point separators that stretch along their orientation.

use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default)]
pub struct Separator {
    orientation: SeparatorOrientation,
    label: Option<SharedString>,
}

impl Separator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Expose a named break to assistive technology.
    ///
    /// Native Shell exposes the label as static text. An unlabeled line is decorative.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the native view so callers can apply margins and more styles.
    pub fn surface(self) -> View {
        let line = view()
            .flex_shrink_0()
            .focusable(false)
            .pointer_events(PointerEvents::None)
            .bg(move || theme().border);
        let line = match self.orientation {
            SeparatorOrientation::Horizontal => line.w_full().h(px(1.0)),
            SeparatorOrientation::Vertical => line.self_stretch().w(px(1.0)),
        };
        match self.label {
            Some(label) => line
                .role(AccessibilityRole::Text)
                .accessible(true)
                .label(label),
            None => line.accessibility_elements_hidden(true),
        }
    }
}

impl IntoElement for Separator {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}
