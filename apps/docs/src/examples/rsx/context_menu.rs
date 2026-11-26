use components::{context_menu::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {ContextMenu::new(view().p_4().child(text("Right-click here"))).child(ContextMenuItem::new("Refresh"))}
        </View>
    }
}
