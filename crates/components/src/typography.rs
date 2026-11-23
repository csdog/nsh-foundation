//! Typography utilities layered on Native Shell text and semantic views.

use nsh_base::{table, table_body, table_cell, table_head, table_header, table_row, theme};
use nsh_core::{WebElement, prelude::*};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
}

pub fn heading(level: HeadingLevel, content: impl IntoReactive<SharedString>) -> View {
    let (number, size, leading, weight) = match level {
        HeadingLevel::H1 => (1u32, 36.0, 40.0, FontWeight::EXTRA_BOLD),
        HeadingLevel::H2 => (2, 30.0, 36.0, FontWeight::SEMIBOLD),
        HeadingLevel::H3 => (3, 24.0, 32.0, FontWeight::SEMIBOLD),
        HeadingLevel::H4 => (4, 20.0, 28.0, FontWeight::SEMIBOLD),
    };
    view()
        .w_full()
        .min_w(px(0.0))
        .when(level == HeadingLevel::H2, |heading| {
            heading
                .pb_2()
                .border_b(px(1.0))
                .border_color(|| theme().border)
        })
        .child(
            copy(text(content))
                .role(AccessibilityRole::Header)
                .accessibility_level(number)
                .text_size(px(size))
                .line_height(px(leading))
                .letter_spacing(px(-size * 0.025))
                .font_weight(weight)
                .test_id("typography-heading"),
        )
}

fn copy(content: Text) -> Text {
    let direction = use_direction();
    content
        .min_w(px(0.0))
        .text_color(|| theme().foreground)
        .style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right().writing_direction(WritingDirection::Rtl),
        )
}

pub fn paragraph(content: impl IntoReactive<SharedString>) -> Text {
    copy(text(content))
        .web_element(WebElement::P)
        .text_base()
        .line_height(px(28.0))
}

pub fn quote(content: impl IntoReactive<SharedString>) -> View {
    let direction = use_direction();
    view()
        .w_full()
        .min_w(px(0.0))
        .direction(direction.clone().map(LayoutDirection::from))
        .web_element(WebElement::Blockquote)
        .style_with(move |style| {
            if direction.get() == Direction::Rtl {
                style.border_r(px(2.0)).pr_6()
            } else {
                style.border_l(px(2.0)).pl_6()
            }
        })
        .border_color(|| theme().border)
        .test_id("typography-quote")
        .child(paragraph(content).italic())
}

pub fn list(items: impl IntoIterator<Item = &'static str>) -> View {
    let direction = use_direction();
    view()
        .w_full()
        .min_w(px(0.0))
        .direction(direction.clone().map(LayoutDirection::from))
        .role(AccessibilityRole::List)
        .web_element(WebElement::Ul)
        .style_with(move |style| {
            if direction.get() == Direction::Rtl {
                style.pr_6()
            } else {
                style.pl_6()
            }
        })
        .gap_2()
        .test_id("typography-list")
        .children(items.into_iter().map(|item| {
            view()
                .w_full()
                .min_w(px(0.0))
                .web_element(WebElement::Li)
                .flex_row()
                .items_start()
                .gap_2()
                .child(
                    text("•")
                        .text_base()
                        .line_height(px(28.0))
                        .flex_shrink_0()
                        .accessibility_elements_hidden(true)
                        .test_id("typography-bullet"),
                )
                .child(paragraph(item).flex_1())
        }))
}

pub fn inline_code(content: impl IntoReactive<SharedString>) -> View {
    view()
        .min_w(px(0.0))
        .self_start()
        .px(px(4.8))
        .py(px(3.2))
        .rounded(px(4.0))
        .bg(|| theme().muted)
        .test_id("typography-code")
        .child(
            copy(text(content))
                .text_sm()
                .font_family("monospace")
                .font_weight(FontWeight::SEMIBOLD),
        )
}

pub fn lead(content: impl IntoReactive<SharedString>) -> Text {
    paragraph(content)
        .text_xl()
        .line_height(px(28.0))
        .text_color(|| theme().muted_foreground)
}

pub fn large(content: impl IntoReactive<SharedString>) -> Text {
    copy(text(content))
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
}

pub fn small(content: impl IntoReactive<SharedString>) -> Text {
    copy(text(content))
        .text_sm()
        .line_height(px(14.0))
        .font_weight(FontWeight::MEDIUM)
}

pub fn muted(content: impl IntoReactive<SharedString>) -> Text {
    paragraph(content)
        .text_sm()
        .line_height(px(20.0))
        .text_color(|| theme().muted_foreground)
}

/// A two-column comparison with wrapped cell text and alternating row shading.
pub fn comparison(
    label: impl IntoReactive<SharedString>,
    headers: [&'static str; 2],
    rows: impl IntoIterator<Item = [&'static str; 2]>,
) -> View {
    let direction = use_direction();
    let rtl = computed({
        let direction = direction.clone();
        move || direction.get() == Direction::Rtl
    });
    let cell_style = move |view: View, last: bool| {
        view.w_full()
            .min_w(px(0.0))
            .px_4()
            .py_2()
            .style_when(move || !last && rtl.get(), |style| style.border_l(px(1.0)))
            .style_when(move || !last && !rtl.get(), |style| style.border_r(px(1.0)))
            .border_color(|| theme().border)
    };
    let label = label.into_reactive();
    view().w_full().min_w(px(0.0)).child(
        table()
            .w_full()
            .min_w(px(0.0))
            .direction(direction.map(LayoutDirection::from))
            .accessibility_label(label.map(Some))
            .border(px(1.0))
            .border_color(|| theme().border)
            .test_id("typography-table")
            .child(
                table_header().child(table_row().w_full().items_stretch().children(
                    headers.into_iter().enumerate().map(|(column, title)| {
                        table_head().flex_1().min_w(px(0.0)).child(cell_style(
                            view()
                                .child(copy(text(title)).text_base().font_weight(FontWeight::BOLD)),
                            column == 1,
                        ))
                    }),
                )),
            )
            .child(
                table_body().children(rows.into_iter().enumerate().map(|(index, values)| {
                    table_row()
                        .w_full()
                        .items_stretch()
                        .border_t(px(1.0))
                        .border_color(|| theme().border)
                        .when(index % 2 == 1, |row| row.bg(|| theme().muted))
                        .children(values.into_iter().enumerate().map(|(column, value)| {
                            table_cell().flex_1().min_w(px(0.0)).child(cell_style(
                                view().child(copy(text(value)).text_base()),
                                column == 1,
                            ))
                        }))
                })),
            ),
    )
}
