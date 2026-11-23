use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
        dropdown_menu_separator,
    },
    table::{
        Table, TableBody, TableCaption, TableCell, TableColumn, TableFooter, TableHead,
        TableHeader, TableRow,
    },
};
use nsh_core::{element::announce_for_accessibility, prelude::*};

struct Invoice {
    id: &'static str,
    status: usize,
    method: usize,
    amount: u32,
}

const INVOICES: [Invoice; 7] = [
    Invoice {
        id: "INV001",
        status: 0,
        method: 0,
        amount: 250,
    },
    Invoice {
        id: "INV002",
        status: 1,
        method: 1,
        amount: 150,
    },
    Invoice {
        id: "INV003",
        status: 2,
        method: 2,
        amount: 350,
    },
    Invoice {
        id: "INV004",
        status: 0,
        method: 0,
        amount: 450,
    },
    Invoice {
        id: "INV005",
        status: 0,
        method: 1,
        amount: 550,
    },
    Invoice {
        id: "INV006",
        status: 1,
        method: 2,
        amount: 200,
    },
    Invoice {
        id: "INV007",
        status: 2,
        method: 0,
        amount: 300,
    },
];

struct Labels {
    name: &'static str,
    caption: &'static str,
    headings: [&'static str; 4],
    statuses: [&'static str; 3],
    methods: [&'static str; 3],
    total: &'static str,
}

const ENGLISH: Labels = Labels {
    name: "Recent invoices",
    caption: "A list of your recent invoices.",
    headings: ["Invoice", "Status", "Method", "Amount"],
    statuses: ["Paid", "Pending", "Unpaid"],
    methods: ["Credit Card", "PayPal", "Bank Transfer"],
    total: "Total",
};

const ARABIC: Labels = Labels {
    name: "الفواتير الأخيرة",
    caption: "قائمة بفواتيرك الأخيرة.",
    headings: ["الفاتورة", "الحالة", "الطريقة", "المبلغ"],
    statuses: ["مدفوع", "قيد الانتظار", "غير مدفوع"],
    methods: ["بطاقة ائتمانية", "PayPal", "تحويل بنكي"],
    total: "المجموع",
};

const HEBREW: Labels = Labels {
    name: "חשבוניות אחרונות",
    caption: "רשימת החשבוניות האחרונות שלך.",
    headings: ["חשבונית", "סטטוס", "שיטה", "סכום"],
    statuses: ["שולם", "ממתין", "לא שולם"],
    methods: ["כרטיס אשראי", "PayPal", "העברה בנקאית"],
    total: "סה\"כ",
};

fn currency(amount: u32) -> String {
    if amount < 1000 {
        format!("${amount}.00")
    } else {
        format!("${},{:03}.00", amount / 1000, amount % 1000)
    }
}

fn invoices(labels: &'static Labels, invoices: &[Invoice]) -> Table {
    let [invoice, status, method, amount] = labels.headings;
    let columns = [
        TableColumn::fixed(100.0),
        TableColumn::flexible(100.0),
        TableColumn::flexible(152.0),
        TableColumn::flexible(108.0),
    ];
    let total = invoices.iter().map(|row| row.amount).sum();
    Table::new(labels.name, columns)
        .header(
            TableHeader::new().row(
                TableRow::new()
                    .head(TableHead::new(invoice))
                    .head(TableHead::new(status))
                    .head(TableHead::new(method))
                    .head(TableHead::new(amount).align_end()),
            ),
        )
        .body(TableBody::new().rows(invoices.iter().map(|row| {
            TableRow::new()
                .cell(TableCell::text(row.id).medium())
                .cell(TableCell::text(labels.statuses[row.status]))
                .cell(TableCell::text(labels.methods[row.method]))
                .cell(TableCell::text(currency(row.amount)).align_end())
        })))
        .footer(
            TableFooter::new().row(
                TableRow::new()
                    .cell(TableCell::text(labels.total).span(3))
                    .cell(TableCell::text(currency(total)).align_end()),
            ),
        )
        .caption(TableCaption::new(labels.caption))
}

pub fn demo() -> impl IntoElement {
    invoices(&ENGLISH, &INVOICES)
}

pub fn footer() -> impl IntoElement {
    invoices(&ENGLISH, &INVOICES[..3])
}

pub fn actions() -> View {
    let feedback = signal(String::new());
    let product = |name: &'static str, price: &'static str| {
        let open = signal(false);
        let item = |action, destructive| {
            let item = DropdownMenuItem::new(action).on_select(move || {
                let message = format!("{action} · {name}");
                announce_for_accessibility(message.clone());
                feedback.set(message);
            });
            if destructive {
                item.destructive()
            } else {
                item
            }
        };
        let menu = DropdownMenu::new(
            DropdownMenuTrigger::render(
                Button::new(format!("Open menu for {name}"))
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Icon)
                    .icon_start(text("⋯").text_base().accessibility_elements_hidden(true))
                    .surface(),
            ),
            DropdownMenuContent::new()
                .align(Align::End)
                .child(item("Edit", false))
                .child(item("Duplicate", false))
                .child(dropdown_menu_separator())
                .child(item("Delete", true)),
        )
        .open(open);
        TableRow::new()
            .expanded(open)
            .cell(TableCell::text(name).medium())
            .cell(TableCell::text(price))
            .cell(TableCell::new(menu).align_end())
    };

    let columns = [
        TableColumn::flexible(170.0),
        TableColumn::flexible(86.0),
        TableColumn::fixed(76.0),
    ];
    view()
        .w_full()
        .gap_3()
        .child(
            Table::new("Product actions", columns)
                .header(
                    TableHeader::new().row(
                        TableRow::new()
                            .head(TableHead::new("Product"))
                            .head(TableHead::new("Price"))
                            .head(TableHead::new("Actions").align_end()),
                    ),
                )
                .body(TableBody::new().rows([
                    product("Wireless Mouse", "$29.99"),
                    product("Mechanical Keyboard", "$129.99"),
                    product("USB-C Hub", "$49.99"),
                ])),
        )
        .child(when(
            move || !feedback.with(String::is_empty),
            move || {
                text(move || feedback.get())
                    .text_sm()
                    .text_color(|| nsh_base::theme().muted_foreground)
            },
        ))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    view()
        .w_full()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, label)| {
                        Button::new(label)
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || language.set(index))
                    },
                )),
        )
        .child(dynamic(move || {
            let (direction, labels) = match language.get() {
                0 => (Direction::Ltr, &ENGLISH),
                1 => (Direction::Rtl, &ARABIC),
                _ => (Direction::Rtl, &HEBREW),
            };
            DirectionProvider::new(direction, move || invoices(labels, &INVOICES))
        }))
}
