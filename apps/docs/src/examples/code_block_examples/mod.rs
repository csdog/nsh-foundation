mod fixtures;

use std::time::Duration;

use components::{
    bubble::{Bubble, BubbleContent, BubbleVariant},
    button::{Button, ButtonSize, ButtonVariant},
    code_block::{
        CodeBlock, CodeBlockActions, CodeBlockContent, CodeBlockCopyButton, CodeBlockFooter,
        CodeBlockHeader, CodeBlockLine, CodeBlockSize, CodeBlockTitle, CodeBlockVariant,
        CodeLineVariant, strip_ansi,
    },
};
use nsh_base::{text_view::text_view, theme};
use nsh_core::{AccessibilityState, Checked, element::LayoutDirection, prelude::*};

use fixtures::*;

pub fn demo() -> impl IntoElement {
    interactive("app/api/chat/route.ts", ROUTE, "ts", true, false)
}

pub fn basic() -> impl IntoElement {
    frame(CodeBlock::new(CodeBlockContent::new(BASIC)))
}

pub fn header() -> impl IntoElement {
    frame(
        CodeBlock::new(CodeBlockContent::new(HEADER))
            .header(titled("Terminal", "pnpm add ai @ai-sdk/react zod")),
    )
}

pub fn actions() -> impl IntoElement {
    interactive("Terminal", ACTIONS, "", false, true)
}

pub fn floating() -> impl IntoElement {
    frame(CodeBlock::new(CodeBlockContent::new(FLOATING)).actions(copy(FLOATING)))
}

pub fn line_numbers() -> impl IntoElement {
    frame(CodeBlock::new(
        CodeBlockContent::new(LINE_NUMBERS).line_numbers(1),
    ))
}

pub fn start_line() -> impl IntoElement {
    frame(
        CodeBlock::new(
            CodeBlockContent::new(START)
                .language("tsx")
                .line_numbers(995),
        )
        .header(CodeBlockHeader::new(CodeBlockTitle::new(
            "app/api/projects/route.ts",
        ))),
    )
}

pub fn wrap() -> impl IntoElement {
    frame(
        view()
            .w_full()
            .gap_4()
            .child(CodeBlock::new(CodeBlockContent::new(WRAP)))
            .child(CodeBlock::new(CodeBlockContent::new(WRAP).wrap(true))),
    )
}

pub fn highlighted() -> impl IntoElement {
    let lines = HIGHLIGHTED.lines().enumerate().map(|(i, line)| {
        CodeBlockLine::new(line).variant(if matches!(i + 1, 2 | 3) {
            CodeLineVariant::Highlighted
        } else {
            CodeLineVariant::Default
        })
    });
    frame(CodeBlock::new(
        CodeBlockContent::lines(lines).line_numbers(1),
    ))
}

pub fn diff() -> impl IntoElement {
    frame(
        CodeBlock::new(CodeBlockContent::new(DIFF).diff())
            .header(titled("components/greeting.tsx", DIFF)),
    )
}

