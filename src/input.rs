//! Rhea-styled Native Shell text input and file picker.

use std::rc::Rc;

use nsh_base::theme;
use nsh_core::{
    AccessibilityState,
    element::{Input, input as native_input, pick_files},
    picker::{FilePickRequest, PickedFile},
    prelude::*,
};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, use_direction},
};

/// A styled native input. Configure its label, value, keyboard, and events directly.
pub fn input() -> Input {
    invalid_input(false)
}

/// Adds reactive validation styling and the matching accessible invalid state.
pub fn invalid_input(invalid: impl IntoReactive<bool>) -> Input {
    let invalid = computed({
        let invalid = invalid.into_reactive();
        move || invalid.get()
    });
    let direction = use_direction();
    native_input()
        .w_full()
        .min_w(px(0.0))
        .h(px(32.0))
        .px(px(10.0))
        .rounded(px(16.0))
        .border(px(1.0))
        .border_color(move || {
            if invalid.get() {
                theme().destructive.opacity(if dark() { 0.5 } else { 1.0 })
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .bg(move || theme().input.opacity(0.5))
        .text_color(move || theme().foreground)
        .style_when(
            move || direction.get() == Direction::Rtl,
            |style| style.text_right(),
        )
        .text_size(move || px(if window().width.0 < 768.0 { 16.0 } else { 14.0 }))
        .transition()
        .duration(200.0)
        .disabled_style(|style| style.opacity(0.5))
        .focus_visible(move |style| {
            style
                .border_color(move || {
                    let theme = theme();
                    if invalid.get() {
                        theme.destructive.opacity(if dark() { 0.5 } else { 1.0 })
                    } else {
                        theme.ring
                    }
                })
                .ring(px(3.0))
                .ring_color(move || {
                    let theme = theme();
                    if invalid.get() {
                        theme.destructive.opacity(if dark() { 0.4 } else { 0.2 })
                    } else {
                        theme.ring.opacity(0.3)
                    }
                })
        })
        .style_when(invalid, |style| {
            style
                .ring(px(3.0))
                .ring_color(theme().destructive.opacity(if dark() { 0.4 } else { 0.2 }))
        })
        .accessibility_state(move || AccessibilityState {
            invalid: invalid.get(),
            ..Default::default()
        })
}

/// Opens the system picker and forwards a nonempty result. Cancelling keeps the current value.
pub fn file_input(
    label: impl IntoReactive<SharedString>,
    request: FilePickRequest,
    on_pick: impl Fn(Vec<PickedFile>) + 'static,
) -> View {
    let all_names = signal(SharedString::from("No file chosen"));
    let picking = signal(false);
    let on_pick = Rc::new(on_pick);
    view()
        .w_full()
        .min_w(px(0.0))
        .h(px(32.0))
        .flex_row()
        .items_center()
        .gap_1()
        .px_2()
        .rounded(px(16.0))
        .bg(move || theme().input.opacity(0.5))
        .focus_within(|style| style.ring(px(3.0)).ring_color(theme().ring.opacity(0.3)))
        .child(
            Button::new(label)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::ExtraSmall)
                .disabled(picking)
                .on_press(move || {
                    if picking.get_untracked() {
                        return;
                    }
                    picking.set(true);
                    let pending = pick_files(request.clone());
                    let callback = Rc::clone(&on_pick);
                    spawn(async move {
                        let files = pending.await;
                        picking.set(false);
                        if !files.is_empty() {
                            all_names.set(
                                files
                                    .iter()
                                    .map(|file| file.name.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                                    .into(),
                            );
                            callback(files);
                        }
                    });
                }),
        )
        .child(text(all_names).text_sm().min_w(px(0.0)).truncate())
}
