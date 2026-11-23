use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "item-1",
            "How do I reset my password?",
            "Click on 'Forgot Password' on the login page, enter your email address, and we'll send you a link to reset your password. The link will expire in 24 hours.",
            false,
        ),
        (
            "item-2",
            "Can I change my subscription plan?",
            "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes will be reflected in your next billing cycle.",
            false,
        ),
        (
            "item-3",
            "What payment methods do you accept?",
            "We accept all major credit cards, PayPal, and bank transfers. All payments are processed securely through our payment partners.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["item-1"]);
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
