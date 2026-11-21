//! Rhea button surfaces with text, icons, and native input semantics.

use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    ExtraSmall,
    Small,
    #[default]
    Default,
    Large,
    IconExtraSmall,
    IconSmall,
    Icon,
    IconLarge,
}

impl ButtonSize {
    fn metrics(self) -> (f32, f32, f32, bool) {
        match self {
            Self::ExtraSmall => (24.0, 10.0, 4.0, false),
            Self::Small => (28.0, 12.0, 4.0, false),
            Self::Default => (32.0, 12.0, 6.0, false),
            Self::Large => (36.0, 16.0, 6.0, false),
            Self::IconExtraSmall => (24.0, 0.0, 0.0, true),
            Self::IconSmall => (28.0, 0.0, 0.0, true),
            Self::Icon => (32.0, 0.0, 0.0, true),
            Self::IconLarge => (36.0, 0.0, 0.0, true),
        }
    }
}

pub struct Button {
    label: Reactive<SharedString>,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: f32,
    disabled: Reactive<bool>,
    loading: Reactive<bool>,
    start: Option<AnyElement>,
    end: Option<AnyElement>,
    cursor_pointer: bool,
}

impl Button {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self {
            label: label.into_reactive(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            radius: 16.0,
            disabled: Reactive::Static(false),
            loading: Reactive::Static(false),
            start: None,
            end: None,
            cursor_pointer: false,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, points: f32) -> Self {
        self.radius = points;
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn loading(mut self, loading: impl IntoReactive<bool>) -> Self {
        self.loading = loading.into_reactive();
        self
    }

    pub fn icon_start(mut self, icon: impl IntoElement) -> Self {
        self.start = Some(icon.into_element());
        self
    }

    pub fn icon_end(mut self, icon: impl IntoElement) -> Self {
        self.end = Some(icon.into_element());
        self
    }

    pub fn pointer_cursor(mut self) -> Self {
        self.cursor_pointer = true;
        self
    }

    pub fn on_press(self, press: impl Fn() + 'static) -> View {
        self.surface().on_press(press)
    }

    pub fn href(self, href: &'static str) -> View {
        self.surface()
            .role(AccessibilityRole::Link)
            .on_press(move || {
                let _ = open_url(href);
            })
    }

    /// Build a styled trigger for native menus and other compound controls.
    pub fn surface(self) -> View {
        let (height, padding, gap, icon_only) = self.size.metrics();
        let has_start = self.start.is_some();
        let has_end = self.end.is_some();
        let small_text = self.size == ButtonSize::ExtraSmall;
        let disabled = self.disabled;
        let loading = self.loading;
        let loading_for_state = loading.clone();
        let loading_for_style = loading.clone();
        let variant = self.variant;

        let mut surface = view()
            .role(AccessibilityRole::Button)
            .focusable(true)
            .label(self.label.clone())
            .flex_row()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .min_w(px(0.0))
            .h(px(height))
            .gap(px(gap))
            .when(icon_only, |view| view.w(px(height)))
            .pl(px(if has_start {
                (padding - 2.0).max(0.0)
            } else {
                padding
            }))
            .pr(px(if has_end {
                (padding - 2.0).max(0.0)
            } else {
                padding
            }))
            .rounded(px(self.radius))
            .border(px(1.0))
            .border_color(move || {
                if variant == ButtonVariant::Outline {
                    theme().border
                } else {
                    AdaptiveColor::TRANSPARENT
                }
            })
            .bg(move || colors(variant).0)
            .text_color(move || colors(variant).1)
            .when(self.cursor_pointer, |view| view.cursor_pointer())
            .disabled(move || disabled.get() || loading_for_style.get())
            .disabled_style(|style| style.opacity(0.5))
            .accessibility_state(move || AccessibilityState {
                busy: loading_for_state.get(),
                ..Default::default()
            })
            .hover(move |style| match variant {
                ButtonVariant::Link => style.underline().underline_offset(px(4.0)),
                ButtonVariant::Ghost | ButtonVariant::Outline => style.bg(theme().muted),
                ButtonVariant::Destructive => style.bg(theme().destructive.opacity(0.2)),
                _ => style.opacity(0.8),
            })
            .focus_visible(|style| style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3)))
            .child(when(loading.clone(), move || spinner(variant)));
        if let Some(icon) = self.start {
            let loading = loading.clone();
            surface = surface.child(
                view()
                    .accessibility_elements_hidden(true)
                    .style_when(move || loading.get(), |style| style.hidden())
                    .child(icon),
            );
        }
        if !icon_only {
            surface = surface.child(
                text(self.label)
                    .truncate()
                    .font_weight(FontWeight::MEDIUM)
                    .when(small_text, |label| label.text_xs())
                    .when(!small_text, |label| label.text_sm()),
            );
        }
        if let Some(icon) = self.end {
            surface = surface.child(view().accessibility_elements_hidden(true).child(icon));
        }
        surface
    }
}

impl IntoElement for Button {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

fn colors(variant: ButtonVariant) -> (AdaptiveColor, AdaptiveColor) {
    let t = theme();
    let transparent = AdaptiveColor::TRANSPARENT;
    match variant {
        ButtonVariant::Default => (t.primary, t.primary_foreground),
        ButtonVariant::Outline => (
            AdaptiveColor::new(t.background.light, transparent.dark),
            t.foreground,
        ),
        ButtonVariant::Secondary => (t.secondary, t.secondary_foreground),
        ButtonVariant::Ghost => (transparent, t.foreground),
        ButtonVariant::Destructive => (
            AdaptiveColor::new(
                t.destructive.light.opacity(0.1),
                t.destructive.dark.opacity(0.2),
            ),
            t.destructive,
        ),
        ButtonVariant::Link => (transparent, t.primary),
    }
}

pub(crate) fn spinner(variant: ButtonVariant) -> View {
    let rotation = animated_value(0.0);
    let motion =
        Animation::timing(&rotation, 360.0, Timing::new(800.0, Easing::Linear)).looped(None);
    effect({
        let motion = motion.clone();
        move || {
            if reduce_motion() {
                motion.stop();
            } else {
                let _ = motion.start();
            }
        }
    });
    cleanup(move || motion.stop());
    view()
        .w(px(12.0))
        .h(px(12.0))
        .rounded_full()
        .border(px(1.0))
        .border_color(move || colors(variant).1.opacity(0.30))
        .border_t_color(move || colors(variant).1)
        .accessibility_elements_hidden(true)
        .bind_rotate(&rotation)
}
