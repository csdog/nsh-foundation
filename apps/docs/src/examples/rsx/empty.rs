use components::{button::Button, empty::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Empty::new()
                    .child(
                        EmptyHeader::new()
                            .child(EmptyTitle::new("No data"))
                            .child(EmptyDescription::new("No data found")),
                    )
                    .child(EmptyContent::new().child(Button::new("Add data")))
            }
        </View>
    }
}
