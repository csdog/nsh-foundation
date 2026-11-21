//! Composable file cards, independent card actions, and horizontal attachment rows.

use nsh_base::theme;
use nsh_core::{keyboard::KeyCapture, keymap::Keystroke, prelude::*};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}

nsh_core::static_reactive!(AttachmentState: AttachmentState);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct Attachment {
    state: Reactive<AttachmentState>,
    size: AttachmentSize,
    orientation: AttachmentOrientation,
    media: Option<AttachmentMedia>,
    content: Option<AttachmentContent>,
    actions: Option<AttachmentActions>,
    trigger: Option<AttachmentTrigger>,
}

impl Attachment {
    pub fn new() -> Self {
        Self {
            state: Reactive::Static(AttachmentState::Done),
            size: AttachmentSize::Default,
            orientation: AttachmentOrientation::Horizontal,
            media: None,
            content: None,
            actions: None,
            trigger: None,
        }
    }

    pub fn state(mut self, state: impl IntoReactive<AttachmentState>) -> Self {
        self.state = state.into_reactive();
        self
    }

    pub fn size(mut self, size: AttachmentSize) -> Self {
        self.size = size;
        self
    }

    pub fn orientation(mut self, orientation: AttachmentOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn media(mut self, media: AttachmentMedia) -> Self {
        self.media = Some(media);
        self
    }

    pub fn content(mut self, content: AttachmentContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn actions(mut self, actions: AttachmentActions) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn trigger(mut self, trigger: AttachmentTrigger) -> Self {
        self.trigger = Some(trigger);
        self
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Attachment {
    fn into_element(self) -> AnyElement {
        let vertical = self.orientation == AttachmentOrientation::Vertical;
        let (media_width, padding, gap, radius) = match self.size {
            AttachmentSize::Default => (40.0, 8.0, 8.0, 16.0),
            AttachmentSize::Small => (32.0, 6.0, 10.0, 16.0),
            AttachmentSize::ExtraSmall => (28.0, 4.0, 6.0, 12.0),
        };
        let error = self.state.clone().map(|s| s == AttachmentState::Error);
        let error_border = error.clone();
        let busy = self
            .state
            .clone()
            .map(|s| matches!(s, AttachmentState::Uploading | AttachmentState::Processing));
        let state = self.state;
        let idle = state.clone().map(|state| state == AttachmentState::Idle);
        let mut card = view()
            .role(AccessibilityRole::Group)
            .w_full()
            .min_w(px(0.0))
            .gap(px(gap))
            .p(px(padding))
            .rounded(px(radius))
            .border(px(1.0))
            .border_color(move || {
                if error_border.get() {
                    theme().destructive.opacity(0.3)
                } else {
                    theme().border
                }
            })
            .style_when(move || idle.get(), |style| style.border_dashed())
            .bg(move || theme().card)
            .text_color(move || theme().card_foreground)
            .focus_within(|style| style.ring(px(1.0)).ring_color(theme().ring.opacity(0.3)))
            .when(!vertical, |card| card.flex_row().items_center());
        if let Some(trigger) = self.trigger {
            card = card.hover(|style| style.bg(theme().muted)).child(trigger.0);
        }
        if let Some(media) = self.media {
            let media_error = error.clone();
            let is_image = media.image;
            card = card.child(
                view()
                    .flex_shrink_0()
                    .aspect_ratio(1.0)
                    .when(vertical, |view| view.w_full())
                    .when(!vertical, |view| view.w(px(media_width)))
                    .overflow_hidden()
                    .rounded(px(8.0))
                    .items_center()
                    .justify_center()
                    .bg(move || {
                        if media_error.get() {
                            theme().destructive.opacity(0.1)
                        } else {
                            theme().muted
                        }
                    })
                    .text_color({
                        let error = error.clone();
                        move || {
                            if error.get() {
                                theme().destructive
                            } else {
                                theme().foreground
                            }
                        }
                    })
                    .style_when(
                        move || is_image && (busy.get() || error.get()),
                        |style| style.opacity(0.6),
                    )
                    .child(media.child),
            );
        }
        if let Some(content) = self.content {
            card = card.child(content.render(self.size, state));
        }
        if let Some(actions) = self.actions {
            card = card.child(actions.0.when(vertical, |actions| {
                actions.absolute().top(px(10.0)).right(px(10.0))
            }));
        }
        card.into_element()
    }
}

pub struct AttachmentMedia {
    child: AnyElement,
    image: bool,
}

impl AttachmentMedia {
    pub fn icon(child: impl IntoElement) -> Self {
        Self {
            child: child.into_element(),
            image: false,
        }
    }

    pub fn image(child: impl IntoElement) -> Self {
        Self {
            child: child.into_element(),
            image: true,
        }
    }
}

pub struct AttachmentContent {
    title: AttachmentTitle,
    description: Option<AttachmentDescription>,
}

impl AttachmentContent {
    pub fn new(title: AttachmentTitle) -> Self {
        Self {
            title,
            description: None,
        }
    }

    pub fn description(mut self, description: AttachmentDescription) -> Self {
        self.description = Some(description);
        self
    }

    fn render(self, size: AttachmentSize, state: Reactive<AttachmentState>) -> View {
        let title_state = state.clone();
        let mut content = view().flex_1().min_w(px(0.0)).gap(px(2.0)).child(
            text(self.title.0)
                .truncate()
                .font_weight(FontWeight::MEDIUM)
                .when(size == AttachmentSize::Default, |title| title.text_sm())
                .when(size != AttachmentSize::Default, |title| title.text_xs())
                .style_when(
                    move || {
                        matches!(
                            title_state.get(),
                            AttachmentState::Uploading | AttachmentState::Processing
                        )
                    },
                    |style| style.opacity(0.8),
                ),
        );
        if let Some(description) = self.description {
            content = content.child(text(description.0).truncate().text_xs().text_color(
                move || {
                    if state.get() == AttachmentState::Error {
                        theme().destructive.opacity(0.8)
                    } else {
                        theme().muted_foreground
                    }
                },
            ));
        }
        content
    }
}

pub struct AttachmentTitle(Reactive<SharedString>);
pub struct AttachmentDescription(Reactive<SharedString>);

impl AttachmentTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(title.into_reactive())
    }
}

impl AttachmentDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self(description.into_reactive())
    }
}

