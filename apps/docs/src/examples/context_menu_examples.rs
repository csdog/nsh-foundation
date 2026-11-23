use components::{
    button::{Button, ButtonSize, ButtonVariant},
    context_menu::{
        ContextMenu, ContextMenuItem, context_menu_label, context_menu_separator,
        context_menu_submenu,
    },
};
use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};

fn trigger(id: &'static str, instruction: &'static str) -> View {
    view()
        .test_id(id)
        .label(instruction)
        .w_full()
        .max_w(px(320.0))
        .aspect_ratio(16.0 / 9.0)
        .rounded(px(12.0))
        .border(px(1.0))
        .border_dashed()
        .border_color(move || theme().border)
        .items_center()
        .justify_center()
        .px_2()
        .child(text(instruction).text_sm().text_center())
}

fn area(id: &'static str) -> ContextMenu {
    ContextMenu::new(trigger(
        id,
        if is_desktop() {
            "Right click here"
        } else {
            "Long press here"
        },
    ))
}

fn feedback(menu: ContextMenu, status: Signal<&'static str>) -> View {
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_2()
        .child(menu)
        .child(when(
            move || !status.get().is_empty(),
            move || {
                text(move || format!("Selected in this preview: {}", status.get()))
                    .text_xs()
                    .text_color(move || theme().muted_foreground)
            },
        ))
}

fn action(status: Signal<&'static str>, label: &'static str) -> ContextMenuItem {
    ContextMenuItem::new(label).on_select(move || status.set(label))
}

fn history(status: Signal<&'static str>, shortcuts: bool, disabled: bool) -> [ContextMenuItem; 3] {
    [("Back", "⌘["), ("Forward", "⌘]"), ("Reload", "⌘R")].map(|(label, key)| {
        let item = action(status, label).disabled(disabled && label == "Forward");
        if shortcuts { item.shortcut(key) } else { item }
    })
}

