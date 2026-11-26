use components::{popover::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Popover::new(
                    PopoverTrigger::new("Open Popover"),
                    PopoverContent::new()
                        .label("Dimensions")
                        .align(Align::Start)
                        .child(
                            PopoverHeader::new()
                                .child(PopoverTitle::new("Dimensions"))
                                .child(PopoverDescription::new(
                                    "Set the dimensions for the layer.",
                                )),
                        ),
                )
            }
        </View>
    }
}
