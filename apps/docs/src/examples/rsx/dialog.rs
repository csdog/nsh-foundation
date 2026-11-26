use components::{dialog::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let dialog = Dialog::new(signal(false));
    rsx! {
        <View>
            {DialogTrigger::new(dialog, "Edit profile")}
            {
                DialogContent::new(
                    dialog,
                    DialogHeader::new(DialogTitle::new("Edit profile")),
                )
                .body(|| view().child(text("Update your display name.")))
            }
        </View>
    }
}
