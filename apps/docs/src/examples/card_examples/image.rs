use components::{
    aspect_ratio::AspectRatio,
    badge::{Badge, BadgeVariant},
    button::Button,
    card::{Card, CardAction, CardDescription, CardFooter, CardHeader, CardTitle},
};
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let showing = signal(false);
    let cover = AspectRatio::new(
        16.0 / 9.0,
        view()
            .flex_1()
            .child(
                image(asset!("aspect-ratio.png"))
                    .label("Event cover")
                    .w_full()
                    .h_full()
                    .object_fit(ResizeMode::Cover)
                    .grayscale(1.0)
                    .brightness(0.6)
                    .style_when(dark, |style| style.brightness(0.4)),
            )
            .child(view().absolute().inset_0().bg(hsla(0.0, 0.0, 0.0, 0.35))),
    );
    Card::new()
        .media(cover.test_id("card-event-cover"))
        .header(
            CardHeader::new()
                .title(CardTitle::new("Design systems meetup"))
                .description(CardDescription::new(
                    "A practical talk on component APIs, accessibility, and shipping faster.",
                ))
                .action(CardAction::new(
                    Badge::new("Featured").variant(BadgeVariant::Secondary),
                )),
        )
        .footer(CardFooter::new(
            view()
                .gap_2()
                .child(
                    Button::new("View Event")
                        .on_press(move || showing.update(|open| *open = !*open)),
                )
                .child(when(showing, || {
                    text("Design systems meetup · Online").text_sm()
                })),
        ))
        .surface()
        .max_w(px(384.0))
        .test_id("card-image")
}
