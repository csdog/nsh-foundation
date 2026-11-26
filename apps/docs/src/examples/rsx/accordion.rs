use components::{accordion::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Accordion::new(&["shipping"]).child(AccordionItem::new(
                    "shipping",
                    AccordionTrigger::new("Shipping"),
                    AccordionContent::new(text("Orders ship on weekdays.")),
                ))
            }
        </View>
    }
}
