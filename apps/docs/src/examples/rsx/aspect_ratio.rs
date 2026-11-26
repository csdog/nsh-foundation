use components::{aspect_ratio::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                AspectRatio::new(
                    16.0 / 9.0,
                    view()
                        .w_full()
                        .h_full()
                        .items_center()
                        .justify_center()
                        .child(text("16:9")),
                )
            }
        </View>
    }
}
