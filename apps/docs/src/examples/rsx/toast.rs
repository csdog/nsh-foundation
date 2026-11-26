use components::{rsx, toast::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let toaster = Toaster::new();
    rsx! {
        <View>
            <Button onPress={move || { toaster.show(Toast::new("Event created")); }}>Show toast</Button>
            {toaster.viewport()}
        </View>
    }
}
