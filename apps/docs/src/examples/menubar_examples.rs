use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    menubar::{
        Menubar, MenubarContent as Content, MenubarItem as Item, MenubarMenu as Menu,
        MenubarSub as Sub, MenubarTrigger as Trigger, menubar_separator as separator,
    },
    ui::muted,
};

fn menu(label: &'static str, items: Content) -> Menu {
    Menu::new(Trigger::new(label), items)
}

fn command(status: Signal<&'static str>, label: &'static str) -> Item {
    Item::new(label).on_select(move || status.set(label))
}

fn checkbox(label: &'static str, state: Signal<bool>) -> Item {
    Item::new(label)
        .checkbox(move || state.get())
        .leading_check()
        .close_on_select(false)
        .on_select(move || state.update(|checked| *checked = !*checked))
}

fn radios<const COUNT: usize>(
    choices: [&'static str; COUNT],
    selected: Signal<usize>,
) -> [Item; COUNT] {
    std::array::from_fn(|index| {
        Item::new(choices[index])
            .radio(move || selected.get() == index)
            .leading_check()
            .close_on_select(false)
            .on_select(move || selected.set(index))
    })
}

fn frame(bar: Menubar, status: Signal<&'static str>) -> View {
    view()
        .w_full()
        .gap_3()
        .items_center()
        .child(bar)
        .child(when(
            move || !status.get().is_empty(),
            move || muted(move || format!("Selected in this preview: {}", status.get())),
        ))
}

fn file(locale: &Locale, status: Signal<&'static str>, full: bool) -> Menu {
    let mut items = Content::new();
    if full {
        items = items
            .child(command(status, locale.new_tab).shortcut("⌘T"))
            .child(command(status, locale.new_window).shortcut("⌘N"))
            .child(Item::new(locale.new_incognito_window).disabled(true))
            .child(separator());
    }
    menu(
        locale.file,
        items
            .child(
                Sub::new(locale.share).children(
                    [locale.email_link, locale.messages, locale.notes]
                        .map(|label| command(status, label)),
                ),
            )
            .child(separator())
            .child(command(status, locale.print).shortcut("⌘P")),
    )
}

fn edit(locale: &Locale, status: Signal<&'static str>, full: bool) -> Menu {
    let mut find = Sub::new(locale.find);
    if full {
        find = find
            .child(command(status, locale.search_the_web))
            .child(separator());
    }
    menu(
        locale.edit,
        Content::new()
            .child(command(status, locale.undo).shortcut("⌘Z"))
            .child(command(status, locale.redo).shortcut("⇧⌘Z"))
            .child(separator())
            .child(
                find.children(
                    [locale.find_item, locale.find_next, locale.find_previous]
                        .map(|label| command(status, label)),
                ),
            )
            .child(separator())
            .children([locale.cut, locale.copy, locale.paste].map(|label| command(status, label))),
    )
}

fn reload(status: Signal<&'static str>, reload: &'static str, force: &'static str) -> [Item; 2] {
    [
        command(status, reload).shortcut("⌘R").inset(),
        Item::new(force).disabled(true).shortcut("⇧⌘R").inset(),
    ]
}

fn application(
    locale: &Locale,
    status: Signal<&'static str>,
    bookmarks: Signal<bool>,
    urls: Signal<bool>,
    profile: Signal<usize>,
) -> View {
    frame(
        Menubar::new("Application menu")
            .menu(file(locale, status, true))
            .menu(edit(locale, status, true))
            .menu(menu(
                locale.view,
                Content::new()
                    .width(176)
                    .child(checkbox(locale.bookmarks_bar, bookmarks))
                    .child(checkbox(locale.full_urls, urls))
                    .child(separator())
                    .children(reload(status, locale.reload, locale.force_reload))
                    .child(separator())
                    .child(command(status, locale.toggle_fullscreen).inset())
                    .child(separator())
                    .child(command(status, locale.hide_sidebar).inset()),
            ))
            .menu(profiles(locale, status, profile)),
        status,
    )
}

fn profiles(locale: &Locale, status: Signal<&'static str>, profile: Signal<usize>) -> Menu {
    menu(
        locale.profiles,
        Content::new()
            .child(
                view()
                    .role(AccessibilityRole::Radiogroup)
                    .label(locale.profiles)
                    .children(radios([locale.andy, locale.benoit, locale.luis], profile)),
            )
            .child(separator())
            .child(command(status, locale.edit_profile).inset())
            .child(command(status, locale.add_profile).inset()),
    )
}

pub fn demo() -> View {
    application(&ENGLISH, signal(""), signal(false), signal(true), signal(1))
}

pub fn checkboxes() -> View {
    let status = signal("");
    frame(
        Menubar::new("View and format")
            .menu(menu(
                "View",
                Content::new()
                    .width(256)
                    .child(checkbox("Always Show Bookmarks Bar", signal(false)))
                    .child(checkbox("Always Show Full URLs", signal(true)))
                    .child(separator())
                    .children(reload(status, "Reload", "Force Reload")),
            ))
            .menu(menu(
                "Format",
                Content::new().children([
                    checkbox("Strikethrough", signal(true)),
                    checkbox("Code", signal(false)),
                    checkbox("Superscript", signal(false)),
                ]),
            )),
        status,
    )
}

pub fn radio() -> View {
    let status = signal("");
    frame(
        Menubar::new("Profiles and theme")
            .menu(profiles(&ENGLISH, status, signal(1)))
            .menu(menu(
                "Theme",
                Content::new().child(
                    view()
                        .role(AccessibilityRole::Radiogroup)
                        .label("Theme")
                        .children(radios(["Light", "Dark", "System"], signal(2))),
                ),
            )),
        status,
    )
}

pub fn submenu() -> View {
    let status = signal("");
    frame(
        Menubar::new("File and edit")
            .menu(file(&ENGLISH, status, false))
            .menu(edit(&ENGLISH, status, false)),
        status,
    )
}

pub fn icons() -> View {
    let status = signal("");
    frame(
        Menubar::new("File and more")
            .menu(menu(
                "File",
                Content::new()
                    .children([
                        command(status, "New File").icon("▤").shortcut("⌘N"),
                        command(status, "Open Folder").icon("▱"),
                    ])
                    .child(separator())
                    .child(command(status, "Save").icon("▣").shortcut("⌘S")),
            ))
            .menu(menu(
                "More",
                Content::new()
                    .children([
                        command(status, "Settings").icon("⚙"),
                        command(status, "Help").icon("ⓘ"),
                    ])
                    .child(separator())
                    .child(command(status, "Delete").icon("×").destructive()),
            )),
        status,
    )
}

pub fn rtl() -> impl IntoElement {
    let locale = signal(1_usize);
    let status = signal("");
    let bookmarks = signal(false);
    let urls = signal(true);
    let profile = signal(1);
    view()
        .w_full()
        .items_center()
        .gap_3()
        .child(
            view().flex_row().gap_1().children(
                ["English", "Arabic", "Hebrew"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || {
                                status.set("");
                                locale.set(index);
                            })
                            .accessibility_state(move || AccessibilityState {
                                selected: locale.get() == index,
                                ..Default::default()
                            })
                            .style_when(
                                move || locale.get() == index,
                                |style| style.bg(nsh_base::theme().muted),
                            )
                    }),
            ),
        )
        .child(dynamic(move || {
            let index = locale.get();
            let locale = [&ENGLISH, &ARABIC, &HEBREW][index];
            let direction = if index == 0 {
                Direction::Ltr
            } else {
                Direction::Rtl
            };
            DirectionProvider::new(direction, move || {
                application(locale, status, bookmarks, urls, profile)
            })
        }))
}

