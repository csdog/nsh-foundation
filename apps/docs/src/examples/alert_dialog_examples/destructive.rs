use components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogTitle, AlertDialogTrigger,
};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let dialog = AlertDialog::new(signal(false));
    let confirmed = signal(false);
    let header = AlertDialogHeader::new(AlertDialogTitle::new("Delete chat?"), AlertDialogDescription::new("This will permanently delete this chat conversation. View{\" \"} Settings delete any memories saved during this chat.")).media(AlertDialogMedia::new("×").destructive(true));
    let footer = AlertDialogFooter::new(
        AlertDialogCancel::new("Cancel"),
        AlertDialogAction::new("Delete", move || confirmed.set(true)).destructive(true),
    );
    view()
        .gap_2()
        .items_center()
        .child(AlertDialogTrigger::new(dialog, "Delete Chat"))
        .child(AlertDialogContent::new(dialog, header, footer))
        .child(when(confirmed, || {
            text("Confirmed in this preview.")
                .role(AccessibilityRole::Alert)
                .text_sm()
        }))
}
