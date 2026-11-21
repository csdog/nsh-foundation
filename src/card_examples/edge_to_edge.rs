use nsh_base::theme;
use nsh_core::prelude::*;
use nsh_foundation::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
};

pub fn preview() -> impl IntoElement {
    let feedback = signal(None::<&'static str>);
    let mut terms = view().px(px(20.0)).py_4().gap_4();
    for paragraph in [
        "These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools.",
        "You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it.",
        "We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators.",
        "By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies.",
    ] {
        terms = terms.child(text(paragraph).text_sm());
    }
    let body = scroll()
        .h(px(192.0))
        .w_full()
        .test_id("card-terms")
        .bg(move || theme().muted.opacity(0.5))
        .border_t(px(1.0))
        .border_color(move || theme().border)
        .child(terms);
    let actions = view()
        .gap_2()
        .child(
            view()
                .flex_row()
                .justify_end()
                .gap_2()
                .test_id("card-terms-actions")
                .child(
                    Button::new("Decline")
                        .variant(ButtonVariant::Outline)
                        .on_press(move || feedback.set(Some("Terms declined."))),
                )
                .child(
                    Button::new("Accept").on_press(move || feedback.set(Some("Terms accepted."))),
                ),
        )
        .child(when(
            move || feedback.get().is_some(),
            move || text(move || SharedString::from(feedback.get().unwrap_or(""))).text_sm(),
        ));
    Card::new()
        .header(
            CardHeader::new()
                .title(CardTitle::new("Terms of Service"))
                .description(CardDescription::new(
                    "Review the terms before accepting the agreement.",
                )),
        )
        .content(CardContent::new(body).edge_to_edge().flush_footer())
        .footer(CardFooter::new(actions))
        .surface()
        .max_w(px(384.0))
        .test_id("card-edge")
}
