use components::{rsx, scroll_area::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {ScrollArea::new("Release tags", rsx! {
                <View><Text>v1.0</Text><Text>v1.1</Text><Text>v1.2</Text></View>
            }).height(100.0).scroll_bar(ScrollBar::new(ScrollAreaOrientation::Vertical))}
        </View>
    }
}
