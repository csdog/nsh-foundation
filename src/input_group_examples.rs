use nsh_base::{popover, theme};
use nsh_core::{
    element::{Input, Selection, apple_keys, set_clipboard},
    prelude::*,
};

use crate::{
    button::{Button, ButtonSize, ButtonVariant, spinner as indicator},
    direction::{Direction, DirectionProvider},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    field::{Field, FieldDescription, FieldLabel},
    input_group::{
        InputGroup, InputGroupAddon as Addon,
        InputGroupAlign::{BlockEnd, BlockStart, InlineEnd, InlineStart},
        input_group_button as action, input_group_input, input_group_text as copy_text,
        input_group_textarea,
    },
    ui::muted,
};

fn stage(content: impl IntoElement) -> View {
    view().w_full().max_w(px(384.0)).gap_4().child(content)
}

fn entry(label: &'static str, placeholder: &'static str) -> Input {
    input_group_input().label(label).placeholder(placeholder)
}

fn area(label: &'static str, placeholder: &'static str) -> Input {
    input_group_textarea().label(label).placeholder(placeholder)
}

fn bound(control: Input, value: Signal<String>) -> Input {
    control
        .value(value)
        .on_change(move |text: &str| value.set(text.to_owned()))
}

fn icon(symbol: &'static str) -> Text {
    copy_text(symbol).accessibility_elements_hidden(true)
}

fn icon_button(label: &'static str, symbol: &'static str) -> Button {
    action(label)
        .size(ButtonSize::IconExtraSmall)
        .icon_start(icon(symbol))
}

fn labeled(label: &'static str, group: InputGroup, detail: &'static str) -> Field {
    let control = element_ref();
    Field::new()
        .child(FieldLabel::new(label).for_control(control))
        .child(group.control_ref(control))
        .child(FieldDescription::new(detail))
}

fn search_group(
    label: &'static str,
    placeholder: &'static str,
    results: &'static str,
) -> InputGroup {
    InputGroup::new(entry(label, placeholder))
        .label("Search group")
        .addon(Addon::new(InlineStart).child(icon("⌕")))
        .addon(Addon::new(InlineEnd).child(copy_text(results)))
}

pub fn demo() -> impl IntoElement {
    stage(search_group("Search", "Search...", "12 results"))
}

pub fn inline_start() -> impl IntoElement {
    stage(labeled(
        "Input",
        InputGroup::new(entry("Input", "Search..."))
            .addon(Addon::new(InlineStart).child(icon("⌕"))),
        "Icon positioned at the start.",
    ))
}

pub fn inline_end() -> impl IntoElement {
    stage(labeled(
        "Input",
        InputGroup::new(entry("Input", "Enter password").masked(true))
            .addon(Addon::new(InlineEnd).child(icon("◉"))),
        "Icon positioned at the end.",
    ))
}

pub fn block_start() -> impl IntoElement {
    let code = signal(String::new());
    stage(
        view()
            .gap_4()
            .child(labeled(
                "Input",
                InputGroup::new(entry("Input", "Enter your name"))
                    .addon(Addon::new(BlockStart).child(copy_text("Full Name"))),
                "Label positioned above the input.",
            ))
            .child(labeled(
                "Textarea",
                InputGroup::new(bound(
                    area("Textarea", "console.log('Hello, world!');"),
                    code,
                ))
                .addon(
                    Addon::new(BlockStart)
                        .separator()
                        .child(copy_text("script.js").font_family("monospace").flex_1())
                        .action(
                            icon_button("Copy code", "▣")
                                .on_press(move || set_clipboard(code.get())),
                        ),
                ),
                "Header positioned above the textarea.",
            )),
    )
}

fn comment(
    label: &'static str,
    placeholder: &'static str,
    post: &'static str,
    sent: &'static str,
    arabic: bool,
    value: Signal<String>,
    status: Signal<&'static str>,
) -> InputGroup {
    let count = move || {
        let count = value.with(|value| value.chars().count());
        if arabic {
            let numerals = |number: usize| {
                number
                    .to_string()
                    .bytes()
                    .map(|byte| char::from_u32(0x0660 + u32::from(byte - b'0')).unwrap())
                    .collect::<String>()
            };
            format!("{}/{}", numerals(count), numerals(280))
        } else {
            format!("{count}/280")
        }
    };
    InputGroup::new(bound(area(label, placeholder).max_length(280), value)).addon(
        Addon::new(BlockEnd)
            .child(copy_text(count).flex_1())
            .action(
                action(post)
                    .variant(ButtonVariant::Default)
                    .size(ButtonSize::Small)
                    .on_press(move || {
                        if !value.with(|value| value.trim().is_empty()) {
                            value.set(String::new());
                            status.set(sent);
                        }
                    }),
            ),
    )
}

