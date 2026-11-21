//! A paginated table for typed Rust records and composable Native Shell cells.

use std::{cmp::Ordering, collections::BTreeSet, rc::Rc};

use nsh_base::{
    input, menu, menu_item, table, table_body, table_cell, table_head, table_header, table_row,
    theme,
};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    checkbox::{Checkbox, CheckboxState},
};

type Cell<T> = Box<dyn Fn(&T) -> AnyElement>;
type Compare<T> = fn(&T, &T) -> Ordering;

pub struct DataTableColumn<T> {
    label: &'static str,
    width: f32,
    flexible: bool,
    end_aligned: bool,
    hideable: bool,
    cell: Cell<T>,
    compare: Option<Compare<T>>,
}

impl<T> DataTableColumn<T> {
    pub fn new<E: IntoElement>(
        label: &'static str,
        width: f32,
        cell: impl Fn(&T) -> E + 'static,
    ) -> Self {
        Self {
            label,
            width,
            flexible: false,
            end_aligned: false,
            hideable: true,
            cell: Box::new(move |row| cell(row).into_element()),
            compare: None,
        }
    }

    /// Takes an equal share of any space beyond the columns' minimum widths.
    pub fn flexible(mut self) -> Self {
        self.flexible = true;
        self
    }

    pub fn align_end(mut self) -> Self {
        self.end_aligned = true;
        self
    }

    pub fn always_visible(mut self) -> Self {
        self.hideable = false;
        self
    }

    pub fn sortable(mut self, compare: Compare<T>) -> Self {
        self.compare = Some(compare);
        self
    }

    fn layout<E: Element>(&self, cell: E) -> E {
        cell.min_w(px(self.width))
            .when(self.flexible, |cell| cell.flex_1())
            .when(!self.flexible, |cell| {
                cell.w(px(self.width)).flex_shrink_0()
            })
            .when(self.end_aligned, |cell| cell.items_end())
            .when(!self.end_aligned, |cell| cell.items_start())
    }
}

#[derive(Clone, Copy)]
pub struct DataTableLabels {
    pub columns: &'static str,
    pub select_all: &'static str,
    pub select_row: &'static str,
    pub empty: &'static str,
    pub selection_count: fn(usize, usize) -> String,
    pub previous: &'static str,
    pub next: &'static str,
}

