use components::{command::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Command::new(
                    "Commands",
                    [
                        CommandItem::new("search", "Search"),
                        CommandItem::new("settings", "Settings"),
                    ],
                )
            }
        </View>
    }
}
