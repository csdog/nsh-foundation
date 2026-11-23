//! Composable, theme-aware callouts.

use nsh_base::theme;
use nsh_core::{element::LayoutDirection, prelude::*};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(Clone, Copy, Debug)]
pub struct AlertColors {
    pub background: AdaptiveColor,
    pub border: AdaptiveColor,
    pub foreground: AdaptiveColor,
    pub description: AdaptiveColor,
}

#[derive(Default)]
pub struct Alert {
    variant: AlertVariant,
    direction: LayoutDirection,
    colors: Option<AlertColors>,
    icon: Option<AnyElement>,
    title: Option<AlertTitle>,
    description: Option<AlertDescription>,
    action: Option<AlertAction>,
}

impl Alert {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn colors(mut self, colors: AlertColors) -> Self {
        self.colors = Some(colors);
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    pub fn title(mut self, title: AlertTitle) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: AlertDescription) -> Self {
        self.description = Some(description);
        self
    }

    pub fn action(mut self, action: AlertAction) -> Self {
        self.action = Some(action);
        self
    }
}

impl IntoElement for Alert {
    fn into_element(self) -> AnyElement {
        let variant = self.variant;
        let colors = move || {
            self.colors.unwrap_or_else(|| {
                let t = theme();
                let destructive = variant == AlertVariant::Destructive;
                AlertColors {
                    background: t.card,
                    border: t.border,
                    foreground: if destructive {
                        t.destructive
                    } else {
                        t.card_foreground
                    },
                    description: if destructive {
                        t.destructive.opacity(0.90)
                    } else {
                        t.muted_foreground
                    },
                }
            })
        };
        let mut frame = view()
            .role(AccessibilityRole::Alert)
            .direction(self.direction)
            .w_full()
            .flex_row()
            .items_start()
            .gap(px(10.0))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || colors().border)
            .bg(move || colors().background)
            .text_color(move || colors().foreground)
            .px_4()
            .py_3();

        if let Some(icon) = self.icon {
            frame = frame.child(
                view()
                    .flex_shrink_0()
                    .pt(px(2.0))
                    .accessibility_elements_hidden(true)
                    .child(icon),
            );
        }
        let mut body = view().flex_1().min_w(px(0.0)).gap(px(2.0));
        if let Some(title) = self.title {
            body = body.child(title);
        }
        if let Some(description) = self.description {
            body = body.child(
                view()
                    .text_color(move || colors().description)
                    .child(description),
            );
        }
        frame = frame.child(body);
        if let Some(action) = self.action {
            frame = frame.child(action);
        }
        frame.into_element()
    }
}

pub struct AlertTitle(Text);

impl AlertTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Self {
        Self(text(title).text_sm().font_weight(FontWeight::MEDIUM))
    }
}

impl IntoElement for AlertTitle {
    fn into_element(self) -> AnyElement {
        self.0.into_element()
    }
}

pub struct AlertDescription(AnyElement);

impl AlertDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Self {
        Self::render(text(description).text_sm())
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self(content.into_element())
    }
}

impl IntoElement for AlertDescription {
    fn into_element(self) -> AnyElement {
        self.0
    }
}

pub struct AlertAction(AnyElement);

impl AlertAction {
    pub fn new(action: impl IntoElement) -> Self {
        Self(action.into_element())
    }
}

impl IntoElement for AlertAction {
    fn into_element(self) -> AnyElement {
        view().flex_shrink_0().child(self.0).into_element()
    }
}
