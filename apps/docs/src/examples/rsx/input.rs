use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            <Input label="Email" placeholder="name@example.com" />
        </View>
    }
}
