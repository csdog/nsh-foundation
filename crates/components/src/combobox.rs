//! Searchable Rhea choices using Native Shell's combobox and select primitives.

use std::rc::Rc;

use nsh_base::{SelectValue, combobox, select, theme};
use nsh_core::{AccessibilityState, element::LayoutDirection, prelude::*};

pub use nsh_base::{SelectEntry as ComboboxEntry, SelectItem as ComboboxItem};

type ItemRenderer = Rc<dyn Fn(&ComboboxEntry) -> AnyElement>;

pub struct Combobox {
    label: &'static str,
    placeholder: &'static str,
    empty: &'static str,
    items: Rc<[ComboboxItem]>,
    value: Option<Signal<Vec<SharedString>>>,
    initial: Vec<SharedString>,
    disabled: bool,
    invalid: bool,
    multiple: bool,
    show_clear: bool,
    popup: bool,
    direction: LayoutDirection,
    prefix: Option<AnyElement>,
    render_item: Option<ItemRenderer>,
}

impl Combobox {
    pub fn new(label: &'static str, items: impl IntoIterator<Item = ComboboxItem>) -> Self {
        Self {
            label,
            placeholder: label,
            empty: "No items found.",
            items: items.into_iter().collect::<Vec<_>>().into(),
            value: None,
            initial: Vec::new(),
            disabled: false,
            invalid: false,
            multiple: false,
            show_clear: false,
            popup: false,
            direction: LayoutDirection::Inherit,
            prefix: None,
            render_item: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn empty(mut self, message: &'static str) -> Self {
        self.empty = message;
        self
    }

    pub fn value(mut self, value: Signal<Vec<SharedString>>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(
        mut self,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.initial = values.into_iter().map(Into::into).collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn show_clear(mut self, show: bool) -> Self {
        self.show_clear = show;
        self
    }

    /// Uses a button as the anchor for a searchable, single-selection popup.
    pub fn popup(mut self) -> Self {
        self.popup = true;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_element());
        self
    }

    pub fn render_item<E: IntoElement>(
        mut self,
        render: impl Fn(&ComboboxEntry) -> E + 'static,
    ) -> Self {
        self.render_item = Some(Rc::new(move |item| render(item).into_element()));
        self
    }

    pub fn surface(self) -> View {
        let value = self.value.unwrap_or_else(|| signal(self.initial));
        let selected = self.items.clone();
        let direction = self.direction;
        let invalid = self.invalid;
        let disabled = self.disabled;
        let label = self.label;
        let items = self.items.to_vec();
        let item_body = self
            .render_item
            .unwrap_or_else(|| Rc::new(|item| text(item.title.clone()).text_sm().into_element()));
        let render = move |item: &ComboboxEntry| {
            let key = item.value.clone();
            view()
                .w_full()
                .min_w(px(0.0))
                .flex_row()
                .items_center()
                .gap_2()
                .child(view().flex_1().min_w(px(0.0)).child(item_body(item)))
                .child(
                    text("✓")
                        .text_sm()
                        .accessibility_elements_hidden(true)
                        .opacity(move || {
                            if value.get().contains(&key) {
                                1.0_f32
                            } else {
                                0.0
                            }
                        }),
                )
        };
        let wrapper = view()
            .w_full()
            .max_w(px(320.0))
            .min_w(px(0.0))
            .direction(direction)
            .role(AccessibilityRole::Group)
            .accessibility_state(AccessibilityState {
                invalid,
                disabled,
                ..Default::default()
            })
            .when(invalid, |wrapper| {
                wrapper.accessibility_description("Invalid selection")
            })
            .disabled(disabled)
            .disabled_style(|style| style.opacity(0.5));

        if self.popup {
            let placeholder = self.placeholder;
            let label_items = self.items.clone();
            let trigger = view()
                .w_full()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    text(move || {
                        let current = value.get();
                        current
                            .first()
                            .and_then(|choice| {
                                label_items.iter().find(|item| item.value == *choice)
                            })
                            .map_or(SharedString::from(placeholder), |item| item.title.clone())
                    })
                    .text_sm()
                    .flex_1()
                    .min_w(px(0.0)),
                )
                .child(drop_arrow());
            return wrapper.child(
                select()
                    .items(items)
                    .searchable(true)
                    .cover(false)
                    .trigger(trigger)
                    .accessibility_label(label)
                    .search_accessibility_label("Search options")
                    .search_placeholder("Search")
                    .search_class("h-8 mx-1 px-2 text-sm rounded-xl bg-input/50")
                    .value(move || SelectValue::Controlled(value.get().first().cloned()))
                    .on_change(move |choice: &Option<SharedString>| {
                        value.set(choice.iter().cloned().collect())
                    })
                    .disabled(disabled)
                    .offset(px(4.0))
                    .margin(px(8.0))
                    .popup_class(POPUP)
                    .list_class(LIST)
                    .item_class(ITEM)
                    .empty(empty_message(self.empty))
                    .render_item(move |item: &ComboboxEntry| render(item))
                    .app_class(FRAME),
            );
        }

        let mut field = combobox()
            .items(items)
            .accessibility_label(label)
            .placeholder(self.placeholder)
            .value(move || Some(value.get()))
            .on_change(move |next: &Vec<SharedString>| value.set(next.clone()))
            .disabled(disabled)
            .multiple(self.multiple)
            .cleanable(self.show_clear)
            .clear_button_accessibility_label("Clear selection")
            .clear_button_class("size-6 items-center justify-center rounded-md hover:bg-muted")
            .clear_button(
                text("×")
                    .text_lg()
                    .text_color(move || theme().muted_foreground),
            )
            .offset(px(4.0))
            .margin(px(8.0))
            .popup_class(POPUP)
            .list_class(LIST)
            .item_class(ITEM)
            .group_class("border-t border-border px-2 py-1.5 text-xs text-muted-foreground")
            .input_class(
                "h-7 min-w-10 flex-1 text-sm text-foreground placeholder:text-muted-foreground",
            )
            .empty(empty_message(self.empty))
            .render_item(move |item: &ComboboxEntry| render(item))
            .app_class(FRAME)
            .when(invalid, |field| {
                field
                    .border_color(move || theme().destructive)
                    .ring(px(3.0))
                    .ring_color(move || theme().destructive.opacity(0.2))
            });

        if self.multiple {
            field = field.flex_wrap().prefix(
                view()
                    .flex_row()
                    .flex_wrap()
                    .min_w(px(0.0))
                    .gap_1()
                    .items_center()
                    .child(dynamic(move || {
                        value
                            .get()
                            .into_iter()
                            .filter_map(|key| {
                                let item = selected.iter().find(|item| item.value == key)?;
                                Some(chip(item.title.clone(), key, value))
                            })
                            .collect::<Vec<_>>()
                    })),
            );
        } else {
            field = field
                .trigger_button(
                    view()
                        .size(px(24.0))
                        .items_center()
                        .justify_center()
                        .child(drop_arrow()),
                )
                .when_some(self.prefix, |field, prefix| field.prefix(prefix));
        }
        wrapper.child(field)
    }
}

impl IntoElement for Combobox {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

const FRAME: &str = "w-full min-w-0 min-h-8 flex-row items-center gap-1 rounded-2xl border border-transparent bg-input/50 px-2 focus-within:ring-3 focus-within:ring-ring/30";
const POPUP: &str = "min-w-36 max-h-72 rounded-2xl bg-popover text-popover-foreground shadow-lg ring-1 ring-foreground/10";
const LIST: &str = "max-h-64 p-1";
const ITEM: &str = "min-h-7 rounded-xl px-2 py-1.5 text-sm focus:bg-accent focus:text-accent-foreground disabled:opacity-50";

fn empty_message(message: &'static str) -> Text {
    text(message)
        .text_sm()
        .text_center()
        .py_2()
        .text_color(move || theme().muted_foreground)
}

fn drop_arrow() -> Text {
    text("⌄")
        .text_base()
        .text_color(move || theme().muted_foreground)
}

fn chip(title: SharedString, key: SharedString, value: Signal<Vec<SharedString>>) -> View {
    view()
        .flex_row()
        .items_center()
        .gap_1()
        .h(px(22.0))
        .pl(px(6.0))
        .pr(px(2.0))
        .rounded_full()
        .bg(move || theme().input)
        .child(
            text(title.clone())
                .text_xs()
                .font_weight(FontWeight::MEDIUM),
        )
        .child(
            view()
                .label(format!("Remove {title}"))
                .role(AccessibilityRole::Button)
                .focusable(true)
                .size(px(18.0))
                .items_center()
                .justify_center()
                .rounded_full()
                .hover(|style| style.bg(theme().muted))
                .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                .on_press(move || value.update(|values| values.retain(|item| *item != key)))
                .child(
                    text("×")
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                ),
        )
}
