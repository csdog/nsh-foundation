use components::{rsx, toggle::ToggleVariant, toggle_group::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                ToggleGroup::new("Call filter")
                    .default_value(["all"])
                    .variant(ToggleVariant::Outline)
                    .spacing(0.0)
                    .items([
                        ToggleGroupItem::new("all", "All"),
                        ToggleGroupItem::new("missed", "Missed"),
                    ])
            }
        </View>
    }
}
