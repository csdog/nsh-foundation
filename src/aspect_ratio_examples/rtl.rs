use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;
use nsh_foundation::aspect_ratio::AspectRatio;

pub fn preview() -> impl IntoElement {
    let photo = image(asset!("aspect-ratio.png"))
        .label("Photo")
        .w_full()
        .h_full()
        .object_fit(ResizeMode::Cover)
        .grayscale(1.0)
        .style_when(dark, |style| style.brightness(0.2));
    let frame = AspectRatio::new(16.0 / 9.0, photo)
        .radius(8.0)
        .test_id("aspect-rtl");
    view()
        .w_full()
        .max_w(px(384.0))
        .direction(LayoutDirection::Rtl)
        .gap_2()
        .child(frame)
        .child(
            text("منظر طبيعي جميل")
                .text_sm()
                .text_center()
                .text_color(move || nsh_base::theme().muted_foreground),
        )
}
