//! Visible labels that direct presses to their associated Native Shell control.

use nsh_core::prelude::*;

pub struct Label {
    surface: View,
    target: Option<ElementRef>,
    activate: Option<Box<dyn Fn()>>,
    disabled: Reactive<bool>,
}

impl Label {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self::render(
            text(label)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .selectable(false),
        )
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self::from_surface(
            view()
                .flex_row()
                .items_center()
                .min_w(px(0.0))
                .gap_2()
                .child(content),
        )
    }

    pub(crate) fn from_surface(surface: View) -> Self {
        Self {
            surface,
            target: None,
            activate: None,
            disabled: false.into_reactive(),
        }
    }

    /// Focuses the associated control. Set the control's accessible name separately.
    pub fn for_control(mut self, control: ElementRef) -> Self {
        self.target = Some(control);
        self
    }

    /// Runs a control's action when the label is pressed, such as toggling a checkbox.
    pub fn on_activate(mut self, activate: impl Fn() + 'static) -> Self {
        self.activate = Some(Box::new(activate));
        self
    }

    pub fn disabled(mut self, disabled: impl IntoReactive<bool>) -> Self {
        self.disabled = disabled.into_reactive();
        self
    }

    pub fn surface(self) -> View {
        let disabled = computed(move || self.disabled.get());
        let surface = self.surface.style_when(disabled, |style| {
            style.opacity(0.5).cursor(Cursor::NotAllowed)
        });
        let target = self.target;
        let activate = self.activate;
        if target.is_none() && activate.is_none() {
            return surface;
        }
        surface
            .style_when(move || !disabled.get(), |style| style.cursor_pointer())
            .on_press(move || {
                if disabled.get_untracked() {
                    return;
                }
                if let Some(control) = target {
                    control.focus();
                }
                if let Some(activate) = &activate {
                    activate();
                }
            })
            .role(AccessibilityRole::Text)
            .focusable(false)
    }
}

impl IntoElement for Label {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

#[cfg(test)]
mod tests {
    use nsh_core::element::Root;
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;
    use crate::button::Button;

    #[component]
    fn Labels() -> impl IntoElement {
        let disabled = signal(true);
        let toggled = signal(false);
        let control = element_ref();
        view()
            .child(
                Label::new("Visible label")
                    .for_control(control)
                    .disabled(disabled)
                    .on_activate(move || toggled.update(|value| *value = !*value))
                    .surface()
                    .test_id("label-trigger"),
            )
            .child(Button::new("Enable control").on_press(move || disabled.set(false)))
            .child(
                nsh_core::element::input()
                    .label("Related control")
                    .element_ref(&control)
                    .disabled(disabled)
                    .h(px(32.0))
                    .w(px(180.0)),
            )
            .child(text(
                move || if toggled.get() { "Activated" } else { "Idle" },
            ))
    }

    #[test]
    fn a_disabled_label_suppresses_activation_until_enabled() {
        let mut app = Headless::start(EngineConfig::new(Root::of::<Labels>())).unwrap();
        app.first_render().unwrap();
        app.press("id:label-trigger").unwrap();
        assert!(app.shows("Idle"));
        app.press("Enable control").unwrap();
        app.press("id:label-trigger").unwrap();
        assert!(app.shows("Activated"));
    }
}
