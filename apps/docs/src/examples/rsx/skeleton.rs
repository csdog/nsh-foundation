use components::{rsx, skeleton::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Skeleton::new()
                    .surface()
                    .w(px(100.0))
                    .h(px(20.0))
                    .rounded_full()
            }
        </View>
    }
}
