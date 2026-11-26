use components::{radio_group::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                RadioGroup::new("Display density")
                    .default_value("comfortable")
                    .item(RadioGroupItem::new("default", "Default"))
                    .item(RadioGroupItem::new("comfortable", "Comfortable"))
                    .item(RadioGroupItem::new("compact", "Compact"))
            }
        </View>
    }
}
