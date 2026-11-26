use components::{rsx, trace::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Trace::new().child(
                    TraceLine::new(TraceTitle::new("Ran pnpm test"))
                        .description(TraceDescription::new("14s")),
                )
            }
        </View>
    }
}
