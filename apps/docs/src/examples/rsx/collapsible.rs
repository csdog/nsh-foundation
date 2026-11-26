use components::{collapsible::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Collapsible::new(
                    CollapsibleTrigger::new("Show details"),
                    CollapsibleContent::new(text("Order details")),
                )
            }
        </View>
    }
}
