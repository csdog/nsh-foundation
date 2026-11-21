use nsh_core::prelude::*;
use nsh_foundation::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "item-1",
            "Can I access my account history?",
            "Yes, you can view your complete account history including all transactions, plan changes, and support tickets in the Account History section of your dashboard.",
            false,
        ),
        (
            "item-2",
            "Premium feature information",
            "This section contains information about premium features. Upgrade your plan to access this content.",
            true,
        ),
        (
            "item-3",
            "How do I update my email address?",
            "You can update your email address in your account settings. You'll receive a verification email at your new address to confirm the change.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&[]);
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
