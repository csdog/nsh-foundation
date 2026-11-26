use components::{item::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Item::new().variant(ItemVariant::Outline).content(
                    ItemContent::new()
                        .title(ItemTitle::new("Project notes"))
                        .description(ItemDescription::new("Edited today")),
                )
            }
        </View>
    }
}
