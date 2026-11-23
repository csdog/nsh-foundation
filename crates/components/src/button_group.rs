//! Join adjacent native controls while preserving each control's semantics.

use nsh_base::theme;
use nsh_core::{
    element::{FluentBuilder, LayoutDirection},
    prelude::*,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct ButtonGroup {
    label: &'static str,
    orientation: ButtonGroupOrientation,
    direction: LayoutDirection,
    parts: Vec<Part>,
}

enum Part {
    Control(Box<dyn FnOnce(ButtonGroupJoin) -> AnyElement>),
    Separator(ButtonGroupSeparator),
    Separate(AnyElement),
}

impl ButtonGroup {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            orientation: ButtonGroupOrientation::Horizontal,
            direction: LayoutDirection::Inherit,
            parts: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Join a styled surface, such as a button or text input.
    pub fn child<E: Styled + FluentBuilder + IntoElement + 'static>(self, child: E) -> Self {
        self.control(move |join| join.apply(child))
    }

    /// Apply the join to the visible trigger inside a menu, popover, or similar control.
    pub fn control<E: IntoElement>(
        mut self,
        make: impl FnOnce(ButtonGroupJoin) -> E + 'static,
    ) -> Self {
        self.parts.push(Part::Control(Box::new(move |join| {
            make(join).into_element()
        })));
        self
    }

    pub fn separator(mut self, separator: ButtonGroupSeparator) -> Self {
        self.parts.push(Part::Separator(separator));
        self
    }

    /// Include a subgroup or a standalone control with space around the join.
    pub fn separate(mut self, group: impl IntoElement) -> Self {
        self.parts.push(Part::Separate(group.into_element()));
        self
    }
}

impl IntoElement for ButtonGroup {
    fn into_element(self) -> AnyElement {
        let count = self.parts.len();
        let vertical = self.orientation == ButtonGroupOrientation::Vertical;
        let mut group = view()
            .role(AccessibilityRole::Group)
            .label(self.label)
            .direction(self.direction)
            .min_w(px(0.0))
            .items_stretch()
            .when(!vertical, |view| view.flex_row());
        let joined = self
            .parts
            .iter()
            .map(|part| !matches!(part, Part::Separate(_)))
            .collect::<Vec<_>>();
        for (index, part) in self.parts.into_iter().enumerate() {
            let attached_before = index > 0 && joined[index - 1];
            let attached_after = index + 1 < count && joined[index + 1];
            let child = match part {
                Part::Control(make) => make(ButtonGroupJoin {
                    orientation: self.orientation,
                    before: attached_before,
                    after: attached_after,
                }),
                Part::Separator(separator) => separator.render(vertical).into_element(),
                Part::Separate(child) => child,
            };
            if index > 0 && !(joined[index] && attached_before) {
                group = group.child(
                    view()
                        .w(px(if vertical { 0.0 } else { 8.0 }))
                        .h(px(if vertical { 8.0 } else { 0.0 }))
                        .flex_shrink_0()
                        .accessibility_elements_hidden(true),
                );
            }
            group = group.child(child);
        }
        group.into_element()
    }
}

/// The adjacent edges to apply to the control's visible surface.
#[derive(Clone, Copy)]
pub struct ButtonGroupJoin {
    orientation: ButtonGroupOrientation,
    before: bool,
    after: bool,
}

impl ButtonGroupJoin {
    pub fn apply<E: Styled + FluentBuilder>(self, surface: E) -> E {
        let square = px(0.0);
        match self.orientation {
            ButtonGroupOrientation::Horizontal => surface
                .when(self.before, |s| s.rounded_s(square).border_s(square))
                .when(self.after, |s| s.rounded_e(square)),
            ButtonGroupOrientation::Vertical => surface
                .when(self.before, |s| s.rounded_t(square).border_t(square))
                .when(self.after, |s| s.rounded_b(square)),
        }
    }
}

#[derive(Default)]
pub struct ButtonGroupSeparator;

impl ButtonGroupSeparator {
    pub fn new() -> Self {
        Self
    }

    fn render(self, vertical: bool) -> View {
        view()
            .self_stretch()
            .flex_shrink_0()
            .accessibility_elements_hidden(true)
            .bg(move || theme().input)
            .when(vertical, |line| line.h(px(1.0)).mx_1())
            .when(!vertical, |line| line.w(px(1.0)).my_1())
    }
}

pub struct ButtonGroupText;

impl ButtonGroupText {
    pub fn new(label: impl IntoReactive<SharedString>) -> View {
        Self::render(text(label).text_sm().font_weight(FontWeight::MEDIUM))
    }

    pub fn render(content: impl IntoElement) -> View {
        view()
            .flex_row()
            .items_center()
            .gap_2()
            .px_3()
            .min_h(px(32.0))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || theme().border)
            .bg(move || theme().muted)
            .text_color(move || theme().foreground)
            .child(content)
    }
}