impl Default for DataTableLabels {
    fn default() -> Self {
        Self {
            columns: "Columns",
            select_all: "Select all",
            select_row: "Select row",
            empty: "No results.",
            selection_count: |selected, total| format!("{selected} of {total} row(s) selected."),
            previous: "Previous",
            next: "Next",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Sort {
    column: usize,
    descending: bool,
}

/// The supplied records stay in their original order and keep their identities
/// across sorting, filtering, paging and column changes.
pub struct DataTable<T> {
    name: &'static str,
    rows: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    filter: Option<(&'static str, fn(&T) -> &str)>,
    page_size: usize,
    direction: LayoutDirection,
    labels: DataTableLabels,
}

impl<T> DataTable<T> {
    pub fn new(name: &'static str, rows: Vec<T>, columns: Vec<DataTableColumn<T>>) -> Self {
        Self {
            name,
            rows,
            columns,
            filter: None,
            page_size: 10,
            direction: LayoutDirection::Ltr,
            labels: DataTableLabels::default(),
        }
    }

    pub fn filter(mut self, label: &'static str, text: fn(&T) -> &str) -> Self {
        self.filter = Some((label, text));
        self
    }

    pub fn page_size(mut self, rows: usize) -> Self {
        self.page_size = rows.max(1);
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn labels(mut self, labels: DataTableLabels) -> Self {
        self.labels = labels;
        self
    }
}

impl<T: 'static> IntoElement for DataTable<T> {
    fn into_element(self) -> AnyElement {
        let Self {
            name,
            rows,
            columns,
            filter,
            page_size,
            direction,
            labels,
        } = self;
        let rows = Rc::new(rows);
        let columns = Rc::new(columns);
        let hidden = signal(BTreeSet::<usize>::new());
        let selected = signal(BTreeSet::<usize>::new());
        let query = signal(String::new());
        let page = signal(0_usize);
        let sort = signal(None::<Sort>);
        let filtered = computed({
            let rows = rows.clone();
            let columns = columns.clone();
            move || {
                let query = query.get().to_lowercase();
                let mut indices: Vec<_> = (0..rows.len()).collect();
                if let Some((_, text)) = filter.filter(|_| !query.is_empty()) {
                    indices.retain(|&row| text(&rows[row]).to_lowercase().contains(&query));
                }
                if let Some(Sort { column, descending }) = sort.get()
                    && let Some(compare) = columns[column].compare
                {
                    indices.sort_by(|&left, &right| {
                        let order = compare(&rows[left], &rows[right]);
                        if descending { order.reverse() } else { order }
                    });
                }
                indices
            }
        });
        let visible_rows = computed(move || {
            filtered.with(|indices| {
                indices
                    .iter()
                    .copied()
                    .skip(page.get() * page_size)
                    .take(page_size)
                    .collect::<Vec<_>>()
            })
        });
        let select_page = Checkbox::new(labels.select_all)
            .icon_only()
            .disabled(move || visible_rows.with(Vec::is_empty))
            .checked(move || {
                visible_rows.with(|rows| {
                    let count = selected
                        .with(|selected| rows.iter().filter(|row| selected.contains(row)).count());
                    match count {
                        0 => CheckboxState::False,
                        n if n == rows.len() => CheckboxState::True,
                        _ => CheckboxState::Mixed,
                    }
                })
            })
            .on_change(move |checked| {
                selected.update(|selected| {
                    for id in visible_rows.get() {
                        set_membership(selected, id, checked);
                    }
                })
            });
        let header = table_row()
            .direction(direction)
            .items_center()
            .min_h(px(40.0))
            .border_b(px(1.0))
            .border_color(move || theme().border)
            .child(
                table_head()
                    .w_10()
                    .flex_shrink_0()
                    .items_center()
                    .child(select_page),
            )
            .children(columns.iter().enumerate().map(|(index, column)| {
                let label = column.label;
                let sort_button = || {
                    Button::new(label)
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Ghost)
                        .icon_end(
                            text(move || match sort.get() {
                                Some(Sort {
                                    column,
                                    descending: true,
                                }) if column == index => "↓",
                                Some(Sort {
                                    column,
                                    descending: false,
                                }) if column == index => "↑",
                                _ => "↕",
                            })
                            .text_sm()
                            .accessibility_elements_hidden(true),
                        )
                        .on_press(move || {
                            sort.update(|value| {
                                let descending = value
                                    .is_some_and(|last| last.column == index && !last.descending);
                                *value = Some(Sort {
                                    column: index,
                                    descending,
                                });
                            });
                            page.set(0);
                        })
                };
                let content = if column.compare.is_some() {
                    sort_button().into_element()
                } else {
                    text(label)
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .into_element()
                };
                column
                    .layout(table_head().px_2().child(content))
                    .style_when(
                        move || hidden.with(|hidden| hidden.contains(&index)),
                        |style| style.hidden(),
                    )
            }));

        let body = table_body()
            .child(each(move || visible_rows.get(), |id| *id, {
                let columns = columns.clone();
                move |id, index| {
                    let id = id.get_untracked();
                    let checked = move || selected.with(|selected| selected.contains(&id));
                    view()
                        .role(AccessibilityRole::Row)
                        .flex_row()
                        .direction(direction)
                        .items_center()
                        .min_h(px(44.0))
                        .border_color(move || theme().border)
                        .style_when(
                            move || index.get() + 1 < visible_rows.with(Vec::len),
                            |style| style.border_b(px(1.0)),
                        )
                        .hover(|style| style.bg(theme().muted.opacity(0.5)))
                        .style_when(checked, |style| style.bg(theme().muted))
                        .accessibility_state(move || AccessibilityState {
                            selected: checked(),
                            ..Default::default()
                        })
                        .child(
                            table_cell().w_10().flex_shrink_0().items_center().child(
                                Checkbox::new(labels.select_row)
                                    .icon_only()
                                    .checked(move || {
                                        if checked() {
                                            CheckboxState::True
                                        } else {
                                            CheckboxState::False
                                        }
                                    })
                                    .on_change(move |checked| {
                                        selected.update(|set| set_membership(set, id, checked))
                                    }),
                            ),
                        )
                        .children(columns.iter().enumerate().map(|(index, column)| {
                            column
                                .layout(table_cell().px_2().child((column.cell)(&rows[id])))
                                .style_when(
                                    move || hidden.with(|hidden| hidden.contains(&index)),
                                    |style| style.hidden(),
                                )
                        }))
                }
            }))
            .child(when(
                move || visible_rows.with(Vec::is_empty),
                move || {
                    table_row()
                        .h_24()
                        .items_center()
                        .justify_center()
                        .child(table_cell().child(text(labels.empty).text_sm()))
                },
            ));

        let width = signal(0.0_f32);
        let table_width = {
            let columns = columns.clone();
            move || {
                let minimum = hidden.with(|hidden| {
                    40.0 + columns
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !hidden.contains(i))
                        .map(|(_, column)| column.width)
                        .sum::<f32>()
                });
                px(width.get().max(minimum))
            }
        };
        let column_menu = menu()
            .margin(px(8.0))
            .align(Align::End)
            .offset(px(4.0))
            .popup_class("w-44 p-1 rounded-2xl border bg-popover text-popover-foreground shadow-lg")
            .trigger(
                Button::new(labels.columns)
                    .variant(ButtonVariant::Outline)
                    .icon_end(text("⌄").accessibility_elements_hidden(true))
                    .surface(),
            )
            .children(
                columns
                    .iter()
                    .enumerate()
                    .filter(|(_, column)| column.hideable)
                    .map(|(index, column)| {
                        let shown = move || hidden.with(|hidden| !hidden.contains(&index));
                        menu_item()
                            .accessibility_label(column.label)
                            .checked(shown)
                            .close_on_select(false)
                            .on_select(move || {
                                let shown = shown();
                                hidden.update(|hidden| set_membership(hidden, index, shown));
                            })
                            .min_h(px(32.0))
                            .rounded(px(10.0))
                            .px_2()
                            .justify_center()
                            .hover(|style| style.bg(theme().accent))
                            .focus(|style| style.bg(theme().accent))
                            .child(
                                view()
                                    .direction(direction)
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(text(column.label).text_sm().flex_1())
                                    .child(
                                        text(move || if shown() { "✓" } else { "" })
                                            .w(px(16.0))
                                            .text_sm()
                                            .accessibility_elements_hidden(true),
                                    ),
                            )
                    }),
            );

        view()
            .w_full()
            .direction(direction)
            .gap_3()
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .child(filter.map(|(label, _)| {
                        input()
                            .label(label)
                            .placeholder(label)
                            .value(move || query.get())
                            .flex_1()
                            .min_w(px(0.0))
                            .max_w(px(384.0))
                            .h(px(36.0))
                            .px_3()
                            .rounded(px(10.0))
                            .border(px(1.0))
                            .border_color(move || theme().input)
                            .text_color(move || theme().foreground)
                            .on_change(move |value: &str| {
                                query.set(value.to_owned());
                                page.set(0);
                            })
                    }))
                    .child(column_menu),
            )
            .child(
                view()
                    .w_full()
                    .rounded(px(10.0))
                    .border(px(1.0))
                    .border_color(move || theme().border)
                    .overflow_hidden()
                    .child(
                        scroll()
                            .horizontal()
                            .w_full()
                            .on_layout(move |bounds: &Bounds<Points>| {
                                width.set(bounds.size.width.0)
                            })
                            .child(
                                table()
                                    .accessibility_label(name)
                                    .direction(direction)
                                    .w(table_width)
                                    .child(table_header().child(header))
                                    .child(body),
                            ),
                    ),
            )
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .child(
                        text(move || {
                            filtered.with(|all| {
                                let count = selected
                                    .with(|set| all.iter().filter(|id| set.contains(id)).count());
                                (labels.selection_count)(count, all.len())
                            })
                        })
                        .text_sm()
                        .text_color(move || theme().muted_foreground),
                    )
                    .child(
                        view()
                            .flex_row()
                            .gap_2()
                            .child(
                                Button::new(labels.previous)
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Small)
                                    .disabled(move || page.get() == 0)
                                    .on_press(move || {
                                        page.update(|page| *page = page.saturating_sub(1))
                                    }),
                            )
                            .child(
                                Button::new(labels.next)
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Small)
                                    .disabled(move || {
                                        (page.get() + 1) * page_size >= filtered.with(Vec::len)
                                    })
                                    .on_press(move || page.update(|page| *page += 1)),
                            ),
                    ),
            )
            .into_element()
    }
}

fn set_membership(set: &mut BTreeSet<usize>, id: usize, included: bool) {
    if included {
        set.insert(id);
    } else {
        set.remove(&id);
    }
}
