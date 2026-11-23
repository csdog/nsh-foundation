//! Questionnaire state, validation and Native Shell presentation.

use std::{collections::BTreeMap, rc::Rc};

use nsh_base::theme;
use nsh_core::{AccessibilityValue, Checked, keymap::Keystroke, prelude::*};

use crate::{
    button::{Button, ButtonVariant},
    input::invalid_input,
    ui::muted,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestionnaireAnswers(BTreeMap<&'static str, Vec<String>>);

impl QuestionnaireAnswers {
    pub fn all(&self, name: &str) -> &[String] {
        self.0.get(name).map_or(&[], Vec::as_slice)
    }

    pub fn first(&self, name: &str) -> Option<&str> {
        self.all(name).first().map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &[String])> {
        self.0
            .iter()
            .map(|(&name, answers)| (name, answers.as_slice()))
    }
}

#[derive(Clone, Copy)]
pub struct QuestionnaireChoice {
    value: &'static str,
    label: &'static str,
    description: Option<&'static str>,
}

impl QuestionnaireChoice {
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self {
            value,
            label,
            description: None,
        }
    }

    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }
}

#[derive(Clone, Copy)]
pub struct QuestionnaireInput {
    label: &'static str,
    placeholder: &'static str,
}

impl QuestionnaireInput {
    pub const fn new(label: &'static str, placeholder: &'static str) -> Self {
        Self { label, placeholder }
    }
}

type Enabled = fn(&QuestionnaireAnswers) -> bool;

#[derive(Clone, Copy)]
pub struct QuestionnaireItem {
    name: &'static str,
    title: &'static str,
    description: Option<&'static str>,
    choices: &'static [QuestionnaireChoice],
    input: Option<QuestionnaireInput>,
    required: bool,
    multiple: bool,
    enabled: Option<Enabled>,
}

impl QuestionnaireItem {
    pub const fn new(name: &'static str, title: &'static str) -> Self {
        Self {
            name,
            title,
            description: None,
            choices: &[],
            input: None,
            required: true,
            multiple: false,
            enabled: None,
        }
    }

    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn choices(mut self, choices: &'static [QuestionnaireChoice]) -> Self {
        self.choices = choices;
        self
    }

    pub const fn input(mut self, input: QuestionnaireInput) -> Self {
        self.input = Some(input);
        self
    }

