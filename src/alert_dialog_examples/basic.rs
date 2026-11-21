use nsh_core::prelude::*;
use nsh_foundation::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};

pub fn preview() -> impl IntoElement {
    let dialog = AlertDialog::new(signal(false));
    let confirmed = signal(false);
    let header = AlertDialogHeader::new(
        AlertDialogTitle::new("Are you absolutely sure?"),
        AlertDialogDescription::new(
            "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
        ),
    );
    let footer = AlertDialogFooter::new(
        AlertDialogCancel::new("Cancel"),
        AlertDialogAction::new("Continue", move || confirmed.set(true)),
    );
    view()
        .gap_2()
        .items_center()
        .child(AlertDialogTrigger::new(dialog, "Show Dialog"))
        .child(AlertDialogContent::new(dialog, header, footer))
        .child(when(confirmed, || {
            text("Confirmed in this preview.")
                .role(AccessibilityRole::Alert)
                .text_sm()
        }))
}
