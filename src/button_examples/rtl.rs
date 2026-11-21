use nsh_core::{element::LayoutDirection, prelude::*};
use nsh_foundation::button::{Button, ButtonSize, ButtonVariant};

pub fn preview() -> impl IntoElement {
    let status = signal("");
    view()
        .direction(LayoutDirection::Rtl)
        .items_center()
        .gap_2()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_2()
                .child(
                    Button::new("زر")
                        .variant(ButtonVariant::Outline)
                        .on_press(move || status.set("تم الضغط")),
                )
                .child(
                    Button::new("حذف")
                        .variant(ButtonVariant::Destructive)
                        .on_press(move || status.set("تم تحديد الحذف")),
                )
                .child(
                    Button::new("إرسال")
                        .variant(ButtonVariant::Outline)
                        .icon_end(text("←"))
                        .on_press(move || status.set("تم الإرسال")),
                )
                .child(
                    Button::new("إضافة")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .icon_start(text("+"))
                        .on_press(move || status.set("تمت الإضافة")),
                )
                .child(
                    Button::new("جاري التحميل")
                        .variant(ButtonVariant::Secondary)
                        .loading(true),
                ),
        )
        .child(text(move || status.get()).text_sm())
}
