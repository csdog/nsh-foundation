use components::{
    bubble::{Bubble, BubbleContent},
    message::*,
    rsx,
};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Message::new(
                    MessageContent::new()
                        .child(Bubble::new(BubbleContent::text("How can I help?")))
                        .footer(MessageFooter::text("Delivered")),
                )
            }
        </View>
    }
}
