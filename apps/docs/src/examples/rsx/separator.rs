use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            <Text>Account</Text>
            <Separator />
            <Text>Notifications</Text>
        </View>
    }
}
