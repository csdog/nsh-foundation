//! Rhea radio items with scoped roving focus and controlled selection.

use std::rc::Rc;

use nsh_base::theme;
use nsh_core::{Checked, element::LayoutDirection, input::tab_stop, keymap::Keystroke, prelude::*};

use crate::direction::{Direction, use_direction};

pub struct RadioGroupItem {
    value: SharedString,
    label: SharedString,
    description: Option<SharedString>,
    disabled: Reactive<bool>,
    card: bool,
}

impl RadioGroupItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false.into_reactive(),
            card: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    /// Use the full-width choice card layout with the indicator at the end.
    pub fn card(mut self) -> Self {
        self.card = true;
        self
    }

    fn render(
        self,
        group: Rc<Selection>,
        index: usize,
        invalid: Memo<bool>,
        direction: Reactive<Direction>,
    ) -> View {
        let Self {
            label,
            description,
            card,
            ..
        } = self;
        let option = group.options[index].clone();
        let checked = computed({
            let option = option.clone();
            let value = group.value;
            move || value.with(|value| value.as_ref() == Some(&option.key))
        });
        let disabled = option.disabled;
        let focused = group.focused;
        let stop = group.stop;
        let mark = view()
            .test_id("radio-group-indicator")
            .size(px(16.0))
            .flex_shrink_0()
            .rounded_full()
            .border(px(1.0))
            .bg(move || {
                if checked.get() {
                    theme().primary
                } else {
                    theme().input.opacity(0.9)
                }
            })
            .border_color(move || {
                if invalid.get() && !card {
                    theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .style_when(
                move || invalid.get() && !card,
                |style| {
                    style
                        .ring(px(3.0))
                        .ring_color(theme().destructive.opacity(if dark() { 0.4 } else { 0.2 }))
                },
            )
            .when(!card, |mark| {
                mark.group_focus_visible(|style| {
                    style
                        .border_color(theme().ring)
                        .ring(px(3.0))
                        .ring_color(theme().ring.opacity(0.3))
                })
            })
            .items_center()
            .justify_center()
            .child(when(checked, || {
                view()
                    .size(move || px(if dark() { 10.0 } else { 8.0 }))
                    .rounded_full()
                    .bg(move || theme().primary_foreground)
            }));
        let details = view()
            .flex_1()
            .min_w(px(0.0))
            .gap_1()
            .style_when(
                move || direction.get() == Direction::Rtl,
                |style| style.text_right(),
            )
            .child(
                text(label.clone())
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM),
            )
            .child(description.clone().map(|description| {
                text(description)
                    .text_sm()
                    .text_color(move || theme().muted_foreground)
            }));
        let body = view()
            .flex_row()
            .gap_3()
            .when(description.is_some(), |body| body.items_start())
            .when(description.is_none(), |body| body.items_center())
            .accessibility_elements_hidden(true);
        let body = if card {
            body.child(details).child(mark)
        } else {
            body.child(mark).child(details)
        };
        view()
            .role(AccessibilityRole::Radio)
            .label(label)
            .when_some(description, |item, description| {
                item.accessibility_description(description)
            })
            .accessibility_state(move || AccessibilityState {
                checked: Some(if checked.get() {
                    Checked::True
                } else {
                    Checked::False
                }),
                selected: checked.get(),
                invalid: invalid.get(),
                ..Default::default()
            })
            .disabled(disabled)
            .focusable(move || stop.get() == Some(index))
            .element_ref(&option.node)
            .on_focus(move || focused.set(Some(index)))
            .on_blur(move || {
                if focused.get_untracked() == Some(index) {
                    focused.set(None);
                }
            })
            .on_press(move || group.choose(index))
            .w_full()
            .group()
            .rounded(px(16.0))
            .when(card, |item| {
                item.p_4()
                    .border(px(1.0))
                    .border_color(move || {
                        if invalid.get() {
                            theme().destructive
                        } else if checked.get() {
                            theme().primary.opacity(0.45)
                        } else {
                            theme().border
                        }
                    })
                    .bg(move || {
                        if checked.get() {
                            theme().input.opacity(0.3)
                        } else {
                            AdaptiveColor::TRANSPARENT
                        }
                    })
                    .hover(|style| style.bg(theme().input.opacity(0.4)))
                    .focus_visible(|style| {
                        style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3))
                    })
            })
            .disabled_style(|style| style.opacity(0.5))
            .child(body)
    }
}

type OnChange = dyn Fn(SharedString);

