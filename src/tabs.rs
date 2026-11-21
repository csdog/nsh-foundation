//! Composable Rhea tabs with independent selection and keyboard focus.

use std::rc::Rc;

use nsh_base::theme;
use nsh_core::{element::LayoutDirection, keymap::Keystroke, prelude::*};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

pub struct Tabs {
    list: TabsList,
    panels: Vec<TabsContent>,
    initial: &'static str,
    value: Option<Signal<&'static str>>,
    orientation: TabsOrientation,
    on_change: Option<Box<dyn Fn(&'static str)>>,
}

impl Tabs {
    pub fn new(list: TabsList) -> Self {
        Self {
            list,
            panels: Vec::new(),
            initial: "",
            value: None,
            orientation: TabsOrientation::Horizontal,
            on_change: None,
        }
    }

    pub fn default_value(mut self, value: &'static str) -> Self {
        self.initial = value;
        self
    }

    /// Bind selection to a caller-owned signal.
    pub fn value(mut self, value: Signal<&'static str>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn on_change(mut self, change: impl Fn(&'static str) + 'static) -> Self {
        self.on_change = Some(Box::new(change));
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn content(mut self, panel: TabsContent) -> Self {
        self.panels.push(panel);
        self
    }

    pub fn surface(self) -> View {
        let direction = use_direction();
        let vertical = self.orientation == TabsOrientation::Vertical;
        let value = self.value.unwrap_or_else(|| signal(self.initial));
        let panel_refs = self
            .panels
            .iter()
            .map(|panel| (panel.value, element_ref()))
            .collect::<Vec<_>>();
        let targets = Rc::new(
            self.list
                .triggers
                .iter()
                .map(|trigger| Target {
                    value: trigger.value,
                    label: trigger.label.clone(),
                    disabled: trigger.disabled.clone(),
                    reference: element_ref(),
                    panel: panel_refs
                        .iter()
                        .find_map(|&(value, panel)| (value == trigger.value).then_some(panel)),
                })
                .collect::<Vec<_>>(),
        );
        let selected = computed({
            let targets = targets.clone();
            move || {
                let value = value.get();
                targets
                    .iter()
                    .position(|target| target.value == value && !target.disabled.get())
                    .or_else(|| targets.iter().position(|target| !target.disabled.get()))
            }
        });
        let state = Rc::new(State {
            targets,
            selected,
            value,
            focus: signal(None),
            auto: self.list.auto,
            on_change: self.on_change,
        });
        let line = self.list.variant == TabsVariant::Line;
        let list = view()
            .role(AccessibilityRole::Tablist)
            .label(self.list.label)
            .self_start()
            .max_w(percentage(1.0))
            .min_w(px(0.0))
            .flex_shrink_0()
            .when(vertical, |list| list.p_1().gap_1())
            .when(!vertical, |list| {
                list.flex_row()
                    .flex_wrap()
                    .items_center()
                    .p(px(3.0))
                    .min_h(px(32.0))
            })
            .when(!line, |list| list.rounded(px(16.0)).bg(|| theme().muted))
            .when(line && !vertical, |list| list.gap_1().pb(px(5.0)))
            .test_id("tabs-list")
            .children(
                self.list
                    .triggers
                    .into_iter()
                    .enumerate()
                    .map(|(index, trigger)| {
                        trigger.render(state.clone(), index, vertical, line, direction.clone())
                    }),
            );

        let mut root = view()
            .direction(direction.map(LayoutDirection::from))
            .min_w(px(0.0))
            .gap_2()
            .when(vertical, |root| root.flex_row().items_start())
            .child(list);
        for (panel, (_, reference)) in self.panels.into_iter().zip(panel_refs) {
            let Some(index) = state
                .targets
                .iter()
                .position(|target| target.value == panel.value)
            else {
                continue;
            };
            let label = state.targets[index].label.clone();
            root = root.child(when(
                move || selected.get() == Some(index),
                move || {
                    view()
                        .min_w(px(0.0))
                        .when(vertical, |panel| panel.flex_1())
                        .role(AccessibilityRole::Region)
                        .label(label.clone())
                        .element_ref(&reference)
                        .focusable(true)
                        .focus_visible(|style| {
                            style
                                .ring(px(2.0))
                                .ring_color(theme().ring.opacity(0.4))
                                .rounded_lg()
                        })
                        .test_id("tabs-panel")
                        .child((panel.content)())
                },
            ));
        }
        root
    }
}

impl IntoElement for Tabs {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct TabsList {
    label: Reactive<SharedString>,
    variant: TabsVariant,
    auto: bool,
    triggers: Vec<TabsTrigger>,
}

impl TabsList {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: label.into_reactive(),
            variant: TabsVariant::Default,
            auto: false,
            triggers: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Select the new tab when arrow keys move focus.
    pub fn activate_on_focus(mut self, auto: bool) -> Self {
        self.auto = auto;
        self
    }

