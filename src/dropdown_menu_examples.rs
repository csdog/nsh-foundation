use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};
use nsh_foundation::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider, use_direction},
    dropdown_menu::{
        DropdownMenu, DropdownMenuContent as Content, DropdownMenuItem as Item,
        DropdownMenuSubmenu as Submenu, DropdownMenuTrigger as Trigger, dropdown_menu_group,
        dropdown_menu_label as label, dropdown_menu_separator as separator,
    },
};

type Status = Signal<&'static str>;

fn action(status: Status, name: &'static str) -> Item {
    Item::new(name).on_select(move || status.set(name))
}

fn with_feedback(trigger: Trigger, content: Content, status: Status) -> View {
    view()
        .items_center()
        .gap_2()
        .child(DropdownMenu::new(trigger, content))
        .child(when(
            move || !status.get().is_empty(),
            move || {
                text(move || format!("Selected in this preview: {}", status.get()))
                    .text_xs()
                    .text_color(move || theme().muted_foreground)
            },
        ))
}

fn checkbox(name: &'static str, value: Signal<bool>) -> Item {
    Item::new(name)
        .checkbox(move || value.get())
        .close_on_select(false)
        .on_select(move || value.update(|checked| *checked = !*checked))
}

fn radio(name: &'static str, key: &'static str, selected: Status) -> Item {
    Item::new(name)
        .radio(move || selected.get() == key)
        .close_on_select(false)
        .on_select(move || selected.set(key))
}

fn account(status: Status) -> [Item; 3] {
    [("Profile", "⇧⌘P"), ("Billing", "⌘B"), ("Settings", "⌘S")]
        .map(|(name, key)| action(status, name).shortcut(key))
}

fn help(status: Status) -> [Item; 3] {
    [
        action(status, "GitHub"),
        action(status, "Support"),
        Item::new("API").disabled(true),
    ]
}

fn invite(status: Status, more: bool) -> Submenu {
    let menu =
        Submenu::new("Invite users").children([action(status, "Email"), action(status, "Message")]);
    if more {
        menu.child(
            Submenu::new("More options")
                .children(["Calendly", "Slack"].map(|name| action(status, name)))
                .child(separator())
                .child(action(status, "Webhook")),
        )
        .child(separator())
        .child(action(status, "Advanced..."))
    } else {
        menu.child(separator()).child(action(status, "More..."))
    }
}

pub fn demo() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open").test_id("dropdown-menu-demo-trigger"),
        Content::new()
            .width(160)
            .child(dropdown_menu_group(
                "My Account",
                LayoutDirection::Ltr,
                account(status),
            ))
            .child(separator())
            .child(action(status, "Team"))
            .child(invite(status, false))
            .child(action(status, "New Team").shortcut("⌘+T"))
            .child(separator())
            .children(help(status))
            .child(separator())
            .child(action(status, "Log out").shortcut("⇧⌘Q")),
        status,
    )
}

pub fn basic() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open"),
        Content::new()
            .child(dropdown_menu_group(
                "My Account",
                LayoutDirection::Ltr,
                ["Profile", "Billing", "Settings"].map(|name| action(status, name)),
            ))
            .child(separator())
            .children(help(status)),
        status,
    )
}

pub fn submenu() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open"),
        Content::new()
            .child(action(status, "Team"))
            .child(invite(status, true))
            .child(action(status, "New Team").shortcut("⌘+T")),
        status,
    )
}

pub fn shortcuts() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open"),
        Content::new()
            .child(dropdown_menu_group(
                "My Account",
                LayoutDirection::Ltr,
                account(status),
            ))
            .child(separator())
            .child(action(status, "Log out").shortcut("⇧⌘Q")),
        status,
    )
}

pub fn icons() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open"),
        Content::new()
            .children(
                [("Profile", "◉"), ("Billing", "▤"), ("Settings", "⚙")]
                    .map(|(name, icon)| action(status, name).icon(icon)),
            )
            .child(separator())
            .child(action(status, "Log out").icon("↪").destructive()),
        status,
    )
}

pub fn checkboxes() -> impl IntoElement {
    let status = signal(true);
    let activity = signal(false);
    let panel = signal(false);
    DropdownMenu::new(
        Trigger::new("Open"),
        Content::new().width(160).child(dropdown_menu_group(
            "Appearance",
            LayoutDirection::Ltr,
            [
                checkbox("Status Bar", status),
                checkbox("Activity Bar", activity).disabled(true),
                checkbox("Panel", panel),
            ],
        )),
    )
}

