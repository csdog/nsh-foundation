use components::{dropdown_menu::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                DropdownMenu::new(
                    DropdownMenuTrigger::new("Open"),
                    DropdownMenuContent::new().child(DropdownMenuItem::new("Profile")),
                )
            }
        </View>
    }
}
