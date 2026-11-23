use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    native_select::{NativeSelect, NativeSelectOption},
    pagination::{
        Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
        PaginationNext, PaginationPrevious,
    },
    ui::muted,
};

struct Locale {
    direction: Direction,
    pagination: &'static str,
    page: &'static str,
    previous: &'static str,
    next: &'static str,
    arabic_digits: bool,
}

impl Locale {
    fn number(&self, number: usize) -> String {
        let number = number.to_string();
        if self.arabic_digits {
            const DIGITS: [char; 10] = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
            number
                .bytes()
                .map(|digit| DIGITS[usize::from(digit - b'0')])
                .collect()
        } else {
            number
        }
    }
}

const LOCALES: [Locale; 3] = [
    Locale {
        direction: Direction::Ltr,
        pagination: "Page navigation",
        page: "Page",
        previous: "Previous",
        next: "Next",
        arabic_digits: false,
    },
    Locale {
        direction: Direction::Rtl,
        pagination: "ترقيم الصفحات",
        page: "الصفحة",
        previous: "السابق",
        next: "التالي",
        arabic_digits: true,
    },
    Locale {
        direction: Direction::Rtl,
        pagination: "ניווט בין דפים",
        page: "דף",
        previous: "הקודם",
        next: "הבא",
        arabic_digits: false,
    },
];

fn page_item(
    page: Signal<usize>,
    number: impl Fn() -> usize + Copy + 'static,
    locale: &'static Locale,
) -> PaginationItem {
    PaginationItem::new(
        PaginationLink::action(move || locale.number(number()), move || page.set(number()))
            .label(move || format!("{} {}", locale.page, locale.number(number())))
            .current(move || page.get() == number()),
    )
}

fn previous(page: Signal<usize>, label: &'static str) -> PaginationLink {
    PaginationLink::action(label, move || {
        page.update(|page| *page = page.saturating_sub(1).max(1))
    })
    .disabled(move || page.get() <= 1)
}

fn next(
    page: Signal<usize>,
    last: impl Fn() -> usize + Copy + 'static,
    label: &'static str,
) -> PaginationLink {
    PaginationLink::action(label, move || {
        page.update(|page| *page = page.saturating_add(1).min(last()))
    })
    .disabled(move || page.get() >= last())
}

fn numbered(page: Signal<usize>, locale: &'static Locale) -> Pagination {
    const VISIBLE: usize = 3;
    const TOTAL: usize = 9;
    let first = move || ((page.get() - 1) / VISIBLE) * VISIBLE + 1;
    let mut previous = previous(page, locale.previous);
    let mut next = next(page, || TOTAL, locale.next);
    if locale.direction == Direction::Rtl {
        previous = previous.label(locale.previous);
        next = next.label(locale.next);
    }
    let mut content =
        PaginationContent::new().item(PaginationItem::new(PaginationPrevious::new(previous)));
    for slot in 0..VISIBLE {
        content = content.item(page_item(page, move || first() + slot, locale));
    }
    Pagination::new(
        content
            .item(PaginationItem::new(when(
                move || first() + VISIBLE <= TOTAL,
                PaginationEllipsis::new,
            )))
            .item(PaginationItem::new(PaginationNext::new(next))),
    )
    .label(locale.pagination)
}

pub fn demo() -> Pagination {
    numbered(signal(2), &LOCALES[0])
}

pub fn simple() -> Pagination {
    let page = signal(2);
    let content = (1..=5).fold(PaginationContent::new(), |content, number| {
        content.item(page_item(page, move || number, &LOCALES[0]))
    });
    Pagination::new(content).label("Numbered pages")
}

pub fn icons_only() -> View {
    let page = signal(1_usize);
    let rows = signal(Some(SharedString::from("25")));
    let last = move || {
        let per_page = rows
            .with(|rows| rows.as_ref().and_then(|size| size.parse::<usize>().ok()))
            .unwrap_or(25);
        150_usize.div_ceil(per_page)
    };
    let mut sizes = NativeSelect::new("Rows per page")
        .value(rows)
        .on_change(move |_| page.set(1));
    for size in ["10", "25", "50", "100"] {
        sizes = sizes.option(NativeSelectOption::new(size, size));
    }
    view()
        .max_w(percentage(1.0))
        .items_center()
        .gap_3()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .items_center()
                .gap_4()
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(text("Rows per page").text_sm())
                        .child(view().w(px(80.0)).child(sizes)),
                )
                .child(
                    Pagination::new(
                        PaginationContent::new()
                            .item(PaginationItem::new(PaginationPrevious::new(
                                previous(page, "Previous").icon_only(),
                            )))
                            .item(PaginationItem::new(PaginationNext::new(
                                next(page, last, "Next").icon_only(),
                            ))),
                    )
                    .label("Row pages"),
                ),
        )
        .child(muted(move || format!("Page {} of {}", page.get(), last())))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let page = signal(2);
    view()
        .max_w(percentage(1.0))
        .gap_3()
        .items_center()
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
                            .on_press(move || language.set(index))
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                    },
                )),
        )
        .child(dynamic(move || {
            let locale = &LOCALES[language.get()];
            DirectionProvider::new(locale.direction, move || numbered(page, locale))
        }))
}
