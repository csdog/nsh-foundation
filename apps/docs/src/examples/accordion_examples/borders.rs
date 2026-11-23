use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "billing",
            "How does billing work?",
            "We offer monthly and annual subscription plans. Billing is charged at the beginning of each cycle, and you can cancel anytime. All plans include automatic backups, 24/7 support, and unlimited team members.",
            false,
        ),
        (
            "security",
            "Is my data secure?",
            "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and regular third-party security audits. All data is encrypted at rest and in transit using industry-standard protocols.",
            false,
        ),
        (
            "integration",
            "What integrations do you support?",
            "We integrate with 500+ popular tools including Slack, Zapier, Salesforce, HubSpot, and more. You can also build custom integrations using our REST API and webhooks.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["billing"]).radius(10.0);
    for (id, question, answer, disabled) in items {
        accordion = accordion.child(
            AccordionItem::new(
                id,
                AccordionTrigger::new(question),
                AccordionContent::new(text(answer).text_sm()),
            )
            .disabled(disabled)
            .inset(16.0),
        );
    }
    accordion
}
