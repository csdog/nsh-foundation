use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};
use nsh_foundation::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    button::{Button, ButtonSize, ButtonVariant},
    dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
        dropdown_menu_label, dropdown_menu_separator,
    },
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarMenuButton, SidebarMenuSize, SidebarParts,
        SidebarProvider, SidebarSide, SidebarState, SidebarTrigger, sidebar_content,
        sidebar_footer, sidebar_group, sidebar_group_content, sidebar_group_label, sidebar_header,
        sidebar_inset, sidebar_menu, sidebar_menu_action, sidebar_menu_item, sidebar_menu_sub,
        sidebar_menu_sub_button, sidebar_menu_sub_item,
    },
};

const ICONS: [&str; 4] = ["⌘", "◈", "▤", "⚙"];
const PROJECT_ICONS: [&str; 3] = ["▣", "◕", "◇"];
const TEAM_ICONS: [&str; 3] = ["▣", "≈", "⌘"];

struct Labels {
    sidebar: &'static str,
    toggle: &'static str,
    platform: &'static str,
    projects: &'static str,
    teams: [(&'static str, &'static str); 3],
    team_menu: &'static str,
    add_team: &'static str,
    more: &'static str,
    sections: [(&'static str, &'static [&'static str]); 4],
    project_names: [&'static str; 3],
    project_actions: [&'static str; 3],
    account_actions: [&'static str; 5],
}

const EN: Labels = Labels {
    sidebar: "Workspace sidebar",
    toggle: "Toggle Sidebar",
    platform: "Platform",
    projects: "Projects",
    teams: [
        ("Acme Inc", "Enterprise"),
        ("Acme Corp.", "Startup"),
        ("Evil Corp.", "Free"),
    ],
    team_menu: "Teams",
    add_team: "Add team",
    more: "More",
    sections: [
        ("Playground", &["History", "Starred", "Settings"]),
        ("Models", &["Genesis", "Explorer", "Quantum"]),
        (
            "Documentation",
            &["Introduction", "Get Started", "Tutorials", "Changelog"],
        ),
        ("Settings", &["General", "Team", "Billing", "Limits"]),
    ],
    project_names: ["Design Engineering", "Sales & Marketing", "Travel"],
    project_actions: ["View Project", "Share Project", "Delete Project"],
    account_actions: [
        "Upgrade to Pro",
        "Account",
        "Billing",
        "Notifications",
        "Log out",
    ],
};

const AR: Labels = Labels {
    sidebar: "الشريط الجانبي",
    toggle: "تبديل الشريط الجانبي",
    platform: "المنصة",
    projects: "المشاريع",
    teams: [
        ("شركة أكمي", "المؤسسة"),
        ("أكمي كورب", "ناشئة"),
        ("إيفل كورب", "مجاني"),
    ],
    team_menu: "الفرق",
    add_team: "إضافة فريق",
    more: "المزيد",
    sections: [
        ("ملعب", &["السجل", "المميز", "الإعدادات"]),
        ("النماذج", &["جينيسيس", "إكسبلورر", "كوانتوم"]),
        ("التوثيق", &["مقدمة", "ابدأ", "الدروس", "سجل التغييرات"]),
        ("الإعدادات", &["عام", "الفريق", "الفوترة", "الحدود"]),
    ],
    project_names: ["هندسة التصميم", "المبيعات والتسويق", "السفر"],
    project_actions: ["عرض المشروع", "مشاركة المشروع", "حذف المشروع"],
    account_actions: [
        "ترقية إلى Pro",
        "الحساب",
        "الفوترة",
        "الإشعارات",
        "تسجيل الخروج",
    ],
};

const HE: Labels = Labels {
    sidebar: "סרגל צד",
    toggle: "הצג או הסתר סרגל צד",
    platform: "פלטפורמה",
    projects: "פרויקטים",
    teams: [
        ("אקמי בע״מ", "ארגוני"),
        ("אקמי קורפ", "סטארטאפ"),
        ("אוויל קורפ", "חינם"),
    ],
    team_menu: "צוותים",
    add_team: "הוסף צוות",
    more: "עוד",
    sections: [
        ("מגרש משחקים", &["היסטוריה", "מועדפים", "הגדרות"]),
        ("מודלים", &["ג'נסיס", "אקספלורר", "קוונטום"]),
        ("תיעוד", &["מבוא", "התחל", "מדריכים", "יומן שינויים"]),
        ("הגדרות", &["כללי", "צוות", "חיוב", "מגבלות"]),
    ],
    project_names: ["הנדסת עיצוב", "מכירות ושיווק", "נסיעות"],
    project_actions: ["הצג פרויקט", "שתף פרויקט", "מחק פרויקט"],
    account_actions: ["שדרג ל-Pro", "חשבון", "חיוב", "התראות", "התנתק"],
};

#[derive(Clone, Copy)]
struct Workspace {
    team: Signal<usize>,
    sections: Signal<[bool; 4]>,
    selected: Signal<&'static str>,
    feedback: Signal<String>,
}

impl Workspace {
    fn new() -> Self {
        Self {
            team: signal(0),
            sections: signal([true, false, false, false]),
            selected: signal(""),
            feedback: signal(String::new()),
        }
    }

    fn select(self, state: SidebarState, label: &'static str) {
        self.selected.set(label);
        self.feedback.set(label.to_owned());
        if state.is_mobile() {
            state.open_mobile.set(false);
        }
    }
}

fn symbol(value: impl IntoReactive<SharedString>) -> Text {
    text(value).text_base().accessibility_elements_hidden(true)
}

fn menu_content(state: SidebarState, direction: LayoutDirection) -> DropdownMenuContent {
    let side = if state.is_mobile() {
        Placement::Bottom
    } else if direction == LayoutDirection::Rtl {
        Placement::Left
    } else {
        Placement::Right
    };
    DropdownMenuContent::new()
        .width(224)
        .placement(side)
        .direction(direction)
}

fn team_menu(
    state: SidebarState,
    parts: SidebarParts,
    model: Workspace,
    labels: &'static Labels,
    direction: LayoutDirection,
) -> DropdownMenu {
    let team = model.team;
    let brand = view()
        .size(px(32.0))
        .rounded(px(9.0))
        .items_center()
        .justify_center()
        .bg(|| theme().sidebar_primary)
        .text_color(|| theme().sidebar_primary_foreground)
        .child(symbol(move || TEAM_ICONS[team.get()]));
    let trigger = SidebarMenuButton::new(parts, move || labels.teams[team.get()].0)
        .size(SidebarMenuSize::Large)
        .icon(brand)
        .detail(move || labels.teams[team.get()].1)
        .suffix(symbol("⇅"))
        .surface()
        .test_id("sidebar-team-switcher");
    DropdownMenu::new(
        DropdownMenuTrigger::render(trigger),
        menu_content(state, direction)
            .child(dropdown_menu_label(labels.team_menu, direction))
            .children(
                labels
                    .teams
                    .into_iter()
                    .enumerate()
                    .map(move |(index, (name, _))| {
                        DropdownMenuItem::new(name)
                            .icon(TEAM_ICONS[index])
                            .on_select(move || team.set(index))
                    }),
            )
            .child(dropdown_menu_separator())
            .child(
                DropdownMenuItem::new(labels.add_team)
                    .icon("+")
                    .on_select(move || model.feedback.set(labels.add_team.to_owned())),
            ),
    )
}

fn platform(
    state: SidebarState,
    parts: SidebarParts,
    model: Workspace,
    labels: &'static Labels,
    rtl: bool,
) -> View {
    sidebar_group()
        .child(sidebar_group_label(parts, labels.platform))
        .child(
            sidebar_menu().children(labels.sections.into_iter().enumerate().map(
                move |(index, (title, entries))| {
                    let expanded = move || model.sections.with(|sections| sections[index]);
                    let heading = SidebarMenuButton::new(parts, title)
                        .icon(symbol(ICONS[index]))
                        .suffix(symbol(move || {
                            if expanded() {
                                "⌄"
                            } else if rtl {
                                "‹"
                            } else {
                                "›"
                            }
                        }))
                        .surface()
                        .accessibility_state(move || AccessibilityState {
                            expanded: Some(!parts.icon_only() && expanded()),
                            ..Default::default()
                        })
                        .on_press(move || {
                            if parts.icon_only() {
                                model.sections.update(|sections| sections[index] = true);
                                state.open.set(true);
                            } else {
                                model
                                    .sections
                                    .update(|sections| sections[index] = !sections[index]);
                            }
                        });
                    sidebar_menu_item().child(heading).child(when(
                        move || expanded() && !parts.icon_only(),
                        move || {
                            sidebar_menu_sub(parts).children(entries.iter().copied().map(
                                move |item| {
                                    sidebar_menu_sub_item().child(
                                        sidebar_menu_sub_button(parts, item)
                                            .active(move || model.selected.get() == item)
                                            .surface()
                                            .on_press(move || model.select(state, item)),
                                    )
                                },
                            ))
                        },
                    ))
                },
            )),
        )
}

fn projects(
    state: SidebarState,
    parts: SidebarParts,
    model: Workspace,
    labels: &'static Labels,
    direction: LayoutDirection,
) -> View {
    let rows = labels
        .project_names
        .into_iter()
        .enumerate()
        .map(move |(index, project)| {
            let actions = ["▣", "↗", "×"];
            let choices =
                labels
                    .project_actions
                    .into_iter()
                    .enumerate()
                    .map(move |(action, name)| {
                        DropdownMenuItem::new(name)
                            .icon(actions[action])
                            .on_select(move || {
                                if action == 0 {
                                    model.select(state, project);
                                } else {
                                    model.feedback.set(format!("{name} · {project}"));
                                }
                            })
                    });
            sidebar_menu_item()
                .flex_row()
                .items_center()
                .child(
                    SidebarMenuButton::new(parts, project)
                        .icon(symbol(PROJECT_ICONS[index]))
                        .active(move || model.selected.get() == project)
                        .surface()
                        .flex_1()
                        .on_press(move || model.select(state, project)),
                )
                .child(DropdownMenu::new(
                    DropdownMenuTrigger::render(sidebar_menu_action(labels.more, "⋯", true)),
                    menu_content(state, direction).children(choices),
                ))
        });
    parts.expanded_only(
        sidebar_group()
            .child(sidebar_group_label(parts, labels.projects))
            .child(
                sidebar_menu().children(rows).child(
                    SidebarMenuButton::new(parts, labels.more)
                        .icon(symbol("⋯"))
                        .surface()
                        .on_press(move || model.feedback.set(labels.more.to_owned())),
                ),
            ),
    )
}

fn account(
    state: SidebarState,
    parts: SidebarParts,
    model: Workspace,
    labels: &'static Labels,
    direction: LayoutDirection,
) -> DropdownMenu {
    let avatar = Avatar::new("shadcn")
        .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
        .fallback(AvatarFallback::new("CN"));
    let trigger = SidebarMenuButton::new(parts, "shadcn")
        .icon(avatar)
        .detail("m@example.com")
        .size(SidebarMenuSize::Large)
        .suffix(symbol("⇅"))
        .surface();
    let options = labels.account_actions;
    let item = move |index: usize, icon| {
        DropdownMenuItem::new(options[index])
            .icon(icon)
            .on_select(move || model.feedback.set(options[index].to_owned()))
    };
    DropdownMenu::new(
        DropdownMenuTrigger::render(trigger),
        menu_content(state, direction)
            .align(Align::End)
            .child(dropdown_menu_label("shadcn · m@example.com", direction))
            .child(dropdown_menu_separator())
            .child(item(0, "✧"))
            .child(dropdown_menu_separator())
            .child(item(1, "◉"))
            .child(item(2, "▤"))
            .child(item(3, "♧"))
            .child(dropdown_menu_separator())
            .child(item(4, "↗")),
    )
}

fn workspace(state: SidebarState, labels: &'static Labels, side: SidebarSide) -> View {
    let model = Workspace::new();
    let rtl = side == SidebarSide::Right;
    let direction = if rtl {
        LayoutDirection::Rtl
    } else {
        LayoutDirection::Ltr
    };
    let main = sidebar_inset()
        .child(
            view()
                .h(move || {
                    px(if !state.is_mobile() && !state.open.get() {
                        48.0
                    } else {
                        64.0
                    })
                })
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .child(SidebarTrigger::directional(state, labels.toggle, direction)),
        )
        .child(when(
            move || !model.feedback.with(String::is_empty),
            move || {
                view().px_4().child(
                    text(move || model.feedback.get())
                        .text_sm()
                        .text_color(|| theme().muted_foreground),
                )
            },
        ));
    preview_frame(
        Sidebar::new(state, move |parts| {
            view()
                .size_full()
                .min_h(px(0.0))
                .child(sidebar_header().child(team_menu(state, parts, model, labels, direction)))
                .child(sidebar_content(
                    view()
                        .child(platform(state, parts, model, labels, rtl))
                        .child(projects(state, parts, model, labels, direction)),
                ))
                .child(sidebar_footer().child(account(state, parts, model, labels, direction)))
        })
        .side(side)
        .direction(direction)
        .collapsible(SidebarCollapsible::Icon)
        .label(labels.sidebar)
        .rail(true)
        .layout(main),
    )
}

fn preview_frame(content: View) -> View {
    view()
        .w_full()
        .h(px(500.0))
        .overflow_hidden()
        .rounded(px(12.0))
        .border(px(1.0))
        .border_color(|| theme().border)
        .child(content)
}

pub fn demo() -> impl IntoElement {
    SidebarProvider::new(|state| workspace(state, &EN, SidebarSide::Left))
}

pub fn controlled() -> impl IntoElement {
    let open = signal(true);
    SidebarProvider::new(move |state: SidebarState| {
        let selected = signal("");
        let main = sidebar_inset()
            .child(
                view()
                    .h(px(48.0))
                    .flex_row()
                    .px_3()
                    .items_center()
                    .child(when(
                        move || state.is_mobile(),
                        move || SidebarTrigger::new(state),
                    ))
                    .child(when(
                        move || !state.is_mobile(),
                        move || {
                            Button::new(move || {
                                if open.get() {
                                    "Close Sidebar"
                                } else {
                                    "Open Sidebar"
                                }
                            })
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Small)
                            .on_press(move || open.update(|open| *open = !*open))
                        },
                    )),
            )
            .child(
                view().px_4().child(
                    text(move || selected.get())
                        .text_sm()
                        .text_color(|| theme().muted_foreground),
                ),
            );
        preview_frame(
            Sidebar::new(state, move |parts| {
                view().size_full().child(sidebar_content(
                    sidebar_group()
                        .child(sidebar_group_label(parts, "Projects"))
                        .child(
                            sidebar_group_content().child(
                                sidebar_menu().children(
                                    [
                                        "Design Engineering",
                                        "Sales & Marketing",
                                        "Travel",
                                        "Support",
                                        "Feedback",
                                    ]
                                    .into_iter()
                                    .zip(["▣", "◕", "◇", "◉", "↗"])
                                    .map(
                                        move |(name, icon)| {
                                            sidebar_menu_item().child(
                                                SidebarMenuButton::new(parts, name)
                                                    .icon(symbol(icon))
                                                    .active(move || selected.get() == name)
                                                    .surface()
                                                    .on_press(move || {
                                                        selected.set(name);
                                                        state.open_mobile.set(false);
                                                    }),
                                            )
                                        },
                                    ),
                                ),
                            ),
                        ),
                ))
            })
            .label("Projects sidebar")
            .rail(true)
            .layout(main),
        )
    })
    .open(open)
    .shortcut(false)
}

pub fn rtl() -> View {
    let locale = signal(1_usize);
    view()
        .w_full()
        .gap_3()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: locale.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || locale.set(index))
                    },
                )),
        )
        .child(dynamic(move || {
            let (labels, side) = match locale.get() {
                0 => (&EN, SidebarSide::Left),
                1 => (&AR, SidebarSide::Right),
                _ => (&HE, SidebarSide::Right),
            };
            SidebarProvider::new(move |state| workspace(state, labels, side)).shortcut(false)
        }))
}