pub struct RadioGroup {
    label: SharedString,
    default_value: Option<SharedString>,
    value: Option<Signal<Option<SharedString>>>,
    disabled: Reactive<bool>,
    invalid: Reactive<bool>,
    items: Vec<RadioGroupItem>,
    on_change: Option<Rc<OnChange>>,
}

impl RadioGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            default_value: None,
            value: None,
            disabled: false.into_reactive(),
            invalid: false.into_reactive(),
            items: Vec::new(),
            on_change: None,
        }
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Read and update a value owned by the caller.
    pub fn value(mut self, value: Signal<Option<SharedString>>) -> Self {
        self.value = Some(value);
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

    pub fn item(mut self, item: RadioGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn on_change(mut self, change: impl Fn(SharedString) + 'static) -> Self {
        self.on_change = Some(Rc::new(change));
        self
    }
}

#[derive(Clone)]
struct OptionEntry {
    key: SharedString,
    disabled: Memo<bool>,
    node: ElementRef,
}

struct Selection {
    value: Signal<Option<SharedString>>,
    options: Vec<OptionEntry>,
    focused: Signal<Option<usize>>,
    stop: Memo<Option<usize>>,
    change: Option<Rc<OnChange>>,
}

impl Selection {
    fn choose(&self, index: usize) {
        if self.options[index].disabled.get() {
            return;
        }
        self.focused.set(Some(index));
        self.select(index);
        self.options[index].node.focus();
    }

    fn select(&self, index: usize) {
        let option = &self.options[index];
        if self.value.with(|value| value.as_ref() == Some(&option.key)) {
            return;
        }
        self.value.set(Some(option.key.clone()));
        if let Some(change) = &self.change {
            change(option.key.clone());
        }
    }

    fn step(&self, back: bool) {
        let enabled: Vec<_> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(index, option)| (!option.disabled.get()).then_some(index))
            .collect();
        let focused = self.focused.get_untracked();
        let current = enabled.iter().position(|&index| focused == Some(index));
        if let Some(next) = tab_stop(current, enabled.len(), back, true) {
            self.choose(enabled[next]);
        }
    }
}

impl IntoElement for RadioGroup {
    fn into_element(self) -> AnyElement {
        let direction = use_direction();
        let keyboard_direction = direction.clone();
        let value = self.value.unwrap_or_else(|| signal(self.default_value));
        let disabled = computed(move || self.disabled.get());
        let invalid = computed(move || self.invalid.get());
        let focused = signal(None::<usize>);
        let options: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let item_disabled = item.disabled.clone();
                OptionEntry {
                    key: item.value.clone(),
                    disabled: computed(move || disabled.get() || item_disabled.get()),
                    node: element_ref(),
                }
            })
            .collect();
        let stop = computed({
            let options = options.clone();
            move || {
                let selected = value.get();
                let available = |index: usize| !options[index].disabled.get();
                let focused = focused.get().filter(|&index| available(index));
                focused
                    .or_else(|| {
                        options
                            .iter()
                            .enumerate()
                            .find(|(index, option)| {
                                selected.as_ref() == Some(&option.key) && available(*index)
                            })
                            .map(|(index, _)| index)
                    })
                    .or_else(|| (0..options.len()).find(|&index| available(index)))
            }
        });
        let selection = Rc::new(Selection {
            value,
            options,
            focused,
            stop,
            change: self.on_change,
        });
        let keys = KeyCapture::keys(
            ["up", "down", "left", "right"]
                .into_iter()
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        view()
            .role(AccessibilityRole::Radiogroup)
            .label(self.label)
            .accessibility_state(move || AccessibilityState {
                invalid: invalid.get(),
                ..Default::default()
            })
            .direction(direction.clone().map(LayoutDirection::from))
            .w_full()
            .min_w(px(0.0))
            .gap_3()
            .capture_keys(keys)
            .on_key_down({
                let selection = selection.clone();
                move |event: &KeyEvent| {
                    if !event.default_prevented {
                        return;
                    }
                    let rtl = keyboard_direction.get() == Direction::Rtl;
                    let back = match event.key.as_ref() {
                        "ArrowUp" => true,
                        "ArrowDown" => false,
                        "ArrowLeft" => !rtl,
                        "ArrowRight" => rtl,
                        _ => return,
                    };
                    selection.step(back);
                }
            })
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                item.render(selection.clone(), index, invalid, direction.clone())
            }))
            .into_element()
    }
}