pub fn block_end() -> impl IntoElement {
    let value = signal(String::new());
    let status = signal("");
    stage(
        view()
            .gap_4()
            .child(labeled(
                "Input",
                InputGroup::new(
                    entry("Input", "Enter amount").keyboard_type(KeyboardType::Decimal),
                )
                .addon(Addon::new(BlockEnd).child(copy_text("USD"))),
                "Footer positioned below the input.",
            ))
            .child(labeled(
                "Textarea",
                comment(
                    "Textarea",
                    "Write a comment...",
                    "Post",
                    "Comment posted in preview.",
                    false,
                    value,
                    status,
                ),
                "Footer positioned below the textarea.",
            ))
            .child(muted(move || status.get())),
    )
}

pub fn icons() -> impl IntoElement {
    stage(
        view()
            .gap_3()
            .child(
                InputGroup::new(entry("Search", "Search..."))
                    .addon(Addon::new(InlineStart).child(icon("⌕"))),
            )
            .child(
                InputGroup::new(
                    entry("Email", "Enter your email").keyboard_type(KeyboardType::Email),
                )
                .addon(Addon::new(InlineStart).child(icon("✉"))),
            )
            .child(
                InputGroup::new(
                    entry("Card with icons", "Card number").keyboard_type(KeyboardType::Number),
                )
                .addon(Addon::new(InlineStart).child(icon("▤")))
                .addon(Addon::new(InlineEnd).child(icon("ⓘ"))),
            )
            .child(
                InputGroup::new(
                    entry("Card with end icon", "Card number").keyboard_type(KeyboardType::Number),
                )
                .addon(Addon::new(InlineEnd).child(icon("▤"))),
            ),
    )
}

pub fn text_addons() -> impl IntoElement {
    let message = signal(String::new());
    stage(
        view()
            .gap_3()
            .child(
                InputGroup::new(entry("Amount", "0.00").keyboard_type(KeyboardType::Decimal))
                    .addon(Addon::new(InlineStart).child(copy_text("$")))
                    .addon(Addon::new(InlineEnd).child(copy_text("USD"))),
            )
            .child(
                InputGroup::new(entry("URL", "example.com").keyboard_type(KeyboardType::Url))
                    .addon(Addon::new(InlineStart).child(copy_text("https://")))
                    .addon(Addon::new(InlineEnd).child(copy_text(".com"))),
            )
            .child(
                InputGroup::new(entry("Username", "Enter your username"))
                    .addon(Addon::new(InlineEnd).child(copy_text("@company.com"))),
            )
            .child(
                InputGroup::new(bound(
                    area("Message", "Enter your message").max_length(120),
                    message,
                ))
                .addon(Addon::new(BlockEnd).child(copy_text(move || {
                    format!(
                        "{} characters left",
                        120usize.saturating_sub(message.with(|value| value.chars().count()))
                    )
                }))),
            ),
    )
}

pub fn buttons() -> impl IntoElement {
    let copied = signal(false);
    let favorite = signal(false);
    let query = signal(String::new());
    let status = signal(String::new());
    let submit = move || {
        let value = query.get();
        if !value.trim().is_empty() {
            status.set(format!("Search preview · {}", value.trim()));
        }
    };
    let url = "https://x.com/shadcn";
    let info = popover()
        .align(Align::Start)
        .offset(px(4.0))
        .margin(px(8.0))
        .popup_class("w-56 p-3 bg-popover text-popover-foreground rounded-xl border shadow-md")
        .trigger(
            icon_button("Connection information", "ⓘ")
                .variant(ButtonVariant::Secondary)
                .surface(),
        )
        .child(
            view()
                .gap_1()
                .child(
                    text("Connection preview")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD),
                )
                .child(muted("Enter a sample address to explore the input group.")),
        );
    stage(
        view()
            .gap_3()
            .child(
                InputGroup::new(entry("Profile URL", url).readonly(true)).addon(
                    Addon::new(InlineEnd).action(icon_button("Copy profile URL", "▣").on_press(
                        move || {
                            set_clipboard(url);
                            copied.set(true);
                        },
                    )),
                ),
            )
            .child(when(copied, || muted("Profile URL copied.")))
            .child(
                InputGroup::new(entry("Website", "example.com").keyboard_type(KeyboardType::Url))
                    .addon(
                        Addon::new(InlineStart)
                            .action(info)
                            .child(copy_text("https://")),
                    )
                    .addon(
                        Addon::new(InlineEnd).action(
                            action(move || {
                                if favorite.get() {
                                    "Remove favorite"
                                } else {
                                    "Add favorite"
                                }
                            })
                            .size(ButtonSize::IconExtraSmall)
                            .icon_start(copy_text(move || if favorite.get() { "★" } else { "☆" }))
                            .on_press(move || favorite.update(|value| *value = !*value)),
                        ),
                    ),
            )
            .child(
                InputGroup::new(
                    bound(entry("Search", "Type to search..."), query)
                        .on_submit(move |_: &str| submit()),
                )
                .addon(
                    Addon::new(InlineEnd).action(
                        action("Search")
                            .variant(ButtonVariant::Secondary)
                            .on_press(submit),
                    ),
                ),
            )
            .child(muted(move || status.get())),
    )
}

