use nsh_base::{menu, menu_item, theme};
use nsh_core::element::LayoutDirection;
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
            .child(text(label).text_sm().text_right())
    };
    let dropdown = menu()
        .align(Align::End)
        .offset(px(6.0))
        .margin(px(8.0))
        .popup_class("w-36 p-1 bg-popover text-popover-foreground rounded-lg border shadow-md")
        .trigger(
            view()
                .role(AccessibilityRole::Button)
                .label("المكونات")
                .flex_row()
                .items_center()
                .gap_1()
                .child(text("المكونات").text_sm())
                .child(text("⌄").text_sm())
                .cursor_pointer()
                .rounded(px(4.0))
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring)),
        )
        .child(item("التوثيق", "https://ui.shadcn.com/docs"))
        .child(item("السمات", "https://ui.shadcn.com/themes"))
        .child(item("جيت هاب", "https://github.com/shadcn-ui/ui"));
    Breadcrumb::new(
        BreadcrumbList::new()
            .direction(LayoutDirection::Rtl)
            .child(BreadcrumbItem::new(BreadcrumbLink::new(
                "الرئيسية",
                "https://ui.shadcn.com/",
            )))
            .separator(BreadcrumbSeparator::render_with(text("·").text_sm()))
            .child(BreadcrumbItem::new(dropdown))
            .separator(BreadcrumbSeparator::render_with(text("·").text_sm()))
            .child(BreadcrumbItem::new(BreadcrumbPage::new("مسار التنقل"))),
    )
    .label("مسار التنقل")
}
