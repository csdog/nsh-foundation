//! A composable list row with optional media, actions, and full-width sections.

use nsh_base::{link, theme};
use nsh_core::{element::ElNode, prelude::*};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ItemSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
}

impl ItemSize {
    fn spacing(self) -> (f32, f32, f32) {
        match self {
            Self::Default => (16.0, 14.0, 14.0),
            Self::Small => (14.0, 12.0, 14.0),
            Self::ExtraSmall => (10.0, 8.0, 8.0),
        }
    }

    fn media_size(self) -> f32 {
        match self {
            Self::Default => 40.0,
            Self::Small => 32.0,
            Self::ExtraSmall => 24.0,
        }
    }
}

#[derive(Default)]
pub struct Item {
    variant: ItemVariant,
    size: ItemSize,
    header: Option<ItemHeader>,
    media: Option<ItemMedia>,
    content: Vec<ItemContent>,
    actions: Option<ItemActions>,
    footer: Option<ItemFooter>,
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    pub fn header(mut self, header: ItemHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn media(mut self, media: ItemMedia) -> Self {
        self.media = Some(media);
        self
    }

    /// Adds content. The first block flexes, and later blocks take their intrinsic width.
    pub fn content(mut self, content: ItemContent) -> Self {
        self.content.push(content);
        self
    }

    pub fn actions(mut self, actions: ItemActions) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn footer(mut self, footer: ItemFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn surface(self) -> View {
        let (horizontal, vertical, gap) = self.size.spacing();
        let variant = self.variant;
        let has_description = self.content.iter().any(|content| content.has_description);
        let size = self.size;
        let body = view()
            .flex_row()
            .items_center()
            .flex_wrap()
            .min_w(px(0.0))
            .gap(px(gap))
            .child(self.media.map(|media| media.render(size, has_description)))
            .children(
                self.content
                    .into_iter()
                    .enumerate()
                    .map(|(index, content)| {
                        content
                            .view
                            .gap(px(if size == ItemSize::ExtraSmall {
                                2.0
                            } else {
                                4.0
                            }))
                            .when(index == 0, |content| content.flex_1())
                    }),
            )
            .child(self.actions);
        view()
            .w_full()
            .min_w(px(0.0))
            .gap(px(gap))
            .direction(use_direction().map(Into::into))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || {
                if variant == ItemVariant::Outline {
                    theme().border
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .bg(move || {
                if variant == ItemVariant::Muted {
                    theme().muted.opacity(0.5)
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .text_color(move || theme().foreground)
            .px(px(horizontal))
            .py(px(vertical))
            .transition_colors()
            .duration(100.0)
            .child(self.header)
            .child(body)
            .child(self.footer)
    }

    /// Renders the row as one native link. Use noninteractive trailing content inside it.
    pub fn href(
        self,
        label: impl IntoReactive<SharedString>,
        href: &'static str,
    ) -> impl IntoElement {
        let label = label.into_reactive();
        link()
            .href(href)
            .accessibility_label(move || label.get())
            .w_full()
            .min_w(px(0.0))
            .rounded(px(16.0))
            .focus_visible(|style| style.ring(px(3.0)).ring_color(theme().ring.opacity(0.5)))
            .child(self.surface().hover(|style| style.bg(theme().muted)))
    }
}

impl IntoElement for Item {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

macro_rules! item_part {
    ($( $name:ident ),+ $(,)?) => {
        $(
            pub struct $name(View);

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl IntoElement for $name {
                fn into_element(self) -> AnyElement {
                    self.0.into_element()
                }
            }

            impl Element for $name {
                fn node(&mut self) -> &mut ElNode {
                    self.0.node()
                }
            }

            impl ParentElement for $name {}
        )+
    };
}

item_part!(ItemGroup, ItemHeader, ItemFooter, ItemActions);

impl ItemGroup {
    pub fn new() -> Self {
        Self(
            view()
                .role(AccessibilityRole::List)
                .w_full()
                .min_w(px(0.0))
                .gap_4(),
        )
    }

    pub fn size(self, size: ItemSize) -> Self {
        Self(self.0.gap(px(match size {
            ItemSize::Default => 16.0,
            ItemSize::Small => 10.0,
            ItemSize::ExtraSmall => 8.0,
        })))
    }
}

impl ItemHeader {
    pub fn new() -> Self {
        Self(
            view()
                .w_full()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2(),
        )
    }
}

impl ItemFooter {
    pub fn new() -> Self {
        Self(
            view()
                .w_full()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2(),
        )
    }
}

impl ItemActions {
    pub fn new() -> Self {
        Self(view().flex_row().items_center().flex_shrink_0().gap_2())
    }
}

#[derive(Default)]
pub struct ItemSeparator;

impl ItemSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl IntoElement for ItemSeparator {
    fn into_element(self) -> AnyElement {
        view()
            .w_full()
            .h(px(1.0))
            .bg(move || theme().border)
            .accessibility_elements_hidden(true)
            .into_element()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

pub struct ItemMedia {
    content: AnyElement,
    variant: ItemMediaVariant,
}

impl ItemMedia {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_element(),
            variant: ItemMediaVariant::Default,
        }
    }

    pub fn variant(mut self, variant: ItemMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    fn render(self, size: ItemSize, has_description: bool) -> View {
        view()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .when(has_description, |media| media.self_start().mt(px(2.0)))
            .when(self.variant == ItemMediaVariant::Icon, |media| {
                media.size(px(16.0)).accessibility_elements_hidden(true)
            })
            .when(self.variant == ItemMediaVariant::Image, |media| {
                media
                    .size(px(size.media_size()))
                    .overflow_hidden()
                    .rounded(px(if size == ItemSize::ExtraSmall {
                        8.0
                    } else {
                        12.0
                    }))
            })
            .child(self.content)
    }
}

pub struct ItemContent {
    view: View,
    has_description: bool,
}

impl ItemContent {
    pub fn new() -> Self {
        Self {
            view: view().min_w(px(0.0)),
            has_description: false,
        }
    }

    pub fn title(mut self, title: ItemTitle) -> Self {
        self.view = self.view.child(title);
        self
    }

    pub fn description(mut self, description: ItemDescription) -> Self {
        self.has_description = true;
        self.child(description)
    }

    pub fn child(mut self, content: impl IntoElement) -> Self {
        self.view = self.view.child(content);
        self
    }
}

impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for ItemContent {
    fn into_element(self) -> AnyElement {
        self.view.gap_1().into_element()
    }
}

pub struct ItemTitle(Text);

impl ItemTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        let direction = use_direction();
        Self(
            text(title)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .number_of_lines(1)
                .style_when(
                    move || direction.get() == Direction::Rtl,
                    |style| style.text_right(),
                ),
        )
    }
}

impl IntoElement for ItemTitle {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct ItemDescription(View);

impl ItemDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        let direction = use_direction();
        Self::render(text(description).text_sm().number_of_lines(2).style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right(),
        ))
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self(
            view()
                .min_w(px(0.0))
                .text_color(move || theme().muted_foreground)
                .child(content),
        )
    }
}

impl IntoElement for ItemDescription {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}
