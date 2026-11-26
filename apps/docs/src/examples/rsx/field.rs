use components::{field::*, input::input, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let control = element_ref();
    rsx! {
        <View>
            {
                Field::new()
                    .child(FieldLabel::new("Email").for_control(control))
                    .child(input().label("Email").element_ref(&control))
                    .child(FieldDescription::new("Use your work email."))
            }
        </View>
    }
}
