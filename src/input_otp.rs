//! Composable OTP cells around one accessible native text input.

use nsh_base::theme;
use nsh_core::{AccessibilityState, AutoFill, prelude::*};

#[derive(Clone, Copy, Default)]
pub enum InputOtpPattern {
    #[default]
    Digits,
    Alphanumeric,
    Custom(fn(char) -> bool),
}

impl InputOtpPattern {
    fn accept(self, character: char) -> Option<char> {
        match self {
            Self::Digits | Self::Alphanumeric => {
                let digit = ['0', '０', '٠', '۰'].into_iter().find_map(|zero| {
                    let offset = (character as u32).checked_sub(zero as u32)?;
                    char::from_digit(offset, 10)
                });
                digit.or_else(|| {
                    (matches!(self, Self::Alphanumeric) && character.is_ascii_alphabetic())
                        .then_some(character)
                })
            }
            Self::Custom(accept) => accept(character).then_some(character),
        }
    }

    fn clean(self, raw: &str, length: usize) -> String {
        raw.chars()
            .filter_map(|character| self.accept(character))
            .take(length)
            .collect()
    }
}

pub struct InputOtpSlot {
    index: usize,
    invalid: Reactive<bool>,
}

impl InputOtpSlot {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            invalid: false.into_reactive(),
        }
    }

    pub fn invalid(mut self, invalid: impl IntoReactive<bool>) -> Self {
        self.invalid = invalid.into_reactive();
        self
    }
}

#[derive(Default)]
pub struct InputOtpGroup(Vec<InputOtpSlot>);

impl InputOtpGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, slot: InputOtpSlot) -> Self {
        self.0.push(slot);
        self
    }

    pub fn slots(indices: impl IntoIterator<Item = usize>) -> Self {
        Self(indices.into_iter().map(InputOtpSlot::new).collect())
    }
}

#[derive(Default)]
pub struct InputOtpSeparator;

impl InputOtpSeparator {
    pub fn new() -> Self {
        Self
    }
}

enum Part {
    Group(InputOtpGroup),
    Separator(InputOtpSeparator),
}

type OtpHandler = Box<dyn Fn(&str)>;

pub struct InputOtp {
    label: Reactive<SharedString>,
    length: usize,
    initial: String,
    pattern: InputOtpPattern,
    value: Option<Signal<String>>,
    invalid: Reactive<bool>,
    disabled: Reactive<bool>,
    parts: Vec<Part>,
    field: Option<ElementRef>,
    description: Option<Reactive<Option<ElementRef>>>,
    slot_size: Size<Points>,
    change: Option<OtpHandler>,
    complete: Option<OtpHandler>,
    submit: Option<OtpHandler>,
}

impl InputOtp {
    pub fn new(label: impl IntoReactive<SharedString>, length: usize) -> Self {
        Self {
            label: label.into_reactive(),
            length,
            initial: String::new(),
            pattern: InputOtpPattern::default(),
            value: None,
            invalid: false.into_reactive(),
            disabled: false.into_reactive(),
            parts: Vec::new(),
            field: None,
            description: None,
            slot_size: Size::new(px(32.0), px(32.0)),
            change: None,
            complete: None,
            submit: None,
        }
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.initial = value.into();
        self
    }

