//! Composable activity records for a process or conversation.

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::{
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    direction::{Direction, use_direction},
    marker::shine,
};

pub struct Trace(View);

impl Trace {
    pub fn new() -> Self {
        Self(view().w_full().min_w(px(0.0)).gap_2().test_id("trace"))
    }

    pub fn child(self, child: impl IntoElement) -> Self {
        Self(self.0.child(child))
    }

    pub fn children(self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        Self(self.0.children(children))
    }

    pub fn surface(self) -> View {
        self.0
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Trace {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TraceStatus {
    #[default]
    Default,
    Running,
    Error,
    Pending,
}

impl IntoReactive<TraceStatus> for TraceStatus {
    fn into_reactive(self) -> Reactive<TraceStatus> {
        Reactive::Static(self)
    }
}

pub struct TraceLine {
    title: TraceTitle,
    icon: Option<TraceIcon>,
    indicator: Option<TraceIcon>,
    description: Option<TraceDescription>,
    status: Reactive<TraceStatus>,
}

impl TraceLine {
    pub fn new(title: TraceTitle) -> Self {
        Self {
            title,
            icon: None,
            indicator: None,
            description: None,
            status: TraceStatus::Default.into_reactive(),
        }
    }

    pub fn icon(mut self, icon: TraceIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn indicator(mut self, icon: TraceIcon) -> Self {
        self.indicator = Some(icon);
        self
    }

    pub fn description(mut self, description: TraceDescription) -> Self {
        self.description = Some(description);
        self
    }

    pub fn status(mut self, status: impl IntoReactive<TraceStatus>) -> Self {
        self.status = status.into_reactive();
        self
    }

    pub fn surface(self) -> View {
        self.row(false)
    }

    /// A focusable row for use with a dialog or sheet trigger.
    pub fn trigger(self) -> View {
        let label = self.title.0.clone();
        self.row(true).label(label).focusable(true)
    }

    /// An external resource, announced as opening outside the current page.
    pub fn link(self, uri: &'static str) -> View {
        let title = self.title.0.clone();
        self.indicator(TraceIcon::glyph("↗"))
            .row(true)
            .role(AccessibilityRole::Link)
            .focusable(true)
            .label(move || SharedString::from(format!("{} (opens in a new tab)", title.get())))
            .on_press(move || {
                let _ = open_url(uri);
            })
    }

    fn row(self, interactive: bool) -> View {
        let status = self.status;
        let running = computed({
            let status = status.clone();
            move || status.get() == TraceStatus::Running
        });
        let error = computed({
            let status = status.clone();
            move || status.get() == TraceStatus::Error
        });
        let title = self.title.0;
        let title_for_shine = title.clone();
        view()
            .direction(use_direction().map(LayoutDirection::from))
            .self_start()
            .max_w(percentage(1.0))
            .min_w(px(0.0))
            .min_h(px(20.0))
            .flex_row()
            .items_center()
            .gap_2()
            .rounded_lg()
            .text_color(move || {
                if error.get() {
                    theme().destructive
                } else {
                    theme().muted_foreground
                }
            })
            .opacity(move || {
                if status.get() == TraceStatus::Pending {
                    0.60_f32
                } else {
                    1.0
                }
            })
            .when(interactive, |line| {
                line.cursor_pointer()
                    .hover(move |style| {
                        style.text_color(move || {
                            if error.get() {
                                theme().destructive
                            } else {
                                theme().foreground
                            }
                        })
                    })
                    .focus_visible(|style| {
                        style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3))
                    })
            })
            .test_id("trace-line")
            .child(self.icon)
            .child(
                view()
                    .relative()
                    .min_w(px(0.0))
                    .flex_shrink(1.0)
                    .overflow_hidden()
                    .child(
                        text(title)
                            .text_sm()
                            .line_height(px(20.0))
                            .number_of_lines(1)
                            .truncate()
                            .test_id("trace-title"),
                    )
                    .child(when(
                        move || running.get() && !reduce_motion(),
                        move || shine(title_for_shine.clone()),
                    )),
            )
            .child(self.description)
            .child(self.indicator)
    }
}

impl IntoElement for TraceLine {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct TraceTitle(Reactive<SharedString>);

impl TraceTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(title.into_reactive())
    }
}

pub struct TraceIcon(AnyElement);

impl TraceIcon {
    pub fn new(child: impl IntoElement) -> Self {
        Self(child.into_element())
    }

    pub fn glyph(glyph: &'static str) -> Self {
        Self::new(text(glyph).text_sm())
    }
}

impl IntoElement for TraceIcon {
    fn into_element(self) -> AnyElement {
        view()
            .size(px(16.0))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .pointer_events(PointerEvents::None)
            .accessibility_elements_hidden(true)
            .test_id("trace-icon")
            .child(self.0)
            .into_element()
    }
}

pub struct TraceDescription(Reactive<SharedString>);

impl TraceDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self(description.into_reactive())
    }
}

impl IntoElement for TraceDescription {
    fn into_element(self) -> AnyElement {
        text(self.0)
            .flex_shrink_0()
            .text_sm()
            .number_of_lines(1)
            .text_color(|| theme().muted_foreground.opacity(0.70))
            .test_id("trace-description")
            .into_element()
    }
}

pub struct TraceCollapsibleTrigger(TraceLine);

impl TraceCollapsibleTrigger {
    pub fn new(line: TraceLine) -> Self {
        Self(line)
    }
}

pub struct TraceCollapsibleContent(AnyElement);

impl TraceCollapsibleContent {
    pub fn new(content: impl IntoElement) -> Self {
        Self(content.into_element())
    }
}

pub struct TraceCollapsible(Collapsible);

impl TraceCollapsible {
    pub fn new(trigger: TraceCollapsibleTrigger, content: TraceCollapsibleContent) -> Self {
        let direction = use_direction();
        let trigger = CollapsibleTrigger::render(move |open| {
            let chevron = text("›");
            let direction = use_direction();
            trigger
                .0
                .indicator(TraceIcon::new(
                    chevron
                        .text_base()
                        .style_with(move |style| {
                            let angle = if open.get() {
                                90.0
                            } else if direction.get() == Direction::Rtl {
                                180.0
                            } else {
                                0.0
                            };
                            style.rotate(deg(angle))
                        })
                        .style_when(
                            || !reduce_motion(),
                            |style| style.transition_transform().duration(150.0),
                        ),
                ))
                .trigger()
                .test_id("trace-trigger")
        });
        let content = CollapsibleContent::new(
            view()
                .min_w(px(0.0))
                .mt_2()
                .mb_1()
                .pr_1()
                .text_color(|| theme().muted_foreground)
                .style_with(move |style| {
                    if direction.get() == Direction::Rtl {
                        style.mr(px(7.0)).border_r(px(1.0)).pr_4()
                    } else {
                        style.ml(px(7.0)).border_l(px(1.0)).pl_4()
                    }
                })
                .border_color(|| theme().border)
                .test_id("trace-disclosure")
                .child(content.0),
        );
        Self(Collapsible::new(trigger, content))
    }

    pub fn default_open(self, open: bool) -> Self {
        Self(self.0.default_open(open))
    }

    pub fn open(self, open: impl IntoReactive<bool>) -> Self {
        Self(self.0.open(open))
    }

    pub fn on_open_change(self, change: impl Fn(bool) + 'static) -> Self {
        Self(self.0.on_open_change(change))
    }
}

impl IntoElement for TraceCollapsible {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}