pub fn height() -> impl IntoElement {
    let logs = (0..40)
        .map(|i| {
            format!(
                "[{:02}] GET /api/chat 200 in {}ms",
                i + 1,
                12 + (i * 7) % 40
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    frame(
        CodeBlock::new(CodeBlockContent::new(logs.clone()).max_height(192.0))
            .header(titled("Logs", logs)),
    )
}

pub fn show_more() -> impl IntoElement {
    frame(expanding(false))
}

pub fn show_more_scroll() -> impl IntoElement {
    frame(expanding(true))
}

fn expanding(scrolls: bool) -> CodeBlock {
    let expanded = signal(false);
    let footer = Button::new(move || {
        SharedString::from(if expanded.get() {
            "Show less"
        } else {
            "Show more"
        })
    })
    .size(ButtonSize::Small)
    .variant(ButtonVariant::Ghost)
    .on_press(move || expanded.update(|open| *open = !*open))
    .accessibility_state(move || AccessibilityState {
        expanded: Some(expanded.get()),
        ..Default::default()
    });
    let content = CodeBlockContent::new(SHOW_MORE)
        .language("tsx")
        .line_numbers(1)
        .limit(move || (!expanded.get()).then_some(8));
    CodeBlock::new(if scrolls {
        content.max_height(240.0)
    } else {
        content
    })
    .header(titled("components/chat.tsx", SHOW_MORE))
    .footer(CodeBlockFooter::new(footer))
}

pub fn size() -> impl IntoElement {
    frame(
        view()
            .w_full()
            .gap_4()
            .child(
                CodeBlock::new(CodeBlockContent::new(SIZE).wrap(true))
                    .header(titled("Terminal", SIZE)),
            )
            .child(
                CodeBlock::new(CodeBlockContent::new(SIZE).wrap(true))
                    .header(titled("Terminal", SIZE))
                    .size(CodeBlockSize::Small),
            ),
    )
}

pub fn destructive() -> impl IntoElement {
    frame(
        CodeBlock::new(CodeBlockContent::new(ERROR).wrap(true))
            .header(titled("Error", ERROR))
            .variant(CodeBlockVariant::Destructive),
    )
}

pub fn trace() -> impl IntoElement {
    let mut demo = view().gap_2().w_full();
    for (label, source, variant) in [
        ("Ran node --version", TRACE, CodeBlockVariant::Default),
        (
            "Failed to run pnpm dev",
            TRACE_ERROR,
            CodeBlockVariant::Destructive,
        ),
    ] {
        let open = signal(true);
        demo = demo
            .child(
                Button::new(label)
                    .variant(ButtonVariant::Ghost)
                    .on_press(move || open.update(|v| *v = !*v))
                    .accessibility_state(move || AccessibilityState {
                        expanded: Some(open.get()),
                        ..Default::default()
                    }),
            )
            .child(when(open, move || {
                CodeBlock::new(CodeBlockContent::new(source).wrap(true).max_height(256.0))
                    .size(CodeBlockSize::Small)
                    .variant(variant)
            }));
    }
    frame(demo)
}

pub fn tabs() -> impl IntoElement {
    let selected = signal(0);
    let files = [
        ("app/api/chat/route.ts", "ts", TAB_0),
        ("components/chat.tsx", "tsx", TAB_1),
        (".env.local", "bash", TAB_2),
    ];
    let mut tablist = view()
        .role(AccessibilityRole::Tablist)
        .label("Files")
        .flex_row()
        .flex_wrap()
        .gap_1();
    for (i, (title, _, _)) in files.into_iter().enumerate() {
        tablist = tablist.child(
            Button::new(title)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Small)
                .on_press(move || selected.set(i))
                .role(AccessibilityRole::Tab)
                .accessibility_state(move || AccessibilityState {
                    selected: selected.get() == i,
                    ..Default::default()
                })
                .style_when(move || selected.get() == i, |style| style.bg(theme().muted)),
        );
    }
    frame(
        view()
            .w_full()
            .gap_2()
            .child(tablist)
            .child(dynamic(move || {
                let (title, lang, source) = files[selected.get()];
                view().role(AccessibilityRole::Region).label(title).child(
                    CodeBlock::new(CodeBlockContent::new(source).language(lang).line_numbers(1))
                        .header(titled(title, source)),
                )
            })),
    )
}

pub fn chat() -> impl IntoElement {
    frame(
        Bubble::new(BubbleContent::new(
            view()
                .gap_2()
                .child(text("Create a route handler that streams the response.").text_sm())
                .child(
                    CodeBlock::new(CodeBlockContent::new(CHAT).language("ts"))
                        .header(titled("app/api/chat/route.ts", CHAT)),
                ),
        ))
        .variant(BubbleVariant::Muted),
    )
}

pub fn typeset() -> impl IntoElement {
    frame(view().w_full().gap_3()
        .child(text("Stream a response").text_lg().font_weight(FontWeight::SEMIBOLD))
        .child(text("Create a route handler that calls streamText and returns the stream to the client.").text_sm())
        .child(CodeBlock::new(CodeBlockContent::new(TYPESET).language("ts").line_numbers(1))
            .header(titled("app/api/chat/route.ts", TYPESET)))
        .child(text("The useChat hook reads the stream on the client and renders the answer while it is written.").text_sm()))
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1);
    let mut controls = view().flex_row().flex_wrap().gap_1();
    for (i, label) in ["English", "العربية", "עברית"].into_iter().enumerate() {
        controls = controls.child(
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || language.set(i))
                .accessibility_state(move || AccessibilityState {
                    selected: language.get() == i,
                    ..Default::default()
                }),
        );
    }
    frame(
        view()
            .w_full()
            .gap_3()
            .child(controls)
            .child(dynamic(move || {
                let (description, title, direction) = match language.get() {
                    1 => (
                        "ثبّت الحزم، ثم شغّل خادم التطوير.",
                        "الطرفية",
                        LayoutDirection::Rtl,
                    ),
                    2 => (
                        "התקן את החבילות, ואז הפעל את שרת הפיתוח.",
                        "מסוף",
                        LayoutDirection::Rtl,
                    ),
                    _ => (
                        "Install the packages, then start the dev server.",
                        "Terminal",
                        LayoutDirection::Ltr,
                    ),
                };
                view()
                    .direction(direction)
                    .gap_2()
                    .child(text(description).text_sm())
                    .child(
                        CodeBlock::new(CodeBlockContent::new(RTL).language("bash").line_numbers(1))
                            .direction(direction)
                            .header(titled(title, RTL)),
                    )
            })),
    )
}

pub fn highlighting() -> impl IntoElement {
    frame(
        CodeBlock::new(CodeBlockContent::new(JSON).language("json"))
            .header(CodeBlockHeader::new(CodeBlockTitle::new("package.json"))),
    )
}

pub fn languages() -> impl IntoElement {
    let mut samples = view().w_full().gap_3();
    for (title, lang, source) in [
        ("app/globals.css", "css", LANG_0),
        ("main.py", "python", LANG_1),
        ("notes.txt", "text", LANG_2),
    ] {
        samples = samples.child(
            CodeBlock::new(CodeBlockContent::new(source).language(lang))
                .header(CodeBlockHeader::new(CodeBlockTitle::new(title))),
        );
    }
    frame(samples)
}

