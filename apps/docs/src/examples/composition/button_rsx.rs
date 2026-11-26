use components::rsx;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let saved = signal(false);
    rsx! {
        <View style={{ gap: 8 }}>
            <Button onPress={move || saved.set(true)} style={{ paddingHorizontal: 16 }}>
                <Text>Save</Text>
            </Button>
            <Text>{text(move || if saved.get() { "Saved." } else { "" })}</Text>
        </View>
    }
}
