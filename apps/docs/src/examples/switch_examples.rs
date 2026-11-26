use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    field::{Field, FieldContent, FieldDescription, FieldOrientation, FieldTitle},
    label::Label,
    switch::{Switch, SwitchSize, switch},
};
use nsh_base::theme;
use nsh_core::prelude::*;

const SHARE: &str = "Share across devices";
const SHARE_DESCRIPTION: &str =
    "Focus is shared across devices, and turns off when you leave the app.";

#[derive(Clone, Copy)]
struct Setting {
    checked: Signal<bool>,
    target: ElementRef,
}

impl Setting {
    fn new(checked: bool) -> Self {
        Self {
            checked: signal(checked),
            target: element_ref(),
        }
    }

    fn toggle(self) {
        self.checked.update(|checked| *checked = !*checked);
        let target = self.target;
        spawn(async move {
            next_frame().await;
            target.focus();
        });
    }

    fn control(self, label: impl IntoReactive<SharedString>) -> Switch {
        switch()
            .accessibility_label(label)
            .checked(self.checked)
            .control_ref(self.target)
            .on_change(move |checked| self.checked.set(checked))
    }

    fn label(self, title: impl IntoReactive<SharedString>) -> Label {
        Label::new(title).on_activate(move || self.toggle())
    }
}

fn labelled(title: &'static str, size: SwitchSize, disabled: bool) -> View {
    let setting = Setting::new(false);
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .child(setting.control(title).size(size).disabled(disabled))
        .child(
            setting
                .label(title)
                .disabled(disabled)
                .surface()
                .test_id("switch-label"),
        )
}

pub fn demo() -> View {
    labelled("Airplane Mode", SwitchSize::Default, false)
}

fn description_field(
    setting: Setting,
    title: impl IntoReactive<SharedString>,
    description: impl IntoReactive<SharedString>,
    invalid: impl IntoReactive<bool>,
) -> View {
    let title = title.into_reactive();
    let description = description.into_reactive();
    let invalid = computed({
        let invalid = invalid.into_reactive();
        move || invalid.get()
    });
    view().w_full().max_w(px(384.0)).child(
        Field::new()
            .orientation(FieldOrientation::Horizontal)
            .invalid(invalid)
            .child(
                FieldContent::new()
                    .child(
                        setting
                            .label(title.clone())
                            .surface()
                            .test_id("switch-label"),
                    )
                    .child(FieldDescription::new(description.clone())),
            )
            .child(
                setting
                    .control(title)
                    .description(description)
                    .invalid(invalid),
            ),
    )
}

pub fn description() -> View {
    description_field(Setting::new(false), SHARE, SHARE_DESCRIPTION, false)
}

pub fn choice_card() -> View {
    let card = |title, description, default_checked| {
        let setting = Setting::new(default_checked);
        view()
            .w_full()
            .p_4()
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || {
                if setting.checked.get() {
                    theme().primary
                } else {
                    theme().border
                }
            })
            .bg(move || {
                if setting.checked.get() {
                    theme().primary.opacity(0.05)
                } else {
                    theme().background
                }
            })
            .hover(|style| style.bg(theme().muted.opacity(0.5)))
            .focusable(false)
            .role(AccessibilityRole::Group)
            .on_press(move || setting.toggle())
            .test_id("switch-choice-card")
            .child(
                Field::new()
                    .orientation(FieldOrientation::Horizontal)
                    .child(
                        FieldContent::new()
                            .child(FieldTitle::new(title))
                            .child(FieldDescription::new(description)),
                    )
                    .child(setting.control(title).description(description)),
            )
    };
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(card(SHARE, SHARE_DESCRIPTION, false))
        .child(card(
            "Enable notifications",
            "Receive notifications when focus mode is enabled or disabled.",
            true,
        ))
}

pub fn disabled() -> View {
    labelled("Disabled", SwitchSize::Default, true)
}

pub fn invalid() -> View {
    let setting = Setting::new(false);
    description_field(
        setting,
        "Accept terms and conditions",
        "You must accept the terms and conditions to continue.",
        move || !setting.checked.get(),
    )
}

pub fn sizes() -> View {
    view()
        .w_full()
        .max_w(px(160.0))
        .gap_6()
        .child(labelled("Small", SwitchSize::Small, false))
        .child(labelled("Default", SwitchSize::Default, false))
}

const LOCALES: [(Direction, &str, &str); 3] = [
    (Direction::Ltr, SHARE, SHARE_DESCRIPTION),
    (
        Direction::Rtl,
        "المشاركة عبر الأجهزة",
        "يتم مشاركة التركيز عبر الأجهزة، ويتم إيقاف تشغيله عند مغادرة التطبيق.",
    ),
    (
        Direction::Rtl,
        "שיתוף בין מכשירים",
        "המיקוד משותף בין מכשירים, וכבה כשאתה עוזב את האפליקציה.",
    ),
];

pub fn rtl() -> View {
    let setting = Setting::new(false);
    let language = signal(1_usize);
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_1()
                .justify_center()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || language.set(index))
                    },
                )),
        )
        .child(DirectionProvider::new(
            move || LOCALES[language.get()].0,
            move || {
                description_field(
                    setting,
                    move || LOCALES[language.get()].1,
                    move || LOCALES[language.get()].2,
                    false,
                )
            },
        ))
}
