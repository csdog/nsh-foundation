use components::{
    button::{Button, ButtonSize, ButtonVariant},
    checkbox::{Checkbox, CheckboxState},
};
use nsh_base::theme;
use nsh_core::{AccessibilityState, element::LayoutDirection, prelude::*};

const ENGLISH: [&str; 4] = [
    "Accept terms and conditions",
    "By clicking this checkbox, you agree to the terms.",
    "Enable notifications",
    "You can enable or disable notifications at any time.",
];

pub fn demo() -> impl IntoElement {
    fields(ENGLISH, LayoutDirection::Ltr, "terms-checkbox")
}

pub fn basic() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(224.0))
        .child(Checkbox::new(ENGLISH[0]))
}

pub fn invalid() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(224.0))
        .child(Checkbox::new(ENGLISH[0]).invalid(true))
}

pub fn description() -> impl IntoElement {
    view().w_full().max_w(px(288.0)).child(
        Checkbox::new(ENGLISH[0])
            .description(ENGLISH[1])
            .default_checked(true),
    )
}

pub fn disabled() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(224.0))
        .child(Checkbox::new(ENGLISH[2]).disabled(true))
}

pub fn group() -> impl IntoElement {
    const TITLE: &str = "Show these items on the desktop:";
    let mut fields = view()
        .w_full()
        .max_w(px(336.0))
        .gap_3()
        .role(AccessibilityRole::Group)
        .label(TITLE)
        .child(
            view()
                .gap_1()
                .child(text(TITLE).text_sm().font_weight(FontWeight::MEDIUM))
                .child(
                    text("Select the items you want to show on the desktop.")
                        .text_sm()
                        .text_color(move || theme().muted_foreground),
                ),
        );
    for (name, checked) in [
        ("Hard disks", true),
        ("External disks", true),
        ("CDs, DVDs, and iPods", false),
        ("Connected servers", false),
    ] {
        fields = fields.child(Checkbox::new(name).default_checked(checked));
    }
    fields
}

pub fn table() -> impl IntoElement {
    const PEOPLE: [(&str, &str, &str); 4] = [
        ("Sarah Chen", "sarah.chen@example.com", "Admin"),
        ("Marcus Rodriguez", "marcus.rodriguez@example.com", "User"),
        ("Priya Patel", "priya.patel@example.com", "User"),
        ("David Kim", "david.kim@example.com", "Editor"),
    ];
    let checked = signal([true, false, false, false]);
    let all = Checkbox::new("Select all rows")
        .icon_only()
        .checked(move || {
            let checked = checked.get();
            if checked.iter().all(|item| *item) {
                CheckboxState::True
            } else if checked.iter().any(|item| *item) {
                CheckboxState::Mixed
            } else {
                CheckboxState::False
            }
        })
        .on_change(move |select| checked.set([select; PEOPLE.len()]));
    let cell = |content: &str, width, role| {
        view()
            .w(px(width))
            .flex_shrink_0()
            .role(role)
            .child(text(content.to_owned()).text_sm())
    };
    let row = || {
        view()
            .role(AccessibilityRole::Row)
            .flex_row()
            .items_center()
            .gap_3()
            .p_2()
            .border_b(px(1.0))
            .border_color(move || theme().border)
    };
    let mut table = view()
        .min_w(px(536.0))
        .role(AccessibilityRole::Table)
        .label("Team members")
        .child(
            row()
                .child(
                    view()
                        .w_8()
                        .role(AccessibilityRole::ColumnHeader)
                        .child(all),
                )
                .child(cell("Name", 140.0, AccessibilityRole::ColumnHeader))
                .child(cell("Email", 230.0, AccessibilityRole::ColumnHeader))
                .child(cell("Role", 70.0, AccessibilityRole::ColumnHeader)),
        );
    for (index, (name, email, role)) in PEOPLE.into_iter().enumerate() {
        let control = Checkbox::new(format!("Select {name}"))
            .icon_only()
            .checked(move || {
                if checked.get()[index] {
                    CheckboxState::True
                } else {
                    CheckboxState::False
                }
            })
            .on_change(move |next| checked.update(|selected| selected[index] = next));
        table = table.child(
            row()
                .style_when(
                    move || checked.get()[index],
                    |style| style.bg(theme().muted),
                )
                .accessibility_state(move || AccessibilityState {
                    selected: checked.get()[index],
                    ..Default::default()
                })
                .child(view().w_8().role(AccessibilityRole::Cell).child(control))
                .child(cell(name, 140.0, AccessibilityRole::Cell))
                .child(cell(email, 230.0, AccessibilityRole::Cell))
                .child(cell(role, 70.0, AccessibilityRole::Cell)),
        );
    }
    scroll().horizontal().w_full().child(table)
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1);
    let controls = [("English", 0), ("العربية", 1), ("עברית", 2)].map(|(label, i)| {
        Button::new(label)
            .size(ButtonSize::Small)
            .variant(ButtonVariant::Outline)
            .on_press(move || language.set(i))
            .accessibility_state(move || AccessibilityState {
                selected: language.get() == i,
                ..Default::default()
            })
            .style_when(move || language.get() == i, |style| style.bg(theme().muted))
    });
    view()
        .w_full()
        .items_center()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .child(controls),
        )
        .child(dynamic(move || match language.get() {
            1 => fields(
                [
                    "قبول الشروط والأحكام",
                    "بالنقر على هذا المربع، فإنك توافق على الشروط.",
                    "تفعيل الإشعارات",
                    "يمكنك تفعيل أو إلغاء تفعيل الإشعارات في أي وقت.",
                ],
                LayoutDirection::Rtl,
                "localized-checkbox",
            ),
            2 => fields(
                [
                    "קבל תנאים והגבלות",
                    "על ידי לחיצה על תיבת הסימון הזו, אתה מסכים לתנאים.",
                    "הפעל התראות",
                    "אתה יכול להפעיל או להשבית התראות בכל עת.",
                ],
                LayoutDirection::Rtl,
                "localized-checkbox",
            ),
            _ => fields(ENGLISH, LayoutDirection::Ltr, "localized-checkbox"),
        }))
}

fn fields(copy: [&'static str; 4], direction: LayoutDirection, id: &'static str) -> View {
    let [terms, terms_help, notifications, notifications_help] = copy;
    view()
        .direction(direction)
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(Checkbox::new(terms).test_id(id))
        .child(
            Checkbox::new(terms)
                .description(terms_help)
                .default_checked(true),
        )
        .child(Checkbox::new(notifications).disabled(true))
        .child(
            Checkbox::new(notifications)
                .description(notifications_help)
                .card(),
        )
}
