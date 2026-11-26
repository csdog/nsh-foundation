use components::{input::input, label::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let target = element_ref();
    rsx! {
        <View>
            {Label::new("Email").for_control(target)}
            {input().label("Email").element_ref(&target)}
        </View>
    }
}
