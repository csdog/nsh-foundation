use components::{progress::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Progress::new(56.0)
                    .label(ProgressLabel::new("Upload progress"))
                    .display(ProgressValue::new())
            }
        </View>
    }
}
