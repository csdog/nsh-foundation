use nsh_core::prelude::*;
use nsh_foundation::alert::{Alert, AlertAction, AlertDescription, AlertTitle};

pub fn preview() -> impl IntoElement {
    let enabled = signal(false);
    Alert::new()
        .title(AlertTitle::new("Dark mode is now available"))
        .description(AlertDescription::new(move || {
            if enabled.get() {
                "Preference saved for this example."
            } else {
                "Enable it under your profile settings to get started."
            }
        }))
        .action(AlertAction::new(
            view()
                .role(AccessibilityRole::Button)
                .label("Enable")
                .cursor_pointer()
                .disabled(enabled)
                .on_press(move || enabled.set(true))
                .bg(move || nsh_base::theme().primary)
                .text_color(move || nsh_base::theme().primary_foreground)
                .rounded(px(6.0))
                .px_2()
                .py_1()
                .focus_visible(|style| style.ring(px(2.0)).ring_color(nsh_base::theme().ring))
                .child(text(move || if enabled.get() { "Enabled" } else { "Enable" }).text_xs()),
        ))
}
