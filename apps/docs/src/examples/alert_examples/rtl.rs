use components::alert::{Alert, AlertDescription, AlertTitle};
use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view().w_full().gap_4()
    .child(Alert::new().direction(LayoutDirection::Rtl).icon(text("✓").text_xs().text_center().w(px(16.0)).h(px(16.0)).border(px(1.0)).rounded_full()).title(AlertTitle::new("تم الدفع بنجاح")).description(AlertDescription::new("تمت معالجة دفعتك البالغة 29.99 دولارًا. تم إرسال إيصال إلى عنوان بريدك الإلكتروني.")))
    .child(Alert::new().direction(LayoutDirection::Rtl).icon(text("i").text_xs().text_center().w(px(16.0)).h(px(16.0)).border(px(1.0)).rounded_full()).title(AlertTitle::new("ميزة جديدة متاحة")).description(AlertDescription::new("لقد أضفنا دعم الوضع الداكن. يمكنك تفعيله في إعدادات حسابك.")))
}
