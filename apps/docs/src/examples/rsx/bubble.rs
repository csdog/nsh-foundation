use components::{bubble::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                BubbleGroup::new()
                    .child(Bubble::new(BubbleContent::text("Your order has shipped.")))
            }
        </View>
    }
}
