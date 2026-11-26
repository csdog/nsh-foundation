use components::{direction::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                DirectionProvider::new(Direction::Rtl, || {
                    view().child(text("Content follows right-to-left layout"))
                })
            }
        </View>
    }
}
