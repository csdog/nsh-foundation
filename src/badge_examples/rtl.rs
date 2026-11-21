use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;
use nsh_foundation::badge::{Badge, BadgeVariant};

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .direction(LayoutDirection::Rtl)
        .child(Badge::new("شارة"))
        .child(Badge::new("ثانوي").variant(BadgeVariant::Secondary))
        .child(Badge::new("مدمر").variant(BadgeVariant::Destructive))
        .child(Badge::new("مخطط").variant(BadgeVariant::Outline))
        .child(
            Badge::new("متحقق")
                .variant(BadgeVariant::Secondary)
                .icon_start(text("✓").text_xs()),
        )
        .child(
            Badge::new("إشارة مرجعية")
                .variant(BadgeVariant::Outline)
                .icon_end(text("♧").text_xs()),
        )
}
