use components::aspect_ratio::AspectRatio;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let photo = image(asset!("aspect-ratio.png"))
        .label("Photo")
        .w_full()
        .h_full()
        .object_fit(ResizeMode::Cover)
        .grayscale(1.0)
        .style_when(dark, |style| style.brightness(0.2));
    let frame = AspectRatio::new(9.0 / 16.0, photo)
        .radius(8.0)
        .test_id("aspect-portrait");
    view().w_full().max_w(px(160.0)).child(frame)
}
