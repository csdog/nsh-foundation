use components::{
    button::{Button, ButtonSize, ButtonVariant},
    sheet::{Sheet, SheetContent, SheetHeader, SheetSide, SheetTitle, SheetTrigger},
    textarea::textarea,
};
use nsh_base::theme;
use nsh_core::{SubmitBehavior, picker::FilePickRequest, prelude::*};

const FILE_LIMIT: usize = 8;

fn ink() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0x212a37), rgb(0xe9edf4))
}

fn canvas_color() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0xf1f3f7), rgb(0x151822))
}

fn accent() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0x4338aa), rgb(0xb8afff))
}

fn tint() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0xe9e5ff), rgb(0x2c2651))
}

#[derive(Clone, PartialEq)]
struct Attachment {
    id: usize,
    name: String,
    bytes: u64,
}

#[derive(Clone, PartialEq)]
struct Message {
    id: usize,
    local_reply: bool,
    text: String,
    attachments: Vec<Attachment>,
}

#[derive(Clone, Copy, PartialEq)]
struct Thread {
    id: usize,
    title: Signal<String>,
    draft: Signal<String>,
    attachments: Signal<Vec<Attachment>>,
    messages: Signal<Vec<Message>>,
}

impl Thread {
    fn new(id: usize) -> Self {
        Self {
            id,
            title: signal("New chat".to_owned()),
            draft: signal(String::new()),
            attachments: signal(Vec::new()),
            messages: signal(Vec::new()),
        }
    }

    fn can_send(self) -> bool {
        self.draft.with(|draft| !draft.trim().is_empty())
            || self.attachments.with(|files| !files.is_empty())
    }

    fn send(self) {
        if !untracked(|| self.can_send()) {
            return;
        }
        let content = self.draft.get_untracked();
        let attachments = self.attachments.get_untracked();
        self.draft.set(String::new());
        self.attachments.set(Vec::new());
        let count = attachments.len();
        if self.messages.with_untracked(Vec::is_empty) {
            let title = content.trim().chars().take(42).collect::<String>();
            self.title.set(if title.is_empty() {
                attachments[0].name.clone()
            } else {
                title
            });
        }
        self.messages.update(|messages| {
            let id = messages.len();
            messages.push(Message {
                id,
                local_reply: false,
                text: content.trim().into(),
                attachments,
            });
            let text = match count {
                0 => "Message received. This is a local chat demo.".into(),
                1 => {
                    "Message received with one attachment. Its contents stay on your device.".into()
                }
                count => format!(
                    "Message received with {count} attachments. Their contents stay on your device."
                ),
            };
            messages.push(Message {
                id: id + 1,
                local_reply: true,
                text,
                attachments: Vec::new(),
            });
        });
    }
}

#[derive(Clone, Copy)]
struct Chat {
    owner: Owner,
    threads: Signal<Vec<Thread>>,
    active: Signal<Thread>,
    history: Sheet,
    picker_open: Signal<bool>,
    next_file: Stored<usize>,
    transcript: ElementRef,
    editor: ElementRef,
}

impl Chat {
    fn new() -> Self {
        let initial = Thread::new(0);
        Self {
            owner: Owner::current(),
            threads: signal(vec![initial]),
            active: signal(initial),
            history: Sheet::new(signal(false)),
            picker_open: signal(false),
            next_file: stored(0),
            transcript: element_ref(),
            editor: element_ref(),
        }
    }

    fn select(self, thread: Thread) {
        self.active.set(thread);
        self.history.close();
    }

    fn new_thread(self) {
        let current = self.active.get_untracked();
        if !current.messages.with_untracked(Vec::is_empty) || untracked(|| current.can_send()) {
            let id = self.threads.with_untracked(Vec::len);
            let thread = self.owner.run(|| Thread::new(id));
            self.threads.update(|threads| threads.insert(0, thread));
            self.select(thread);
        } else {
            self.history.close();
        }
        if untracked(desktop) {
            self.editor.focus();
        }
    }

    fn pick(self) {
        if self.picker_open.get_untracked() {
            return;
        }
        let thread = self.active.get_untracked();
        let slots = FILE_LIMIT.saturating_sub(thread.attachments.with_untracked(Vec::len));
        if slots == 0 {
            return;
        }
        self.picker_open.set(true);
        let pending = pick_files(FilePickRequest::new(true, slots as u32).max_size(0));
        spawn(async move {
            let files = pending.await;
            self.picker_open.set(false);
            if files.is_empty() {
                return;
            }
            thread.attachments.update(|attachments| {
                for file in files.into_iter().take(FILE_LIMIT - attachments.len()) {
                    let id = self.next_file.with(|next| {
                        *next += 1;
                        *next
                    });
                    attachments.push(Attachment {
                        id,
                        name: file.name,
                        bytes: file.len,
                    });
                }
            });
        });
    }
}

