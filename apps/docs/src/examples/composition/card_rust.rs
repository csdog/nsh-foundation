use components::{
    button::button,
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    Card::new()
        .header(
            CardHeader::new()
                .title(CardTitle::new("Project settings"))
                .description(CardDescription::new("Manage this project.")),
        )
        .content(CardContent::new(view().child(text("Visibility: Private"))))
        .footer(CardFooter::new(
            view().child(
                button()
                    .child(text("Native Shell docs"))
                    .href("https://nsh.dev/docs/"),
            ),
        ))
}
