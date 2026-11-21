use nsh_base::{menu, menu_item, menu_separator, theme};
use nsh_core::{
    element::{LayoutDirection, set_clipboard},
    prelude::*,
};
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    data_table::{DataTable, DataTableColumn, DataTableLabels},
};

#[derive(Clone, Copy)]
struct Payment {
    id: &'static str,
    cents: u32,
    status: usize,
    email: &'static str,
}

const PAYMENTS: [Payment; 5] = [
    Payment {
        id: "m5gr84i9",
        cents: 31600,
        status: 0,
        email: "ken99@example.com",
    },
    Payment {
        id: "3u1reuv4",
        cents: 24200,
        status: 0,
        email: "Abe45@example.com",
    },
    Payment {
        id: "derv1ws0",
        cents: 83700,
        status: 1,
        email: "Monserrat44@example.com",
    },
    Payment {
        id: "5kma53ae",
        cents: 87400,
        status: 0,
        email: "Silas22@example.com",
    },
    Payment {
        id: "bhqecj4p",
        cents: 72100,
        status: 2,
        email: "carmella@example.com",
    },
];

#[derive(Clone, Copy)]
struct Copy {
    table: &'static str,
    filter: &'static str,
    fields: [&'static str; 3],
    statuses: [&'static str; 3],
    actions: [&'static str; 5],
    labels: DataTableLabels,
}

fn english() -> Copy {
    Copy {
        table: "Payments",
        filter: "Filter emails...",
        fields: ["Status", "Email", "Amount"],
        statuses: ["Success", "Processing", "Failed"],
        actions: [
            "Open menu",
            "Actions",
            "Copy payment ID",
            "View customer",
            "View payment details",
        ],
        labels: DataTableLabels::default(),
    }
}

const ARABIC: Copy = Copy {
    table: "المدفوعات",
    filter: "تصفية البريد الإلكتروني...",
    fields: ["الحالة", "البريد الإلكتروني", "المبلغ"],
    statuses: ["ناجح", "قيد المعالجة", "فشل"],
    actions: [
        "فتح القائمة",
        "الإجراءات",
        "نسخ معرف الدفع",
        "عرض العميل",
        "عرض تفاصيل الدفع",
    ],
    labels: DataTableLabels {
        columns: "الأعمدة",
        select_all: "تحديد الكل",
        select_row: "تحديد الصف",
        empty: "لا توجد نتائج.",
        selection_count: |selected, total| format!("{selected} من {total} صف(وف) محدد."),
        previous: "السابق",
        next: "التالي",
    },
};

const HEBREW: Copy = Copy {
    table: "תשלומים",
    filter: "סנן אימיילים...",
    fields: ["סטטוס", "אימייל", "סכום"],
    statuses: ["הצליח", "מעבד", "נכשל"],
    actions: [
        "פתח תפריט",
        "פעולות",
        "העתק מזהה תשלום",
        "צפה בלקוח",
        "צפה בפרטי תשלום",
    ],
    labels: DataTableLabels {
        columns: "עמודות",
        select_all: "בחר הכל",
        select_row: "בחר שורה",
        empty: "אין תוצאות.",
        selection_count: |selected, total| format!("{selected} מתוך {total} שורות נבחרו."),
        previous: "הקודם",
        next: "הבא",
    },
};

pub fn demo() -> impl IntoElement {
    payments(english(), LayoutDirection::Ltr, 10)
}

pub fn pagination() -> impl IntoElement {
    payments(english(), LayoutDirection::Ltr, 2)
}

pub fn rtl() -> impl IntoElement {
    let hebrew = signal(false);
    view()
        .w_full()
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
        .child(
            when(hebrew, || payments(HEBREW, LayoutDirection::Rtl, 10))
                .otherwise(|| payments(ARABIC, LayoutDirection::Rtl, 10)),
        )
}

fn payments(copy: Copy, direction: LayoutDirection, page_size: usize) -> View {
    let status = signal(String::new());
    let columns = vec![
        DataTableColumn::new(copy.fields[0], 104.0, move |row: &Payment| {
            text(copy.statuses[row.status]).text_sm()
        }),
        DataTableColumn::new(copy.fields[1], 224.0, |row: &Payment| {
            text(row.email.to_ascii_lowercase()).text_sm()
        })
        .flexible()
        .sortable(|a, b| {
            a.email
                .to_ascii_lowercase()
                .cmp(&b.email.to_ascii_lowercase())
        }),
        DataTableColumn::new(copy.fields[2], 90.0, move |row: &Payment| {
            text(amount(row.cents, direction))
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
        })
        .align_end(),
        DataTableColumn::new("", 48.0, move |row: &Payment| {
            actions(*row, copy, direction, status)
        })
        .always_visible(),
    ];
    view()
        .w_full()
        .direction(direction)
        .gap_2()
        .child(
            DataTable::new(copy.table, PAYMENTS.to_vec(), columns)
                .filter(copy.filter, |row| row.email)
                .page_size(page_size)
                .direction(direction)
                .labels(copy.labels),
        )
        .child(when(
            move || status.with(|s| !s.is_empty()),
            move || {
                text(move || status.get())
                    .text_sm()
                    .text_color(move || theme().muted_foreground)
            },
        ))
}

fn amount(cents: u32, direction: LayoutDirection) -> String {
    if direction == LayoutDirection::Ltr {
        return format!("${}.{:02}", cents / 100, cents % 100);
    }
    let number = format!("{}٫{:02}", cents / 100, cents % 100);
    let digits: String = number
        .chars()
        .map(|c| match c.to_digit(10) {
            Some(digit) => char::from_u32('٠' as u32 + digit).unwrap(),
            None => c,
        })
        .collect();
    format!("{digits} US$")
}

fn actions(
    row: Payment,
    copy: Copy,
    dir: LayoutDirection,
    status: Signal<String>,
) -> impl IntoElement {
    let [open, title, copy_id, customer, details] = copy.actions;
    let item = move |label: &'static str, value: String| {
        menu_item()
            .accessibility_label(label)
            .px_2()
            .py_1()
            .min_h(px(32.0))
            .rounded(px(10.0))
            .hover(|style| style.bg(theme().accent))
            .focus(|style| style.bg(theme().accent))
            .on_select(move || {
                if label == copy_id {
                    set_clipboard(row.id);
                }
                status.set(format!("{label} · {value}"));
            })
            .child(view().direction(dir).child(text(label).text_sm()))
    };
    menu()
        .align(Align::End)
        .margin(px(8.0))
        .offset(px(4.0))
        .popup_class("min-w-44 p-1 rounded-2xl border bg-popover text-popover-foreground shadow-lg")
        .trigger(
            Button::new(open)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconExtraSmall)
                .icon_start(text("⋯").text_lg().accessibility_elements_hidden(true))
                .surface(),
        )
        .child(
            view().direction(dir).px_2().py_1().child(
                text(title)
                    .text_xs()
                    .text_color(move || theme().muted_foreground),
            ),
        )
        .child(item(copy_id, row.id.to_owned()))
        .child(
            menu_separator()
                .my_1()
                .h(px(1.0))
                .bg(move || theme().border),
        )
        .child(item(customer, row.email.to_ascii_lowercase()))
        .child(item(
            details,
            format!(
                "{} · {} · {}",
                row.id,
                copy.statuses[row.status],
                amount(row.cents, dir)
            ),
        ))
}
