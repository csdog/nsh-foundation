use nsh_base::{menu, menu_item, theme};
use nsh_core::prelude::*;
use nsh_foundation::breadcrumb::*;

pub fn preview() -> impl IntoElement {
    let item = |label: &'static str, url: &'static str| {
        menu_item()
            .accessibility_label(label)
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || {
                let _ = open_url(url);
            })
            .child(text(label).text_sm())
    };
    let dropdown = menu()
        .offset(px(6.0))
        .margin(px(8.0))
        .popup_class("w-36 p-1 bg-popover text-popover-foreground rounded-lg border shadow-md")
        .trigger(
            BreadcrumbEllipsis::surface()
                .role(AccessibilityRole::Button)
                .label("Toggle breadcrumb menu")
                .cursor_pointer()
                .rounded(px(4.0))
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring)),
        )
        .child(item("Documentation", "https://ui.shadcn.com/docs"))
        .child(item("Themes", "https://ui.shadcn.com/themes"))
        .child(item("GitHub", "https://github.com/shadcn-ui/ui"));
    Breadcrumb::new(
        BreadcrumbList::new()
            .child(BreadcrumbItem::new(BreadcrumbLink::new(
                "Home",
                "https://ui.shadcn.com/",
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(dropdown))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbLink::new(
                "Components",
                "https://ui.shadcn.com/docs/components",
            )))
            .separator(BreadcrumbSeparator::new())
            .child(BreadcrumbItem::new(BreadcrumbPage::new("Breadcrumb"))),
    )
}