fn desktop() -> bool {
    window().width.0 >= 840.0
}

pub(crate) fn page(go_components: impl Fn() + 'static, toggle_theme: impl Fn() + 'static) -> View {
    let chat = Chat::new();
    let messages = computed(move || {
        let thread = chat.active.get();
        (thread.id, thread.messages.get())
    });

    effect(move || {
        let (thread, len) = messages.with(|(thread, list)| (*thread, list.len()));
        spawn(async move {
            next_frame().await;
            if messages.with_untracked(|(current, list)| *current == thread && list.len() == len) {
                chat.transcript
                    .scroll_to(point(px(0.0), px(f32::MAX)), false);
            }
        });
    });

    view()
        .test_id("blocks-chat")
        .flex_1()
        .min_h(px(0.0))
        .bg(canvas_color())
        .text_color(ink())
        .child(
            view()
                .flex_row()
                .flex_1()
                .min_h(px(0.0))
                .child(when(desktop, move || {
                    view()
                        .test_id("desktop-chat-history")
                        .w(px(248.0))
                        .flex_shrink_0()
                        .border_r(px(1.0))
                        .border_color(|| theme().border)
                        .bg(|| theme().background)
                        .child(history(chat))
                }))
                .child(
                    view()
                        .flex_1()
                        .min_w(px(0.0))
                        .min_h(px(0.0))
                        .child(header(chat, go_components, toggle_theme))
                        .child(
                            scroll()
                                .flex_1()
                                .min_h(px(0.0))
                                .min_w(px(0.0))
                                .element_ref(&chat.transcript)
                                .test_id("chat-transcript")
                                .child(
                                    view()
                                        .w_full()
                                        .max_w(px(780.0))
                                        .self_center()
                                        .p_4()
                                        .gap_3()
                                        .child(when(
                                            move || messages.with(|(_, items)| items.is_empty()),
                                            move || welcome(chat),
                                        ))
                                        .child(each(
                                            move || {
                                                messages.with(|(thread, items)| {
                                                    items
                                                        .iter()
                                                        .cloned()
                                                        .map(|message| (*thread, message))
                                                        .collect::<Vec<_>>()
                                                })
                                            },
                                            |(thread, message)| (*thread, message.id),
                                            move |row, _| message_view(row.get_untracked().1),
                                        )),
                                ),
                        )
                        .child(composer(chat)),
                ),
        )
        .child(
            SheetContent::new(
                chat.history,
                SheetHeader::new(SheetTitle::new("Conversations")),
            )
            .side(SheetSide::Left)
            .close_button(Some("Close history"))
            .body(move || history(chat)),
        )
}

fn header(
    chat: Chat,
    go_components: impl Fn() + 'static,
    toggle_theme: impl Fn() + 'static,
) -> View {
    view()
        .flex_row()
        .justify_between()
        .items_center()
        .flex_shrink_0()
        .gap_1()
        .px_3()
        .min_h(px(58.0))
        .border_b(px(1.0))
        .border_color(|| theme().border)
        .bg(|| theme().background)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .min_w(px(0.0))
                .child(when(
                    || !desktop(),
                    move || {
                        SheetTrigger::render(chat.history, action("History").surface().h(px(44.0)))
                    },
                ))
                .child(
                    text("Blocks")
                        .font_weight(FontWeight::BOLD)
                        .text_color(accent())
                        .truncate(),
                ),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .child(action("Components").on_press(go_components).h(px(44.0)))
                .child(action("Theme").on_press(toggle_theme).h(px(44.0))),
        )
}

fn action(label: impl IntoReactive<SharedString>) -> Button {
    Button::new(label)
        .variant(ButtonVariant::Ghost)
        .radius(8.0)
        .pointer_cursor()
}

fn history(chat: Chat) -> View {
    view()
        .gap_3()
        .p_3()
        .min_h(px(0.0))
        .child(
            Button::new("New chat")
                .variant(ButtonVariant::Outline)
                .radius(8.0)
                .on_press(move || chat.new_thread())
                .h(px(44.0)),
        )
        .child(
            scroll()
                .min_h(px(0.0))
                .max_h(|| px((window().height.0 - 185.0).max(48.0)))
                .child(
                    view()
                        .gap_1()
                        .role(AccessibilityRole::Navigation)
                        .label("Chat history")
                        .child(each(
                            chat.threads,
                            |thread| thread.id,
                            move |thread, _| {
                                let thread = thread.get_untracked();
                                action(move || SharedString::from(thread.title.get()))
                                    .on_press(move || chat.select(thread))
                                    .justify_start()
                                    .h(px(44.0))
                                    .style_when(
                                        move || chat.active.get().id == thread.id,
                                        |s| s.bg(tint()).text_color(accent()),
                                    )
                            },
                        )),
                ),
        )
}

