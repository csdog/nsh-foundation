//! Composable form-field layout, labels, descriptions, and validation feedback.

use nsh_base::theme;
use nsh_core::{AccessibilityState, element::ElNode, prelude::*};

macro_rules! field_part {
    ($( $name:ident ),+ $(,)?) => {
        $(
            pub struct $name(View);

            impl IntoElement for $name {
                fn into_element(self) -> AnyElement {
                    self.0.into_element()
                }
            }

            impl Element for $name {
                fn node(&mut self) -> &mut ElNode {
                    self.0.node()
                }
            }

            impl ParentElement for $name {}
        )+
    };
}

field_part!(
    Field,
    FieldSet,
    FieldGroup,
    FieldContent,
    FieldLabel,
    FieldDescription,
    FieldError
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl Field {
    pub fn new() -> Self {
        Self(
            view()
                .w_full()
                .min_w(px(0.0))
                .gap_3()
                .role(AccessibilityRole::Group),
        )
    }

    /// Responsive fields switch at 448 points of their own available width.
    pub fn orientation(self, orientation: FieldOrientation) -> Self {
        Self(match orientation {
            FieldOrientation::Vertical => self.0.flex_col(),
            FieldOrientation::Horizontal => self.0.flex_row().items_center(),
            FieldOrientation::Responsive => {
                let wide = signal(false);
                self.0
                    .on_layout(move |bounds: &Bounds<Points>| {
                        wide.set(bounds.size.width.0 >= 448.0);
                    })
                    .style_when(wide, |style| style.flex_row().items_center())
            }
        })
    }

    /// Styles the group and exposes its invalid state. Set the state on the control too.
    pub fn invalid(self, invalid: impl IntoReactive<bool>) -> Self {
        let invalid = invalid.into_reactive();
        let state = invalid.clone();
        Self(
            self.0
                .accessibility_state(move || AccessibilityState {
                    invalid: state.get(),
                    ..Default::default()
                })
                .style_when(invalid, |style| style.text_color(theme().destructive)),
        )
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldSet {
    pub fn new(legend: FieldLegend) -> Self {
        Self(
            view()
                .w_full()
                .min_w(px(0.0))
                .role(AccessibilityRole::Group)
                .label(legend.label.clone())
                .gap_3()
                .child(legend),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

pub struct FieldLegend {
    label: Reactive<SharedString>,
    variant: FieldLegendVariant,
}

impl FieldLegend {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: label.into_reactive(),
            variant: FieldLegendVariant::Legend,
        }
    }

    pub fn variant(mut self, variant: FieldLegendVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl IntoElement for FieldLegend {
    fn into_element(self) -> AnyElement {
        text(self.label)
            .text_size(px(if self.variant == FieldLegendVariant::Label {
                14.0
            } else {
                16.0
            }))
            .font_weight(FontWeight::MEDIUM)
            .into_element()
    }
}

impl FieldGroup {
    pub fn new() -> Self {
        Self(view().w_full().min_w(px(0.0)).gap_6())
    }
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldContent {
    pub fn new() -> Self {
        Self(view().min_w(px(0.0)).flex_1().gap_1())
    }
}

impl Default for FieldContent {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldLabel {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self::render(FieldTitle::new(label))
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self(view().min_w(px(0.0)).gap_2().child(content))
    }

    /// Focuses an associated control when the visible label is pressed.
    /// Give the control its accessible name with Native Shell's label property.
    pub fn for_control(self, control: ElementRef) -> Self {
        Self(self.0.on_press(move || control.focus()).focusable(false))
    }
}

pub struct FieldTitle(Text);

impl FieldTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(text(title).text_sm().font_weight(FontWeight::MEDIUM))
    }
}

impl IntoElement for FieldTitle {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

impl FieldDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self::render(text(description).text_sm())
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self(
            view()
                .min_w(px(0.0))
                .text_color(move || theme().muted_foreground)
                .child(content),
        )
    }
}

pub struct FieldSeparator(View);

impl FieldSeparator {
    pub fn new() -> Self {
        Self::line(None)
    }

    pub fn labeled(label: impl IntoReactive<SharedString>) -> Self {
        Self::line(Some(label.into_reactive()))
    }

    fn line(label: Option<Reactive<SharedString>>) -> Self {
        let rule = || view().h(px(1.0)).flex_1().bg(move || theme().border);
        Self(
            view()
                .w_full()
                .flex_row()
                .items_center()
                .gap_2()
                .accessibility_elements_hidden(true)
                .child(rule())
                .child(label.map(|label| {
                    text(label)
                        .text_sm()
                        .text_color(move || theme().muted_foreground)
                }))
                .child(rule()),
        )
    }
}

impl Default for FieldSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for FieldSeparator {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

impl FieldError {
    pub fn new(message: impl IntoReactive<SharedString>) -> Self {
        Self::render(text(message).text_sm())
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self(
            view()
                .role(AccessibilityRole::Alert)
                .text_color(move || theme().destructive)
                .child(content),
        )
    }

    /// Shows each non-empty error once, preserving the validator's message order.
    pub fn messages(messages: impl IntoReactive<Vec<String>> + 'static) -> impl IntoElement {
        let messages = messages.into_reactive();
        dynamic(move || {
            let messages = unique_messages(messages.get());
            if messages.is_empty() {
                return None;
            }
            if messages.len() == 1 {
                return Some(Self::new(messages.into_iter().next().unwrap()).into_element());
            }
            let list =
                view()
                    .role(AccessibilityRole::List)
                    .gap_1()
                    .children(messages.into_iter().map(|message| {
                        view()
                            .flex_row()
                            .gap_2()
                            .child(
                                text("•")
                                    .text_sm()
                                    .flex_shrink_0()
                                    .accessibility_elements_hidden(true),
                            )
                            .child(text(message).text_sm().flex_1())
                    }));
            Some(Self::render(list).into_element())
        })
    }
}

fn unique_messages(messages: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for message in messages {
        if !message.trim().is_empty() && !unique.contains(&message) {
            unique.push(message);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::unique_messages;

    #[test]
    fn field_errors_keep_the_original_order_and_skip_empty_duplicates() {
        let messages = ["Missing name", "", "Missing ZIP", "Missing name", "  "].map(str::to_owned);
        assert_eq!(
            unique_messages(messages.to_vec()),
            ["Missing name", "Missing ZIP"]
        );
    }
}
