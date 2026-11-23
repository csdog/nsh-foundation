use std::time::Duration;

use nsh_base::theme;
use nsh_core::{element::set_clipboard, prelude::*};

use components::{
    attachment::{
        Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
        AttachmentMedia, AttachmentOrientation, AttachmentTitle,
    },
    avatar::{Avatar, AvatarFallback, AvatarImage},
    bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant},
    button::{Button, ButtonSize, ButtonVariant},
    marker::{Marker, MarkerContent},
    message::{
        Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter, MessageGroup,
        MessageHeader,
    },
};

const COPY_TEXT: &str = "The install failure is coming from the workspace package.";

fn stage(gap: f32) -> View {
    view().w_full().max_w(px(384.0)).gap(px(gap)).py(px(48.0))
}

fn received(label: &'static str) -> Bubble {
    Bubble::new(BubbleContent::text(label)).variant(BubbleVariant::Muted)
}

fn sent(label: &'static str) -> Bubble {
    Bubble::new(BubbleContent::text(label))
}

fn outgoing(content: MessageContent) -> Message {
    Message::new(content).align(MessageAlign::End)
}

fn me() -> MessageAvatar {
    MessageAvatar::new(
        Avatar::new("@me")
            .image(AvatarImage::new(asset!("message-avatar-10.png")))
            .fallback(AvatarFallback::new("ME")),
    )
}

fn rabbit() -> MessageAvatar {
    MessageAvatar::new(
        Avatar::new("@rabbit")
            .image(AvatarImage::new(asset!("message-avatar-02.png")))
            .fallback(AvatarFallback::new("R")),
    )
}

fn teammate() -> MessageAvatar {
    MessageAvatar::new(
        Avatar::new("@avatar")
            .image(AvatarImage::new(asset!("message-avatar-03.png")))
            .fallback(AvatarFallback::new("R")),
    )
}

pub fn demo() -> View {
    stage(24.0)
        .child(
            outgoing(MessageContent::new().child(sent("Deploying to prod real quick.")))
                .avatar(me()),
        )
        .child(
            Message::new(MessageContent::new().child(received("It's 4:55 PM. On a Friday.")))
                .avatar(rabbit()),
        )
        .child(
            outgoing(
                MessageContent::new()
                    .child(sent("It's a one-line change."))
                    .footer(MessageFooter::text("Delivered")),
            )
            .avatar(me()),
        )
        .child(
            Message::new(
                MessageContent::new().child(
                    BubbleGroup::new()
                        .child(received("It's always a one-line change 😭."))
                        .child(
                            received("Alright, let me take a look.").reactions(
                                BubbleReactions::new()
                                    .label("Reactions: thumbs up")
                                    .child(text("👍").accessibility_elements_hidden(true)),
                            ),
                        ),
                ),
            )
            .avatar(rabbit()),
        )
        .child(Marker::new(MarkerContent::new("Oliver is typing...").shimmer()).status())
}

pub fn avatars() -> View {
    stage(24.0)
        .child(
            Message::new(
                MessageContent::new()
                    .child(received("The build failed during dependency installation.")),
            )
            .avatar(teammate()),
        )
        .child(
            outgoing(MessageContent::new().child(sent("Can you share the exact error?")))
                .avatar(me()),
        )
        .child(
            Message::new(
                MessageContent::new().child(
                    BubbleGroup::new()
                        .child(received("Here's the error from the logs"))
                        .child(received(
                            "Something went wrong with the build. The libraries are not installed correctly. Try running the build again.",
                        )),
                ),
            )
            .avatar(teammate()),
        )
}

pub fn group() -> View {
    stage(24.0).child(
        MessageGroup::new()
            .child(
                Message::new(
                    MessageContent::new().child(received("I checked the registry addresses.")),
                )
                .avatar(MessageAvatar::empty()),
            )
            .child(
                Message::new(MessageContent::new().child(received(
                    "The component and example JSON now live under the UI registry.",
                )))
                .avatar(rabbit()),
            ),
    )
}

pub fn header_footer() -> View {
    stage(32.0)
        .child(Message::new(
            MessageContent::new()
                .header(MessageHeader::text("Olivia"))
                .child(received("I already checked the logs.")),
        ))
        .child(outgoing(
            MessageContent::new()
                .child(sent(
                    "Send the report to the team. Ping @shadcn if you need help.",
                ))
                .footer(
                    MessageFooter::new().child(
                        view()
                            .flex_row()
                            .gap_1()
                            .child(text("Read").text_xs())
                            .child(text("Yesterday").text_xs().font_weight(FontWeight::NORMAL)),
                    ),
                ),
        ))
}

