use nsh_core::prelude::*;
use nsh_foundation::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

pub fn preview() -> impl IntoElement {
    Alert::new()
        .variant(AlertVariant::Destructive)
        .icon(
            text("!")
                .text_xs()
                .text_center()
                .w(px(16.0))
                .h(px(16.0))
                .border(px(1.0))
                .rounded_full(),
        )
        .title(AlertTitle::new("Payment failed"))
        .description(AlertDescription::new(
            "Your payment could not be processed. Please check your payment method and try again.",
        ))
}
