use components::{breadcrumb::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Breadcrumb::new(
                    BreadcrumbList::new()
                        .child(BreadcrumbItem::new(BreadcrumbLink::new(
                            "Home",
                            "https://nsh.dev/",
                        )))
                        .child(BreadcrumbItem::new(BreadcrumbPage::new("Settings"))),
                )
            }
        </View>
    }
}
