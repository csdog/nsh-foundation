use components::{hover_card::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                HoverCard::new(
                    HoverCardTrigger::new("Hover Here"),
                    HoverCardContent::new().child(text("Preview content")),
                )
            }
        </View>
    }
}
