//! Composable tables with shared column widths and Native Shell table roles.

use nsh_base::{
    table, table_body, table_caption, table_cell, table_footer, table_head, table_header,
    table_row, theme,
};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy)]
pub struct TableColumn {
    minimum: f32,
    flexible: bool,
}

impl TableColumn {
    pub fn fixed(width: f32) -> Self {
        Self {
            minimum: if width.is_finite() {
                width.max(0.0)
            } else {
                0.0
            },
            flexible: false,
        }
    }

    pub fn flexible(minimum: f32) -> Self {
        Self {
            minimum: if minimum.is_finite() {
                minimum.max(0.0)
            } else {
                0.0
            },
            flexible: true,
        }
    }
}

pub struct Table {
    label: Reactive<SharedString>,
    columns: Vec<TableColumn>,
    header: Option<TableHeader>,
    body: Option<TableBody>,
    footer: Option<TableFooter>,
    caption: Option<TableCaption>,
}

impl Table {
    pub fn new(
        label: impl IntoReactive<SharedString>,
        columns: impl IntoIterator<Item = TableColumn>,
    ) -> Self {
        Self {
            label: label.into_reactive(),
            columns: columns.into_iter().collect(),
            header: None,
            body: None,
            footer: None,
            caption: None,
        }
    }

    pub fn header(mut self, header: TableHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, body: TableBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn footer(mut self, footer: TableFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn caption(mut self, caption: TableCaption) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn surface(self) -> Scroll {
        let direction = use_direction();
        let rtl = direction.clone();
        let minimum: f32 = self.columns.iter().map(|column| column.minimum).sum();
        let flexible = self.columns.iter().filter(|column| column.flexible).count();
        let available = signal(minimum);
        let table_width = computed(move || available.get().max(minimum));
        let share = computed(move || (table_width.get() - minimum) / flexible.max(1) as f32);
        let layout = Columns {
            share,
            values: self.columns,
        };
        let header = self.header.map(|header| {
            let mut group = table_header().test_id("table-header");
            for row in header.rows {
                group = group.child(row.render(&layout).min_h(px(40.0)).border_b(px(1.0)));
            }
            group
        });
        let body = self.body.map(|body| {
            let last = body.rows.len().saturating_sub(1);
            let mut group = table_body().test_id("table-body");
            for (index, row) in body.rows.into_iter().enumerate() {
                group = group.child(
                    row.render(&layout)
                        .when(index != last, |row| row.border_b(px(1.0))),
                );
            }
            group
        });
        let footer = self.footer.map(|footer| {
            let mut group = table_footer()
                .test_id("table-footer")
                .border_t(px(1.0))
                .border_color(|| theme().border)
                .bg(|| theme().muted.opacity(0.5))
                .font_weight(FontWeight::MEDIUM);
            for row in footer.rows {
                group = group.child(row.render(&layout));
            }
            group
        });
        let caption = self.caption.map(|caption| {
            table_caption()
                .test_id("table-caption")
                .mt_4()
                .items_center()
                .child(
                    text(caption.0)
                        .text_sm()
                        .text_color(|| theme().muted_foreground),
                )
        });

        let viewport = element_ref();
        let aligned = stored(None);
        watch_scroll(
            move || viewport.node(),
            move |event: &ScrollEvent| {
                let overflow = event.content_size.width - event.size.width;
                let rtl = rtl.get() == Direction::Rtl;
                if event.size.width.0 > 0.0 && overflow.0 > 0.0 && aligned.get() != Some(rtl) {
                    aligned.replace(Some(rtl));
                    viewport.scroll_to(point(if rtl { overflow } else { px(0.0) }, px(0.0)), false);
                }
            },
        );
        let name = self.label.clone();
        scroll()
            .horizontal()
            .direction(LayoutDirection::Ltr)
            .w_full()
            .min_w(px(0.0))
            .focusable(true)
            .role(AccessibilityRole::Group)
            .label(move || format!("Scroll {}", name.get()))
            .focus_visible(|s| {
                s.ring_inset()
                    .ring(px(2.0))
                    .ring_color(theme().ring.opacity(0.5))
            })
            .element_ref(&viewport)
            .on_layout(move |bounds: &Bounds<Points>| available.set(bounds.size.width.0))
            .test_id("table-viewport")
            .child(
                table()
                    .accessibility_label(self.label.map(Some))
                    .direction(direction.map(LayoutDirection::from))
                    .w(move || px(table_width.get()))
                    .test_id("table")
                    .child(header)
                    .child(body)
                    .child(footer)
                    .child(caption),
            )
    }
}

impl IntoElement for Table {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

macro_rules! row_group {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Default)]
        pub struct $name {
            rows: Vec<TableRow>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn row(mut self, row: TableRow) -> Self {
                self.rows.push(row);
                self
            }

            pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow>) -> Self {
                self.rows.extend(rows);
                self
            }
        }
    )+};
}

