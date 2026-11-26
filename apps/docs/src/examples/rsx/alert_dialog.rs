use components::{alert_dialog::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let dialog = AlertDialog::new(signal(false));
    rsx! {
        <View>
            {AlertDialogTrigger::new(dialog, "Remove item")}
            {
                AlertDialogContent::new(
                    dialog,
                    AlertDialogHeader::new(
                        AlertDialogTitle::new("Remove item?"),
                        AlertDialogDescription::new("You can add it again later."),
                    ),
                    AlertDialogFooter::new(
                        AlertDialogCancel::new("Cancel"),
                        AlertDialogAction::new("Remove", || {}),
                    ),
                )
            }
        </View>
    }
}