struct Locale {
    file: &'static str,
    new_tab: &'static str,
    new_window: &'static str,
    new_incognito_window: &'static str,
    share: &'static str,
    email_link: &'static str,
    messages: &'static str,
    notes: &'static str,
    print: &'static str,
    edit: &'static str,
    undo: &'static str,
    redo: &'static str,
    find: &'static str,
    search_the_web: &'static str,
    find_item: &'static str,
    find_next: &'static str,
    find_previous: &'static str,
    cut: &'static str,
    copy: &'static str,
    paste: &'static str,
    view: &'static str,
    bookmarks_bar: &'static str,
    full_urls: &'static str,
    reload: &'static str,
    force_reload: &'static str,
    toggle_fullscreen: &'static str,
    hide_sidebar: &'static str,
    profiles: &'static str,
    andy: &'static str,
    benoit: &'static str,
    luis: &'static str,
    edit_profile: &'static str,
    add_profile: &'static str,
}

const ENGLISH: Locale = Locale {
    file: "File",
    new_tab: "New Tab",
    new_window: "New Window",
    new_incognito_window: "New Incognito Window",
    share: "Share",
    email_link: "Email link",
    messages: "Messages",
    notes: "Notes",
    print: "Print...",
    edit: "Edit",
    undo: "Undo",
    redo: "Redo",
    find: "Find",
    search_the_web: "Search the web",
    find_item: "Find...",
    find_next: "Find Next",
    find_previous: "Find Previous",
    cut: "Cut",
    copy: "Copy",
    paste: "Paste",
    view: "View",
    bookmarks_bar: "Bookmarks Bar",
    full_urls: "Full URLs",
    reload: "Reload",
    force_reload: "Force Reload",
    toggle_fullscreen: "Toggle Fullscreen",
    hide_sidebar: "Hide Sidebar",
    profiles: "Profiles",
    andy: "Andy",
    benoit: "Benoit",
    luis: "Luis",
    edit_profile: "Edit...",
    add_profile: "Add Profile...",
};

