use components::{menubar::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Menubar::new("Application menu").menu(MenubarMenu::new(
                    MenubarTrigger::new("File"),
                    MenubarContent::new().child(MenubarItem::new("New Tab")),
                ))
            }
        </View>
    }
}
