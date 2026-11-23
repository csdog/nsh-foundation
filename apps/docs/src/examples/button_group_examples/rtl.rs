use components::button::{Button, ButtonSize, ButtonVariant};
use components::button_group::ButtonGroup;
use nsh_base::{menu, menu_item, menu_separator, submenu, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

pub fn preview() -> impl IntoElement {
    let status = signal("");
    let label = signal("شخصي");
    let outline = |name| Button::new(name).variant(ButtonVariant::Outline);
    let item = move |name: &'static str| {
        menu_item()
            .accessibility_label(name)
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || status.set(name))
            .child(text(name).text_sm().text_color(move || {
                if name == "سلة المهملات" {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            }))
    };
    let radio = move |name: &'static str| {
        menu_item()
            .accessibility_label(name)
            .role(AccessibilityRole::MenuItemRadio)
            .checked(move || Some(label.get() == name))
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|s| s.bg(theme().muted))
            .focus(|s| s.bg(theme().muted))
            .on_select(move || {
                label.set(name);
                status.set(name);
            })
            .child(text(name).text_sm())
    };
    let more = move |join: components::button_group::ButtonGroupJoin| {
        menu()
            .align(Align::Start)
            .offset(px(5.0))
            .margin(px(8.0))
            .popup_class("w-40 p-1 bg-popover text-popover-foreground rounded-xl border shadow-md")
            .trigger(
                join.apply(
                    outline("خيارات إضافية")
                        .size(ButtonSize::Icon)
                        .icon_start(text("…"))
                        .surface(),
                ),
            )
            .child(item("وضع علامة كمقروء"))
            .child(item("أرشفة"))
            .child(separator())
            .child(item("تأجيل"))
            .child(item("إضافة إلى التقويم"))
            .child(item("إضافة إلى القائمة"))
            .child(
                submenu()
                    .trigger(view().px_2().py_1().child(text("تصنيف كـ…").text_sm()))
                    .child(radio("شخصي"))
                    .child(radio("عمل"))
                    .child(radio("آخر")),
            )
            .child(separator())
            .child(item("سلة المهملات"))
    };

    view()
        .items_center()
        .gap_2()
        .child(
            view()
                .direction(LayoutDirection::Rtl)
                .flex_row()
                .gap_2()
                .child(when(
                    || window().width.0 >= 640.0,
                    move || {
                        outline("رجوع")
                            .size(ButtonSize::Icon)
                            .icon_start(text("→"))
                            .on_press(move || status.set("رجوع"))
                    },
                ))
                .child(
                    ButtonGroup::new("إجراءات الرسائل")
                        .direction(LayoutDirection::Rtl)
                        .separate(
                            ButtonGroup::new("الأرشفة والتقرير")
                                .child(outline("أرشفة").on_press(move || status.set("أرشفة")))
                                .child(outline("تقرير").on_press(move || status.set("تقرير"))),
                        )
                        .separate(
                            ButtonGroup::new("التأجيل والمزيد")
                                .child(outline("تأجيل").on_press(move || status.set("تأجيل")))
                                .control(more),
                        ),
                ),
        )
        .child(when(
            move || !status.get().is_empty(),
            move || text(move || format!("اختيار المعاينة: {}", status.get())).text_xs(),
        ))
}

fn separator() -> impl IntoElement {
    menu_separator()
        .h(px(1.0))
        .my_1()
        .bg(move || theme().border)
}
