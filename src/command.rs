//! Rhea command palettes backed by Native Shell's searchable list.

use nsh_base::{command, dialog, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

pub use nsh_base::{CommandEntry, CommandItem};

type ItemRenderer = Box<dyn Fn(&CommandEntry) -> AnyElement>;
type SelectHandler = Box<dyn Fn(&str)>;

pub struct Command {
    items: Vec<CommandItem>,
    label: &'static str,
    placeholder: &'static str,
    empty: &'static str,
    query: Option<Signal<SharedString>>,
    direction: LayoutDirection,
    select: Option<SelectHandler>,
    render_item: Option<ItemRenderer>,
}

impl Command {
    pub fn new(label: &'static str, items: impl IntoIterator<Item = CommandItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            label,
            placeholder: "Type a command or search...",
            empty: "No results found.",
            query: None,
            direction: LayoutDirection::Inherit,
            select: None,
            render_item: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn empty(mut self, empty: &'static str) -> Self {
        self.empty = empty;
        self
    }

    pub fn query(mut self, query: Signal<SharedString>) -> Self {
        self.query = Some(query);
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Reports the selected item's stable id.
    pub fn on_select(mut self, handler: impl Fn(&str) + 'static) -> Self {
        self.select = Some(Box::new(handler));
        self
    }

    /// Replaces the item body. The shortcut hint still appears after it.
    pub fn render_item<E: IntoElement>(
        mut self,
        render: impl Fn(&CommandEntry) -> E + 'static,
    ) -> Self {
        self.render_item = Some(Box::new(move |item| render(item).into_element()));
        self
    }

    pub fn surface(self) -> View {
        self.render(None)
    }

    fn render(self, dialog_open: Option<Signal<bool>>) -> View {
        let in_dialog = dialog_open.is_some();
        let direction = self.direction;
        let list = command()
            .items(self.items)
            .accessibility_label(self.label)
            .search_accessibility_label(self.label)
            .placeholder(self.placeholder)
            .auto_focus(in_dialog)
            .when_some(self.query, |palette, query| {
                palette
                    .query(move || Some(query.get()))
                    .on_query_change(move |value: &SharedString| query.set(value.clone()))
            })
            .search_bar_class(
                "mx-1 mt-1 mb-1.5 h-8 items-center gap-2 rounded-2xl bg-input/50 px-2",
            )
            .search_class(
                "h-8 min-w-0 flex-1 text-sm text-foreground placeholder:text-muted-foreground",
            )
            .search_prefix(
                text("⌕")
                    .text_base()
                    .text_color(move || theme().muted_foreground),
            )
            .list_class("max-h-72 px-1 pb-1")
            .item_class("min-h-7 rounded-xl px-2 py-1.5 focus:bg-muted disabled:opacity-50")
            .group_class("px-2 py-1.5 text-xs text-muted-foreground")
            .separator_class("my-1 h-px bg-border/50")
            .empty(
                text(self.empty)
                    .py_6()
                    .text_sm()
                    .text_center()
                    .text_color(move || theme().muted_foreground),
            )
            .render_item(move |item: &CommandEntry| {
                let label = self.render_item.as_ref().map_or_else(
                    || text(item.label.clone()).text_sm().into_element(),
                    |render| render(item),
                );
                view()
                    .w_full()
                    .min_w(px(0.0))
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(view().flex_1().min_w(px(0.0)).child(label))
                    .child(item.shortcut_label.clone().map(|keys| {
                        text(keys)
                            .text_xs()
                            .font_family("monospace")
                            .flex_shrink_0()
                            .text_color(move || theme().muted_foreground)
                    }))
            })
            .on_select(move |id: &SharedString| {
                if let Some(handler) = &self.select {
                    handler(id.as_str());
                }
                if let Some(open) = dialog_open {
                    open.set(false);
                }
            })
            .on_cancel(move || {
                if let Some(open) = dialog_open {
                    open.set(false);
                }
            });

        view()
            .direction(direction)
            .w_full()
            .min_w(px(0.0))
            .rounded(px(24.0))
            .border(px(1.0))
            .border_color(move || theme().border)
            .bg(move || theme().popover)
            .text_color(move || theme().popover_foreground)
            .child(list)
    }
}

impl IntoElement for Command {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct CommandDialog {
    open: Signal<bool>,
    content: Box<dyn Fn() -> Command>,
    shortcut: Option<&'static str>,
}

impl CommandDialog {
    pub fn new(open: Signal<bool>, content: impl Fn() -> Command + 'static) -> Self {
        Self {
            open,
            content: Box::new(content),
            shortcut: None,
        }
    }

    /// Binds a Native Shell keystroke, such as `mod+j`, to toggle visibility.
    pub fn shortcut(mut self, keys: &'static str) -> Self {
        self.shortcut = Some(keys);
        self
    }
}

impl IntoElement for CommandDialog {
    fn into_element(self) -> AnyElement {
        let open = self.open;
        if let Some(keys) = self.shortcut {
            effect(move || {
                if !open.get() {
                    on_key(keys, move |_| open.set(true));
                }
            });
        }
        when(open, move || {
            if let Some(keys) = self.shortcut {
                on_key(keys, move |_| open.set(false));
            }
            dialog()
                .label("Command Palette")
                .accessibility_description("Search for a command to run")
                .animation(ModalAnimation::Fade)
                .close_on_escape(false)
                .close_on_swipe(false)
                .on_cancel(move || open.set(false))
                .child(
                    view()
                        .absolute()
                        .inset_0()
                        .bg(hsla(0.0, 0.0, 0.0, 0.30))
                        .backdrop_blur(px(4.0)),
                )
                .child(
                    view()
                        .absolute()
                        .inset_0()
                        .items_center()
                        .justify_center()
                        .pointer_events(PointerEvents::BoxNone)
                        .child(
                            view()
                                .w(move || px((window().width.0 - 32.0).clamp(0.0, 512.0)))
                                .test_id("command-dialog-panel")
                                .child((self.content)().render(Some(open))),
                        ),
                )
        })
        .into_element()
    }
}