    pub fn trigger(mut self, trigger: TabsTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    pub fn triggers(mut self, triggers: impl IntoIterator<Item = TabsTrigger>) -> Self {
        self.triggers.extend(triggers);
        self
    }
}

pub struct TabsContent {
    value: &'static str,
    content: Box<dyn Fn() -> AnyElement>,
}

impl TabsContent {
    pub fn new<E: IntoElement>(value: &'static str, content: impl Fn() -> E + 'static) -> Self {
        Self {
            value,
            content: Box::new(move || content().into_element()),
        }
    }
}

pub struct TabsTrigger {
    value: &'static str,
    label: Reactive<SharedString>,
    icon: Option<AnyElement>,
    disabled: Reactive<bool>,
}

impl TabsTrigger {
    pub fn new(value: &'static str, label: impl IntoReactive<SharedString>) -> Self {
        Self {
            value,
            label: label.into_reactive(),
            icon: None,
            disabled: false.into_reactive(),
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    fn render(
        self,
        state: Rc<State>,
        index: usize,
        vertical: bool,
        line: bool,
        direction: Reactive<Direction>,
    ) -> View {
        let target = &state.targets[index];
        let reference = target.reference;
        let panel = target.panel;
        let disabled = computed(move || self.disabled.get());
        let selected = state.selected;
        let focus = state.focus;
        let active = move || selected.get() == Some(index);
        let axes = if vertical {
            ["up", "down"]
        } else {
            ["left", "right"]
        };
        let keys = KeyCapture::keys(
            axes.into_iter()
                .chain(["home", "end"])
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        let keyboard = state.clone();
        let rtl = direction.clone();
        let marker = || {
            view()
                .absolute()
                .bg(|| theme().foreground)
                .accessibility_elements_hidden(true)
                .pointer_events(PointerEvents::None)
                .when(vertical, |marker| {
                    marker
                        .top_0()
                        .bottom_0()
                        .w(px(2.0))
                        .style_with(move |style| {
                            if direction.get() == Direction::Rtl {
                                style.left(px(-4.0))
                            } else {
                                style.right(px(-4.0))
                            }
                        })
                })
                .when(!vertical, |marker| {
                    marker.left_0().right_0().bottom(px(-4.0)).h(px(2.0))
                })
                .opacity(move || if active() { 1.0_f32 } else { 0.0 })
                .style_when(
                    || !reduce_motion(),
                    |style| style.transition_opacity().duration(150.0),
                )
                .test_id("tabs-marker")
        };
        view()
            .role(AccessibilityRole::Tab)
            .label(self.label.clone())
            .accessibility_controls(move || if active() { panel } else { None })
            .accessibility_state(move || AccessibilityState {
                selected: active(),
                disabled: disabled.get(),
                ..Default::default()
            })
            .disabled(disabled)
            .focusable(move || {
                !disabled.get() && focus.get().or_else(|| selected.get()) == Some(index)
            })
            .element_ref(&reference)
            .test_id("tabs-trigger")
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .min_w(px(0.0))
            .min_h(px(26.0))
            .px(px(if vertical { 12.0 } else { 6.0 }))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(AdaptiveColor::TRANSPARENT)
            .text_color(move || {
                if dark() {
                    theme().muted_foreground
                } else {
                    theme().foreground.opacity(0.60)
                }
            })
            .when(vertical, |trigger| trigger.w_full())
            .when(!vertical, |trigger| trigger.justify_center())
            .hover(|style| style.text_color(theme().foreground))
            .style_when(active, |style| style.text_color(theme().foreground))
            .style_when(
                move || active() && !line,
                |style| {
                    style
                        .bg(|| {
                            if dark() {
                                theme().input.opacity(0.3)
                            } else {
                                theme().background
                            }
                        })
                        .border(px(1.0))
                        .border_color(|| {
                            if dark() {
                                theme().input
                            } else {
                                AdaptiveColor::TRANSPARENT
                            }
                        })
                },
            )
            .disabled_style(|style| style.opacity(0.5))
            .focus_visible(|style| style.ring(px(3.0)).ring_color(theme().ring.opacity(0.5)))
            .style_when(
                || !reduce_motion(),
                |style| style.transition_colors().duration(150.0),
            )
            .on_focus(move || focus.set(Some(index)))
            .on_blur(move || {
                if focus.get_untracked() == Some(index) {
                    focus.set(None);
                }
            })
            .capture_keys(keys)
            .on_key_down(move |event: &KeyEvent| {
                if event.default_prevented {
                    let mirrored = !vertical && rtl.get() == Direction::Rtl;
                    keyboard.navigate(index, event.key.as_ref(), mirrored);
                }
            })
            .on_press(move || {
                if !disabled.get_untracked() {
                    state.select(index);
                    state.focus_on(index);
                }
            })
            .child(
                self.icon
                    .map(|icon| view().accessibility_elements_hidden(true).child(icon)),
            )
            .child(
                text(self.label)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .number_of_lines(1),
            )
            .child(line.then(marker))
    }
}

struct Target {
    value: &'static str,
    label: Reactive<SharedString>,
    disabled: Reactive<bool>,
    reference: ElementRef,
    panel: Option<ElementRef>,
}

struct State {
    targets: Rc<Vec<Target>>,
    selected: Memo<Option<usize>>,
    value: Signal<&'static str>,
    focus: Signal<Option<usize>>,
    auto: bool,
    on_change: Option<Box<dyn Fn(&'static str)>>,
}

impl State {
    fn select(&self, index: usize) {
        if self.selected.get_untracked() != Some(index) {
            let value = self.targets[index].value;
            self.value.set(value);
            if let Some(change) = &self.on_change {
                change(value);
            }
        }
    }

    fn focus_on(&self, index: usize) {
        self.focus.set(Some(index));
        let (target, focus) = (self.targets[index].reference, self.focus);
        spawn(async move {
            next_frame().await;
            if focus.get_untracked() == Some(index) {
                target.focus();
            }
        });
    }

    fn navigate(&self, origin: usize, key: &str, mirrored: bool) {
        let n = self.targets.len();
        let current = self.focus.get_untracked().unwrap_or(origin);
        let enabled = |&index: &usize| !self.targets[index].disabled.get();
        let forward = matches!(key, "ArrowRight" | "ArrowDown") != mirrored;
        let next = match key {
            "Home" => (0..n).find(enabled),
            "End" => (0..n).rev().find(enabled),
            "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => (1..=n)
                .map(|step| {
                    if forward {
                        (current + step) % n
                    } else {
                        (current + n - step) % n
                    }
                })
                .find(enabled),
            _ => return,
        };
        if let Some(index) = next {
            if self.auto {
                self.select(index);
            }
            self.focus_on(index);
        }
    }
}