const ARABIC: Locale = Locale {
    file: "ملف",
    new_tab: "علامة تبويب جديدة",
    new_window: "نافذة جديدة",
    new_incognito_window: "نافذة التصفح المتخفي الجديدة",
    share: "مشاركة",
    email_link: "رابط البريد الإلكتروني",
    messages: "الرسائل",
    notes: "الملاحظات",
    print: "طباعة...",
    edit: "تعديل",
    undo: "تراجع",
    redo: "إعادة",
    find: "بحث",
    search_the_web: "البحث على الويب",
    find_item: "بحث...",
    find_next: "البحث التالي",
    find_previous: "البحث السابق",
    cut: "قص",
    copy: "نسخ",
    paste: "لصق",
    view: "عرض",
    bookmarks_bar: "شريط الإشارات المرجعية",
    full_urls: "عناوين URL الكاملة",
    reload: "إعادة تحميل",
    force_reload: "إعادة تحميل قسري",
    toggle_fullscreen: "تبديل وضع ملء الشاشة",
    hide_sidebar: "إخفاء الشريط الجانبي",
    profiles: "الملفات الشخصية",
    andy: "Andy",
    benoit: "Benoit",
    luis: "Luis",
    edit_profile: "تعديل...",
    add_profile: "إضافة ملف شخصي...",
};

const HEBREW: Locale = Locale {
    file: "קובץ",
    new_tab: "כרטיסייה חדשה",
    new_window: "חלון חדש",
    new_incognito_window: "חלון גלישה בסתר חדש",
    share: "שתף",
    email_link: "קישור אימייל",
    messages: "הודעות",
    notes: "הערות",
    print: "הדפס...",
    edit: "ערוך",
    undo: "בטל",
    redo: "בצע שוב",
    find: "מצא",
    search_the_web: "חפש באינטרנט",
    find_item: "מצא...",
    find_next: "מצא הבא",
    find_previous: "מצא הקודם",
    cut: "גזור",
    copy: "העתק",
    paste: "הדבק",
    view: "תצוגה",
    bookmarks_bar: "סרגל סימניות",
    full_urls: "כתובות URL מלאות",
    reload: "רענן",
    force_reload: "רענן בכוח",
    toggle_fullscreen: "החלף מסך מלא",
    hide_sidebar: "הסתר סרגל צד",
    profiles: "פרופילים",
    andy: "Andy",
    benoit: "Benoit",
    luis: "Luis",
    edit_profile: "ערוך...",
    add_profile: "הוסף פרופיל...",
};
