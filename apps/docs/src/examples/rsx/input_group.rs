use components::{input_group::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                InputGroup::new(
                    input_group_input()
                        .label("Website")
                        .placeholder("example.com"),
                )
                .addon(
                    InputGroupAddon::new(InputGroupAlign::InlineStart)
                        .child(input_group_text("https://")),
                )
            }
        </View>
    }
}
