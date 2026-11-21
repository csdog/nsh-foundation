use nsh_core::prelude::*;
use nsh_foundation::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "shipping",
            "What are your shipping options?",
            "We offer standard (5-7 days), express (2-3 days), and overnight shipping. Free shipping on international orders.",
            false,
        ),
        (
            "returns",
            "What is your return policy?",
            "Returns accepted within 30 days. Items must be unused and in original packaging. Refunds processed within 5-7 business days.",
            false,
        ),
        (
            "support",
            "How can I contact customer support?",
            "Reach us via email, live chat, or phone. We respond within 24 hours during business days.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["shipping"]);
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
