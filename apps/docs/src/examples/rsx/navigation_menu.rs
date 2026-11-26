use components::{navigation_menu::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                NavigationMenu::new(
                    "Site navigation",
                    NavigationMenuList::new().item(NavigationMenuItem::link(
                        NavigationMenuLink::new("Docs", "https://nsh.dev/docs/"),
                    )),
                )
            }
        </View>
    }
}
