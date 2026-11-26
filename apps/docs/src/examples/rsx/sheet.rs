use components::{rsx, sheet::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let sheet = Sheet::new(signal(false));
    rsx! {
        <View>
            {
                view().child(SheetTrigger::new(sheet, "Open")).child(
                    SheetContent::new(
                        sheet,
                        SheetHeader::new(SheetTitle::new("Edit profile")),
                    )
                    .side(SheetSide::Right)
                    .footer(move || {
                        SheetFooter::new().child(SheetClose::new(sheet, "Close"))
                    }),
                )
            }
        </View>
    }
}