fn icon_button(label: &'static str, icon: &'static str, size: ButtonSize) -> Button {
    Button::new(label)
        .variant(ButtonVariant::Ghost)
        .size(size)
        .icon_start(text(icon).text_sm().accessibility_elements_hidden(true))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Delivery {
    Failed,
    Sending,
    Delivered,
}

pub fn actions() -> View {
    let feedback = signal(None::<bool>);
    let copied = signal(false);
    let delivery = signal(Delivery::Failed);
    let feedback_button = |label, symbol, positive| {
        let selected = move || feedback.get() == Some(positive);
        icon_button(label, symbol, ButtonSize::Icon)
            .on_press(move || feedback.set((!selected()).then_some(positive)))
            .accessibility_state(move || AccessibilityState {
                pressed: Some(if selected() {
                    nsh_core::Checked::True
                } else {
                    nsh_core::Checked::False
                }),
                ..Default::default()
            })
            .style_when(selected, |style| style.bg(theme().muted))
    };
    stage(32.0)
        .child(Message::new(
            MessageContent::new().child(received(COPY_TEXT)).footer(
                MessageFooter::new()
                    .child(
                        icon_button("Copy", "▢", ButtonSize::Icon).on_press(move || {
                            set_clipboard(COPY_TEXT);
                            copied.set(true);
                            announce_for_accessibility("Message copied");
                        }),
                    )
                    .child(feedback_button("Like", "👍", true))
                    .child(feedback_button("Dislike", "👎", false))
                    .child(when(copied, move || text("Copied").text_xs())),
            ),
        ))
        .child(outgoing(
            MessageContent::new()
                .child(sent("Okay drop me a link. Taking a look..."))
                .footer(
                    MessageFooter::new()
                        .child(
                            text(move || match delivery.get() {
                                Delivery::Failed => "Failed to send",
                                Delivery::Sending => "Sending…",
                                Delivery::Delivered => "Delivered",
                            })
                            .text_xs()
                            .font_weight(FontWeight::NORMAL)
                            .style_when(
                                move || delivery.get() == Delivery::Failed,
                                |style| style.text_color(theme().destructive),
                            ),
                        )
                        .child(
                            icon_button("Retry", "↻", ButtonSize::IconExtraSmall)
                                .disabled(move || delivery.get() != Delivery::Failed)
                                .on_press(move || {
                                    delivery.set(Delivery::Sending);
                                    set_timeout(Duration::from_millis(450), move || {
                                        delivery.set(Delivery::Delivered);
                                        announce_for_accessibility("Message delivered");
                                    });
                                }),
                        ),
                ),
        ))
}

pub fn attachment() -> View {
    let requested = signal(false);
    let preview_image = Attachment::new()
        .orientation(AttachmentOrientation::Vertical)
        .media(AttachmentMedia::image(
            image(asset!("attachment-workspace.jpg"))
                .role(AccessibilityRole::Image)
                .label("Workspace")
                .w_full()
                .h_full()
                .object_fit(ResizeMode::Cover),
        ));
    stage(32.0)
        .child(outgoing(
            MessageContent::new()
                .child(
                    view()
                        .w_full()
                        .max_w(px(280.0))
                        .self_end()
                        .child(preview_image),
                )
                .child(sent(
                    "Here's the image. Can you add it to the PDF? Use it for the cover page.",
                )),
        ))
        .child(Message::new(
            MessageContent::new()
                .child(received(
                    "Done. Here's the PDF with the image added as the cover page.",
                ))
                .child(
                    view().w_full().max_w(px(280.0)).child(
                        Attachment::new()
                            .media(AttachmentMedia::icon(
                                text("▤").accessibility_elements_hidden(true),
                            ))
                            .content(
                                AttachmentContent::new(AttachmentTitle::new("sales-dashboard.pdf"))
                                    .description(AttachmentDescription::new("PDF · 2.4 MB")),
                            )
                            .actions(AttachmentActions::new().child(AttachmentAction::new(
                                "Download",
                                "↓",
                                move || {
                                    requested.set(true);
                                    announce_for_accessibility(
                                        "Download requested for sales-dashboard.pdf",
                                    );
                                },
                            ))),
                    ),
                )
                .footer(MessageFooter::new().child(when(requested, move || {
                    text("Download requested for sales-dashboard.pdf").text_xs()
                }))),
        ))
        .child(outgoing(
            MessageContent::new().child(sent("Thanks. Looks good.")),
        ))
}
