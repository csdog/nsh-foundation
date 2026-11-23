use components::{
    button::{Button, ButtonSize, ButtonVariant},
    command::{Command, CommandDialog, CommandEntry, CommandItem},
};
use nsh_base::theme;
use nsh_core::{
    AccessibilityState,
    element::{LayoutDirection, apple_keys},
    prelude::*,
};

pub fn demo() -> impl IntoElement {
    let selected = signal(SharedString::default());
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_2()
        .child(palette(
            suggestions("Suggestions", true)
                .into_iter()
                .chain(settings("Settings"))
                .collect(),
            selected,
        ))
        .child(feedback(selected))
}

pub fn dialog() -> impl IntoElement {
    let open = signal(false);
    let selected = signal(SharedString::default());
    let keys = if apple_keys() { "⌘J" } else { "Ctrl+J" };
    view()
        .items_center()
        .gap_3()
        .w_full()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    text("Press")
                        .text_sm()
                        .text_color(move || theme().muted_foreground),
                )
                .child(
                    text(keys)
                        .font_family("monospace")
                        .text_xs()
                        .p_1()
                        .rounded(px(6.0))
                        .bg(move || theme().muted),
                ),
        )
        .child(
            Button::new("Open Command Palette")
                .variant(ButtonVariant::Outline)
                .on_press(move || open.set(true)),
        )
        .child(feedback(selected))
        .child(
            CommandDialog::new(open, move || {
                palette(
                    suggestions("Suggestions", false)
                        .into_iter()
                        .chain(settings("Settings"))
                        .collect(),
                    selected,
                )
            })
            .shortcut("mod+j"),
        )
}

pub fn basic() -> impl IntoElement {
    dialog_button(|| suggestions("Suggestions", false))
}

pub fn shortcuts() -> impl IntoElement {
    dialog_button(|| settings("Settings"))
}

pub fn groups() -> impl IntoElement {
    dialog_button(|| {
        suggestions("Suggestions", false)
            .into_iter()
            .chain(settings("Settings"))
            .collect()
    })
}

pub fn scrollable() -> impl IntoElement {
    dialog_button(|| {
        [
            (
                "Navigation",
                &[
                    ("home", "Home", "mod+h"),
                    ("inbox", "Inbox", "mod+i"),
                    ("documents", "Documents", "mod+d"),
                    ("folders", "Folders", "mod+f"),
                ][..],
            ),
            (
                "Actions",
                &[
                    ("new-file", "New File", "mod+n"),
                    ("new-folder", "New Folder", "mod+shift+n"),
                    ("copy", "Copy", "mod+c"),
                    ("cut", "Cut", "mod+x"),
                    ("paste", "Paste", "mod+v"),
                    ("delete", "Delete", "backspace"),
                ][..],
            ),
            (
                "View",
                &[
                    ("grid-view", "Grid View", ""),
                    ("list-view", "List View", ""),
                    ("zoom-in", "Zoom In", "mod+shift+="),
                    ("zoom-out", "Zoom Out", "mod+-"),
                ][..],
            ),
            (
                "Account",
                &[
                    ("profile", "Profile", "mod+p"),
                    ("billing", "Billing", "mod+b"),
                    ("settings", "Settings", "mod+s"),
                    ("notifications", "Notifications", ""),
                    ("help", "Help & Support", ""),
                ][..],
            ),
            (
                "Tools",
                &[
                    ("calculator", "Calculator", ""),
                    ("calendar", "Calendar", ""),
                    ("image-editor", "Image Editor", ""),
                    ("code-editor", "Code Editor", ""),
                ][..],
            ),
        ]
        .into_iter()
        .flat_map(|(group, entries)| {
            entries.iter().map(move |&(id, title, keys)| {
                let item = CommandItem::new(id, title).group(group);
                if keys.is_empty() {
                    item
                } else {
                    item.shortcut(keys)
                }
            })
        })
        .collect()
    })
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1);
    let selected = signal(SharedString::default());
    let controls = ["English", "العربية", "עברית"]
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || {
                    selected.set(SharedString::default());
                    language.set(index);
                })
                .accessibility_state(move || AccessibilityState {
                    selected: language.get() == index,
                    ..Default::default()
                })
                .style_when(
                    move || language.get() == index,
                    |style| style.bg(theme().muted),
                )
        })
        .collect::<Vec<_>>();
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_3()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .child(controls),
        )
        .child(dynamic(move || {
            let locale = language.get();
            let (search, empty, suggestion_group, settings_group, labels) = match locale {
                1 => (
                    "اكتب أمرًا أو ابحث...",
                    "لم يتم العثور على نتائج.",
                    "اقتراحات",
                    "الإعدادات",
                    [
                        "التقويم",
                        "البحث عن الرموز التعبيرية",
                        "الآلة الحاسبة",
                        "الملف الشخصي",
                        "الفوترة",
                        "الإعدادات",
                    ],
                ),
                2 => (
                    "הקלד פקודה או חפש...",
                    "לא נמצאו תוצאות.",
                    "הצעות",
                    "הגדרות",
                    [
                        "לוח שנה",
                        "חפש אמוג'י",
                        "מחשבון",
                        "פרופיל",
                        "חיוב",
                        "הגדרות",
                    ],
                ),
                _ => (
                    "Type a command or search...",
                    "No results found.",
                    "Suggestions",
                    "Settings",
                    [
                        "Calendar",
                        "Search Emoji",
                        "Calculator",
                        "Profile",
                        "Billing",
                        "Settings",
                    ],
                ),
            };
            let direction = if locale == 0 {
                LayoutDirection::Ltr
            } else {
                LayoutDirection::Rtl
            };
            let mut items = suggestions(suggestion_group, true);
            items.extend(settings(settings_group));
            for (item, label) in items.iter_mut().zip(labels) {
                item.keywords.push(item.label.clone());
                item.label = label.into();
            }
            palette(items, selected)
                .placeholder(search)
                .empty(empty)
                .direction(direction)
        }))
        .child(feedback(selected))
}

