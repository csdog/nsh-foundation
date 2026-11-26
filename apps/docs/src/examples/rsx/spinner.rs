use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Spinner size={24.0} accessibilityLabel="Loading messages" />
    }
}
