use components::{date_picker::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {DatePicker::new("Pick a date").label("Delivery date")}
        </View>
    }
}
