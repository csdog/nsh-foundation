use components::{pagination::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let page = signal(2);
    rsx! {
        <View>
            {
                Pagination::new(
                    PaginationContent::new().item(PaginationItem::new(
                        PaginationLink::action("1", move || page.set(1))
                            .label("Page 1")
                            .current(move || page.get() == 1),
                    )),
                )
            }
        </View>
    }
}
