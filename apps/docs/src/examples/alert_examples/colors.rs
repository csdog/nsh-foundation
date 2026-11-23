use components::alert::{Alert, AlertColors, AlertDescription, AlertTitle};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Alert::new()
        .colors(AlertColors {
            background: AdaptiveColor::rgb(0xfffbeb, 0x451a03),
            border: AdaptiveColor::rgb(0xfde68a, 0x78350f),
            foreground: AdaptiveColor::rgb(0x78350f, 0xfffbeb),
            description: AdaptiveColor::rgb(0xb45309, 0xfcd34d),
        })
        .icon(text("⚠").text_xs().text_center().w(px(16.0)).h(px(16.0)))
        .title(AlertTitle::new("Your subscription will expire in 3 days."))
        .description(AlertDescription::new(concat!(
            "Renew now to avoid service interruption or upgrade to a paid plan ",
            "to continue using the service.",
        )))
}
