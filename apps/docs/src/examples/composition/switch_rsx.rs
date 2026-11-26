use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let notifications = signal(false);
    rsx! {
        <View style={{ gap: 8 }}>
            <Text>Notifications</Text>
            <Switch
                accessibilityLabel="Notifications"
                checked={notifications}
                onChange={move |checked| notifications.set(checked)}
            />
            <Text>{text(move || if notifications.get() { "On" } else { "Off" })}</Text>
        </View>
    }
}
