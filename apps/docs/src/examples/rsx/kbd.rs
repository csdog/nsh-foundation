use components::{kbd::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {Kbd::new("Ctrl").named("Control")}
        </View>
    }
}
