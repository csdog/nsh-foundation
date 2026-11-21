use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;
use nsh_foundation::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "item-1",
            "كيف يمكنني إعادة تعيين كلمة المرور؟",
            "انقر على 'نسيت كلمة المرور' في صفحة تسجيل الدخول، أدخل عنوان بريدك الإلكتروني، وسنرسل لك رابطًا لإعادة تعيين كلمة المرور. سينتهي صلاحية الرابط خلال 24 ساعة.",
            false,
        ),
        (
            "item-2",
            "هل يمكنني تغيير خطة الاشتراك الخاصة بي؟",
            "نعم، يمكنك ترقية أو تخفيض خطتك في أي وقت من إعدادات حسابك. ستظهر التغييرات في دورة الفوترة التالية.",
            false,
        ),
        (
            "item-3",
            "ما هي طرق الدفع التي تقبلونها؟",
            "نقبل جميع بطاقات الائتمان الرئيسية و PayPal والتحويلات المصرفية. تتم معالجة جميع المدفوعات بأمان من خلال شركاء الدفع لدينا.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["item-1"]).direction(LayoutDirection::Rtl);
    for (id, question, answer, disabled) in items {
        accordion = accordion.child(
            AccordionItem::new(
                id,
                AccordionTrigger::new(question),
                AccordionContent::new(text(answer).text_sm()),
            )
            .disabled(disabled),
        );
    }
    accordion
}
