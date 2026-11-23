use components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogSize, AlertDialogTitle,
    AlertDialogTrigger,
};
use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .gap_2()
        .direction(LayoutDirection::Rtl)
        .child({
            let dialog = AlertDialog::new(signal(false));
            let confirmed = signal(false);
            let header = AlertDialogHeader::new(
                AlertDialogTitle::new("هل أنت متأكد تمامًا؟"),
                AlertDialogDescription::new(
                    "لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائيًا من خوادمنا.",
                ),
            );
            let footer = AlertDialogFooter::new(
                AlertDialogCancel::new("إلغاء"),
                AlertDialogAction::new("متابعة", move || confirmed.set(true)),
            );
            view()
                .gap_2()
                .items_center()
                .child(AlertDialogTrigger::new(dialog, "إظهار الحوار"))
                .child(
                    AlertDialogContent::new(dialog, header, footer).direction(LayoutDirection::Rtl),
                )
                .child(when(confirmed, || {
                    text("تم التأكيد في المعاينة.")
                        .role(AccessibilityRole::Alert)
                        .text_sm()
                }))
        })
        .child({
            let dialog = AlertDialog::new(signal(false));
            let confirmed = signal(false);
            let header = AlertDialogHeader::new(
                AlertDialogTitle::new("السماح للملحق بالاتصال؟"),
                AlertDialogDescription::new("هل تريد السماح لملحق USB بالاتصال بهذا الجهاز؟"),
            )
            .media(AlertDialogMedia::new("ᛒ"));
            let footer = AlertDialogFooter::new(
                AlertDialogCancel::new("عدم السماح"),
                AlertDialogAction::new("السماح", move || confirmed.set(true)),
            );
            view()
                .gap_2()
                .items_center()
                .child(AlertDialogTrigger::new(dialog, "إظهار الحوار (صغير)"))
                .child(
                    AlertDialogContent::new(dialog, header, footer)
                        .size(AlertDialogSize::Small)
                        .direction(LayoutDirection::Rtl),
                )
                .child(when(confirmed, || {
                    text("تم التأكيد في المعاينة.")
                        .role(AccessibilityRole::Alert)
                        .text_sm()
                }))
        })
}
