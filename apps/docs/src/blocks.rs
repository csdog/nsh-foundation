use std::time::Duration;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    input::input,
    popover::{Popover, PopoverContent, PopoverSide, PopoverTrigger},
    textarea::textarea,
};
use nsh_base::theme;
use nsh_core::{
    SubmitBehavior, element::set_clipboard, file_dialog::FileFilter, keymap::When,
    picker::FilePickRequest, prelude::*,
};

const FILE_LIMIT: usize = 8;

fn ink() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0x182523), rgb(0xe9f1ef))
}

fn canvas_color() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0xf9fbfa), rgb(0x101817))
}

fn accent() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0x14594d), rgb(0x9be0ce))
}

fn tint() -> AdaptiveColor {
    AdaptiveColor::new(rgb(0xe4f2ed), rgb(0x233c35))
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
    pending: Signal<Option<usize>>,
}

impl Thread {
    fn new(id: usize) -> Self {
        Self {
            id,
            title: signal("New chat".to_owned()),
            draft: signal(String::new()),
            attachments: signal(Vec::new()),
            messages: signal(Vec::new()),
            pending: signal(None),
        }
    }

    fn can_send(self) -> bool {
        self.draft.with(|draft| !draft.trim().is_empty())
            || self.attachments.with(|files| !files.is_empty())
    }

    fn send(self) {
        if self.pending.get_untracked().is_some() || !untracked(|| self.can_send()) {
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
        let id = self.messages.with_untracked(Vec::len);
        self.messages.update(|messages| {
            messages.push(Message {
                id,
                local_reply: false,
                text: content.trim().into(),
                attachments,
            });
        });
        self.pending.set(Some(id));
        spawn(async move {
            timer(Duration::from_millis(650)).await;
            if self.pending.get_untracked() != Some(id) {
                return;
            }
            let text = match count {
                0 => "Message received. This is a local chat demo.".into(),
                1 => {
                    "Message received with one attachment. Its contents stay on your device.".into()
                }
                count => format!(
                    "Message received with {count} attachments. Their contents stay on your device."
                ),
            };
            self.messages.update(|messages| {
                messages.push(Message {
                    id: id + 1,
                    local_reply: true,
                    text,
                    attachments: Vec::new(),
                });
            });
            self.pending.set(None);
        });
    }
}

#[derive(Clone, Copy)]
struct Chat {
    owner: Owner,
    threads: Signal<Vec<Thread>>,
    active: Signal<Thread>,
    history_open: Signal<bool>,
    history_trigger: ElementRef,
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
            history_open: signal(false),
            history_trigger: element_ref(),
            picker_open: signal(false),
            next_file: stored(0),
            transcript: element_ref(),
            editor: element_ref(),
        }
    }

    fn select(self, thread: Thread) {
        self.active.set(thread);
        self.close_history();
    }

    fn close_history(self) -> bool {
        if !self.history_open.get_untracked() {
            return false;
        }
        self.history_open.set(false);
        self.owner.run(|| {
            spawn(async move {
                next_frame().await;
                if !self.history_open.get_untracked() && !desktop() {
                    self.history_trigger.focus();
                }
            });
        });
        true
    }

    fn new_thread(self) {
        let current = self.active.get_untracked();
        if !current.messages.with_untracked(Vec::is_empty) || untracked(|| current.can_send()) {
            let id = self.threads.with_untracked(Vec::len);
            let thread = self.owner.run(|| Thread::new(id));
            self.threads.update(|threads| threads.insert(0, thread));
            self.select(thread);
        } else {
            self.close_history();
        }
        if untracked(desktop) {
            self.editor.focus();
        }
    }

    fn send(self) {
        let thread = self.active.get_untracked();
        self.owner.run(|| thread.send());
    }

    fn pick(self, photos: bool) {
        if self.picker_open.get_untracked() {
            return;
        }
        let thread = self.active.get_untracked();
        let slots = FILE_LIMIT.saturating_sub(thread.attachments.with_untracked(Vec::len));
        if slots == 0 {
            return;
        }
        self.picker_open.set(true);
        let request = FilePickRequest::new(true, slots as u32).max_size(0);
        let request = if photos {
            request.filter(FileFilter::new(
                "Images",
                ["jpg", "jpeg", "png", "gif", "webp", "heic", "heif", "avif"],
            ))
        } else {
            request
        };
        let pending = pick_files(request);
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

fn history_width() -> f32 {
    let width = window().width.0;
    (width * 0.78).min((width - 56.0).max(0.0))
}

fn mobile_history(chat: Chat) -> View {
    on_key_with(
        "escape",
        KeyOptions {
            when: When::Always,
            for_focus: true,
            ..Default::default()
        },
        move |_| {
            chat.close_history();
        },
    );
    let open = chat.history_open;
    let close = element_ref();
    effect(move || {
        if open.get() {
            spawn(async move {
                next_frame().await;
                if open.get_untracked() {
                    close.focus();
                }
            });
        }
    });
    view()
        .absolute()
        .top_0()
        .bottom_0()
        .left_0()
        .w(|| px(history_width()))
        .pt(|| safe_area().top + px(6.0))
        .pb(|| safe_area().bottom)
        .bg(tint())
        .role(AccessibilityRole::Dialog)
        .label("Conversations")
        .accessibility_elements_hidden(move || !open.get())
        .test_id("chat-history-underlay")
        .child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .px_4()
                .pb_2()
                .child(
                    text("Relay")
                        .text_xl()
                        .text_color(accent())
                        .font_weight(FontWeight::BOLD),
                )
                .child(
                    icon_action("Close history", "×")
                        .on_press(move || {
                            chat.close_history();
                        })
                        .size(px(44.0))
                        .element_ref(&close),
                ),
        )
        .child(history(chat))
}

