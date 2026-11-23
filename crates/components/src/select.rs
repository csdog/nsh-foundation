//! Styled select composition over Native Shell's listbox and touch sheet.

use nsh_base::{
    SelectEntry, SelectEvent, SelectGroup as NativeGroup, SelectItem as NativeItem, select, theme,
};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::direction::{Direction, use_direction};

pub struct SelectValue(SharedString);

impl SelectValue {
    pub fn new(placeholder: impl Into<SharedString>) -> Self {
        Self(placeholder.into())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectTriggerSize {
    #[default]
    Default,
    Small,
}

pub struct SelectTrigger {
    value: SelectValue,
    size: SelectTriggerSize,
}

impl SelectTrigger {
    pub fn new(value: SelectValue) -> Self {
        Self {
            value,
            size: SelectTriggerSize::Default,
        }
    }

    pub fn size(mut self, size: SelectTriggerSize) -> Self {
        self.size = size;
        self
    }
}

pub struct SelectItem(NativeItem);

impl SelectItem {
    pub fn new(value: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self(NativeItem::new(value, title))
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }
}

pub struct SelectGroup {
    label: SharedString,
    items: Vec<SelectItem>,
}

impl SelectGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }

    pub fn item(mut self, item: SelectItem) -> Self {
        self.items.push(item);
        self
    }
}

pub struct SelectContent {
    items: Vec<NativeItem>,
    cover: Reactive<bool>,
    align: Align,
    placement: Placement,
    max_height: f32,
}

impl Default for SelectContent {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            cover: true.into_reactive(),
            align: Align::Start,
            placement: Placement::Bottom,
            max_height: 320.0,
        }
    }
}

impl SelectContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: SelectItem) -> Self {
        self.items.push(item.0);
        self
    }

    /// Insert an identified group. Groups are separated by a rule.
    pub fn group(mut self, group: SelectGroup) -> Self {
        self.items.extend(
            group
                .items
                .into_iter()
                .map(|item| item.0.group(group.label.clone())),
        );
        self
    }

    pub fn align_item_with_trigger(mut self, align_item: impl IntoReactive<bool>) -> Self {
        self.cover = align_item.into_reactive();
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn max_height(mut self, points: f32) -> Self {
        self.max_height = points;
        self
    }
}

type OnChange = Box<dyn Fn(Option<SharedString>)>;

pub struct Select {
    label: SharedString,
    trigger: SelectTrigger,
    content: SelectContent,
    value: Option<Signal<Option<SharedString>>>,
    default_value: Option<SharedString>,
    open: Option<Signal<bool>>,
    disabled: Reactive<bool>,
    invalid: Reactive<bool>,
    on_change: Option<OnChange>,
}

impl Select {
    pub fn new(
        label: impl Into<SharedString>,
        trigger: SelectTrigger,
        content: SelectContent,
    ) -> Self {
        Self {
            label: label.into(),
            trigger,
            content,
            value: None,
            default_value: None,
            open: None,
            disabled: false.into_reactive(),
            invalid: false.into_reactive(),
            on_change: None,
        }
    }

