use components::{rsx, toggle::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Toggle::new("Bookmark")
                    .label("Toggle bookmark")
                    .variant(ToggleVariant::Outline)
                    .size(ToggleSize::Small)
            }
        </View>
    }
}
