use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle},
};
use nsh_base::theme;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let feedback = signal(None::<&'static str>);
    let mut list = view().gap_2().py_2();
    for line in [
        "Choose a schedule (daily, or weekly).",
        "Send to channels or specific teammates.",
        "Include charts, tables, and key metrics.",
    ] {
        list = list.child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    text("›")
                        .text_sm()
                        .text_color(move || theme().muted_foreground),
                )
                .child(text(line).text_sm().flex_1()),
        );
    }
    Card::new()
        .size(CardSize::Small)
        .header(
            CardHeader::new()
                .title(CardTitle::new("Scheduled reports"))
                .description(CardDescription::new(
                    "Weekly snapshots. No more manual exports.",
                )),
        )
        .content(CardContent::new(list.test_id("card-small-body")))
        .footer(CardFooter::new(
            view()
                .gap_2()
                .child(
                    Button::new("Set up scheduled reports")
                        .size(ButtonSize::Small)
                        .on_press(move || feedback.set(Some("Report setup selected."))),
                )
                .child(
                    Button::new("See what's new")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || {
                            feedback.set(Some("Weekly reports can include charts and metrics."))
                        }),
                )
                .child(when(
                    move || feedback.get().is_some(),
                    move || {
                        text(move || SharedString::from(feedback.get().unwrap_or(""))).text_sm()
                    },
                )),
        ))
        .surface()
        .max_w(px(320.0))
        .test_id("card-small")
}