fn tools(
    status: Signal<&'static str>,
    labels: [&'static str; 6],
    dir: LayoutDirection,
) -> impl IntoElement {
    let row = move |label| action(status, label).direction(dir);
    context_menu_submenu(
        labels[0],
        dir,
        [
            row(labels[1]).into_element(),
            row(labels[2]).into_element(),
            row(labels[3]).into_element(),
            context_menu_separator().into_element(),
            row(labels[4]).into_element(),
            context_menu_separator().into_element(),
            row(labels[5]).destructive().into_element(),
        ],
    )
}

const TOOLS: [&str; 6] = [
    "More Tools",
    "Save Page...",
    "Create Shortcut...",
    "Name Window...",
    "Developer Tools",
    "Delete",
];

fn checkbox(label: &'static str, value: Signal<bool>, dir: LayoutDirection) -> ContextMenuItem {
    ContextMenuItem::new(label)
        .direction(dir)
        .checkbox(move || value.get())
        .on_select(move || value.update(|on| *on = !*on))
}

fn radio(
    label: &'static str,
    value: &'static str,
    selected: Signal<&'static str>,
    dir: LayoutDirection,
) -> ContextMenuItem {
    ContextMenuItem::new(label)
        .direction(dir)
        .radio(move || selected.get() == value)
        .on_select(move || selected.set(value))
}

pub fn demo() -> impl IntoElement {
    let status = signal("");
    let bookmarks = signal(true);
    let full_urls = signal(false);
    let person = signal("pedro");
    let dir = LayoutDirection::Ltr;
    feedback(
        area("context-menu-demo")
            .children(history(status, true, true))
            .child(tools(status, TOOLS, dir))
            .child(context_menu_separator())
            .child(checkbox("Show Bookmarks", bookmarks, dir))
            .child(checkbox("Show Full URLs", full_urls, dir))
            .child(context_menu_separator())
            .child(context_menu_label("People", dir))
            .child(radio("Pedro Duarte", "pedro", person, dir))
            .child(radio("Colm Tuite", "colm", person, dir)),
        status,
    )
}

pub fn basic() -> impl IntoElement {
    let status = signal("");
    feedback(
        area("context-menu-basic").children(history(status, false, true)),
        status,
    )
}

pub fn submenu() -> impl IntoElement {
    let status = signal("");
    feedback(
        area("context-menu-submenu")
            .child(action(status, "Copy").shortcut("⌘C"))
            .child(action(status, "Cut").shortcut("⌘X"))
            .child(tools(status, TOOLS, LayoutDirection::Ltr)),
        status,
    )
}

pub fn shortcuts() -> impl IntoElement {
    let status = signal("");
    feedback(
        area("context-menu-shortcuts")
            .children(history(status, true, true))
            .child(context_menu_separator())
            .child(action(status, "Save").shortcut("⌘S"))
            .child(action(status, "Save As...").shortcut("⇧⌘S")),
        status,
    )
}

pub fn groups() -> impl IntoElement {
    let status = signal("");
    let mut preview = area("context-menu-groups");
    for (label, entries) in [
        (
            "File",
            &[("New File", "⌘N"), ("Open File", "⌘O"), ("Save", "⌘S")][..],
        ),
        ("Edit", &[("Undo", "⌘Z"), ("Redo", "⇧⌘Z")][..]),
        ("", &[("Cut", "⌘X"), ("Copy", "⌘C"), ("Paste", "⌘V")][..]),
    ] {
        if !label.is_empty() {
            preview = preview.child(context_menu_label(label, LayoutDirection::Ltr));
        }
        preview = preview
            .children(
                entries
                    .iter()
                    .map(|&(label, key)| action(status, label).shortcut(key)),
            )
            .child(context_menu_separator());
    }
    feedback(
        preview.child(action(status, "Delete").shortcut("⌫").destructive()),
        status,
    )
}

pub fn icons() -> impl IntoElement {
    let status = signal("");
    feedback(
        area("context-menu-icons")
            .children(
                [("Copy", "▣"), ("Cut", "✂"), ("Paste", "▤")]
                    .map(|(label, icon)| action(status, label).icon(icon)),
            )
            .child(context_menu_separator())
            .child(action(status, "Delete").icon("×").destructive()),
        status,
    )
}

pub fn checkboxes() -> impl IntoElement {
    let dir = LayoutDirection::Ltr;
    let bookmarks = signal(true);
    let urls = signal(false);
    let tools = signal(true);
    view().w_full().max_w(px(320.0)).child(
        area("context-menu-checkboxes")
            .child(checkbox("Show Bookmarks Bar", bookmarks, dir))
            .child(checkbox("Show Full URLs", urls, dir))
            .child(checkbox("Show Developer Tools", tools, dir)),
    )
}

pub fn radio_items() -> impl IntoElement {
    let dir = LayoutDirection::Ltr;
    let person = signal("pedro");
    let theme = signal("light");
    view().w_full().max_w(px(320.0)).child(
        area("context-menu-radio")
            .child(context_menu_label("People", dir))
            .child(radio("Pedro Duarte", "pedro", person, dir))
            .child(radio("Colm Tuite", "colm", person, dir))
            .child(context_menu_separator())
            .child(context_menu_label("Theme", dir))
            .child(radio("Light", "light", theme, dir))
            .child(radio("Dark", "dark", theme, dir))
            .child(radio("System", "system", theme, dir)),
    )
}

pub fn destructive() -> impl IntoElement {
    let status = signal("");
    feedback(
        area("context-menu-destructive")
            .child(action(status, "Edit").icon("✎"))
            .child(action(status, "Share").icon("↗"))
            .child(context_menu_separator())
            .child(action(status, "Delete").icon("×").destructive()),
        status,
    )
}

pub fn sides() -> impl IntoElement {
    let status = signal("");
    let tile = move |id, mouse, touch, side| {
        view().flex_1().min_w(px(0.0)).child(
            ContextMenu::new(trigger(id, if is_desktop() { mouse } else { touch }))
                .placement(side)
                .children(history(status, false, false)),
        )
    };
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .gap_4()
                .child(tile(
                    "context-menu-top",
                    "Right click (top)",
                    "Long press (top)",
                    Placement::Top,
                ))
                .child(tile(
                    "context-menu-right",
                    "Right click (right)",
                    "Long press (right)",
                    Placement::Right,
                )),
        )
        .child(
            view()
                .flex_row()
                .gap_4()
                .child(tile(
                    "context-menu-bottom",
                    "Right click (bottom)",
                    "Long press (bottom)",
                    Placement::Bottom,
                ))
                .child(tile(
                    "context-menu-left",
                    "Right click (left)",
                    "Long press (left)",
                    Placement::Left,
                )),
        )
        .child(when(
            move || !status.get().is_empty(),
            move || text(move || status.get()).text_xs(),
        ))
}

