use nsh_core::prelude::*;
use nsh_foundation::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogSize, AlertDialogTitle,
    AlertDialogTrigger,
};

pub fn preview() -> impl IntoElement {
    let dialog = AlertDialog::new(signal(false));
    let confirmed = signal(false);
    let header = AlertDialogHeader::new(
        AlertDialogTitle::new("Allow accessory to connect?"),
        AlertDialogDescription::new(
            "Do you want to allow the USB accessory to connect to this device?",
        ),
    )
    .media(AlertDialogMedia::new("ᛒ"));
    let footer = AlertDialogFooter::new(
        AlertDialogCancel::new("Don't allow"),
        AlertDialogAction::new("Allow", move || confirmed.set(true)),
    );
    view()
        .gap_2()
        .items_center()
        .child(AlertDialogTrigger::new(dialog, "Show Dialog"))
        .child(AlertDialogContent::new(dialog, header, footer).size(AlertDialogSize::Small))
        .child(when(confirmed, || {
            text("Confirmed in this preview.")
                .role(AccessibilityRole::Alert)
                .text_sm()
        }))
}