fn palette(items: Vec<CommandItem>, selected: Signal<SharedString>) -> Command {
    Command::new("Command search", items)
        .on_select(move |id| selected.set(id.to_owned().into()))
        .render_item(render_row)
}

fn dialog_button(items: impl Fn() -> Vec<CommandItem> + 'static) -> View {
    let open = signal(false);
    let selected = signal(SharedString::default());
    view()
        .w_full()
        .items_center()
        .gap_2()
        .child(
            Button::new("Open Menu")
                .variant(ButtonVariant::Outline)
                .on_press(move || open.set(true)),
        )
        .child(feedback(selected))
        .child(CommandDialog::new(open, move || palette(items(), selected)))
}

fn feedback(selected: Signal<SharedString>) -> impl IntoElement {
    when(
        move || !selected.get().is_empty(),
        move || {
            text(move || format!("Selected: {}", selected.get()))
                .text_sm()
                .role(AccessibilityRole::Summary)
        },
    )
}

fn suggestions(group: &'static str, disabled: bool) -> Vec<CommandItem> {
    vec![
        CommandItem::new("calendar", "Calendar")
            .group(group)
            .keywords(["date", "events"]),
        CommandItem::new("emoji", "Search Emoji")
            .group(group)
            .keywords(["smile", "faces"]),
        CommandItem::new("calculator", "Calculator")
            .group(group)
            .disabled(disabled),
    ]
}

fn settings(group: &'static str) -> Vec<CommandItem> {
    vec![
        CommandItem::new("profile", "Profile")
            .group(group)
            .shortcut("mod+p")
            .keywords(["account"]),
        CommandItem::new("billing", "Billing")
            .group(group)
            .shortcut("mod+b"),
        CommandItem::new("settings", "Settings")
            .group(group)
            .shortcut("mod+s"),
    ]
}

fn render_row(item: &CommandEntry) -> View {
    let icon = match item.id.as_str() {
        "calendar" => "▦",
        "emoji" => "☺",
        "calculator" => "▤",
        "profile" => "♙",
        "billing" => "▱",
        "settings" => "⚙",
        "home" => "⌂",
        "folders" | "new-folder" => "▰",
        _ => "▧",
    };
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .min_w(px(0.0))
        .child(
            text(icon)
                .w(px(16.0))
                .text_center()
                .text_color(move || theme().muted_foreground),
        )
        .child(text(item.label.clone()).text_sm().flex_1().min_w(px(0.0)))
}