pub(crate) fn page(go_components: impl Fn() + 'static, toggle_theme: impl Fn() + 'static) -> View {
    let chat = Chat::new();
    let revealing_history = computed(move || !desktop() && chat.history_open.get());
    let history_mounted = signal(false);
    let chat_offset = animated_value(0.0);
    let static_offset = move || {
        px(if revealing_history.get() && reduce_motion() {
            history_width()
        } else {
            0.0
        })
    };
    effect(move || {
        let open = revealing_history.get();
        let offset = if open { history_width() } else { 0.0 };
        if open {
            history_mounted.set(true);
        }
        if reduce_motion() || desktop() {
            chat_offset.set(0.0);
            history_mounted.set(open);
            if desktop() {
                chat.history_open.set(false);
            }
        } else {
            let transition =
                chat_offset.timing(offset, 260.0, Easing::Bezier(0.25, 0.8, 0.25, 1.0));
            if !open {
                spawn(async move {
                    if transition.await && !revealing_history.get_untracked() {
                        history_mounted.set(false);
                    }
                });
            }
        }
    });
    on_back_press(move || chat.close_history());
    let messages = computed(move || {
        let thread = chat.active.get();
        (thread.id, thread.messages.get())
    });

    let empty = computed(move || messages.with(|(_, list)| list.is_empty()));
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
        .relative()
        .overflow_hidden()
        .bg(tint())
        .text_color(ink())
        .child(when(history_mounted, move || mobile_history(chat)))
        .child(
            view()
                .flex_row()
                .flex_1()
                .min_h(px(0.0))
                .pointer_events(PointerEvents::BoxNone)
                .child(when(desktop, move || {
                    view()
                        .test_id("desktop-chat-history")
                        .w(px(244.0))
                        .flex_shrink_0()
                        .border_r(px(1.0))
                        .border_color(|| theme().border)
                        .bg(tint().opacity(0.45))
                        .child(
                            text("Relay")
                                .px_5()
                                .pt_5()
                                .pb_2()
                                .text_lg()
                                .text_color(accent())
                                .font_weight(FontWeight::BOLD),
                        )
                        .child(history(chat))
                }))
                .child(
                    view()
                        .flex_1()
                        .min_w(px(0.0))
                        .min_h(px(0.0))
                        .test_id("chat-main-surface")
                        .relative()
                        .left(static_offset)
                        .bind_translate_x(&chat_offset)
                        .bg(canvas_color())
                        .style_when(history_mounted, |style| {
                            style.rounded_l(px(22.0)).shadow_xl().overflow_hidden()
                        })
                        .pointer_events(move || {
                            if revealing_history.get() {
                                PointerEvents::None
                            } else {
                                PointerEvents::Auto
                            }
                        })
                        .accessibility_elements_hidden(revealing_history)
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
                                        .px_4()
                                        .pt_6()
                                        .pb_4()
                                        .gap_5()
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
                                        ))
                                        .child(when(
                                            move || chat.active.get().pending.get().is_some(),
                                            || {
                                                text("Preparing a reply…")
                                                    .test_id("chat-pending")
                                                    .text_sm()
                                                    .text_color(accent())
                                            },
                                        )),
                                ),
                        )
                        .child(when(empty, move || welcome(chat)))
                        .child(view().flex_grow(move || {
                            if empty.get() && !desktop() {
                                1.0_f32
                            } else {
                                0.0
                            }
                        }))
                        .child(composer(chat))
                        .child(
                            view()
                                .flex_grow(move || {
                                    if empty.get() && desktop() {
                                        1.5_f32
                                    } else {
                                        0.0
                                    }
                                })
                                .test_id("chat-composer-spacer"),
                        ),
                ),
        )
        .child(when(revealing_history, move || {
            view()
                .absolute()
                .left(static_offset)
                .top_0()
                .bottom_0()
                .w(|| px(window().width.0 - history_width()))
                .bind_translate_x(&chat_offset)
                .bg(ink().opacity(0.08))
                .rounded_l(px(22.0))
                .role(AccessibilityRole::Button)
                .label("Return to chat")
                .test_id("chat-history-return")
                .on_press(move || {
                    chat.close_history();
                })
        }))
}