pub fn ansi() -> impl IntoElement {
    frame(
        CodeBlock::new(CodeBlockContent::new(ANSI).language("ansi").wrap(true))
            .header(titled("Terminal", strip_ansi(ANSI))),
    )
}

pub fn streaming() -> impl IntoElement {
    let files = [
        ("hooks/use-debounce.ts", "ts", STREAM_0),
        ("Terminal", "bash", STREAM_1),
        ("components/search.tsx", "tsx", STREAM_2),
        ("app/api/chat/route.ts", "ts", STREAM_3),
        (".env.local", "bash", STREAM_4),
    ];
    let total = files
        .iter()
        .map(|(_, _, source)| source.len())
        .sum::<usize>();
    let shown = replay(total);
    let mut gallery = view().gap_3().w_full();
    let mut start = 0;
    for (title, language, source) in files {
        let offset = start;
        start += source.len();
        gallery = gallery.child(when(
            move || shown.get() > offset,
            move || {
                let prefix =
                    move || SharedString::from(prefix(source, shown.get().saturating_sub(offset)));
                let content = CodeBlockContent::new(prefix).language(language);
                let content = match title {
                    "components/search.tsx" => content.diff().line_numbers(1),
                    "app/api/chat/route.ts" => content.highlighted(&[5, 6, 7, 8]).line_numbers(1),
                    "Terminal" | ".env.local" => content,
                    _ => content.line_numbers(1),
                };
                CodeBlock::new(content).header(
                    CodeBlockHeader::new(CodeBlockTitle::new(title)).actions(
                        CodeBlockActions::new().child(
                            CodeBlockCopyButton::new(prefix)
                                .disabled(move || shown.get() < offset + source.len()),
                        ),
                    ),
                )
            },
        ));
    }
    frame(gallery.child(replay_button(shown, total)))
}

pub fn markdown() -> impl IntoElement {
    let total = RESPONSE.len();
    let shown = replay(total);
    let response = dynamic(move || {
        let value = prefix(RESPONSE, shown.get());
        let Some((before, fence)) = value.split_once("```tsx\n") else {
            return text_view().markdown(value.to_owned()).into_element();
        };
        let (code, after) = fence.split_once("```").unwrap_or((fence, ""));
        view()
            .gap_3()
            .child(text_view().markdown(before.to_owned()))
            .child(
                CodeBlock::new(
                    CodeBlockContent::new(code.trim_end_matches('\n').to_owned()).language("tsx"),
                )
                .header(
                    CodeBlockHeader::new(CodeBlockTitle::new("TypeScript")).actions(
                        CodeBlockActions::new().child(
                            CodeBlockCopyButton::new(code.to_owned())
                                .disabled(!fence.contains("```")),
                        ),
                    ),
                ),
            )
            .child(text_view().markdown(after.to_owned()))
            .into_element()
    });
    frame(
        view()
            .w_full()
            .gap_3()
            .child(
                text("How do I debounce a search input in React?")
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM),
            )
            .child(response)
            .child(replay_button(shown, total)),
    )
}

fn replay(total: usize) -> Signal<usize> {
    let shown = signal(total);
    effect(move || {
        let count = shown.get();
        if count < total {
            let task = set_timeout(Duration::from_millis(35), move || {
                shown.set((count + 12).min(total))
            });
            cleanup(move || task.cancel());
        }
    });
    shown
}

fn replay_button(shown: Signal<usize>, total: usize) -> View {
    Button::new("Replay")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Small)
        .disabled(move || shown.get() < total)
        .on_press(move || shown.set(0))
}

fn prefix(source: &'static str, length: usize) -> &'static str {
    &source[..source.floor_char_boundary(length.min(source.len()))]
}

fn interactive(
    title: &'static str,
    source: &'static str,
    language: &'static str,
    numbered: bool,
    initial_wrap: bool,
) -> impl IntoElement {
    let wrap = signal(initial_wrap);
    let toggle = Button::new("Wrap lines")
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Small)
        .on_press(move || wrap.update(|wrap| *wrap = !*wrap))
        .accessibility_state(move || AccessibilityState {
            pressed: Some(if wrap.get() {
                Checked::True
            } else {
                Checked::False
            }),
            ..Default::default()
        });
    let content = CodeBlockContent::new(source).language(language).wrap(wrap);
    frame(
        CodeBlock::new(if numbered {
            content.line_numbers(1)
        } else {
            content
        })
        .header(
            CodeBlockHeader::new(CodeBlockTitle::new(title)).actions(copy(source).child(toggle)),
        ),
    )
}

fn titled(title: &'static str, value: impl IntoReactive<SharedString>) -> CodeBlockHeader {
    CodeBlockHeader::new(CodeBlockTitle::new(title)).actions(copy(value))
}

fn copy(value: impl IntoReactive<SharedString>) -> CodeBlockActions {
    CodeBlockActions::new().child(CodeBlockCopyButton::new(value))
}

fn frame(content: impl IntoElement) -> View {
    view().w_full().max_w(px(448.0)).child(content)
}