pub fn checkboxes_icons() -> impl IntoElement {
    let email = signal(true);
    let sms = signal(false);
    let push = signal(true);
    DropdownMenu::new(
        Trigger::new("Notifications"),
        Content::new().width(192).child(dropdown_menu_group(
            "Notification Preferences",
            LayoutDirection::Ltr,
            [
                checkbox("Email notifications", email).icon("✉"),
                checkbox("SMS notifications", sms).icon("▤"),
                checkbox("Push notifications", push).icon("●"),
            ],
        )),
    )
}

pub fn radio_group() -> impl IntoElement {
    let selected = signal("bottom");
    DropdownMenu::new(
        Trigger::new("Open"),
        Content::new().width(128).child(dropdown_menu_group(
            "Panel Position",
            LayoutDirection::Ltr,
            [
                radio("Top", "top", selected),
                radio("Bottom", "bottom", selected),
                radio("Right", "right", selected),
            ],
        )),
    )
}

pub fn radio_icons() -> impl IntoElement {
    let selected = signal("card");
    DropdownMenu::new(
        Trigger::new("Payment Method"),
        Content::new().width(224).child(dropdown_menu_group(
            "Select Payment Method",
            LayoutDirection::Ltr,
            [
                radio("Credit Card", "card", selected).icon("▤"),
                radio("PayPal", "paypal", selected).icon("P"),
                radio("Bank Transfer", "bank", selected).icon("▥"),
            ],
        )),
    )
}

pub fn destructive() -> impl IntoElement {
    let status = signal("");
    with_feedback(
        Trigger::new("Open"),
        Content::new()
            .children([
                action(status, "Edit").icon("✎"),
                action(status, "Share").icon("↗"),
            ])
            .child(separator())
            .child(action(status, "Delete").icon("×").destructive()),
        status,
    )
}

pub fn avatar() -> impl IntoElement {
    let status = signal("");
    let avatar = Avatar::new("shadcn")
        .image(AvatarImage::new(asset!("avatar-shadcn.jpg")))
        .fallback(AvatarFallback::new("LR"));
    let trigger = Trigger::render(
        Button::new("Open avatar menu")
            .size(ButtonSize::Icon)
            .variant(ButtonVariant::Ghost)
            .radius(99.0)
            .icon_start(avatar)
            .surface(),
    );
    with_feedback(
        trigger,
        Content::new()
            .align(Align::End)
            .children(
                [("Account", "◉"), ("Billing", "▤"), ("Notifications", "●")]
                    .map(|(name, icon)| action(status, name).icon(icon)),
            )
            .child(separator())
            .child(action(status, "Sign Out").icon("↪")),
        status,
    )
}

pub fn complex() -> impl IntoElement {
    let status = signal("");
    let email = signal(true);
    let sms = signal(false);
    let push = signal(true);
    let appearance = signal("light");
    let item = |name, icon| action(status, name).icon(icon);
    let title = |name| label(name, LayoutDirection::Ltr);
    let projects = Submenu::new("Open Recent")
        .icon("▣")
        .child(title("Recent Projects"))
        .child(item("Project Alpha", "▤"))
        .child(item("Project Beta", "▤"))
        .child(
            Submenu::new("More Projects")
                .icon("⋯")
                .children([item("Project Gamma", "▤"), item("Project Delta", "▤")]),
        )
        .child(separator())
        .child(item("Browse...", "⌕"));
    let theme = Submenu::new("Theme")
        .icon("◈")
        .child(title("Appearance"))
        .children(
            [
                ("Light", "light", "☀"),
                ("Dark", "dark", "☾"),
                ("System", "system", "▣"),
            ]
            .map(|(name, key, icon)| radio(name, key, appearance).icon(icon)),
        );
    let notifications = Submenu::new("Notifications")
        .icon("●")
        .child(title("Notification Types"))
        .children([
            checkbox("Push Notifications", push).icon("●"),
            checkbox("Email Notifications", email).icon("✉"),
        ]);
    let settings = Submenu::new("Settings")
        .icon("⚙")
        .child(title("Preferences"))
        .children([item("Keyboard Shortcuts", "⌘"), item("Language", "◎")])
        .child(notifications)
        .child(separator())
        .child(item("Privacy & Security", "◆"));
    with_feedback(
        Trigger::new("Complex Menu"),
        Content::new()
            .width(176)
            .child(title("File"))
            .children([
                item("New File", "▤").shortcut("⌘N"),
                item("New Folder", "▣").shortcut("⇧⌘N"),
            ])
            .child(projects)
            .child(separator())
            .children([
                item("Save", "↓").shortcut("⌘S"),
                item("Export", "↗").shortcut("⇧⌘E"),
            ])
            .child(separator())
            .child(title("View"))
            .children([
                checkbox("Show Sidebar", email).icon("◉"),
                checkbox("Show Status Bar", sms).icon("▤"),
            ])
            .child(theme)
            .child(separator())
            .child(title("Account"))
            .children([item("Profile", "◉").shortcut("⇧⌘P"), item("Billing", "▤")])
            .child(settings)
            .child(separator())
            .children([item("Help & Support", "?"), item("Documentation", "▤")])
            .child(separator())
            .child(item("Sign Out", "↪").shortcut("⇧⌘Q").destructive()),
        status,
    )
}