fn header(
    chat: Chat,
    go_components: impl Fn() + 'static,
    toggle_theme: impl Fn() + 'static,
) -> View {
    let menu = signal(false);
    view()
        .flex_row()
        .justify_between()
        .items_center()
        .flex_shrink_0()
        .gap_2()
        .px_3()
        .min_h(px(56.0))
        .border_b(px(1.0))
        .border_color(|| theme().border)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .min_w(px(0.0))
                .child(when(
                    || !desktop(),
                    move || {
                        icon_action("History", "☰")
                            .on_press(move || chat.history_open.set(true))
                            .size(px(44.0))
                            .element_ref(&chat.history_trigger)
                            .accessibility_state(move || AccessibilityState {
                                expanded: Some(chat.history_open.get()),
                                ..Default::default()
                            })
                    },
                ))
                .child(
                    view()
                        .min_w(px(0.0))
                        .child(
                            text(move || SharedString::from(chat.active.get().title.get()))
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .truncate(),
                        )
                        .child(
                            text("Relay workspace")
                                .text_xs()
                                .text_color(accent())
                                .truncate(),
                        ),
                ),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .child(
                    icon_action("Start new chat", "＋")
                        .on_press(move || chat.new_thread())
                        .size(px(44.0))
                        .test_id("chat-new"),
                )
                .child(
                    Popover::new(
                        PopoverTrigger::render(
                            icon_action("Chat options", "⋯").surface().size(px(44.0)),
                        ),
                        PopoverContent::new()
                            .label("Chat options")
                            .align(Align::End)
                            .width(196.0)
                            .child(
                                action("Theme")
                                    .on_press(move || {
                                        menu.set(false);
                                        toggle_theme();
                                    })
                                    .justify_start()
                                    .h(px(44.0)),
                            )
                            .child(
                                action("Components")
                                    .on_press(go_components)
                                    .justify_start()
                                    .h(px(44.0)),
                            ),
                    )
                    .open(menu),
                ),
        )
}

