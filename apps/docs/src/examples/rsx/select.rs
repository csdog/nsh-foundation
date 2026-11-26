use components::{rsx, select::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Select::new(
                    "Theme",
                    SelectTrigger::new(SelectValue::new("Choose a theme")),
                    SelectContent::new()
                        .item(SelectItem::new("light", "Light"))
                        .item(SelectItem::new("dark", "Dark"))
                        .item(SelectItem::new("system", "System")),
                )
            }
        </View>
    }
}
