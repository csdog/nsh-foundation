use std::time::Duration;

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::{
    bubble::{Bubble, BubbleContent, BubbleVariant},
    button::{Button, ButtonSize, ButtonVariant},
    input::input,
    marker::{Marker, MarkerContent, MarkerVariant},
    message::{Message, MessageAlign, MessageContent, MessageHeader},
    message_scroller::{
        MessageScrollAlign, MessageScroller, MessageScrollerHandle, MessageScrollerItem,
        OpeningPosition,
    },
};

type Item = MessageScrollerItem<ChatRow>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Speaker {
    User,
    Assistant,
    Teammate,
    Marker,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entrance {
    Fade,
    Rise,
    Spring,
}

#[derive(Clone, Debug, PartialEq)]
struct ChatRow {
    speaker: Speaker,
    text: SharedString,
    entrance: Option<Entrance>,
}

fn item(id: impl Into<SharedString>, speaker: Speaker, text: impl Into<SharedString>) -> Item {
    Item::new(
        id,
        ChatRow {
            speaker,
            text: text.into(),
            entrance: None,
        },
    )
    .anchor(speaker == Speaker::User)
}

fn transcript() -> Vec<Item> {
    use Speaker::{Assistant, User};
    vec![
        item(
            "activation",
            User,
            "We're seeing activation dip after workspace creation. Can you help me find the likely step?",
        ),
        item(
            "activation-reply",
            Assistant,
            "The sharpest drop is between creating the workspace and inviting the first teammate.\n\nWorkspace creation is still healthy, but the invite step is where users pause. That gives us a specific point to investigate.",
        ),
        item(
            "compare",
            User,
            "What should I compare before we change the onboarding flow?",
        ),
        item(
            "compare-reply",
            Assistant,
            "Compare three cohorts:\n\n1. Users who choose a template before inviting teammates.\n2. Users who start from a blank workspace.\n3. Users who skip invites and return within 24 hours.\n\nIf template users invite faster, test better first-run guidance.",
        ),
        item("experiment", User, "Can you turn that into an experiment?"),
        item(
            "experiment-reply",
            Assistant,
            "Create a short checklist after workspace creation. Pick a template and add one project detail. Invite a teammate when the workspace has context.\n\nMeasure first invite completion, 24-hour return rate, and whether teams create a second project.",
        ),
        item(
            "risk",
            User,
            "What's the risk if we delay the invite prompt?",
        ),
        item(
            "risk-reply",
            Assistant,
            "The main risk is reducing team creation for accounts that already know who they want to invite.\n\nKeep the invite action visible in the header and change the primary empty-state guidance. Confident teams get a direct route without forcing uncertain users through the invite step too early.\n\nCompare both activation and early retention before shipping the experiment more widely. Review the results with the onboarding team.\n\nBefore launch, pick a conversion window for each cohort and keep it fixed. Summarize the affected steps by account size, template use, and whether the workspace has a teammate. Keep the comparison readable for the people reviewing the rollout.\n\nAfter a week, read the qualitative feedback alongside the metric. Check whether someone who skipped the prompt found a different path to invite a teammate or simply abandoned the setup flow.",
        ),
    ]
}

fn render_row(row: ReadSignal<Item>) -> AnyElement {
    let initial = row.get_untracked().value;
    let speaker = initial.speaker;
    let label = move || row.with(|item| item.value.text.clone());
    let content = if speaker == Speaker::Marker {
        Marker::new(MarkerContent::new(label))
            .variant(MarkerVariant::Separator)
            .into_element()
    } else {
        let end = speaker == Speaker::User;
        let mut message =
            MessageContent::new().child(Bubble::new(BubbleContent::text(label)).variant(if end {
                BubbleVariant::Muted
            } else {
                BubbleVariant::Ghost
            }));
        if speaker == Speaker::Teammate {
            message = message.flush().header(MessageHeader::text("Marcus"));
        }
        Message::new(message)
            .align(if end {
                MessageAlign::End
            } else {
                MessageAlign::Start
            })
            .into_element()
    };
    let Some(entrance) = initial.entrance.filter(|_| !reduce_motion()) else {
        return content;
    };
    let offset = match entrance {
        Entrance::Fade => 0.0,
        Entrance::Rise => 22.0,
        Entrance::Spring => 46.0,
    };
    let y = animated_value(offset);
    let opacity = animated_value(0.0);
    let motion = match entrance {
        Entrance::Spring => Easing::Bezier(0.22, 1.30, 0.36, 1.0),
        _ => Easing::Bezier(0.2, 0.8, 0.2, 1.0),
    };
    let _ = y.timing(0.0, 320.0, motion);
    let _ = opacity.timing(1.0, 230.0, Easing::Linear);
    view()
        .bind_translate_y(&y)
        .bind_opacity(&opacity)
        .child(content)
        .into_element()
}

fn control(label: &'static str, handler: impl Fn() + 'static) -> View {
    Button::new(label)
        .size(ButtonSize::Small)
        .variant(ButtonVariant::Outline)
        .on_press(handler)
}

fn selected(
    label: &'static str,
    active: impl Fn() -> bool + Copy + 'static,
    handler: impl Fn() + 'static,
) -> View {
    control(label, handler)
        .accessibility_state(move || AccessibilityState {
            pressed: Some(if active() {
                nsh_core::Checked::True
            } else {
                nsh_core::Checked::False
            }),
            ..Default::default()
        })
        .style_when(active, |style| style.bg(theme().muted))
}

fn controls() -> View {
    view().flex_row().flex_wrap().items_center().gap_2().p_3()
}

fn card(title: &'static str, viewport: View, actions: View) -> View {
    view()
        .w_full()
        .max_w(px(384.0))
        .overflow_hidden()
        .rounded(px(24.0))
        .border(px(1.0))
        .border_color(|| theme().border)
        .bg(|| theme().card)
        .text_color(|| theme().card_foreground)
        .child(
            view()
                .px_4()
                .py_3()
                .border_b(px(1.0))
                .border_color(|| theme().border)
                .child(text(title).text_sm().font_weight(FontWeight::SEMIBOLD)),
        )
        .child(viewport)
        .child(actions.border_t(px(1.0)).border_color(|| theme().border))
}

const PROMPTS: [&str; 3] = [
    "How do I keep a streaming reply in view?",
    "What if the reader scrolls up?",
    "Can the next turn leave some context visible?",
];
const REPLY: &str = "Follow the latest text while the reader is at the bottom. Let the viewport stay where they leave it when they scroll up. A jump to the latest message resumes following.\n\nKeep each row keyed by its message id so older messages can be inserted without losing the reader's place. The newest turn can start near the top with some of the previous exchange still visible.\n\nAllow streamed replies to grow below the turn until they fill the viewport. Keep controls available at any scroll position so the reader can easily return to the live edge or jump to the beginning.\n\nFor longer responses, group paragraphs and preserve the reader's selection across updates.";

#[derive(Clone, Copy)]
struct Composer {
    rows: Signal<Vec<Item>>,
    revision: Signal<usize>,
    busy: Signal<bool>,
    input: Signal<String>,
}

impl Composer {
    fn new(initial: Vec<Item>) -> Self {
        Self {
            rows: signal(initial),
            revision: signal(0),
            busy: signal(false),
            input: signal(String::new()),
        }
    }

    fn reset(self, initial: Vec<Item>, handle: MessageScrollerHandle) {
        self.revision.update(|value| *value += 1);
        self.busy.set(false);
        self.input.set(String::new());
        self.rows.set(initial);
        handle.scroll_to_end(false);
    }

    fn send(self, anchor_assistant: bool, streaming: bool, entrance: Option<Entrance>) {
        if self.busy.get_untracked() {
            return;
        }
        let revision = self.revision.get_untracked() + 1;
        self.revision.set(revision);
        let prompt = self.input.get_untracked();
        self.input.set(String::new());
        let prompt = if prompt.trim().is_empty() {
            PROMPTS[(revision - 1) % PROMPTS.len()].to_owned()
        } else {
            prompt
        };
        let mut message =
            item(format!("turn-{revision}"), Speaker::User, prompt).anchor(!anchor_assistant);
        message.value.entrance = entrance;
        let response_id = format!("reply-{revision}");
        self.rows.update(|rows| {
            rows.push(message);
            rows.push(
                item(
                    response_id,
                    Speaker::Assistant,
                    if streaming {
                        "Follow the latest text"
                    } else {
                        REPLY
                    },
                )
                .anchor(anchor_assistant),
            );
        });
        if !streaming {
            return;
        }
        self.busy.set(true);
        spawn(async move {
            let mut reply = String::new();
            let mut cursor = REPLY.split_inclusive(' ').peekable();
            while cursor.peek().is_some() {
                timer(Duration::from_millis(95)).await;
                if self.revision.get_untracked() != revision {
                    return;
                }
                for part in (&mut cursor).take(4) {
                    reply.push_str(part);
                }
                self.rows.update(|rows| {
                    if let Some(last) = rows.last_mut() {
                        last.value.text = SharedString::from(reply.clone());
                    }
                });
            }
            self.busy.set(false);
        });
    }
}

fn composer_preview(
    title: &'static str,
    editing: bool,
    context_choices: bool,
    streaming: bool,
) -> View {
    let chat = Composer::new(transcript());
    let handle = MessageScrollerHandle::new();
    let peek = signal(32.0_f32);
    let viewport = MessageScroller::new(handle, chat.rows)
        .default_scroll_position(OpeningPosition::End)
        .auto_scroll(streaming)
        .previous_item_peek(peek)
        .render(render_row);
    let send = move || chat.send(false, streaming, None);
    let mut actions = controls();
    if editing {
        actions = actions.child(
            input()
                .label("Message")
                .placeholder("Type a message")
                .value(move || chat.input.get())
                .on_change(move |value: &str| chat.input.set(value.to_owned()))
                .on_submit(move |_: &str| send())
                .flex_1()
                .min_w(px(100.0))
                .h(px(32.0))
                .px_2()
                .rounded_md()
                .border(px(1.0))
                .border_color(|| theme().border),
        );
    }
    if context_choices {
        actions = actions.child(text("Previous context").text_xs()).children(
            [("0 px", 0.0), ("32 px", 32.0), ("64 px", 64.0)].map(|(label, amount)| {
                selected(
                    label,
                    move || peek.get() == amount,
                    move || peek.set(amount),
                )
            }),
        );
    }
    actions = actions
        .child(
            Button::new("Send")
                .size(ButtonSize::Small)
                .disabled(chat.busy)
                .on_press(send),
        )
        .child(control("Reset", move || chat.reset(transcript(), handle)));
    card(title, viewport, actions)
}

pub fn demo() -> View {
    composer_preview("Conversation", true, false, true)
}

pub fn streaming() -> View {
    composer_preview("Streaming", true, false, true)
}

pub fn previous_context() -> View {
    composer_preview("Keeping Context Visible", true, true, false)
}

pub fn anchoring() -> View {
    let chat = Composer::new(transcript());
    let handle = MessageScrollerHandle::new();
    let assistant = signal(false);
    card(
        "Anchoring Turns",
        MessageScroller::new(handle, chat.rows)
            .default_scroll_position(OpeningPosition::End)
            .render(render_row),
        controls()
            .child(selected(
                "Anchor user messages",
                move || !assistant.get(),
                move || assistant.set(false),
            ))
            .child(selected(
                "Anchor assistant messages",
                move || assistant.get(),
                move || assistant.set(true),
            ))
            .child(control("Send Message", move || {
                chat.send(assistant.get_untracked(), false, None)
            }))
            .child(control("Reset", move || chat.reset(transcript(), handle))),
    )
}

pub fn group_chat() -> View {
    let chat = Composer::new(transcript());
    let handle = MessageScrollerHandle::new();
    card("Group Chat",
        MessageScroller::new(handle, chat.rows).default_scroll_position(OpeningPosition::End).render(render_row),
        controls()
            .child(control("Simulate group chat", move || {
                let revision = chat.revision.get_untracked() + 1;
                chat.revision.set(revision);
                chat.rows.update(|rows| {
                    rows.push(item(format!("joined-{revision}"), Speaker::Marker, "Marcus joined the chat").anchor(true));
                    rows.push(item(format!("marcus-{revision}"), Speaker::Teammate, "I can take a look at the onboarding experiment. What should we compare first?"));
                });
            }))
            .child(control("Reset", move || chat.reset(transcript(), handle))),
    )
}

pub fn opening_position() -> impl IntoElement {
    let selected_position = signal(OpeningPosition::LastAnchor);
    dynamic(move || {
        let position = selected_position.get();
        let handle = MessageScrollerHandle::new();
        card(
            "Opening Position",
            MessageScroller::new(handle, move || transcript())
                .default_scroll_position(position)
                .render(render_row),
            controls().children(
                [
                    ("start", OpeningPosition::Start),
                    ("end", OpeningPosition::End),
                    ("last-anchor", OpeningPosition::LastAnchor),
                ]
                .map(|(label, mode)| {
                    selected(
                        label,
                        move || selected_position.get() == mode,
                        move || selected_position.set(mode),
                    )
                }),
            ),
        )
    })
}

pub fn load_history() -> View {
    let all = transcript();
    let rows = signal(all[4..].to_vec());
    let earlier = stored(all[..4].to_vec());
    let loaded = signal(false);
    let handle = MessageScrollerHandle::new();
    card(
        "Load History",
        MessageScroller::new(handle, rows)
            .default_scroll_position(OpeningPosition::Start)
            .render(render_row),
        controls()
            .child(
                Button::new("Load earlier messages")
                    .size(ButtonSize::Small)
                    .disabled(loaded)
                    .on_press(move || {
                        rows.update(|rows| rows.splice(0..0, earlier.get()).for_each(drop));
                        loaded.set(true);
                    }),
            )
            .child(control("Reset", move || {
                rows.set(transcript()[4..].to_vec());
                loaded.set(false);
                handle.scroll_to_start(false);
            })),
    )
}

pub fn animation() -> View {
    let chat = Composer::new(transcript());
    let handle = MessageScrollerHandle::new();
    let preset = signal(Entrance::Rise);
    card(
        "Animation",
        MessageScroller::new(handle, chat.rows)
            .default_scroll_position(OpeningPosition::End)
            .render(render_row),
        controls()
            .children(
                [
                    ("Fade", Entrance::Fade),
                    ("Rise", Entrance::Rise),
                    ("Spring", Entrance::Spring),
                ]
                .map(|(label, style)| {
                    selected(
                        label,
                        move || preset.get() == style,
                        move || preset.set(style),
                    )
                }),
            )
            .child(control("Send Message", move || {
                chat.send(false, false, Some(preset.get_untracked()))
            }))
            .child(control("Reset", move || chat.reset(transcript(), handle))),
    )
}

fn jump_controls(handle: MessageScrollerHandle) -> View {
    controls()
        .child(control("Go to start", move || handle.scroll_to_start(true)))
        .child(control("Go to end", move || handle.scroll_to_end(true)))
        .children(
            [
                ("Activation", "activation"),
                ("Compare", "compare"),
                ("Experiment", "experiment"),
                ("Risk", "risk"),
            ]
            .map(|(label, id)| {
                control(label, move || {
                    handle.scroll_to_message(id, MessageScrollAlign::Start, true);
                })
            }),
        )
}

pub fn commands() -> View {
    let handle = MessageScrollerHandle::new();
    card(
        "Commands",
        MessageScroller::new(handle, move || transcript())
            .default_scroll_position(OpeningPosition::End)
            .render(render_row),
        jump_controls(handle),
    )
}

pub fn visibility() -> View {
    let handle = MessageScrollerHandle::new();
    let state = handle.state();
    card(
        "Transcript Outline",
        MessageScroller::new(handle, move || transcript())
            .default_scroll_position(OpeningPosition::Start)
            .render(render_row),
        view().child(jump_controls(handle).border_t(px(0.0))).child(
            view()
                .px_3()
                .pb_3()
                .gap_1()
                .child(
                    text(move || {
                        state.with(|state| {
                            format!(
                                "Current turn: {}",
                                state.current_anchor.as_deref().unwrap_or("—")
                            )
                        })
                    })
                    .text_xs(),
                )
                .child(
                    text(move || {
                        state.with(|state| {
                            let visible = state
                                .visible_messages
                                .iter()
                                .map(|id| id.as_ref())
                                .collect::<Vec<_>>()
                                .join(", ");
                            format!("Visible: {visible}")
                        })
                    })
                    .text_xs(),
                ),
        ),
    )
}

pub fn scrollable() -> View {
    let handle = MessageScrollerHandle::new();
    let state = handle.state();
    card(
        "Scroll Status",
        MessageScroller::new(handle, move || transcript())
            .default_scroll_position(OpeningPosition::Start)
            .render(render_row),
        controls()
            .child(control("Go to start", move || handle.scroll_to_start(true)))
            .child(control("Go to end", move || handle.scroll_to_end(true)))
            .child(
                text(move || {
                    if state.with(|s| s.can_scroll_start) {
                        "Can scroll up"
                    } else {
                        "At start"
                    }
                })
                .text_xs(),
            )
            .child(
                text(move || {
                    if state.with(|s| s.can_scroll_end) {
                        "Can scroll down"
                    } else {
                        "At end"
                    }
                })
                .text_xs(),
            ),
    )
}