fn welcome(chat: Chat) -> View {
    view()
        .gap_3()
        .pt_6()
        .pb_4()
        .items_start()
        .child(
            text("Relay")
                .text_size(px(34.0))
                .font_weight(FontWeight::BOLD)
                .text_color(accent()),
        )
        .child(
            text("Start a conversation")
                .role(AccessibilityRole::Header)
                .text_xl()
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            text("Try a prompt or type your own message.")
                .text_sm()
                .text_color(|| theme().muted_foreground),
        )
        .child(
            view().flex_row().flex_wrap().gap_2().children(
                [
                    "Sketch a project plan",
                    "Explain a concept",
                    "Draft an outline",
                ]
                .into_iter()
                .map(move |prompt| {
                    Button::new(prompt)
                        .variant(ButtonVariant::Outline)
                        .radius(8.0)
                        .on_press(move || {
                            chat.active.get_untracked().draft.set(prompt.into());
                            chat.editor.focus();
                        })
                        .h(px(44.0))
                }),
            ),
        )
}

fn file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn file_label(file: &Attachment) -> View {
    view()
        .gap_1()
        .min_w(px(0.0))
        .flex_1()
        .child(text(file.name.clone()).text_sm().truncate())
        .child(
            text(file_size(file.bytes))
                .text_xs()
                .text_color(|| theme().muted_foreground),
        )
}

fn message_view(message: Message) -> View {
    view()
        .test_id(if message.local_reply {
            "chat-reply"
        } else {
            "chat-message"
        })
        .gap_2()
        .p_3()
        .rounded(px(12.0))
        .border(px(1.0))
        .border_color(|| theme().border)
        .bg(move || {
            if message.local_reply {
                theme().background
            } else {
                tint()
            }
        })
        .child(
            text(if message.local_reply {
                "Relay · Demo"
            } else {
                "You"
            })
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD),
        )
        .child((!message.text.is_empty()).then(|| text(message.text).text_sm().selectable(true)))
        .children(
            message
                .attachments
                .iter()
                .map(|file| file_label(file).flex_grow(0.0)),
        )
}

fn composer(chat: Chat) -> View {
    let pending = computed(move || {
        let thread = chat.active.get();
        thread
            .attachments
            .get()
            .into_iter()
            .map(|file| (thread, file))
            .collect::<Vec<_>>()
    });
    view()
        .flex_shrink_0()
        .w_full()
        .max_w(px(780.0))
        .self_center()
        .px_3()
        .pt_2()
        .pb(|| safe_area().bottom + px(10.0))
        .gap_2()
        .child(
            view()
                .bg(|| theme().background)
                .rounded(px(12.0))
                .border(px(1.0))
                .border_color(|| theme().border)
                .p_2()
                .child(when(
                    move || !pending.with(Vec::is_empty),
                    move || {
                        scroll().max_h(px(100.0)).child(view().gap_1().child(each(
                            pending,
                            |(thread, file)| (thread.id, file.id),
                            move |row, _| {
                                let (thread, file) = row.get_untracked();
                                view()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .min_w(px(0.0))
                                    .px_2()
                                    .rounded(px(8.0))
                                    .bg(canvas_color())
                                    .child(file_label(&file))
                                    .child(
                                        action(format!("Remove {}", file.name))
                                            .size(ButtonSize::Icon)
                                            .icon_start(text("×"))
                                            .on_press(move || {
                                                thread.attachments.update(|files| {
                                                    files.retain(|f| f.id != file.id)
                                                })
                                            })
                                            .size(px(44.0)),
                                    )
                            },
                        )))
                    },
                ))
                .child(
                    textarea()
                        .label("Message")
                        .placeholder("Write a message")
                        .element_ref(&chat.editor)
                        .min_h(px(48.0))
                        .max_h(px(120.0))
                        .submit_behavior(SubmitBehavior::Submit)
                        .value(move || chat.active.get().draft.get())
                        .on_change(move |value: &str| {
                            chat.active.get_untracked().draft.set(value.to_owned())
                        })
                        .on_submit(move |_: &str| chat.active.get_untracked().send()),
                )
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            action("Add attachment")
                                .disabled(move || {
                                    chat.picker_open.get()
                                        || chat.active.get().attachments.with(Vec::len)
                                            >= FILE_LIMIT
                                })
                                .on_press(move || chat.pick())
                                .h(px(44.0)),
                        )
                        .child(
                            Button::new("Send")
                                .radius(8.0)
                                .disabled(move || !chat.active.get().can_send())
                                .on_press(move || chat.active.get_untracked().send())
                                .h(px(44.0)),
                        ),
                ),
        )
        .child(
            text("Local demo · Conversation and attachments stay in this session.")
                .text_xs()
                .text_color(|| theme().muted_foreground),
        )
}
