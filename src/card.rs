//! Composable Rhea cards for Native Shell.

use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};

const DEFAULT_SPACING: f32 = 20.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    #[default]
    Default,
    Small,
}

#[derive(Default)]
pub struct Card {
    size: CardSize,
    spacing: Option<Reactive<f32>>,
    direction: LayoutDirection,
    media: Option<AnyElement>,
    header: Option<CardHeader>,
    content: Option<CardContent>,
    footer: Option<CardFooter>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the padding and the gap between sections in points.
    pub fn spacing(mut self, points: impl IntoReactive<f32>) -> Self {
        self.spacing = Some(points.into_reactive());
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Places a full-width cover at the top edge.
    pub fn media(mut self, cover: impl IntoElement) -> Self {
        self.media = Some(cover.into_element());
        self
    }

    pub fn header(mut self, header: CardHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn content(mut self, content: CardContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn footer(mut self, footer: CardFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn surface(self) -> View {
        let spacing = self
            .spacing
            .unwrap_or_else(|| {
                match self.size {
                    CardSize::Default => DEFAULT_SPACING,
                    CardSize::Small => 16.0,
                }
                .into_reactive()
            })
            .map(|value| {
                px(if value.is_finite() {
                    value.max(0.0)
                } else {
                    0.0
                })
            });
        let inset = spacing.clone().map(Length::from);
        let has_footer = self.footer.is_some();
        view()
            .w_full()
            .min_w(px(0.0))
            .direction(self.direction)
            .rounded(px(24.0))
            .overflow_hidden()
            .bg(move || theme().card)
            .text_color(move || theme().card_foreground)
            .shadow_sm()
            .ring(px(1.0))
            .ring_color(move || theme().foreground.opacity(if dark() { 0.10 } else { 0.05 }))
            .gap(inset.clone())
            .when(self.media.is_none(), |card| card.pt(inset.clone()))
            .pb(inset)
            .child(self.media)
            .child(
                self.header
                    .map(|header| header.surface(spacing.clone(), self.direction)),
            )
            .child(
                self.content
                    .map(|content| content.surface(spacing.clone(), has_footer)),
            )
            .child(self.footer.map(|footer| footer.surface(spacing)))
    }
}

impl IntoElement for Card {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

#[derive(Default)]
pub struct CardHeader {
    title: Option<CardTitle>,
    description: Option<CardDescription>,
    action: Option<CardAction>,
}

impl CardHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: CardTitle) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: CardDescription) -> Self {
        self.description = Some(description);
        self
    }

    pub fn action(mut self, action: CardAction) -> Self {
        self.action = Some(action);
        self
    }

    fn surface(self, spacing: Reactive<Points>, direction: LayoutDirection) -> View {
        let rtl = direction == LayoutDirection::Rtl;
        view()
            .px(move || spacing.get())
            .flex_row()
            .items_start()
            .gap(px(6.0))
            .child(
                view()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap(px(6.0))
                    .child(
                        self.title
                            .map(|title| title.0.when(rtl, |label| label.text_right())),
                    )
                    .child(
                        self.description
                            .map(|description| description.0.when(rtl, |label| label.text_right())),
                    ),
            )
            .child(
                self.action
                    .map(|action| view().flex_shrink_0().child(action.0)),
            )
    }
}

impl IntoElement for CardHeader {
    fn into_element(self) -> AnyElement {
        self.surface(
            px(DEFAULT_SPACING).into_reactive(),
            LayoutDirection::Inherit,
        )
        .into_element()
    }
}

pub struct CardTitle(Text);

impl CardTitle {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self(text(label).text_base().font_weight(FontWeight::MEDIUM))
    }
}

impl IntoElement for CardTitle {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct CardDescription(Text);

impl CardDescription {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self(
            text(label)
                .text_sm()
                .text_color(move || theme().muted_foreground),
        )
    }
}

impl IntoElement for CardDescription {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct CardAction(AnyElement);

impl CardAction {
    pub fn new(control: impl IntoElement) -> Self {
        Self(control.into_element())
    }
}

impl IntoElement for CardAction {
    fn into_element(self) -> AnyElement {
        self.0
    }
}

pub struct CardContent {
    body: AnyElement,
    edge_to_edge: bool,
    flush_footer: bool,
}

impl CardContent {
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            body: body.into_element(),
            edge_to_edge: false,
            flush_footer: false,
        }
    }

    pub fn edge_to_edge(mut self) -> Self {
        self.edge_to_edge = true;
        self
    }

    pub fn flush_footer(mut self) -> Self {
        self.flush_footer = true;
        self
    }

    fn surface(self, spacing: Reactive<Points>, has_footer: bool) -> View {
        view()
            .when(!self.edge_to_edge, |content| {
                let spacing = spacing.clone();
                content.px(move || spacing.get())
            })
            .when(self.flush_footer && has_footer, |content| {
                content.mb(move || -spacing.get())
            })
            .child(self.body)
    }
}

impl IntoElement for CardContent {
    fn into_element(self) -> AnyElement {
        self.surface(px(DEFAULT_SPACING).into_reactive(), false)
            .into_element()
    }
}

pub struct CardFooter(AnyElement);

impl CardFooter {
    pub fn new(actions: impl IntoElement) -> Self {
        Self(actions.into_element())
    }

    fn surface(self, spacing: Reactive<Points>) -> View {
        view().px(move || spacing.get()).child(self.0)
    }
}

impl IntoElement for CardFooter {
    fn into_element(self) -> AnyElement {
        self.surface(px(DEFAULT_SPACING).into_reactive())
            .into_element()
    }
}
