//! Compact, single-value selection backed by Native Shell.

use nsh_base::{SelectEntry, SelectGroup, SelectItem, select, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NativeSelectSize {
    #[default]
    Default,
    Small,
}

pub struct NativeSelectOption(SelectItem);

impl NativeSelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self(SelectItem::new(value, label))
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }
}

pub struct NativeSelectOptGroup {
    label: SharedString,
    disabled: bool,
    options: Vec<NativeSelectOption>,
}

impl NativeSelectOptGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            options: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn option(mut self, option: NativeSelectOption) -> Self {
        self.options.push(option);
        self
    }
}

type OnChange = Box<dyn Fn(Option<SharedString>)>;

pub struct NativeSelect {
    label: SharedString,
    placeholder: SharedString,
    items: Vec<SelectItem>,
    value: Option<Signal<Option<SharedString>>>,
    default_value: Option<SharedString>,
    disabled: Reactive<bool>,
    invalid: Reactive<bool>,
    size: NativeSelectSize,
    on_change: Option<OnChange>,
}

impl NativeSelect {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            placeholder: "".into(),
            items: Vec::new(),
            value: None,
            default_value: None,
            disabled: false.into_reactive(),
            invalid: false.into_reactive(),
            size: NativeSelectSize::Default,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn option(mut self, option: NativeSelectOption) -> Self {
        self.items.push(option.0);
        self
    }

    pub fn group(mut self, group: NativeSelectOptGroup) -> Self {
        self.items.extend(group.options.into_iter().map(|option| {
            let disabled = group.disabled || option.0.disabled;
            option.0.group(group.label.clone()).disabled(disabled)
        }));
        self
    }

    /// Supply a signal to share the selection with its form or owner.
    pub fn value(mut self, value: Signal<Option<SharedString>>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
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

    pub fn size(mut self, size: NativeSelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_change(mut self, on_change: impl Fn(Option<SharedString>) + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    pub fn surface(self) -> View {
        let direction = use_direction();
        let initial = self.default_value.or_else(|| {
            if !self.placeholder.is_empty() {
                return None;
            }
            self.items
                .iter()
                .find(|item| !item.disabled)
                .map(|item| item.value.clone())
        });
        let value = self.value.unwrap_or_else(|| signal(initial));
        let invalid = computed(move || self.invalid.get());
        let disabled = self.disabled;
        let change = self.on_change;
        let labels = self.items.clone();
        let placeholder = self.placeholder.clone();
        let height = match self.size {
            NativeSelectSize::Default => 32.0,
            NativeSelectSize::Small => 28.0,
        };
        let title = move || {
            let selected = value.get();
            labels
                .iter()
                .find(|item| Some(&item.value) == selected.as_ref())
                .map_or_else(|| placeholder.clone(), |item| item.title.clone())
        };
        let trigger_direction = direction.clone();
        let item_direction = direction.clone();
        let group_direction = direction.clone();

        view().min_w(px(0.0)).child(
            select()
                .direction(direction.map(LayoutDirection::from))
                .accessibility_label(self.label)
                .placeholder(self.placeholder)
                .value(move || value.get())
                .disabled(disabled)
                .on_change(move |next: &Option<SharedString>| {
                    value.set(next.clone());
                    if let Some(change) = &change {
                        change(next.clone());
                    }
                })
                .offset(px(4.0))
                .margin(px(8.0))
                .trigger(
                    view()
                        .test_id("native-select-trigger")
                        .direction(trigger_direction.map(LayoutDirection::from))
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .h(px(height))
                        .w_full()
                        .min_w(px(0.0))
                        .px(px(10.0))
                        .rounded(px(16.0))
                        .border(px(1.0))
                        .border_color(move || if invalid.get() {
                            theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
                        } else {
                            AdaptiveColor::TRANSPARENT
                        })
                        .bg(|| theme().input.opacity(0.5))
                        .disabled_style(|style| style.opacity(0.5))
                        .focus_visible(move |style| {
                            style.ring(px(3.0)).ring_color(if invalid.get() {
                                theme().destructive.opacity(if dark() { 0.4 } else { 0.2 })
                            } else {
                                theme().ring.opacity(0.3)
                            })
                        })
                        .style_when(invalid, |style| {
                            style.ring(px(3.0)).ring_color(theme().destructive.opacity(if dark() { 0.4 } else { 0.2 }))
                        })
                        .accessibility_state(move || AccessibilityState { invalid: invalid.get(), ..Default::default() })
                        .child(
                            text(title).test_id("native-select-value").min_w(px(0.0)).truncate().text_sm()
                                .selectable(false)
                                .text_color(move || if value.with(Option::is_none) { theme().muted_foreground } else { theme().foreground }),
                        )
                        .child(
                            text("⌄").test_id("native-select-chevron").text_sm().flex_shrink_0()
                                .text_color(|| theme().muted_foreground)
                                .accessibility_elements_hidden(true),
                        ),
                )
                .items(self.items)
                .popup_class("p-1 bg-popover text-popover-foreground rounded-2xl border border-border shadow-md")
                .item_class("px-3 py-2 rounded-lg focus:bg-muted selected:bg-muted disabled:opacity-50")
                .group_class("px-3 py-1")
                .render_item(move |entry: &SelectEntry| {
                    let dir = item_direction.clone();
                    text(entry.title.clone()).text_sm().w_full().style_when(
                        move || dir.get() == Direction::Rtl, |style| style.text_right(),
                    )
                })
                .render_group(move |group: &SelectGroup| {
                    let dir = group_direction.clone();
                    text(group.title.clone()).text_xs().font_weight(FontWeight::MEDIUM)
                        .text_color(|| theme().muted_foreground).w_full().style_when(
                            move || dir.get() == Direction::Rtl, |style| style.text_right(),
                        )
                }),
        )
    }
}

impl IntoElement for NativeSelect {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

#[cfg(test)]
mod tests {
    use nsh_core::{
        element::Root,
        platform::{Idiom, Os},
    };
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;

    #[component]
    fn BoundSelect() -> impl IntoElement {
        let value = signal(Some("alpha".into()));
        view()
            .child(
                NativeSelect::new("Choice")
                    .value(value)
                    .invalid(true)
                    .option(NativeSelectOption::new("alpha", "Alpha"))
                    .group(
                        NativeSelectOptGroup::new("Unavailable")
                            .disabled(true)
                            .option(NativeSelectOption::new("beta", "Beta")),
                    )
                    .option(NativeSelectOption::new("gamma", "Gamma").disabled(true))
                    .option(NativeSelectOption::new("delta", "Delta")),
            )
            .child(text(move || {
                value.with(|value| format!("Selected: {}", value.as_deref().unwrap_or("")))
            }))
    }

    #[test]
    fn keyboard_skips_disabled_options_and_groups_in_a_bound_select() {
        let mut config = EngineConfig::new(Root::of::<BoundSelect>());
        config.environment.os = Os::Web;
        config.environment.idiom = Idiom::Desktop;
        config.environment.reduce_motion = true;
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();
        assert!(
            app.screen()
                .accessibility_state("role:combobox[Choice]")
                .unwrap()
                .invalid
        );
        app.press_key("tab").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for("role:option[Alpha]").unwrap();
        assert_eq!(app.screen().enabled("role:option[Beta]"), Ok(false));
        assert_eq!(app.screen().enabled("role:option[Gamma]"), Ok(false));
        app.press_key("down").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for("Selected: delta").unwrap();
    }
}
