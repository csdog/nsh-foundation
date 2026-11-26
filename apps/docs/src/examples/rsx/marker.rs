use components::{marker::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Marker::new(MarkerContent::new("Explored 4 files"))
                    .icon(MarkerIcon::new(text("✓")))
            }
        </View>
    }
}