row_group!(TableHeader, TableBody, TableFooter);

pub struct TableCaption(Reactive<SharedString>);

impl TableCaption {
    pub fn new(text: impl IntoReactive<SharedString>) -> Self {
        Self(text.into_reactive())
    }
}

struct Columns {
    share: Memo<f32>,
    values: Vec<TableColumn>,
}

#[derive(Default)]
pub struct TableRow {
    cells: Vec<TableCell>,
    selected: Option<Reactive<bool>>,
    expanded: Option<Reactive<bool>>,
}

impl TableRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn head(self, cell: TableHead) -> Self {
        self.cell(cell.0)
    }

    pub fn cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }

    pub fn selected(mut self, selected: impl IntoReactive<bool>) -> Self {
        self.selected = Some(selected.into_reactive());
        self
    }

    /// Highlight a row while its popover or menu is open.
    pub fn expanded(mut self, expanded: impl IntoReactive<bool>) -> Self {
        self.expanded = Some(expanded.into_reactive());
        self
    }

    fn render(self, layout: &Columns) -> nsh_base::TableRowElement {
        let mut start = 0usize;
        let selected = self.selected.unwrap_or_else(|| false.into_reactive());
        let expanded = self.expanded.unwrap_or_else(|| false.into_reactive());
        let row = table_row()
            .w_full()
            .min_h(px(36.0))
            .items_stretch()
            .border_color(|| theme().border)
            .hover(|style| style.bg(theme().muted.opacity(0.5)))
            .style_when(expanded, |style| style.bg(theme().muted.opacity(0.5)))
            .style_when(selected, |style| style.bg(theme().muted))
            .style_when(
                || !reduce_motion(),
                |style| style.transition_colors().duration(150.0),
            );
        let share = layout.share;
        row.children(self.cells.into_iter().map(|cell| {
            let end = start.saturating_add(cell.span).min(layout.values.len());
            let columns = &layout.values[start..end];
            start = end;
            let minimum: f32 = columns.iter().map(|column| column.minimum).sum();
            let weight = columns.iter().filter(|column| column.flexible).count() as f32;
            cell.render(move || px(minimum + share.get() * weight))
        }))
    }
}

pub struct TableHead(TableCell);

impl TableHead {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self(TableCell::text(label).medium().header())
    }

    pub fn align_end(mut self) -> Self {
        self.0 = self.0.align_end();
        self
    }
}

pub struct TableCell {
    content: AnyElement,
    header: bool,
    medium: bool,
    end: bool,
    span: usize,
}

impl TableCell {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_element(),
            header: false,
            medium: false,
            end: false,
            span: 1,
        }
    }

    pub fn text(value: impl IntoReactive<SharedString>) -> Self {
        Self::new(text(value).text_sm().number_of_lines(1))
    }

    pub fn medium(mut self) -> Self {
        self.medium = true;
        self
    }

    pub fn align_end(mut self) -> Self {
        self.end = true;
        self
    }

    /// Share the width of the next count columns.
    pub fn span(mut self, count: usize) -> Self {
        self.span = count.max(1);
        self
    }

    fn header(mut self) -> Self {
        self.header = true;
        self
    }

    fn render(self, width: impl Fn() -> Points + Copy + 'static) -> AnyElement {
        let content = view()
            .px_2()
            .when(!self.header, |content| content.py_2())
            .when(self.medium, |content| {
                content.font_weight(FontWeight::MEDIUM)
            })
            .min_h(px(if self.header { 40.0 } else { 36.0 }))
            .justify_center()
            .when(self.end, |content| content.items_end())
            .when(!self.end, |content| content.items_start())
            .child(self.content);
        if self.header {
            table_head()
                .w(width)
                .flex_shrink_0()
                .justify_center()
                .child(content)
                .into_element()
        } else {
            table_cell()
                .w(width)
                .flex_shrink_0()
                .justify_center()
                .child(content)
                .into_element()
        }
    }
}
