use components::{rsx, slider::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Slider::new([33.0])
                    .range(0.0, 100.0)
                    .step(1.0)
                    .labels(["Volume"])
            }
        </View>
    }
}