    /// Reads edits from and writes edits back to a Rust signal.
    pub fn value(mut self, value: Signal<String>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn pattern(mut self, pattern: InputOtpPattern) -> Self {
        self.pattern = pattern;
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

    pub fn group(mut self, group: InputOtpGroup) -> Self {
        self.parts.push(Part::Group(group));
        self
    }

    pub fn separator(mut self, separator: InputOtpSeparator) -> Self {
        self.parts.push(Part::Separator(separator));
        self
    }

    pub fn slot_size(mut self, width: Points, height: Points) -> Self {
        self.slot_size = Size::new(width, height);
        self
    }

    pub fn control_ref(mut self, field: ElementRef) -> Self {
        self.field = Some(field);
        self
    }

    pub fn described_by(mut self, description: impl IntoReactive<Option<ElementRef>>) -> Self {
        self.description = Some(description.into_reactive());
        self
    }

    pub fn on_change(mut self, change: impl Fn(&str) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    pub fn on_complete(mut self, complete: impl Fn(&str) + 'static) -> Self {
        self.complete = Some(Box::new(complete));
        self
    }

    pub fn on_submit(mut self, submit: impl Fn(&str) + 'static) -> Self {
        self.submit = Some(Box::new(submit));
        self
    }
}

impl IntoElement for InputOtp {
    fn into_element(mut self) -> AnyElement {
        if self.parts.is_empty() {
            self.parts
                .push(Part::Group(InputOtpGroup::slots(0..self.length)));
        }
        let length = self.length;
        let pattern = self.pattern;
        let code = self
            .value
            .unwrap_or_else(|| signal(pattern.clean(&self.initial, length)));
        let displayed = computed(move || code.with(|value| pattern.clean(value, length)));
        let field = signal(displayed.get_untracked());
        let target = self.field.unwrap_or_else(element_ref);
        let focused = signal(false);
        let selection = signal(Selection::default());
        let correction = stored(None::<Spawned<()>>);
        let invalid = self.invalid;
        let is_invalid = computed(move || invalid.get());
        let disabled = self.disabled;
        let is_disabled = computed(move || disabled.get());
        let slot_invalid: Vec<_> = self
            .parts
            .iter()
            .filter_map(|part| match part {
                Part::Group(group) => Some(group.0.iter().map(|slot| slot.invalid.clone())),
                Part::Separator(_) => None,
            })
            .flatten()
            .collect();
        let field_invalid = move || is_invalid.get() || slot_invalid.iter().any(Reactive::get);

        effect(move || {
            let value = displayed.get();
            if field.get_untracked() != value {
                if let Some(task) = correction.replace(None) {
                    task.cancel();
                }
                field.set(value);
            }
        });

        let change = self.change;
        let complete = self.complete;
        let commit = move |raw: &str| {
            if let Some(task) = correction.replace(None) {
                task.cancel();
            }
            let next = pattern.clean(raw, length);
            if next == raw || next != field.get_untracked() {
                field.set(next.clone());
            } else {
                // The native edit changed its text while the last bound value
                // stayed the same. Echo it once so the correction is applied.
                field.set(raw.to_owned());
                correction.replace(Some(spawn({
                    let next = next.clone();
                    async move {
                        field.set(next);
                    }
                })));
            }
            if displayed.get_untracked() != next {
                code.set(next.clone());
                if let Some(change) = &change {
                    change(&next);
                }
                if next.chars().count() == length {
                    if let Some(complete) = &complete {
                        complete(&next);
                    }
                }
            }
        };
        let cells = Cells {
            displayed,
            selection,
            focused,
            invalid: is_invalid,
            length,
            size: self.slot_size,
        };
        let submit = self.submit;
        let parts = view()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .items_center()
            .accessibility_elements_hidden(true)
            .pointer_events(PointerEvents::None)
            .children(self.parts.into_iter().map(|part| {
                match part {
                    Part::Group(group) => cells.group(group).into_element(),
                    Part::Separator(_) => view()
                        .items_center()
                        .justify_center()
                        .child(
                            text("−")
                                .text_sm()
                                .text_color(move || theme().muted_foreground),
                        )
                        .into_element(),
                }
            }));
        let input = nsh_core::element::input()
            .absolute()
            .inset_0()
            .element_ref(&target)
            .label(self.label)
            .accessibility_described_by(self.description.unwrap_or(Reactive::Static(None)))
            .accessibility_state(move || AccessibilityState {
                invalid: field_invalid(),
                ..Default::default()
            })
            .disabled(is_disabled)
            .focus_visible(|style| style.outline_none())
            .border(px(0.0))
            .p_0()
            .bg(AdaptiveColor::TRANSPARENT)
            .text_color(AdaptiveColor::TRANSPARENT)
            .selection_color(hsla(0.0, 0.0, 0.0, 0.0))
            .caret(false)
            .keyboard_type(if matches!(pattern, InputOtpPattern::Digits) {
                KeyboardType::Number
            } else {
                KeyboardType::Default
            })
            .auto_fill(AutoFill::OneTimeCode)
            .auto_correct(false)
            .auto_capitalize(AutoCapitalize::None)
            .paste_behavior(PasteBehavior::Report)
            .value(field)
            .on_change(commit)
            .on_paste(move |paste: &PasteEvent| {
                let pasted = pattern.clean(&paste.text, length);
                if pasted.is_empty() {
                    return;
                }
                if pasted.chars().count() == length {
                    target.set_selection(
                        0,
                        displayed.with_untracked(|value| value.encode_utf16().count() as u32),
                    );
                }
                target.insert_text(pasted);
            })
            .on_selection_change(move |current: &Selection| selection.set(*current))
            .on_focus(move || focused.set(true))
            .on_blur(move || focused.set(false))
            .on_submit(move |_: &str| {
                if let Some(submit) = &submit {
                    displayed.with_untracked(|value| submit(value));
                }
            });

        view()
            .relative()
            .max_w(relative(1.0))
            .self_start()
            .style_when(is_disabled, |style| style.opacity(0.5))
            .test_id("input-otp")
            .child(parts)
            .child(input)
            .into_element()
    }
}

#[derive(Clone, Copy)]
struct Cells {
    displayed: Memo<String>,
    selection: Signal<Selection>,
    focused: Signal<bool>,
    invalid: Memo<bool>,
    length: usize,
    size: Size<Points>,
}

impl Cells {
    fn group(self, group: InputOtpGroup) -> View {
        let total = group.0.len();
        let mut row = view().flex_row().rounded(px(16.0));
        for (index, slot) in group.0.into_iter().enumerate() {
            row = row.child(self.slot(slot, index == 0, index + 1 == total));
        }
        row
    }

    fn slot(self, slot: InputOtpSlot, first: bool, last: bool) -> View {
        let index = slot.index;
        let digit = move || self.displayed.with(|value| value.chars().nth(index));
        let active = move || {
            if !self.focused.get() {
                return false;
            }
            let selection = self.selection.get();
            let position = |offset| self.displayed.with(|value| index_at(value, offset));
            let start = position(selection.start);
            let end = position(selection.end);
            if start == end {
                index == start.min(self.length.saturating_sub(1))
            } else {
                (start..end).contains(&index)
            }
        };
        let invalid = computed(move || self.invalid.get() || slot.invalid.get());
        let size = self.size;
        view()
            .w(move || {
                if window().width.0 < 420.0 {
                    size.width.min(px(36.0))
                } else {
                    size.width
                }
            })
            .h(size.height)
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .border_y(px(1.0))
            .border_r(px(1.0))
            .when(first, |cell| cell.border_l(px(1.0)).rounded_l(px(16.0)))
            .when(last, |cell| cell.rounded_r(px(16.0)))
            .border_color(move || {
                if invalid.get() {
                    theme().destructive
                } else if active() {
                    theme().ring
                } else {
                    theme().input
                }
            })
            .bg(move || theme().input.opacity(0.5))
            .transition()
            .duration(200.0)
            .style_when(active, |style| {
                style.ring(px(3.0)).ring_color(move || {
                    if invalid.get() {
                        theme().destructive.opacity(if dark() { 0.4 } else { 0.2 })
                    } else {
                        theme().ring.opacity(0.3)
                    }
                })
            })
            .test_id("input-otp-slot")
            .child(
                text(move || digit().map_or_else(String::new, |digit| digit.to_string()))
                    .text_size(px(if size.height.0 > 32.0 { 20.0 } else { 14.0 }))
                    .text_color(move || theme().foreground),
            )
            .child(when(
                move || digit().is_none() && active(),
                || {
                    view()
                        .absolute()
                        .w(px(1.0))
                        .h(px(16.0))
                        .self_center()
                        .bg(move || theme().foreground)
                        .style_when(|| !reduce_motion(), |style| style.animate_pulse())
                },
            ))
    }
}

fn index_at(value: &str, offset: u32) -> usize {
    let mut consumed = 0;
    value
        .chars()
        .take_while(|character| {
            consumed += character.len_utf16() as u32;
            consumed <= offset
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_grouped_and_localized_digits_without_accepting_letters() {
        assert_eq!(InputOtpPattern::Digits.clean("a١２-۳ 4", 4), "1234");
        assert_eq!(InputOtpPattern::Alphanumeric.clean("a-B ١!z", 6), "aB1z");
        assert_eq!(InputOtpPattern::Digits.clean("012345", 4), "0123");
    }

    #[test]
    fn caret_offsets_map_to_cells_for_custom_unicode_patterns() {
        assert_eq!(index_at("abc", 2), 2);
        assert_eq!(index_at("a😀b", 3), 2);
        assert_eq!(index_at("a😀b", 4), 3);
    }
}
