use components::breadcrumb::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Breadcrumb::new(
        BreadcrumbList::new()
            .child(BreadcrumbItem::new(BreadcrumbLink::new(
                "Home",
                "https://ui.shadcn.com/",
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbLink::new(
                "Components",
                "https://ui.shadcn.com/docs/components",
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbPage::new("Breadcrumb"))),
    )
}