struct Translation {
    instruction: [&'static str; 2],
    navigation: &'static str,
    history: [&'static str; 3],
    tools: [&'static str; 6],
    settings: [&'static str; 2],
    people: [&'static str; 3],
}

const ARABIC: Translation = Translation {
    instruction: ["انقر بزر الماوس الأيمن هنا", "اضغط مطولاً هنا"],
    navigation: "التنقل",
    history: ["رجوع", "تقدم", "إعادة تحميل"],
    tools: [
        "المزيد من الأدوات",
        "حفظ الصفحة...",
        "إنشاء اختصار...",
        "اسم النافذة...",
        "أدوات المطور",
        "حذف",
    ],
    settings: ["إظهار الإشارات المرجعية", "إظهار عناوين URL الكاملة"],
    people: ["الأشخاص", "بيدرو دوارتي", "كولم تويتي"],
};

const HEBREW: Translation = Translation {
    instruction: ["לחץ לחיצה ימנית כאן", "לחץ לחיצה ארוכה כאן"],
    navigation: "ניווט",
    history: ["חזור", "קדימה", "רענן"],
    tools: [
        "כלים נוספים",
        "שמור עמוד...",
        "צור קיצור דרך...",
        "שם חלון...",
        "כלי מפתח",
        "מחק",
    ],
    settings: ["הצג סימניות", "הצג כתובות URL מלאות"],
    people: ["אנשים", "פדרו דוארטה", "קולם טוויט"],
};

pub fn rtl() -> impl IntoElement {
    let hebrew = signal(false);
    let selected = signal("pedro");
    let status = signal("");
    let bookmarks = signal(true);
    let urls = signal(false);
    let dir = LayoutDirection::Rtl;
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_2()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Arabic")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Small)
                        .on_press(move || hebrew.set(false)),
                )
                .child(
                    Button::new("Hebrew")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Small)
                        .on_press(move || hebrew.set(true)),
                ),
        )
        .child(
            when(hebrew, move || {
                rtl_menu(HEBREW, selected, status, bookmarks, urls, dir)
            })
            .otherwise(move || rtl_menu(ARABIC, selected, status, bookmarks, urls, dir)),
        )
}

fn rtl_menu(
    t: Translation,
    selected: Signal<&'static str>,
    status: Signal<&'static str>,
    bookmarks: Signal<bool>,
    urls: Signal<bool>,
    dir: LayoutDirection,
) -> View {
    let key = if is_desktop() { 0 } else { 1 };
    let navigation = context_menu_submenu(
        t.navigation,
        dir,
        [
            action(status, t.history[0])
                .icon("→")
                .shortcut("⌘[")
                .direction(dir),
            action(status, t.history[1])
                .icon("←")
                .shortcut("⌘]")
                .direction(dir)
                .disabled(true),
            action(status, t.history[2])
                .icon("↻")
                .shortcut("⌘R")
                .direction(dir),
        ],
    );
    feedback(
        ContextMenu::new(trigger("context-menu-rtl", t.instruction[key]).direction(dir))
            .child(navigation)
            .child(tools(status, t.tools, dir))
            .child(context_menu_separator())
            .child(checkbox(t.settings[0], bookmarks, dir))
            .child(checkbox(t.settings[1], urls, dir))
            .child(context_menu_separator())
            .child(context_menu_label(t.people[0], dir))
            .child(radio(t.people[1], "pedro", selected, dir))
            .child(radio(t.people[2], "colm", selected, dir)),
        status,
    )
    .direction(dir)
}
