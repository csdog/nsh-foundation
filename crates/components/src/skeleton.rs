//! Decorative placeholders styled with the Rhea theme.

use nsh_base::theme;
use nsh_core::prelude::*;

pub struct Skeleton {
    animated: bool,
}

impl Skeleton {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set to false for a static placeholder. Reduced motion also disables the pulse.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Style this view to match the footprint of the content being loaded.
    pub fn surface(self) -> View {
        view()
            .rounded(px(16.0))
            .bg(|| theme().muted)
            .when(self.animated, |surface| {
                surface.style_when(|| !reduce_motion(), |style| style.animate_pulse())
            })
            .accessibility_elements_hidden(true)
            .pointer_events(PointerEvents::None)
            .focusable(false)
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self { animated: true }
    }
}

impl IntoElement for Skeleton {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}
