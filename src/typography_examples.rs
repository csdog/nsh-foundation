use nsh_base::{link, theme};
use nsh_core::{AccessibilityState, WebElement, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider, use_direction},
    typography::{self, HeadingLevel, heading, paragraph, quote},
};

mod story;
use story::{ARABIC, ENGLISH, HEBREW, Story};

const REFERENCE: &str = "https://ui.shadcn.com/docs/components/base/typography";

fn comparison(story: &Story) -> View {
    typography::comparison(
        story.kings_treasury,
        [story.kings_treasury, story.peoples_happiness],
        [
            [story.empty, story.overflowing],
            [story.modest, story.satisfied],
            [story.full, story.ecstatic],
        ],
    )
}

fn tax_levels(story: &Story) -> View {
    typography::list([story.level1, story.level2, story.level3])
}

fn article(story: &'static Story) -> View {
    view()
        .w_full()
        .min_w(px(0.0))
        .direction(use_direction().map(LayoutDirection::from))
        .web_element(WebElement::Article)
        .gap_6()
        .test_id("typography-article")
        .child(heading(HeadingLevel::H1, story.title))
        .child(typography::lead(story.lead_paragraph))
        .child(heading(HeadingLevel::H2, story.kings_plan).mt_4())
        .child(
            paragraph(format!("{} ", story.king_thought))
                .child(
                    link()
                        .href(REFERENCE)
                        .accessibility_label(story.brilliant_plan)
                        .child(
                            text(story.brilliant_plan)
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(|| theme().primary)
                                .underline()
                                .underline_offset(px(4.0)),
                        ),
                )
                .child(text(story.tax_jokes)),
        )
        .child(quote(story.blockquote))
        .child(heading(HeadingLevel::H3, story.joke_tax).mt_2())
        .child(paragraph(story.subjects_not_amused))
        .child(tax_levels(story))
        .child(paragraph(story.stopped_telling))
        .child(heading(HeadingLevel::H3, story.jokesters_revolt).mt_2())
        .child(paragraph(story.sneaking))
        .child(paragraph(story.discovered))
        .child(heading(HeadingLevel::H3, story.peoples_rebellion).mt_2())
        .child(paragraph(story.uplifted))
        .child(comparison(story))
        .child(paragraph(story.realized))
        .child(paragraph(story.moral))
}

pub fn demo() -> View {
    article(&ENGLISH)
}

pub fn h1() -> View {
    heading(HeadingLevel::H1, ENGLISH.title).text_center()
}

pub fn h2() -> View {
    heading(HeadingLevel::H2, "The People of the Kingdom")
}

pub fn h3() -> View {
    heading(HeadingLevel::H3, ENGLISH.joke_tax)
}

pub fn h4() -> View {
    heading(HeadingLevel::H4, "People stopped telling jokes")
}

pub fn p() -> Text {
    paragraph(
        "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax.",
    )
}

pub fn blockquote() -> View {
    quote(ENGLISH.blockquote)
}

pub fn table() -> View {
    comparison(&ENGLISH)
}

pub fn list() -> View {
    tax_levels(&ENGLISH)
}

pub fn inline_code() -> View {
    typography::inline_code("@radix-ui/react-alert-dialog")
}

pub fn lead() -> Text {
    typography::lead(
        "A modal dialog that interrupts the user with important content and expects a response.",
    )
}

pub fn large() -> Text {
    typography::large("Are you absolutely sure?")
}

pub fn small() -> Text {
    typography::small("Email address")
}

pub fn muted() -> Text {
    typography::muted("Enter your email address.")
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let languages = [
        ("English", Direction::Ltr, &ENGLISH),
        ("Arabic", Direction::Rtl, &ARABIC),
        ("Hebrew", Direction::Rtl, &HEBREW),
    ];
    view()
        .w_full()
        .min_w(px(0.0))
        .gap_6()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(
                    languages
                        .into_iter()
                        .enumerate()
                        .map(move |(index, (name, _, _))| {
                            Button::new(name)
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Small)
                                .surface()
                                .accessibility_state(move || AccessibilityState {
                                    selected: language.get() == index,
                                    ..Default::default()
                                })
                                .on_press(move || language.set(index))
                        }),
                ),
        )
        .child(dynamic(move || {
            let (_, direction, story) = languages[language.get()];
            DirectionProvider::new(direction, move || article(story))
        }))
}