    pub fn value(mut self, value: Signal<Option<SharedString>>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.open = Some(open);
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn invalid(mut self, invalid: impl IntoReactive<bool>) -> Self {
        self.invalid = invalid.into_reactive();
        self
    }

    pub fn on_change(mut self, on_change: impl Fn(Option<SharedString>) + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl IntoElement for Select {
    fn into_element(self) -> AnyElement {
        let value = self.value.unwrap_or_else(|| signal(self.default_value));
        let open = self.open;
        let invalid = computed(move || self.invalid.get());
        let direction = use_direction();
        let SelectContent {
            items,
            cover,
            align,
            placement,
            max_height,
        } = self.content;
        let placeholder = self.trigger.value.0;
        let labels = items.clone();
        let first_group = items.iter().find_map(|item| item.group.clone());
        let chosen = computed(move || {
            value.with(|value| {
                labels
                    .iter()
                    .find(|item| Some(&item.value) == value.as_ref())
                    .map(|item| item.title.clone())
            })
        });
        let title = {
            let placeholder = placeholder.clone();
            move || chosen.get().unwrap_or_else(|| placeholder.clone())
        };
        let trigger_direction = direction.clone();
        let item_direction = direction.clone();
        let group_direction = direction.clone();
        let popup_direction = direction.clone();
        let trigger_width = signal(px(144.0));
        let desktop = is_desktop();
        let cover_animation = cover.clone();
        let animation = move || {
            if reduce_motion() || cover_animation.get() {
                SharedString::from("")
            } else {
                SharedString::from("opacity-0 scale-95")
            }
        };
        let height = match self.trigger.size {
            SelectTriggerSize::Default => 32.0,
            SelectTriggerSize::Small => 28.0,
        };
        let trigger = view()
            .test_id("select-trigger")
            .direction(trigger_direction.clone().map(LayoutDirection::from))
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(height))
            .w_full()
            .min_w(px(0.0))
            .px(px(12.0))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || {
                if invalid.get() {
                    theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .bg(move || theme().input.opacity(0.5))
            .transition_colors()
            .duration(200.0)
            .disabled_style(|style| style.opacity(0.5))
            .focus_visible(move |style| {
                style
                    .border_color(theme().ring)
                    .ring(px(3.0))
                    .ring_color(if invalid.get() {
                        theme().destructive.opacity(if dark() { 0.4 } else { 0.2 })
                    } else {
                        theme().ring.opacity(0.3)
                    })
            })
            .style_when(invalid, |style| {
                style
                    .ring(px(3.0))
                    .ring_color(theme().destructive.opacity(if dark() { 0.4 } else { 0.2 }))
            })
            .accessibility_state(move || AccessibilityState {
                invalid: invalid.get(),
                ..Default::default()
            })
            .child(
                text(title)
                    .test_id("select-value")
                    .min_w(px(0.0))
                    .flex_1()
                    .truncate()
                    .text_sm()
                    .selectable(false)
                    .style_when(
                        move || trigger_direction.get() == Direction::Rtl,
                        |style| style.text_right(),
                    )
                    .text_color(move || {
                        if chosen.get().is_none() {
                            theme().muted_foreground
                        } else {
                            theme().foreground
                        }
                    }),
            )
            .child(
                text("⌄")
                    .test_id("select-chevron")
                    .text_size(px(16.0))
                    .flex_shrink_0()
                    .text_color(move || theme().muted_foreground)
                    .accessibility_elements_hidden(true),
            );
        view().min_w(px(0.0)).w_full().on_layout(move |bounds: &Bounds<Points>| {
            if bounds.size.width.0 > 0.0 && trigger_width.get_untracked() != bounds.size.width {
                trigger_width.set(bounds.size.width);
            }
        }).child(
            select()
                .direction(direction.map(LayoutDirection::from))
                .accessibility_label(self.label)
                .placeholder(placeholder)
                .value(move || value.get())
                .open(move || open.map(|open| open.get()))
                .on_open_change(move |event: &SelectEvent| {
                    if let (Some(open), SelectEvent::OpenChange(next, _)) = (open, event) {
                        open.set(*next);
                    }
                })
                .disabled(self.disabled)
                .on_change(move |next: &Option<SharedString>| {
                    value.set(next.clone());
                    if let Some(change) = &self.on_change {
                        change(next.clone());
                    }
                })
                .cover(move || Some(cover.get()))
                .align(align)
                .placement(placement)
                .offset(px(4.0))
                .margin(px(8.0))
                .trigger(trigger)
                .items(items)
                .popup_class("p-1 bg-popover text-popover-foreground rounded-2xl border border-border shadow-lg transition duration-100")
                .popup_style(move |style| {
                    let style = style.direction(popup_direction.map(LayoutDirection::from));
                    if desktop {
                        style.min_w(move || {
                            trigger_width.get().max(px(144.0)).min(px((window().width.0 - 16.0).max(0.0)))
                        })
                    } else {
                        style
                    }
                })
                .popup_starting_class(animation.clone())
                .popup_ending_class(animation)
                .list_style(move |style| style.max_h(px(max_height)))
                .item_class("min-h-7 px-2 py-1.5 rounded-xl focus:bg-accent focus:text-accent-foreground disabled:opacity-50")
                .render_item(move |entry: &SelectEntry| {
                    let dir = item_direction.clone();
                    let current = entry.value.clone();
                    view().flex_row().items_center().gap_2().w_full().min_w(px(0.0))
                        .direction(dir.clone().map(LayoutDirection::from))
                        .child(text(entry.title.clone()).text_sm().min_w(px(0.0)).truncate().flex_1()
                            .style_when(move || dir.get() == Direction::Rtl, |style| style.text_right()))
                        .child(
                            view().w(px(16.0)).flex_shrink_0().items_center().accessibility_elements_hidden(true)
                                .child(text(move || if value.with(|value| value.as_ref() == Some(&current)) { "✓" } else { "" })
                                    .text_sm().test_id("select-item-mark")),
                        )
                })
                .render_group(move |group: &NativeGroup| {
                    let dir = group_direction.clone();
                    let separator = first_group.as_ref().is_some_and(|first| *first != group.title);
                    view().w_full().px_2().py_1()
                        .when(separator, |label| label.border_t(px(1.0)).border_color(move || theme().border).mt_1())
                        .child(text(group.title.clone()).text_xs().font_weight(FontWeight::MEDIUM)
                            .text_color(move || theme().muted_foreground)
                            .style_when(move || dir.get() == Direction::Rtl, |style| style.text_right()))
                }),
        ).into_element()
    }
}
