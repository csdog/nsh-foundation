use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "notifications",
            "Notification Settings",
            "Manage how you receive notifications. You can enable email alerts for updates or push notifications for mobile devices.",
            false,
        ),
        (
            "privacy",
            "Privacy & Security",
            "Control your privacy settings and security preferences. Enable two-factor authentication, manage connected devices, review active sessions, and configure data sharing preferences. You can also download your data or delete your account.",
            false,
        ),
        (
            "billing",
            "Billing & Subscription",
            "View your current plan, payment history, and upcoming invoices. Update your payment method, change your subscription tier, or cancel your subscription.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["notifications"]).multiple(true);
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