pub fn kbd() -> impl IntoElement {
    let target = element_ref();
    on_key("mod+k", move |_| target.focus());
    let shortcut = if apple_keys() { "⌘K" } else { "Ctrl K" };
    stage(
        InputGroup::new(entry("Shortcut search", "Search..."))
            .control_ref(target)
            .addon(Addon::new(InlineStart).child(icon("⌕")))
            .addon(
                Addon::new(InlineEnd).child(
                    copy_text(shortcut)
                        .text_xs()
                        .px_1()
                        .rounded_full()
                        .bg(move || theme().muted_foreground.opacity(0.1)),
                ),
            ),
    )
}

pub fn dropdown() -> impl IntoElement {
    let status = signal(String::new());
    let file = signal(String::new());
    let scope = signal("Search In...");
    let mut menu = DropdownMenuContent::new().align(Align::End);
    for name in ["Settings", "Copy path", "Open location"] {
        menu = menu.child(DropdownMenuItem::new(name).on_select(move || {
            if name == "Copy path" && !file.with(String::is_empty) {
                set_clipboard(file.get());
            }
            status.set(format!("Selected · {name}"));
        }));
    }
    let mut scopes = DropdownMenuContent::new().align(Align::End);
    for name in ["Documentation", "Blog Posts", "Changelog"] {
        scopes = scopes.child(DropdownMenuItem::new(name).on_select(move || scope.set(name)));
    }
    stage(
        view()
            .gap_3()
            .child(
                InputGroup::new(bound(entry("File name", "Enter file name"), file)).addon(
                    Addon::new(InlineEnd).action(DropdownMenu::new(
                        DropdownMenuTrigger::render(icon_button("More", "⋯").surface()),
                        menu,
                    )),
                ),
            )
            .child(
                InputGroup::new(entry("Search query", "Enter search query")).addon(
                    Addon::new(InlineEnd).action(DropdownMenu::new(
                        DropdownMenuTrigger::render(
                            action(move || scope.get()).icon_end(icon("⌄")).surface(),
                        ),
                        scopes,
                    )),
                ),
            )
            .child(muted(move || status.get())),
    )
}

pub fn spinners() -> impl IntoElement {
    let loading = || indicator(ButtonVariant::Ghost);
    stage(
        view()
            .gap_3()
            .child(
                InputGroup::new(entry("Searching", "Searching..."))
                    .addon(Addon::new(InlineEnd).child(loading())),
            )
            .child(
                InputGroup::new(entry("Processing", "Processing..."))
                    .addon(Addon::new(InlineStart).child(loading())),
            )
            .child(
                InputGroup::new(entry("Saving changes", "Saving changes...")).addon(
                    Addon::new(InlineEnd)
                        .child(copy_text("Saving..."))
                        .child(loading()),
                ),
            )
            .child(
                InputGroup::new(entry("Refreshing data", "Refreshing data..."))
                    .addon(Addon::new(InlineStart).child(loading()))
                    .addon(Addon::new(InlineEnd).child(copy_text("Please wait..."))),
            ),
    )
}