fn action(label: impl IntoReactive<SharedString>) -> Button {
    Button::new(label)
        .variant(ButtonVariant::Ghost)
        .radius(8.0)
        .pointer_cursor()
}

fn icon_action(label: &'static str, glyph: &'static str) -> Button {
    action(label)
        .size(ButtonSize::Icon)
        .icon_start(text(glyph).text_xl())
        .radius(22.0)
}

fn history(chat: Chat) -> View {
    let query = signal(String::new());
    let visible_threads = computed(move || {
        let query = query.with(|value| value.trim().to_lowercase());
        chat.threads.with(|threads| {
            threads
                .iter()
                .filter(|thread| {
                    query.is_empty()
                        || thread
                            .title
                            .with(|title| title.to_lowercase().contains(&query))
                })
                .copied()
                .collect::<Vec<_>>()
        })
    });
    let list = view()
        .gap_1()
        .role(AccessibilityRole::Navigation)
        .label("Chat history")
        .child(when(
            move || visible_threads.with(Vec::is_empty),
            || {
                text("No matching chats")
                    .text_sm()
                    .px_2()
                    .text_color(|| theme().muted_foreground)
            },
        ))
        .child(each(
            visible_threads,
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
        ));
    view()
        .gap_2()
        .px_3()
        .pb_3()
        .flex_1()
        .min_h(px(0.0))
        .child(
            Button::new("New chat")
                .variant(ButtonVariant::Outline)
                .icon_start(text("＋").text_lg())
                .radius(10.0)
                .on_press(move || chat.new_thread())
                .justify_start()
                .h(px(44.0)),
        )
        .child(
            input()
                .label("Search chats")
                .placeholder("Search chats")
                .h(px(44.0))
                .value(move || query.get())
                .on_change(move |value: &str| query.set(value.to_owned())),
        )
        .child(
            text("Recent chats")
                .text_xs()
                .text_color(accent())
                .px_2()
                .pt_2(),
        )
        .child(scroll().min_h(px(0.0)).flex_1().child(list))
}

fn welcome(chat: Chat) -> View {
    view()
        .w_full()
        .max_w(px(760.0))
        .self_center()
        .px_4()
        .pb_4()
        .items_center()
        .gap_3()
        .child(
            text("Start a conversation")
                .role(AccessibilityRole::Header)
                .text_size(move || px(if desktop() { 32.0 } else { 25.0 }))
                .font_weight(FontWeight::BOLD)
                .text_center(),
        )
        .child(when(
            || window().height.0 >= 500.0,
            move || {
                view()
                    .flex_row()
                    .flex_wrap()
                    .justify_center()
                    .gap_2()
                    .children(
                        [
                            "Sketch a project plan",
                            "Explain a concept",
                            "Draft an outline",
                        ]
                        .into_iter()
                        .map(move |prompt| {
                            action(prompt)
                                .variant(ButtonVariant::Outline)
                                .radius(18.0)
                                .on_press(move || {
                                    chat.active.get_untracked().draft.set(prompt.into());
                                    chat.editor.focus();
                                })
                                .h(px(44.0))
                        }),
                    )
            },
        ))
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
    if message.local_reply {
        let copied = signal(false);
        let body = stored(message.text);
        return view()
            .test_id("chat-reply")
            .gap_1()
            .items_start()
            .child(
                text("Relay")
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(accent()),
            )
            .child(
                text(body.with(|text| text.clone()))
                    .text_base()
                    .line_height(px(24.0))
                    .selectable(true),
            )
            .child(
                action(move || {
                    SharedString::from(if copied.get() { "Copied" } else { "Copy reply" })
                })
                .on_press(move || {
                    set_clipboard(body.with(|text| text.clone()));
                    copied.set(true);
                })
                .h(px(44.0)),
            );
    }
    view()
        .test_id("chat-message")
        .self_end()
        .max_w(percentage(0.86))
        .min_w(px(0.0))
        .gap_2()
        .px_4()
        .py_3()
        .rounded(px(19.0))
        .bg(tint())
        .child((!message.text.is_empty()).then(|| {
            text(message.text)
                .text_base()
                .line_height(px(23.0))
                .selectable(true)
        }))
        .children(
            message
                .attachments
                .iter()
                .map(|file| file_label(file).flex_grow(0.0)),
        )
}

