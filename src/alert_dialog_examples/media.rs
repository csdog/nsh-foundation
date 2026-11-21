use nsh_core::prelude::*;
use nsh_foundation::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogTitle, AlertDialogTrigger,
};

pub fn preview() -> impl IntoElement {
    let dialog = AlertDialog::new(signal(false));
    let confirmed = signal(false);
    let header = AlertDialogHeader::new(
        AlertDialogTitle::new("Share this project?"),
        AlertDialogDescription::new(
            "Anyone with the link will be able to view and edit this project.",
        ),
    )
    .media(AlertDialogMedia::new("+"));
    let footer = AlertDialogFooter::new(
        AlertDialogCancel::new("Cancel"),
        AlertDialogAction::new("Share", move || confirmed.set(true)),
    );
    view()
        .gap_2()
        .items_center()
        .child(AlertDialogTrigger::new(dialog, "Share Project"))
        .child(AlertDialogContent::new(dialog, header, footer))
        .child(when(confirmed, || {
            text("Confirmed in this preview.")
                .role(AccessibilityRole::Alert)
                .text_sm()
        }))
}
