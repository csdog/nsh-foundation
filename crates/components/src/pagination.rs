//! Composable page navigation with links or in-place Rust actions.

use nsh_base::{link, theme};
use nsh_core::{AccessibilityCurrent, element::FluentBuilder, prelude::*};

use crate::direction::{Direction, use_direction};

pub struct Pagination {
    label: SharedString,
    content: PaginationContent,
}

impl Pagination {
    pub fn new(content: PaginationContent) -> Self {
        Self {
            label: "Pagination".into(),
            content,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl IntoElement for Pagination {
    fn into_element(self) -> AnyElement {
        view()
            .role(AccessibilityRole::Navigation)
            .label(self.label)
            .max_w(percentage(1.0))
            .items_center()
            .child(self.content)
            .into_element()
    }
}

pub struct PaginationContent(View);

impl PaginationContent {
    pub fn new() -> Self {
        Self(
            view()
                .flex_row()
                .flex_wrap()
                .items_center()
                .justify_center()
                .gap_1(),
        )
    }

    pub fn item(mut self, item: PaginationItem) -> Self {
        self.0 = self.0.child(item);
        self
    }
}

impl Default for PaginationContent {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for PaginationContent {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct PaginationItem(View);

impl PaginationItem {
    pub fn new(content: impl IntoElement) -> Self {
        Self(view().flex_shrink_0().child(content))
    }
}

impl IntoElement for PaginationItem {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

enum Destination {
    Link(SharedString),
    Action(Box<dyn Fn()>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Previous,
    Next,
}

pub struct PaginationLink {
    text: Reactive<SharedString>,
    label: Option<Reactive<SharedString>>,
    destination: Destination,
    current: Reactive<bool>,
    disabled: Reactive<bool>,
    icon_only: bool,
}

impl PaginationLink {
    pub fn new(label: impl IntoReactive<SharedString>, href: impl Into<SharedString>) -> Self {
        Self::with_destination(label, Destination::Link(href.into()))
    }

    pub fn action(label: impl IntoReactive<SharedString>, action: impl Fn() + 'static) -> Self {
        Self::with_destination(label, Destination::Action(Box::new(action)))
    }

    fn with_destination(label: impl IntoReactive<SharedString>, destination: Destination) -> Self {
        Self {
            text: label.into_reactive(),
            label: None,
            destination,
            current: false.into_reactive(),
            disabled: false.into_reactive(),
            icon_only: false,
        }
    }

    pub fn label(mut self, label: impl IntoReactive<SharedString>) -> Self {
        self.label = Some(label.into_reactive());
        self
    }

    pub fn current(mut self, current: impl IntoReactive<bool>) -> Self {
        self.current = current.into_reactive();
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    /// Show only the chevron on a previous or next control.
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }

    fn render(self, step: Option<Step>) -> AnyElement {
        let Self {
            text: caption,
            label,
            destination,
            current,
            disabled,
            icon_only,
        } = self;
        let label = label.unwrap_or_else(|| match step {
            None => caption.clone(),
            Some(Step::Previous) => "Go to previous page".into_reactive(),
            Some(Step::Next) => "Go to next page".into_reactive(),
        });
        let show_step_text = move || !icon_only && window().width.0 >= 640.0;
        let caption = text(caption)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .when(step.is_some(), |caption| {
                caption.style_when(move || !show_step_text(), |style| style.hidden())
            });
        let contents = view()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_1()
            .accessibility_elements_hidden(true)
            .child((step == Some(Step::Previous)).then(|| chevron(Step::Previous)))
            .child(caption)
            .child((step == Some(Step::Next)).then(|| chevron(Step::Next)));
        match destination {
            Destination::Link(href) => {
                let surface = link()
                    .href(href)
                    .accessibility_label(move || Some(label.get()))
                    .current(current.clone())
                    .disabled(disabled)
                    .child(contents);
                style_control(surface, current, step, show_step_text)
                    .hover(|style| style.bg(theme().muted))
                    .focus_visible(|style| {
                        style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3))
                    })
                    .disabled_style(|style| style.opacity(0.5))
                    .into_element()
            }
            Destination::Action(action) => {
                let selected = current.clone();
                let surface = view()
                    .role(AccessibilityRole::Button)
                    .label(label)
                    .disabled(disabled)
                    .accessibility_state(move || AccessibilityState {
                        current: selected.get().then_some(AccessibilityCurrent::Page),
                        ..Default::default()
                    })
                    .on_press(action)
                    .child(contents);
                style_control(surface, current, step, show_step_text)
                    .hover(|style| style.bg(theme().muted))
                    .focus_visible(|style| {
                        style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3))
                    })
                    .disabled_style(|style| style.opacity(0.5))
                    .into_element()
            }
        }
    }
}

fn style_control<S: Styled + FluentBuilder>(
    surface: S,
    current: Reactive<bool>,
    step: Option<Step>,
    show_step_text: impl Fn() -> bool + Copy + 'static,
) -> S {
    surface
        .h(px(32.0))
        .min_w(px(32.0))
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .rounded(px(16.0))
        .border(px(1.0))
        .border_color(move || {
            if current.get() {
                theme().border
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .text_color(move || theme().foreground)
        .when(step.is_none(), |surface| surface.px_1())
        .when(step.is_some(), |surface| {
            surface.style_when(show_step_text, |surface| match step {
                Some(Step::Previous) => surface.ps(px(6.0)).pe(px(12.0)),
                _ => surface.ps(px(12.0)).pe(px(6.0)),
            })
        })
}

fn chevron(step: Step) -> View {
    let direction = use_direction();
    view().size(px(16.0)).items_center().justify_center().child(
        text(move || {
            if (step == Step::Previous) != (direction.get() == Direction::Rtl) {
                "‹"
            } else {
                "›"
            }
        })
        .text_size(px(18.0)),
    )
}

impl IntoElement for PaginationLink {
    fn into_element(self) -> AnyElement {
        self.render(None)
    }
}

pub struct PaginationPrevious(PaginationLink);

impl PaginationPrevious {
    pub fn new(link: PaginationLink) -> Self {
        Self(link)
    }
}

impl IntoElement for PaginationPrevious {
    fn into_element(self) -> AnyElement {
        self.0.render(Some(Step::Previous))
    }
}

pub struct PaginationNext(PaginationLink);

impl PaginationNext {
    pub fn new(link: PaginationLink) -> Self {
        Self(link)
    }
}

impl IntoElement for PaginationNext {
    fn into_element(self) -> AnyElement {
        self.0.render(Some(Step::Next))
    }
}

#[derive(Default)]
pub struct PaginationEllipsis;

impl PaginationEllipsis {
    pub fn new() -> Self {
        Self
    }
}

impl IntoElement for PaginationEllipsis {
    fn into_element(self) -> AnyElement {
        view()
            .size(px(32.0))
            .items_center()
            .justify_center()
            .accessibility_elements_hidden(true)
            .child(text("…").text_sm())
            .into_element()
    }
}