struct Translation {
    open: &'static str,
    account: [&'static str; 4],
    team: [&'static str; 3],
    invite: [&'static str; 5],
    more: [&'static str; 3],
    view: [&'static str; 4],
    position: [&'static str; 5],
    logout: &'static str,
}

const ARABIC: Translation = Translation {
    open: "افتح القائمة",
    account: ["الحساب", "الملف الشخصي", "الفوترة", "الإعدادات"],
    team: ["الفريق", "دعوة المستخدمين", "فريق جديد"],
    invite: ["البريد الإلكتروني", "رسالة", "المزيد", "متقدم...", "⌘+T"],
    more: ["تقويم", "دردشة", "خطاف ويب"],
    view: ["عرض", "شريط الحالة", "شريط النشاط", "اللوحة"],
    position: ["الموضع", "أعلى", "أسفل", "يمين", "يسار"],
    logout: "تسجيل الخروج",
};

const HEBREW: Translation = Translation {
    open: "פתח תפריט",
    account: ["חשבון", "פרופיל", "חיוב", "הגדרות"],
    team: ["הצוות", "הזמן משתמשים", "צוות חדש"],
    invite: ["אימייל", "הודעה", "עוד", "מתקדם...", "⌘+T"],
    more: ["יומן", "צ'אט", "Webhook"],
    view: ["תצוגה", "שורת סטטוס", "שורת פעילות", "לוח"],
    position: ["מיקום", "למעלה", "למטה", "ימין", "שמאל"],
    logout: "התנתק",
};

pub fn rtl() -> impl IntoElement {
    let hebrew = signal(false);
    let status = signal("");
    let status_bar = signal(true);
    let activity = signal(false);
    let panel = signal(false);
    let position = signal("bottom");
    view()
        .items_center()
        .gap_3()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Arabic")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(false)),
                )
                .child(
                    Button::new("Hebrew")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(true)),
                ),
        )
        .child(DirectionProvider::new(Direction::Rtl, move || {
            let dir = use_direction();
            dynamic(move || {
                let t = if hebrew.get() { &HEBREW } else { &ARABIC };
                let visual = dir.get().into();
                let account =
                    Submenu::new(t.account[0]).children(std::array::from_fn::<_, 3, _>(|i| {
                        action(status, t.account[i + 1]).icon(["◉", "▤", "⚙"][i])
                    }));
                let more = Submenu::new(t.invite[2])
                    .children(t.more[..2].iter().map(|&name| action(status, name)))
                    .child(separator())
                    .child(action(status, t.more[2]));
                let invite = Submenu::new(t.team[1])
                    .children(t.invite[..2].iter().map(|&name| action(status, name)))
                    .child(more)
                    .child(separator())
                    .child(action(status, t.invite[3]));
                with_feedback(
                    Trigger::new(t.open).test_id("dropdown-menu-rtl-trigger"),
                    Content::new()
                        .width(192)
                        .child(account)
                        .child(separator())
                        .child(label(t.team[0], visual))
                        .child(action(status, t.team[0]))
                        .child(invite)
                        .child(action(status, t.team[2]).shortcut(t.invite[4]))
                        .child(separator())
                        .child(dropdown_menu_group(
                            t.view[0],
                            visual,
                            [
                                checkbox(t.view[1], status_bar),
                                checkbox(t.view[2], activity).disabled(true),
                                checkbox(t.view[3], panel),
                            ],
                        ))
                        .child(separator())
                        .child(dropdown_menu_group(
                            t.position[0],
                            visual,
                            [
                                radio(t.position[1], "top", position),
                                radio(t.position[2], "bottom", position),
                                radio(t.position[3], "right", position),
                                radio(t.position[4], "left", position),
                            ],
                        ))
                        .child(separator())
                        .child(action(status, t.logout).destructive()),
                    status,
                )
            })
        }))
}
