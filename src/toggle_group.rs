//! A group of Rhea toggles with shared selection and logical keyboard navigation.

use std::rc::Rc;

use nsh_core::{keymap::Keystroke, prelude::*};

use crate::{
    direction::{Direction, use_direction},
    toggle::{Toggle, ToggleSize, ToggleVariant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

type Values = Vec<&'static str>;

pub struct ToggleGroup {
    label: Reactive<SharedString>,
    items: Vec<ToggleGroupItem>,
    variant: ToggleVariant,
    size: ToggleSize,
    orientation: ToggleGroupOrientation,
    spacing: f32,
    multiple: bool,
    disabled: Reactive<bool>,
    initial: Values,
    controlled: Option<Reactive<Values>>,
    change: Option<Box<dyn Fn(Values)>>,
}

impl ToggleGroup {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: label.into_reactive(),
            items: Vec::new(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            orientation: ToggleGroupOrientation::Horizontal,
            spacing: 8.0,
            multiple: false,
            disabled: false.into_reactive(),
            initial: Vec::new(),
            controlled: None,
            change: None,
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ToggleGroupItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Spacing in points. Zero joins adjacent controls.
    pub fn spacing(mut self, points: f32) -> Self {
        self.spacing = points.max(0.0);
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn default_value(mut self, value: impl IntoIterator<Item = &'static str>) -> Self {
        self.initial = value.into_iter().collect();
        self
    }

    pub fn value(mut self, value: impl IntoReactive<Values>) -> Self {
        self.controlled = Some(value.into_reactive());
        self
    }

    pub fn on_change(mut self, change: impl Fn(Values) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    pub fn surface(self) -> View {
        let (values, local) = match self.controlled {
            Some(values) => (values, None),
            None => {
                let values = signal(self.initial);
                (values.into_reactive(), Some(values))
            }
        };
        let all_disabled = self.disabled;
        let targets: Rc<[Target]> = self
            .items
            .iter()
            .map(|item| {
                let disabled = item.disabled.clone();
                let all_disabled = all_disabled.clone();
                Target {
                    value: item.value,
                    reference: element_ref(),
                    disabled: computed(move || all_disabled.get() || disabled.get()),
                }
            })
            .collect();
        let selected = computed({
            let multiple = self.multiple;
            let targets = targets.clone();
            move || {
                let values = values.get();
                {
                    let matched = values
                        .iter()
                        .copied()
                        .filter(|value| targets.iter().any(|target| target.value == *value));
                    if multiple {
                        let mut selected = Vec::new();
                        for value in matched {
                            if !selected.contains(&value) {
                                selected.push(value);
                            }
                        }
                        selected
                    } else {
                        matched.take(1).collect()
                    }
                }
            }
        });
        let focus = signal(None);
        let stop = computed({
            let targets = targets.clone();
            move || {
                let enabled = |&index: &usize| !targets[index].disabled.get();
                focus
                    .get()
                    .filter(enabled)
                    .or_else(|| {
                        let selected = selected.get();
                        (0..targets.len())
                            .filter(enabled)
                            .find(|&index| selected.contains(&targets[index].value))
                    })
                    .or_else(|| (0..targets.len()).find(enabled))
            }
        });
        let direction = use_direction();
        let vertical = self.orientation == ToggleGroupOrientation::Vertical;
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
        let multiple = self.multiple;
        let change = Rc::new(self.change);
        let count = targets.len();
        let joined = self.spacing == 0.0 && count > 1;
        let mut group = view()
            .role(AccessibilityRole::Toolbar)
            .label(self.label)
            .direction(direction.clone().map(LayoutDirection::from))
            .self_start()
            .gap(px(self.spacing))
            .when(!vertical, |group| group.flex_row().items_center())
            .test_id("toggle-group");
        for (index, item) in self.items.into_iter().enumerate() {
            let Target {
                value,
                reference,
                disabled,
            } = targets[index];
            let on_change = change.clone();
            let targets = targets.clone();
            let item_direction = direction.clone();
            let customize = item.style;
            let mut item = item
                .toggle
                .variant(self.variant)
                .size(self.size)
                .group_surface(
                    computed(move || selected.with(|items| items.contains(&value))),
                    disabled,
                    move |on| {
                        let mut next = selected.get_untracked();
                        if on {
                            if !multiple {
                                next.clear();
                            }
                            next.push(value);
                        } else {
                            next.retain(|current| *current != value);
                        }
                        if let Some(local) = local {
                            local.set(next.clone());
                        }
                        if let Some(change) = on_change.as_ref() {
                            change(next);
                        }
                        focus_on(focus, index, reference);
                    },
                )
                .element_ref(&reference)
                .focusable(move || stop.get() == Some(index))
                .on_focus(move || focus.set(Some(index)))
                .on_blur(move || {
                    if focus.get_untracked() == Some(index) {
                        focus.set(None);
                    }
                })
                .capture_keys(keys.clone())
                .on_key_down(move |event: &KeyEvent| {
                    if event.default_prevented {
                        let mirrored = !vertical && item_direction.get() == Direction::Rtl;
                        let current = focus.get_untracked().unwrap_or(index);
                        if let Some(next) =
                            next_target(&targets, current, event.key.as_ref(), mirrored)
                        {
                            focus_on(focus, next, targets[next].reference);
                        }
                    }
                })
                .flex_shrink_0()
                .style_when(move || focus.get() == Some(index), |style| style.z_index(1))
                .when(vertical, |item| item.self_stretch());
            if joined {
                let before = index > 0;
                let after = index + 1 < count;
                let square = px(0.0);
                let rtl = {
                    let direction = direction.clone();
                    computed(move || direction.get() == Direction::Rtl)
                };
                item = item.px_2();
                item = if vertical {
                    item.when(before, |style| style.rounded_t(square).border_t(square))
                        .when(after, |style| style.rounded_b(square))
                } else {
                    item.style_when(
                        move || before && !rtl.get(),
                        |style| style.rounded_l(square).border_l(square),
                    )
                    .style_when(move || after && !rtl.get(), |style| style.rounded_r(square))
                    .style_when(
                        move || before && rtl.get(),
                        |style| style.rounded_r(square).border_r(square),
                    )
                    .style_when(move || after && rtl.get(), |style| style.rounded_l(square))
                };
            }
            group = group.child(match customize {
                Some(customize) => customize(item),
                None => item,
            });
        }
        group
    }
}

impl IntoElement for ToggleGroup {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct ToggleGroupItem {
    value: &'static str,
    toggle: Toggle,
    disabled: Reactive<bool>,
    style: Option<Box<dyn FnOnce(View) -> View>>,
}

impl ToggleGroupItem {
    pub fn new(value: &'static str, text: impl IntoReactive<SharedString>) -> Self {
        Self {
            value,
            toggle: Toggle::new(text),
            disabled: false.into_reactive(),
            style: None,
        }
    }

    pub fn label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.toggle = self.toggle.label(label);
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.toggle = self.toggle.content(content);
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn style(mut self, style: impl FnOnce(View) -> View + 'static) -> Self {
        self.style = Some(Box::new(style));
        self
    }
}

#[derive(Clone, Copy)]
struct Target {
    value: &'static str,
    reference: ElementRef,
    disabled: Memo<bool>,
}

fn focus_on(focus: Signal<Option<usize>>, index: usize, target: ElementRef) {
    focus.set(Some(index));
    spawn(async move {
        next_frame().await;
        if focus.try_get() == Some(Some(index)) {
            target.focus();
        }
    });
}

fn next_target(targets: &[Target], current: usize, key: &str, mirrored: bool) -> Option<usize> {
    let count = targets.len();
    let enabled = |&index: &usize| !targets[index].disabled.get_untracked();
    match key {
        "Home" => (0..count).find(enabled),
        "End" => (0..count).rev().find(enabled),
        "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => {
            let forward = matches!(key, "ArrowRight" | "ArrowDown") != mirrored;
            (1..=count)
                .map(|step| {
                    if forward {
                        (current + step) % count
                    } else {
                        (current + count - step) % count
                    }
                })
                .find(enabled)
        }
        _ => None,
    }
}