pub struct AttachmentActions(View);

impl AttachmentActions {
    pub fn new() -> Self {
        Self(view().flex_row().gap(px(3.0)).flex_shrink_0().z_index(20))
    }

    pub fn child(mut self, action: AttachmentAction) -> Self {
        self.0 = self.0.child(action.0);
        self
    }
}

impl Default for AttachmentActions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AttachmentAction(View);

impl AttachmentAction {
    pub fn new(
        label: impl IntoReactive<SharedString>,
        icon: &'static str,
        press: impl Fn() + 'static,
    ) -> Self {
        Self(
            view()
                .role(AccessibilityRole::Button)
                .label(label)
                .cursor_pointer()
                .w(px(24.0))
                .h(px(24.0))
                .rounded(px(6.0))
                .bg(move || theme().card)
                .items_center()
                .justify_center()
                .hover(|style| style.bg(theme().muted))
                .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                .on_press(press)
                .child(text(icon).text_sm()),
        )
    }
}

impl AttachmentAction {
    pub fn visible(mut self, visible: impl Fn() -> bool + 'static) -> Self {
        self.0 = self
            .0
            .style_when(move || !visible(), |style| style.hidden());
        self
    }
}

pub struct AttachmentTrigger(View);

impl AttachmentTrigger {
    pub fn new(label: impl IntoReactive<SharedString>, press: impl Fn() + 'static) -> Self {
        Self::render(
            view()
                .role(AccessibilityRole::Button)
                .label(label)
                .on_press(press),
        )
    }

    pub fn render(trigger: View) -> Self {
        Self(trigger.absolute().inset_0().z_index(10).cursor_pointer())
    }
}

pub struct AttachmentGroup {
    label: &'static str,
    item_width: f32,
    items: View,
}

impl AttachmentGroup {
    pub fn new(label: &'static str, item_width: f32) -> Self {
        Self {
            label,
            item_width,
            items: view().flex_row().gap_3().p_1(),
        }
    }

    pub fn child(mut self, item: impl IntoElement) -> Self {
        self.items = self
            .items
            .child(view().w(px(self.item_width)).flex_shrink_0().child(item));
        self
    }

    pub fn child_when<E: IntoElement>(
        mut self,
        visible: impl IntoReactive<bool>,
        item: impl Fn() -> E + 'static,
    ) -> Self {
        let width = self.item_width;
        self.items = self.items.child(when(visible, move || {
            view().w(px(width)).flex_shrink_0().child(item())
        }));
        self
    }
}

impl IntoElement for AttachmentGroup {
    fn into_element(self) -> AnyElement {
        let scroll_ref = element_ref();
        let x = stored(0.0_f32);
        let stride = self.item_width + 12.0;
        let keys = KeyCapture::keys(
            ["left", "right"]
                .into_iter()
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        scroll()
            .horizontal()
            .scrollbar(false)
            .w_full()
            .min_w(px(0.0))
            .role(AccessibilityRole::Group)
            .label(self.label)
            .focusable(true)
            .focus_visible(|style| style.ring_inset().ring(px(2.0)).ring_color(theme().ring))
            .element_ref(&scroll_ref)
            .snap_interval(px(stride))
            .on_scroll(move |event: &ScrollEvent| {
                x.replace(event.offset.x.0);
            })
            .capture_keys(keys)
            .on_key_down(move |event: &KeyEvent| {
                if !event.default_prevented {
                    return;
                }
                let dx = match event.key.as_ref() {
                    "ArrowLeft" => -stride,
                    "ArrowRight" => stride,
                    _ => return,
                };
                scroll_ref.scroll_to(
                    point(px((x.get() + dx).max(0.0)), px(0.0)),
                    !reduce_motion(),
                );
            })
            .child(self.items)
            .into_element()
    }
}
