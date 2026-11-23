//! Conversation rows, avatar gutters, and message metadata.

use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::direction::use_direction;

const GUTTER: f32 = 32.0;
const CONTENT_GAP: f32 = 10.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

pub struct Message {
    align: MessageAlign,
    avatar: Option<MessageAvatar>,
    content: MessageContent,
}

impl Message {
    pub fn new(content: MessageContent) -> Self {
        Self {
            align: MessageAlign::Start,
            avatar: None,
            content,
        }
    }

    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }

    pub fn avatar(mut self, avatar: MessageAvatar) -> Self {
        self.avatar = Some(avatar);
        self
    }

    pub fn surface(self) -> View {
        let end = self.align == MessageAlign::End;
        let direction = use_direction().map(LayoutDirection::from);
        let footer_height = signal(0.0_f32);
        let avatar = self.avatar.map(|avatar| {
            let with_footer = self.content.footer.is_some();
            avatar.0.self_end().when(with_footer, |slot| {
                slot.mb(move || px(footer_height.get() + CONTENT_GAP))
            })
        });
        view()
            .test_id("message-row")
            .w_full()
            .min_w(px(0.0))
            .direction(direction)
            .gap_2()
            .when(end, |row| row.flex_row_reverse())
            .when(!end, |row| row.flex_row())
            .child(avatar)
            .child(self.content.render(end, footer_height))
    }
}

impl IntoElement for Message {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct MessageAvatar(View);

impl MessageAvatar {
    pub fn new(avatar: impl IntoElement) -> Self {
        Self(
            Self::slot()
                .rounded_full()
                .overflow_hidden()
                .bg(|| theme().muted)
                .child(avatar),
        )
    }

    /// Keeps the conversation column aligned when a repeated avatar is omitted.
    pub fn empty() -> Self {
        Self(Self::slot().accessibility_elements_hidden(true))
    }

    fn slot() -> View {
        view()
            .test_id("message-avatar-slot")
            .min_w(px(GUTTER))
            .flex_shrink_0()
            .items_center()
            .justify_center()
    }
}

pub struct MessageContent {
    header: Option<MessageHeader>,
    body: Vec<AnyElement>,
    footer: Option<MessageFooter>,
    inset: bool,
}

impl MessageContent {
    pub fn new() -> Self {
        Self {
            inset: true,
            header: None,
            body: Vec::new(),
            footer: None,
        }
    }

    pub fn header(mut self, header: MessageHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn child(mut self, content: impl IntoElement) -> Self {
        self.body.push(content.into_element());
        self
    }

    pub fn footer(mut self, footer: MessageFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Places metadata flush with content such as a ghost bubble.
    pub fn flush(mut self) -> Self {
        self.inset = false;
        self
    }

    fn render(self, end: bool, footer_height: Signal<f32>) -> View {
        let inset = px(if self.inset { 12.0 } else { 0.0 });
        let mut column = view()
            .test_id("message-content")
            .min_w(px(0.0))
            .flex_1()
            .gap(px(CONTENT_GAP))
            .when(end, |view| view.items_end());
        if let Some(header) = self.header {
            column = column.child(header.0.self_stretch().px(inset));
        }
        column = column.children(self.body.into_iter().map(|child| {
            if end {
                view()
                    .w_full()
                    .min_w(px(0.0))
                    .flex_row()
                    .justify_end()
                    .child(child)
                    .into_element()
            } else {
                child
            }
        }));
        if let Some(footer) = self.footer {
            column = column.child(
                footer
                    .0
                    .px(inset)
                    .on_layout(move |bounds: &Bounds<Points>| {
                        footer_height.set(bounds.size.height.0);
                    }),
            );
        }
        column
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MessageHeader(View);

impl MessageHeader {
    pub fn new(child: impl IntoElement) -> Self {
        Self(metadata().test_id("message-header").child(child))
    }

    pub fn text(label: impl IntoReactive<SharedString>) -> Self {
        Self::new(text(label).text_xs())
    }
}

pub struct MessageFooter(View);

impl MessageFooter {
    pub fn new() -> Self {
        Self(metadata().test_id("message-footer"))
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.0 = self.0.child(child);
        self
    }

    pub fn text(label: impl IntoReactive<SharedString>) -> Self {
        Self::new().child(text(label).text_xs())
    }
}

impl Default for MessageFooter {
    fn default() -> Self {
        Self::new()
    }
}

fn metadata() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .items_center()
        .min_w(px(0.0))
        .max_w(percentage(1.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(|| theme().muted_foreground)
}

pub struct MessageGroup(View);

impl MessageGroup {
    pub fn new() -> Self {
        Self(view().test_id("message-group").min_w(px(0.0)).gap_2())
    }

    pub fn child(mut self, message: Message) -> Self {
        self.0 = self.0.child(message);
        self
    }
}

impl Default for MessageGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for MessageGroup {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

#[cfg(test)]
mod tests {
    use nsh_core::{Size, element::Root};
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;

    #[component]
    fn WithFooter() -> impl IntoElement {
        Message::new(
            MessageContent::new()
                .child(view().test_id("message-body-fixture").size(px(48.0)))
                .footer(MessageFooter::new().child(view().size(px(60.0)))),
        )
        .avatar(MessageAvatar::new(view().size(px(32.0))))
        .align(MessageAlign::End)
    }

    #[test]
    fn end_avatar_stays_at_the_bottom_of_the_body_above_a_tall_footer() {
        let mut config = EngineConfig::new(Root::of::<WithFooter>());
        config.viewport = Size::new(px(320.0), px(240.0));
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();
        let layout = app.screen().layout_nodes();
        let bounds = |id| {
            layout
                .iter()
                .find(|node| node.test_id.as_deref() == Some(id))
                .unwrap()
                .bounds
        };
        let avatar = bounds("message-avatar-slot");
        let body = bounds("message-body-fixture");
        let footer = bounds("message-footer");
        assert!(avatar.origin.x > body.origin.x);
        assert_eq!(
            avatar.origin.y + avatar.size.height,
            body.origin.y + body.size.height
        );
        assert_eq!(
            footer.origin.y,
            body.origin.y + body.size.height + px(CONTENT_GAP)
        );
    }
}
