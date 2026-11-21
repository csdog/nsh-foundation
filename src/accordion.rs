//! Styled, composable Accordion parts backed by Native Shell's primitives.

use std::rc::Rc;

use nsh_base::{
    accordion, accordion_header, accordion_item, accordion_panel, accordion_trigger, theme,
};
use nsh_core::{element::LayoutDirection, keyboard::KeyCapture, keymap::Keystroke, prelude::*};

pub struct Accordion {
    value: Signal<Vec<SharedString>>,
    items: Vec<AccordionItem>,
    multiple: bool,
    radius: f32,
    direction: LayoutDirection,
    change: Option<Box<dyn Fn(Vec<SharedString>)>>,
}

impl Accordion {
    pub fn new(default_values: &[&str]) -> Self {
        Self::with_value(signal(
            default_values
                .iter()
                .map(|id| SharedString::from((*id).to_owned()))
                .collect(),
        ))
    }

    pub fn with_value(value: Signal<Vec<SharedString>>) -> Self {
        Self {
            value,
            items: vec![],
            multiple: false,
            radius: 16.0,
            direction: LayoutDirection::Inherit,
            change: None,
        }
    }

    pub fn on_change(mut self, change: impl Fn(Vec<SharedString>) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn radius(mut self, points: f32) -> Self {
        self.radius = points;
        self
    }

    pub fn child(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }
}

impl IntoElement for Accordion {
    fn into_element(self) -> AnyElement {
        let value = self.value;
        let change = self.change;
        let mut root = accordion()
            .direction(self.direction)
            .w_full()
            .rounded(px(self.radius))
            .overflow_hidden()
            .border(px(1.0))
            .border_color(move || theme().border)
            .value(value)
            .multiple(self.multiple)
            .on_change(move |next: &Vec<SharedString>| {
                if let Some(change) = &change {
                    change(next.clone());
                } else {
                    value.set(next.clone());
                }
            });

        let stops = Rc::new(
            self.items
                .iter()
                .map(|item| (!item.disabled).then(element_ref))
                .collect::<Vec<_>>(),
        );
        let last = self.items.len().saturating_sub(1);
        let capture = KeyCapture::keys(
            ["up", "down", "home", "end"]
                .into_iter()
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        for (index, item) in self.items.into_iter().enumerate() {
            let open = move || value.with(|values| values.iter().any(|id| id.as_ref() == item.id));
            let stop = stops[index].unwrap_or_else(element_ref);
            let stops = stops.clone();
            let rtl = self.direction == LayoutDirection::Rtl;
            root = root.child(
                accordion_item()
                    .value(item.id)
                    .disabled(item.disabled)
                    .px(px(item.inset))
                    .border_b(px(if index == last { 0.0 } else { 1.0 }))
                    .border_color(move || theme().border)
                    .style_when(open, |style| style.bg(theme().muted))
                    .child(
                        accordion_header().child(
                            view()
                                .capture_keys(capture.clone())
                                .on_key_down(move |event: &KeyEvent| {
                                    if !event.default_prevented {
                                        return;
                                    }
                                    let next = match event.key.as_ref() {
                                        "ArrowDown" => stops[index + 1..]
                                            .iter()
                                            .chain(&stops[..=index])
                                            .flatten()
                                            .next(),
                                        "ArrowUp" => stops[..index]
                                            .iter()
                                            .rev()
                                            .chain(stops[index..].iter().rev())
                                            .flatten()
                                            .next(),
                                        "Home" => stops.iter().flatten().next(),
                                        "End" => stops.iter().rev().flatten().next(),
                                        _ => None,
                                    };
                                    if let Some(next) = next {
                                        next.focus();
                                    }
                                })
                                .child(
                                    accordion_trigger()
                                        .element_ref(&stop)
                                        .accessibility_label(item.trigger.label)
                                        .p_4()
                                        .cursor_pointer()
                                        .focus_visible(|style| {
                                            style
                                                .ring(px(2.0))
                                                .ring_inset()
                                                .ring_color(theme().ring)
                                        })
                                        .disabled_style(|style| style.opacity(0.5))
                                        .child(
                                            view()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .gap_6()
                                                .child(
                                                    text(item.trigger.label)
                                                        .text_sm()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .when(rtl, |label| label.text_right())
                                                        .flex_1(),
                                                )
                                                .child(
                                                    text(move || if open() { "⌃" } else { "⌄" })
                                                        .text_color(move || {
                                                            theme().muted_foreground
                                                        }),
                                                ),
                                        ),
                                ),
                        ),
                    )
                    .child(
                        accordion_panel()
                            .reveal(true)
                            .accessibility_label(item.trigger.label)
                            .child(view().px_4().pb_4().child(item.content.0)),
                    ),
            );
        }
        root.into_element()
    }
}

pub struct AccordionItem {
    id: &'static str,
    disabled: bool,
    inset: f32,
    trigger: AccordionTrigger,
    content: AccordionContent,
}

impl AccordionItem {
    pub fn new(id: &'static str, trigger: AccordionTrigger, content: AccordionContent) -> Self {
        Self {
            id,
            disabled: false,
            inset: 0.0,
            trigger,
            content,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn inset(mut self, points: f32) -> Self {
        self.inset = points;
        self
    }
}

pub struct AccordionTrigger {
    label: &'static str,
}

impl AccordionTrigger {
    pub fn new(label: &'static str) -> Self {
        Self { label }
    }
}

pub struct AccordionContent(AnyElement);

impl AccordionContent {
    pub fn new(content: impl IntoElement) -> Self {
        Self(content.into_element())
    }
}