    pub const fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    pub const fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub const fn enabled_when(mut self, enabled: Enabled) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionnaireShortcuts {
    #[default]
    None,
    Letters,
    Numbers,
}

impl IntoReactive<QuestionnaireShortcuts> for QuestionnaireShortcuts {
    fn into_reactive(self) -> Reactive<Self> {
        Reactive::Static(self)
    }
}

impl QuestionnaireShortcuts {
    fn key(self, index: usize) -> Option<char> {
        match self {
            Self::Letters if index < 26 => Some(char::from(b'a' + index as u8)),
            Self::Numbers if index < 9 => Some(char::from(b'1' + index as u8)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuestionnaireProgress {
    pub current: usize,
    pub total: usize,
}

pub struct QuestionnaireError {
    pub item: &'static str,
    pub message: String,
}

impl QuestionnaireError {
    pub fn new(item: &'static str, message: impl Into<String>) -> Self {
        Self {
            item,
            message: message.into(),
        }
    }
}

type Validation = dyn Fn(&QuestionnaireAnswers) -> Result<(), QuestionnaireError>;
type Submission = dyn Fn(QuestionnaireAnswers);
type ProgressRenderer = dyn Fn(QuestionnaireProgress) -> View;

pub struct Questionnaire {
    items: &'static [QuestionnaireItem],
    defaults: QuestionnaireAnswers,
    initial_index: usize,
    active: Option<Signal<usize>>,
    shortcuts: Reactive<QuestionnaireShortcuts>,
    submit_label: &'static str,
    reset: bool,
    skip: bool,
    require_answer: bool,
    animated: bool,
    validate: Option<Box<Validation>>,
    on_submit: Option<Box<Submission>>,
    progress: Option<Box<ProgressRenderer>>,
}

impl Questionnaire {
    pub fn new(items: &'static [QuestionnaireItem]) -> Self {
        Self {
            items,
            defaults: QuestionnaireAnswers::default(),
            initial_index: 0,
            active: None,
            shortcuts: Reactive::Static(QuestionnaireShortcuts::None),
            submit_label: "Submit",
            reset: false,
            skip: true,
            require_answer: false,
            animated: false,
            validate: None,
            on_submit: None,
            progress: None,
        }
    }

    pub fn default_item(mut self, name: &str) -> Self {
        self.initial_index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .unwrap_or(0);
        self
    }

    pub fn default_answers(mut self, name: &'static str, answers: &[&str]) -> Self {
        self.defaults.0.insert(
            name,
            answers.iter().map(|answer| (*answer).to_owned()).collect(),
        );
        self
    }

    /// Read and update the host's index into the declared item collection.
    pub fn active_item(mut self, item: Signal<usize>) -> Self {
        self.active = Some(item);
        self
    }

    pub fn shortcuts(mut self, shortcuts: impl IntoReactive<QuestionnaireShortcuts>) -> Self {
        self.shortcuts = shortcuts.into_reactive();
        self
    }

    pub fn submit_label(mut self, label: &'static str) -> Self {
        self.submit_label = label;
        self
    }

    pub fn reset_action(mut self, show: bool) -> Self {
        self.reset = show;
        self
    }

    pub fn skip_action(mut self, show: bool) -> Self {
        self.skip = show;
        self
    }

    pub fn disable_unanswered(mut self, enabled: bool) -> Self {
        self.require_answer = enabled;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn validate(
        mut self,
        validate: impl Fn(&QuestionnaireAnswers) -> Result<(), QuestionnaireError> + 'static,
    ) -> Self {
        self.validate = Some(Box::new(validate));
        self
    }

    pub fn on_submit(mut self, submit: impl Fn(QuestionnaireAnswers) + 'static) -> Self {
        self.on_submit = Some(Box::new(submit));
        self
    }

    pub fn render_progress(
        mut self,
        render: impl Fn(QuestionnaireProgress) -> View + 'static,
    ) -> Self {
        self.progress = Some(Box::new(render));
        self
    }
}

struct Session {
    config: Questionnaire,
    answers: Signal<QuestionnaireAnswers>,
    text: Signal<BTreeMap<&'static str, String>>,
    index: Signal<usize>,
    error: Signal<Option<(&'static str, String)>>,
    first: ElementRef,
    prompt: ElementRef,
    pending_focus: Stored<Option<bool>>,
}

impl Session {
    fn enabled(&self) -> Vec<usize> {
        let answers = self.snapshot();
        self.config
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                item.enabled
                    .is_none_or(|enabled| enabled(&answers))
                    .then_some(index)
            })
            .collect()
    }

    fn current(&self) -> Option<usize> {
        let index = self.index.get();
        let enabled = self.enabled();
        if enabled.contains(&index) {
            return Some(index);
        }
        enabled
            .iter()
            .copied()
            .find(|candidate| *candidate >= index)
            .or_else(|| enabled.last().copied())
    }

    fn snapshot(&self) -> QuestionnaireAnswers {
        let mut answers = self.answers.get();
        self.text.with(|inputs| {
            for (&name, value) in inputs {
                if !value.trim().is_empty() {
                    answers
                        .0
                        .entry(name)
                        .or_default()
                        .push(value.trim().to_owned());
                }
            }
        });
        answers
    }

    fn restore(&self) {
        let (choices, text) = defaults(&self.config);
        self.answers.set(choices);
        self.text.set(text);
        self.go(self.config.initial_index, false);
    }

    fn select(&self, item: QuestionnaireItem, choice: QuestionnaireChoice) {
        if !item.multiple {
            self.text.update(|text| {
                text.remove(item.name);
            });
        }
        self.answers.update(|answers| {
            let values = answers.0.entry(item.name).or_default();
            if !item.multiple {
                values.clear();
            }
            if let Some(index) = values.iter().position(|value| value == choice.value) {
                values.remove(index);
            } else {
                values.push(choice.value.to_owned());
            }
        });
        self.error.set(None);
    }

    fn next(&self, index: usize, skip: bool) {
        let item = self.config.items[index];
        if skip && !item.required {
            self.answers.update(|answers| {
                answers.0.remove(item.name);
            });
            self.text.update(|text| {
                text.remove(item.name);
            });
        } else if self.unanswered(item) {
            self.show_error(index, "Choose an answer to continue.".to_owned());
            return;
        }
        if let Some(next) = self
            .enabled()
            .into_iter()
            .find(|candidate| *candidate > index)
        {
            self.go(next, false);
            return;
        }
        let enabled = self.enabled();
        if let Some(invalid) = enabled
            .iter()
            .copied()
            .find(|&index| self.unanswered(self.config.items[index]))
        {
            self.show_error(invalid, "Choose an answer to continue.".to_owned());
            return;
        }
        let mut answers = self.snapshot();
        answers.0.retain(|&name, values| {
            values.retain(|value| !value.trim().is_empty());
            !values.is_empty()
                && enabled
                    .iter()
                    .any(|&index| self.config.items[index].name == name)
        });
        if let Some(validate) = &self.config.validate {
            if let Err(error) = validate(&answers) {
                let invalid = enabled
                    .into_iter()
                    .find(|&index| self.config.items[index].name == error.item)
                    .unwrap_or(index);
                self.show_error(invalid, error.message);
                return;
            }
        }
        self.error.set(None);
        if let Some(submit) = &self.config.on_submit {
            submit(answers);
        }
    }

    fn unanswered(&self, item: QuestionnaireItem) -> bool {
        item.required
            && self
                .answers
                .with(|answers| answers.all(item.name).is_empty())
            && self.text.with(|inputs| {
                inputs
                    .get(item.name)
                    .is_none_or(|value| value.trim().is_empty())
            })
    }

    fn go(&self, index: usize, invalid: bool) {
        if !invalid {
            self.error.set(None);
        }
        if self.current() == Some(index) {
            self.focus(invalid);
        } else {
            self.pending_focus.replace(Some(invalid));
            self.index.set(index);
        }
    }

    fn focus(&self, invalid: bool) {
        if invalid && self.first.node().is_some() && is_desktop() {
            self.first.focus();
        } else {
            self.prompt.focus();
        }
    }

    fn show_error(&self, index: usize, message: String) {
        self.error
            .set(Some((self.config.items[index].name, message)));
        self.go(index, true);
    }
}

type TextAnswers = BTreeMap<&'static str, String>;

fn defaults(config: &Questionnaire) -> (QuestionnaireAnswers, TextAnswers) {
    let mut selected = QuestionnaireAnswers::default();
    let mut inputs = TextAnswers::new();
    for &item in config.items {
        for value in config.defaults.all(item.name) {
            if is_choice(item, value) {
                let answers = selected.0.entry(item.name).or_default();
                if item.multiple && !answers.contains(value) || answers.is_empty() {
                    answers.push(value.clone());
                }
            } else if item.input.is_some() && !value.trim().is_empty() {
                inputs.entry(item.name).or_insert_with(|| value.clone());
            }
        }
        if !item.multiple && selected.first(item.name).is_some() {
            inputs.remove(item.name);
        }
    }
    (selected, inputs)
}

impl IntoElement for Questionnaire {
    fn into_element(self) -> AnyElement {
        let index = self.active.unwrap_or_else(|| signal(self.initial_index));
        let (answers, text) = defaults(&self);
        let session = Rc::new(Session {
            index,
            answers: signal(answers),
            text: signal(text),
            error: signal(None),
            first: element_ref(),
            prompt: element_ref(),
            pending_focus: stored(None),
            config: self,
        });
        let current = computed({
            let session = session.clone();
            move || session.current()
        });
        let enabled = computed({
            let session = session.clone();
            move || session.enabled()
        });
        view()
            .w_full()
            .max_w(px(448.0))
            .gap_5()
            .child(dynamic({
                let session = session.clone();
                move || {
                    let enabled = enabled.get();
                    let step = QuestionnaireProgress {
                        current: current
                            .get()
                            .and_then(|index| enabled.iter().position(|&item| item == index))
                            .map_or(0, |index| index + 1),
                        total: enabled.len(),
                    };
                    session
                        .config
                        .progress
                        .as_ref()
                        .map_or_else(|| default_progress(step), |render| render(step))
                }
            }))
            .child(dynamic({
                let session = session.clone();
                move || {
                    current
                        .get()
                        .map(|index| render_item(session.clone(), index))
                }
            }))
            .child(dynamic(move || {
                current
                    .get()
                    .map(|index| actions(session.clone(), index, enabled.get()))
            }))
            .into_element()
    }
}

fn default_progress(progress: QuestionnaireProgress) -> View {
    let label = format!("Question {} of {}", progress.current, progress.total);
    view()
        .role(AccessibilityRole::Progressbar)
        .label("Questionnaire progress")
        .accessibility_value(Some(AccessibilityValue {
            min: (progress.total > 0).then_some(1.0),
            max: (progress.total > 0).then_some(progress.total as f32),
            now: (progress.total > 0).then_some(progress.current as f32),
            text: (progress.total > 0).then(|| label.clone().into()),
        }))
        .child(muted(label).text_xs().tabular_nums())
}

fn actions(session: Rc<Session>, index: usize, enabled: Vec<usize>) -> View {
    let item = session.config.items[index];
    let prev = enabled
        .iter()
        .copied()
        .take_while(|candidate| *candidate < index)
        .last();
    let last = enabled.last() == Some(&index);
    let row = view()
        .flex_row()
        .flex_wrap()
        .justify_end()
        .items_center()
        .gap_2();
    let row = row.child(session.config.reset.then(|| {
        let session = session.clone();
        Button::new("Reset changes")
            .variant(ButtonVariant::Outline)
            .on_press(move || {
                session.restore();
            })
    }));
    let row = row.child(prev.map(|prev| {
        let session = session.clone();
        Button::new("Previous")
            .variant(ButtonVariant::Outline)
            .on_press(move || session.go(prev, false))
    }));
    let row = row.child((!item.required && session.config.skip).then(|| {
        let session = session.clone();
        Button::new("Skip")
            .variant(ButtonVariant::Ghost)
            .on_press(move || session.next(index, true))
    }));
    let label = if last {
        session.config.submit_label
    } else {
        "Next"
    };
    row.child(
        Button::new(label)
            .variant(if session.config.require_answer && !last {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Default
            })
            .disabled({
                let session = session.clone();
                move || session.config.require_answer && session.unanswered(item)
            })
            .on_press(move || session.next(index, false)),
    )
}

fn render_item(session: Rc<Session>, index: usize) -> View {
    let item = session.config.items[index];
    let input_focused = signal(false);
    let choice_position = signal(None::<usize>);
    let refs: Rc<[ElementRef]> = (0..(item.choices.len() + usize::from(item.input.is_some())))
        .map(|position| {
            if position == 0 {
                session.first
            } else {
                element_ref()
            }
        })
        .collect();
    let shortcut_mode = session.config.shortcuts.clone();
    let capture_mode = shortcut_mode.clone();
    let shortcuts =
        move || {
            if input_focused.get() {
                return KeyCapture::None;
            }
            let keys = ["up", "down", "left", "right", "enter"].map(str::to_owned);
            KeyCapture::keys(
                keys.into_iter()
                    .chain((0..refs_len(item)).filter_map(|index| {
                        capture_mode.get().key(index).map(|key| key.to_string())
                    }))
                    .filter_map(|key| Keystroke::parse(&key, os()).ok()),
            )
        };
    let mut panel = view()
        .role(AccessibilityRole::Group)
        .label(item.title)
        .focusable(true)
        .element_ref(&session.prompt)
        .test_id("questionnaire-item")
        .gap_4()
        .on_focus(move || choice_position.set(None))
        .on_layout({
            let session = session.clone();
            move |_: &Bounds<Points>| {
                if let Some(invalid) = session.pending_focus.replace(None) {
                    session.focus(invalid);
                }
            }
        })
        .capture_keys(shortcuts)
        .on_key_down({
            let session = session.clone();
            let refs = refs.clone();
            move |event: &KeyEvent| {
                if !event.default_prevented || event.repeat || input_focused.get_untracked() {
                    return;
                }
                let focused = choice_position.get_untracked();
                let step = match event.key.as_ref() {
                    "ArrowUp" => -1,
                    "ArrowDown" => 1,
                    "ArrowLeft" if focused.is_some() && !item.multiple => -1,
                    "ArrowRight" if focused.is_some() && !item.multiple => 1,
                    "ArrowLeft" => {
                        if let Some(previous) = session
                            .enabled()
                            .into_iter()
                            .take_while(|candidate| *candidate < index)
                            .last()
                        {
                            session.go(previous, false);
                        }
                        return;
                    }
                    "ArrowRight" | "Enter" => {
                        if !session.unanswered(item) {
                            session.next(index, false);
                        }
                        return;
                    }
                    key => {
                        if let Some(position) = (0..refs.len()).find(|position| {
                            shortcut_mode.get().key(*position).is_some_and(|candidate| {
                                candidate.to_string() == key.to_ascii_lowercase()
                            })
                        }) {
                            if let Some(choice) = item.choices.get(position) {
                                session.select(item, *choice);
                            }
                            refs[position].focus();
                        }
                        return;
                    }
                };
                if !refs.is_empty() {
                    let moved = focused.map_or_else(
                        || if step < 0 { refs.len() - 1 } else { 0 },
                        |focused| {
                            (focused as isize + step).rem_euclid(refs.len() as isize) as usize
                        },
                    );
                    if !item.multiple && focused.is_some() {
                        if let Some(choice) = item.choices.get(moved) {
                            session.select(item, *choice);
                        }
                    }
                    refs[moved].focus();
                }
            }
        })
        .child(
            text(item.title)
                .role(AccessibilityRole::Header)
                .text_base()
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(item.description.map(muted));
    if session.config.animated && !reduce_motion() {
        let opacity = animated_value(0.0);
        let y = animated_value(8.0);
        let _ = opacity.timing(1.0, 180.0, Easing::Linear);
        let _ = y.timing(0.0, 220.0, Easing::Bezier(0.22, 1.0, 0.36, 1.0));
        panel = panel.bind_opacity(&opacity).bind_translate_y(&y);
    }
    let controls = view()
        .gap_3()
        .role(if item.multiple || item.choices.is_empty() {
            AccessibilityRole::Group
        } else {
            AccessibilityRole::Radiogroup
        })
        .label(item.title)
        .children(
            item.choices
                .iter()
                .copied()
                .enumerate()
                .map(|(position, choice)| {
                    render_choice(
                        session.clone(),
                        item,
                        choice,
                        position,
                        refs[position],
                        choice_position,
                    )
                }),
        );
    let controls = view()
        .gap_3()
        .child((!item.choices.is_empty()).then_some(controls))
        .child(item.input.map(|field| {
            let clear = session.error;
            let answers = session.answers;
            let text = session.text;
            let invalid = move || {
                clear.with(|error| error.as_ref().is_some_and(|(name, _)| *name == item.name))
            };
            invalid_input(invalid)
                .label(field.label)
                .placeholder(field.placeholder)
                .element_ref(&refs[item.choices.len()])
                .value(move || text.with(|text| text.get(item.name).cloned().unwrap_or_default()))
                .on_focus(move || input_focused.set(true))
                .on_blur(move || input_focused.set(false))
                .on_change(move |value: &str| {
                    if !item.multiple && !value.trim().is_empty() {
                        answers.update(|answers| {
                            answers.0.remove(item.name);
                        });
                    }
                    text.update(|text| {
                        text.insert(item.name, value.to_owned());
                    });
                    clear.set(None);
                })
                .on_submit({
                    let session = session.clone();
                    move |_: &str| session.next(index, false)
                })
        }));
    let error = session.error;
    panel.child(controls).child(dynamic(move || {
        error.with(|error| {
            error
                .as_ref()
                .filter(|(name, _)| *name == item.name)
                .map(|(_, message)| {
                    text(message.clone())
                        .role(AccessibilityRole::Alert)
                        .text_sm()
                        .text_color(move || theme().destructive)
                })
        })
    }))
}

fn refs_len(item: QuestionnaireItem) -> usize {
    item.choices.len() + usize::from(item.input.is_some())
}

fn is_choice(item: QuestionnaireItem, value: &str) -> bool {
    item.choices.iter().any(|choice| choice.value == value)
}

fn render_choice(
    session: Rc<Session>,
    item: QuestionnaireItem,
    choice: QuestionnaireChoice,
    position: usize,
    reference: ElementRef,
    focus: Signal<Option<usize>>,
) -> View {
    let answers = session.answers;
    let checked = computed(move || {
        answers.with(|answers| {
            answers
                .all(item.name)
                .iter()
                .any(|value| value == choice.value)
        })
    });
    let error = session.error;
    let invalid =
        move || error.with(|error| error.as_ref().is_some_and(|(name, _)| *name == item.name));
    let mode = session.config.shortcuts.clone();
    view()
        .flex_row()
        .items_start()
        .gap_3()
        .px_4()
        .py_3()
        .rounded(px(16.0))
        .border(px(1.0))
        .border_color(move || {
            let theme = theme();
            if invalid() {
                theme.destructive
            } else if checked.get() {
                theme.primary.opacity(0.4)
            } else {
                theme.input
            }
        })
        .bg(move || {
            if checked.get() {
                theme().primary.opacity(0.1)
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .hover(|style| style.bg(theme().input.opacity(0.4)))
        .focus_visible(|style| style.ring(px(3.0)).ring_color(theme().ring.opacity(0.4)))
        .role(if item.multiple {
            AccessibilityRole::Checkbox
        } else {
            AccessibilityRole::Radio
        })
        .label(choice.label)
        .when_some(choice.description, |choice, description| {
            choice.hint(description)
        })
        .accessibility_state(move || AccessibilityState {
            checked: Some(if checked.get() {
                Checked::True
            } else {
                Checked::False
            }),
            invalid: invalid(),
            ..Default::default()
        })
        .element_ref(&reference)
        .on_focus(move || focus.set(Some(position)))
        .on_press(move || {
            session.select(item, choice);
            reference.focus();
        })
        .child(
            view()
                .size(px(16.0))
                .mt(px(2.0))
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .rounded(if item.multiple { px(5.0) } else { px(8.0) })
                .bg(move || {
                    if checked.get() {
                        theme().primary
                    } else {
                        theme().input
                    }
                })
                .accessibility_elements_hidden(true)
                .child(
                    text(move || {
                        if checked.get() {
                            if item.multiple { "✓" } else { "●" }
                        } else {
                            ""
                        }
                    })
                    .text_xs()
                    .text_color(move || theme().primary_foreground),
                ),
        )
        .child(
            view()
                .min_w(px(0.0))
                .flex_1()
                .gap_1()
                .accessibility_elements_hidden(true)
                .child(text(choice.label).text_sm().font_weight(FontWeight::MEDIUM))
                .child(choice.description.map(muted)),
        )
        .child(dynamic(move || {
            mode.get().key(position).map(|key| {
                view()
                    .size(px(20.0))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .border(px(1.0))
                    .border_color(move || theme().border)
                    .accessibility_elements_hidden(true)
                    .child(
                        text(key.to_ascii_uppercase().to_string())
                            .text_xs()
                            .text_color(move || theme().muted_foreground),
                    )
            })
        }))
}
