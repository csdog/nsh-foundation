use components::{checkbox::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {Checkbox::new("Send updates").default_checked(true)}
        </View>
    }
}
