use components::breadcrumb::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    Breadcrumb::new(
        BreadcrumbList::new()
            .child(BreadcrumbItem::new(BreadcrumbLink::render(
                "Home",
                view().child(text("Home").text_sm()),
                || {
                    let _ = open_url(
                        "https://ui.shadcn.com/docs/components/base/breadcrumb#link-component",
                    );
                },
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbLink::render(
                "Components",
                view().child(text("Components").text_sm()),
                || {
                    let _ = open_url(
                        "https://ui.shadcn.com/docs/components/base/breadcrumb#link-component",
                    );
                },
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbPage::new("Breadcrumb"))),
    )
}
