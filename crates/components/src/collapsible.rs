//! Composable disclosures using Native Shell's layout reveal.

use nsh_base::collapsible;
use nsh_core::{AccessibilityState, prelude::*};

use crate::button::{Button, ButtonVariant};

pub struct Collapsible {
    trigger: CollapsibleTrigger,
    content: CollapsibleContent,
    header: Option<Box<dyn FnOnce(View) -> View>>,
    open: Option<Reactive<bool>>,
    default_open: bool,
    disabled: Reactive<bool>,
    change: Option<Box<dyn Fn(bool)>>,
    reveal: bool,
}

impl Collapsible {
    pub fn new(trigger: CollapsibleTrigger, content: CollapsibleContent) -> Self {
        Self {
            trigger,
            content,
            header: None,
            open: None,
            default_open: false,
            disabled: false.into_reactive(),
            change: None,
            reveal: true,
        }
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Controls whether the panel is open. Apply requested changes in on_open_change.
    pub fn open(mut self, open: impl IntoReactive<bool>) -> Self {
        self.open = Some(open.into_reactive());
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn on_open_change(mut self, change: impl Fn(bool) + 'static) -> Self {
        self.change = Some(Box::new(change));
        self
    }

    pub fn reveal(mut self, reveal: bool) -> Self {
        self.reveal = reveal;
        self
    }

    /// Places the trigger beside a title, summary, or other persistent controls.
    pub fn header(mut self, header: impl FnOnce(View) -> View + 'static) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    pub fn surface(self) -> View {
        let own = signal(self.default_open);
        let controlled = self.open.is_some();
        let open = self.open.unwrap_or_else(|| own.into_reactive());
        let open = computed(move || open.get());
        let disabled = self.disabled;
        let disabled = computed(move || disabled.get());
        let change = self.change;
        let panel = element_ref();
        let trigger = (self.trigger.0)(open)
            .role(AccessibilityRole::Button)
            .focusable(true)
            .disabled(move || disabled.get())
            .disabled_style(|style| style.opacity(0.5))
            .accessibility_state(move || AccessibilityState {
                expanded: Some(open.get()),
                disabled: disabled.get(),
                ..Default::default()
            })
            .accessibility_controls(move || open.get().then_some(panel))
            .on_press(move || {
                let next = !open.get_untracked();
                if !controlled {
                    own.set(next);
                }
                if let Some(change) = &change {
                    change(next);
                }
            });
        let header = match self.header {
            Some(header) => header(trigger),
            None => trigger,
        };
        view().min_w(px(0.0)).child(
            collapsible()
                .open(open)
                .reveal(self.reveal)
                .child(header)
                .content(
                    self.content
                        .0
                        .element_ref(&panel)
                        .accessibility_elements_hidden(move || !open.get())
                        .pointer_events(move || {
                            if open.get() {
                                PointerEvents::Auto
                            } else {
                                PointerEvents::None
                            }
                        }),
                ),
        )
    }
}

impl IntoElement for Collapsible {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct CollapsibleTrigger(Box<dyn FnOnce(Memo<bool>) -> View>);

impl CollapsibleTrigger {
    pub fn new(label: &'static str) -> Self {
        Self::render(move |open| {
            Button::new(label)
                .variant(ButtonVariant::Ghost)
                .icon_end(text(move || {
                    SharedString::from(if open.get() { "⌃" } else { "⌄" })
                }))
                .surface()
        })
    }

    /// Styles the control using the live open state.
    pub fn render(render: impl FnOnce(Memo<bool>) -> View + 'static) -> Self {
        Self(Box::new(render))
    }
}

pub struct CollapsibleContent(View);

impl CollapsibleContent {
    pub fn new(child: impl IntoElement) -> Self {
        Self(view().child(child))
    }
}
