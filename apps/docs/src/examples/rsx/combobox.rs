use components::{combobox::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Combobox::new(
                    "Choose a framework",
                    [
                        ComboboxItem::new("rust", "Rust"),
                        ComboboxItem::new("swift", "Swift"),
                    ],
                )
            }
        </View>
    }
}