fn cursor_location(value: &str, cursor: u32) -> (u32, u32) {
    let (mut line, mut column, mut offset) = (1, 1, 0);
    for character in value.chars() {
        offset += character.len_utf16() as u32;
        if offset > cursor {
            break;
        }
        if character == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

pub fn textarea() -> impl IntoElement {
    let code = signal(String::new());
    let cursor = signal(0);
    let status = signal("");
    let position = move || {
        let (line, column) = code.with(|value| cursor_location(value, cursor.get()));
        format!("Line {line}, Column {column}")
    };
    stage(
        view()
            .gap_2()
            .child(
                InputGroup::new(
                    bound(area("Script editor", "console.log('Hello, world!');"), code)
                        .h(px(200.0))
                        .font_family("monospace")
                        .on_selection_change(move |value: &Selection| cursor.set(value.end)),
                )
                .addon(
                    Addon::new(BlockEnd)
                        .separator()
                        .child(copy_text(position).text_xs().flex_1())
                        .action(
                            action("Run")
                                .size(ButtonSize::Small)
                                .variant(ButtonVariant::Default)
                                .on_press(move || status.set("Run selected in preview.")),
                        ),
                )
                .addon(
                    Addon::new(BlockStart)
                        .separator()
                        .child(
                            copy_text("JS · script.js")
                                .font_family("monospace")
                                .flex_1(),
                        )
                        .action(icon_button("Reset script", "⟳").on_press(move || {
                            code.set(String::new());
                            cursor.set(0);
                            status.set("");
                        }))
                        .action(icon_button("Copy script", "▣").on_press(move || {
                            set_clipboard(code.get());
                            status.set("Script copied.");
                        })),
                ),
            )
            .child(muted(move || status.get())),
    )
}

pub fn custom() -> impl IntoElement {
    let value = signal(String::new());
    let status = signal("");
    stage(
        view()
            .gap_2()
            .child(
                InputGroup::new(
                    bound(area("Autoresize textarea", "Autoresize textarea..."), value)
                        .h(auto())
                        .min_h(px(64.0))
                        .max_h(px(200.0)),
                )
                .addon(
                    Addon::new(BlockEnd).child(view().flex_1()).action(
                        action("Submit")
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Default)
                            .on_press(move || {
                                if !value.with(|text| text.trim().is_empty()) {
                                    status.set("Sample submitted.");
                                }
                            }),
                    ),
                ),
            )
            .child(muted(move || status.get())),
    )
}

struct Locale {
    language: &'static str,
    search: &'static str,
    results: &'static str,
    searching: &'static str,
    saving: &'static str,
    saving_changes: &'static str,
    textarea: &'static str,
    placeholder: &'static str,
    post: &'static str,
    detail: &'static str,
    sent: &'static str,
}

const LOCALES: [Locale; 3] = [
    Locale {
        language: "English",
        search: "Search...",
        results: "12 results",
        searching: "Searching...",
        saving: "Saving...",
        saving_changes: "Saving changes...",
        textarea: "Textarea",
        placeholder: "Write a comment...",
        post: "Post",
        detail: "Footer positioned below the textarea.",
        sent: "Comment posted in preview.",
    },
    Locale {
        language: "Arabic",
        search: "بحث...",
        results: "١٢ نتيجة",
        searching: "جاري البحث...",
        saving: "جاري الحفظ...",
        saving_changes: "جاري حفظ التغييرات...",
        textarea: "منطقة النص",
        placeholder: "اكتب تعليقًا...",
        post: "نشر",
        detail: "تذييل موضع أسفل منطقة النص.",
        sent: "تم النشر في المعاينة.",
    },
    Locale {
        language: "Hebrew",
        search: "חפש...",
        results: "12 תוצאות",
        searching: "מחפש...",
        saving: "שומר...",
        saving_changes: "שומר שינויים...",
        textarea: "אזור טקסט",
        placeholder: "כתוב תגובה...",
        post: "פרסם",
        detail: "כותרת תחתונה ממוקמת מתחת לאזור הטקסט.",
        sent: "התגובה פורסמה בתצוגה המקדימה.",
    },
];

pub fn rtl() -> impl IntoElement {
    let language = signal(1_usize);
    let value = signal(String::new());
    let status = signal("");
    let direction = move || {
        if language.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    stage(
        view()
            .gap_4()
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .justify_center()
                    .children(LOCALES.iter().enumerate().map(|(i, locale)| {
                        action(locale.language)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || {
                                language.set(i);
                                status.set("");
                            })
                    })),
            )
            .child(DirectionProvider::new(direction, move || {
                dynamic(move || {
                    let index = language.get();
                    let local = &LOCALES[index];
                    view()
                        .gap_3()
                        .child(search_group(local.search, local.search, local.results))
                        .child(
                            InputGroup::new(entry(local.searching, local.searching)).addon(
                                Addon::new(InlineEnd).child(indicator(ButtonVariant::Ghost)),
                            ),
                        )
                        .child(
                            InputGroup::new(entry(local.saving_changes, local.saving_changes))
                                .addon(
                                    Addon::new(InlineEnd)
                                        .child(copy_text(local.saving))
                                        .child(indicator(ButtonVariant::Ghost)),
                                ),
                        )
                        .child(labeled(
                            local.textarea,
                            comment(
                                local.textarea,
                                local.placeholder,
                                local.post,
                                local.sent,
                                index == 1,
                                value,
                                status,
                            ),
                            local.detail,
                        ))
                        .child(muted(move || status.get()))
                })
            })),
    )
}

#[cfg(test)]
mod tests {
    use super::cursor_location;

    #[test]
    fn code_editor_coordinates_use_utf16_caret_positions() {
        let value = "a😀\nxyz";
        assert_eq!(cursor_location(value, 1), (1, 2));
        assert_eq!(cursor_location(value, 3), (1, 3));
        assert_eq!(cursor_location(value, 4), (2, 1));
        assert_eq!(cursor_location(value, 7), (2, 4));
    }
}
