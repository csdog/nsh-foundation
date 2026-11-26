use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            <Textarea label="Message" placeholder="Type your message here." />
        </View>
    }
}
