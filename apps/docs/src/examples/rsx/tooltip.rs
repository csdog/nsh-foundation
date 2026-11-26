use components::{button::button, rsx, tooltip::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                TooltipContent::new("Add to library")
                    .attach(button().child(text("Add")).surface())
            }
        </View>
    }
}
