use nsh_core::prelude::*;
use nsh_foundation::alert::{Alert, AlertDescription, AlertTitle};

pub fn preview() -> impl IntoElement {
    Alert::new()
        .icon(
            text("✓")
                .text_xs()
                .text_center()
                .w(px(16.0))
                .h(px(16.0))
                .border(px(1.0))
                .rounded_full(),
        )
        .title(AlertTitle::new("Account updated successfully"))
        .description(AlertDescription::new(
            "Your profile information has been saved. Changes will be reflected immediately.",
        ))
}
