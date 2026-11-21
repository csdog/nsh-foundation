//! A responsive frame that maintains its width-to-height ratio.

use nsh_core::prelude::*;

pub struct AspectRatio(View);

impl AspectRatio {
    /// Fill the available width and size the height from the ratio.
    /// A nonpositive or nonfinite ratio uses a square frame.
    pub fn new(ratio: impl IntoReactive<f32>, child: impl IntoElement) -> Self {
        let ratio = ratio.into_reactive().map(|value| {
            if value.is_finite() && value > 0.0 {
                value
            } else {
                1.0
            }
        });
        Self(
            view()
                .w_full()
                .aspect_ratio(ratio)
                .overflow_hidden()
                .child(view().absolute().inset_0().child(child)),
        )
    }

    pub fn radius(mut self, points: f32) -> Self {
        self.0 = self.0.rounded(px(points));
        self
    }

    pub fn test_id(mut self, id: &'static str) -> Self {
        self.0 = self.0.test_id(id);
        self
    }
}

impl IntoElement for AspectRatio {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}
