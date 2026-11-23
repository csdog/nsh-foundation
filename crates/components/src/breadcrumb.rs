//! Composable, accessible breadcrumb navigation.

use nsh_base::theme;
use nsh_core::{
    AccessibilityCurrent,
    element::{LayoutDirection, layout_rtl},
    prelude::*,
};

pub struct Breadcrumb {
    label: &'static str,
    list: BreadcrumbList,
}

impl Breadcrumb {
    pub fn new(list: BreadcrumbList) -> Self {
        Self {
            label: "Breadcrumb",
            list,
        }
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }
}

impl IntoElement for Breadcrumb {
    fn into_element(self) -> AnyElement {
        view()
            .role(AccessibilityRole::Navigation)
            .label(self.label)
            .max_w(percentage(1.0))
            .min_w(px(0.0))
            .child(self.list)
            .into_element()
    }
}

#[derive(Default)]
pub struct BreadcrumbList {
    parts: Vec<Part>,
    direction: LayoutDirection,
}

enum Part {
    Item(BreadcrumbItem),
    Separator(BreadcrumbSeparator),
}

impl BreadcrumbList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn child(mut self, item: BreadcrumbItem) -> Self {
        self.parts.push(Part::Item(item));
        self
    }

    pub fn separator(mut self, separator: BreadcrumbSeparator) -> Self {
        self.parts.push(Part::Separator(separator));
        self
    }
}

impl IntoElement for BreadcrumbList {
    fn into_element(self) -> AnyElement {
        let mut list = view()
            .role(AccessibilityRole::List)
            .direction(self.direction)
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .min_w(px(0.0))
            .text_color(move || theme().muted_foreground);
        for part in self.parts {
            list = match part {
                Part::Item(item) => list.child(item.0),
                Part::Separator(separator) => list.child(separator.render(self.direction)),
            };
        }
        list.into_element()
    }
}

pub struct BreadcrumbItem(View);

impl BreadcrumbItem {
    pub fn new(child: impl IntoElement) -> Self {
        Self(view().flex_row().items_center().gap(px(6.0)).child(child))
    }
}

pub struct BreadcrumbLink(View);

impl BreadcrumbLink {
    pub fn new(label: &'static str, href: &'static str) -> Self {
        Self::action(label, move || {
            let _ = open_url(href);
        })
    }

    pub fn action(label: &'static str, activate: impl Fn() + 'static) -> Self {
        Self::render(label, view().child(text(label).text_sm()), activate)
    }

    pub fn render(label: &'static str, surface: View, activate: impl Fn() + 'static) -> Self {
        Self(
            surface
                .role(AccessibilityRole::Link)
                .label(label)
                .cursor_pointer()
                .rounded(px(4.0))
                .hover(|s| s.text_color(theme().foreground))
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                .on_press(activate),
        )
    }
}

impl IntoElement for BreadcrumbLink {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct BreadcrumbPage(&'static str);

impl BreadcrumbPage {
    pub fn new(label: &'static str) -> Self {
        Self(label)
    }
}

impl IntoElement for BreadcrumbPage {
    fn into_element(self) -> AnyElement {
        text(self.0)
            .role(AccessibilityRole::Text)
            .accessibility_state(AccessibilityState {
                current: Some(AccessibilityCurrent::Page),
                ..Default::default()
            })
            .text_sm()
            .text_color(move || theme().foreground)
            .into_element()
    }
}

#[derive(Default)]
pub struct BreadcrumbSeparator(Option<AnyElement>);

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_with(icon: impl IntoElement) -> Self {
        Self(Some(icon.into_element()))
    }

    fn render(self, direction: LayoutDirection) -> View {
        let icon = self.0.unwrap_or_else(|| {
            text(move || match direction {
                LayoutDirection::Rtl => "‹",
                LayoutDirection::Ltr => "›",
                _ if layout_rtl() => "‹",
                _ => "›",
            })
            .text_sm()
            .into_element()
        });
        view()
            .accessibility_elements_hidden(true)
            .focusable(false)
            .w(px(14.0))
            .items_center()
            .justify_center()
            .child(icon)
    }
}

#[derive(Default)]
pub struct BreadcrumbEllipsis;

impl BreadcrumbEllipsis {
    pub fn new() -> Self {
        Self
    }

    pub fn surface() -> View {
        view()
            .w(px(20.0))
            .h(px(20.0))
            .items_center()
            .justify_center()
            .child(text("…").text_sm())
    }
}

impl IntoElement for BreadcrumbEllipsis {
    fn into_element(self) -> AnyElement {
        Self::surface()
            .accessible(true)
            .label("More pages")
            .into_element()
    }
}
