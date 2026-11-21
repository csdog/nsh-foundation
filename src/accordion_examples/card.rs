use nsh_base::theme;
use nsh_core::prelude::*;
use nsh_foundation::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

pub fn preview() -> impl IntoElement {
    let items = [
        (
            "plans",
            "What subscription plans do you offer?",
            "We offer three subscription tiers: Starter ($9/month), Professional ($29/month), and Enterprise ($99/month). Each plan includes increasing storage limits, API access, priority support, and team collaboration features.",
            false,
        ),
        (
            "billing",
            "How does billing work?",
            "Billing occurs automatically at the start of each billing cycle. We accept all major credit cards, PayPal, and ACH transfers for enterprise customers. You'll receive an invoice via email after each payment.",
            false,
        ),
        (
            "cancel",
            "How do I cancel my subscription?",
            "You can cancel your subscription anytime from your account settings. There are no cancellation fees or penalties. Your access will continue until the end of your current billing period.",
            false,
        ),
    ];
    let mut accordion = Accordion::new(&["plans"]);
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
    view()
        .w_full()
        .gap_4()
        .rounded(px(20.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .p_4()
        .child(text("Subscription & Billing").font_weight(FontWeight::SEMIBOLD))
        .child(
            text("Common questions about your account, plans, payments and cancellations.")
                .text_sm(),
        )
        .child(accordion)
}
