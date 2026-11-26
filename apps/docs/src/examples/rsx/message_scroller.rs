use components::{message_scroller::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let handle = MessageScrollerHandle::new();
    let messages = signal(vec![
        MessageScrollerItem::new("hello", "Hello!"),
        MessageScrollerItem::new("reply", "How can I help?"),
    ]);
    rsx! {
        <View>
            {
                MessageScroller::new(handle, messages)
                    .height(160.0)
                    .render(|item| {
                        text(move || item.with(|item| item.value)).into_element()
                    })
            }
        </View>
    }
}