fn attachment_menu(chat: Chat) -> Popover {
    let open = signal(false);
    let trigger = icon_action("Add attachment", "+")
        .disabled(move || {
            chat.picker_open.get() || chat.active.get().attachments.with(Vec::len) >= FILE_LIMIT
        })
        .surface()
        .size(px(44.0));
    let mut menu = PopoverContent::new()
        .label("Add attachment")
        .side(PopoverSide::Top)
        .align(Align::Start)
        .width(212.0)
        .side_offset(12.0);
    for (photos, label, glyph) in [(true, "Photos", "▧"), (false, "Files", "▤")] {
        menu = menu.child(
            action(label)
                .icon_start(text(glyph).text_lg().text_color(accent()))
                .on_press(move || {
                    open.set(false);
                    chat.pick(photos);
                })
                .justify_start()
                .h(px(44.0)),
        );
    }
    Popover::new(PopoverTrigger::render(trigger), menu).open(open)
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
        .max_w(px(760.0))
        .self_center()
        .px_3()
        .pt_2()
        .test_id("chat-composer")
        .pb(|| safe_area().bottom + px(10.0))
        .gap_2()
        .child(
            view()
                .bg(|| theme().background)
                .rounded(px(24.0))
                .shadow_sm()
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
                    view()
                        .flex_row()
                        .items_end()
                        .gap_1()
                        .child(attachment_menu(chat))
                        .child(
                            textarea()
                                .label("Message")
                                .placeholder(move || {
                                    if chat.active.get().messages.with(Vec::is_empty) {
                                        "Ask Relay"
                                    } else {
                                        "Follow up"
                                    }
                                })
                                .element_ref(&chat.editor)
                                .bg(AdaptiveColor::TRANSPARENT)
                                .min_w(px(0.0))
                                .w(auto())
                                .flex_1()
                                .min_h(px(44.0))
                                .max_h(px(120.0))
                                .submit_behavior(SubmitBehavior::Submit)
                                .value(move || chat.active.get().draft.get())
                                .on_change(move |value: &str| {
                                    chat.active.get_untracked().draft.set(value.to_owned())
                                })
                                .on_submit(move |_: &str| chat.send()),
                        )
                        .child(
                            action(move || {
                                if chat.active.get().pending.get().is_some() {
                                    "Stop reply"
                                } else {
                                    "Send"
                                }
                            })
                            .size(ButtonSize::Icon)
                            .radius(22.0)
                            .icon_start(
                                text(move || {
                                    if chat.active.get().pending.get().is_some() {
                                        "■"
                                    } else {
                                        "↑"
                                    }
                                })
                                .text_lg(),
                            )
                            .disabled(move || {
                                let thread = chat.active.get();
                                thread.pending.get().is_none() && !thread.can_send()
                            })
                            .on_press(move || {
                                let thread = chat.active.get_untracked();
                                if thread.pending.get_untracked().is_some() {
                                    thread.pending.set(None);
                                } else {
                                    chat.send();
                                }
                            })
                            .size(px(44.0))
                            .bg(accent())
                            .text_color(canvas_color()),
                        ),
                ),
        )
        .child(
            text("Local demo · No files are uploaded.")
                .text_center()
                .text_xs()
                .text_color(|| theme().muted_foreground),
        )
}
