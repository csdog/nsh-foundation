use components::{button::button, button_group::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                ButtonGroup::new("File actions")
                    .child(button().child(text("Save")).surface())
                    .child(button().child(text("Duplicate")).surface())
            }
        </View>
    }
}
