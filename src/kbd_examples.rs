use nsh_base::{theme, tooltip};
use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
    direction::{Direction, DirectionProvider},
    input_group::{InputGroup, InputGroupAddon, InputGroupAlign, input_group_input},
    kbd::{Kbd, KbdGroup, KbdTone, search_shortcut},
    ui::muted,
};

pub fn demo() -> impl IntoElement {
    view()
        .items_center()
        .gap_4()
        .child(
            KbdGroup::new()
                .child(Kbd::new("⌘").named("Command"))
                .child(Kbd::new("⇧").named("Shift"))
                .child(Kbd::new("⌥").named("Option"))
                .child(Kbd::new("⌃").named("Control")),
        )
        .child(
            KbdGroup::new()
                .child(Kbd::new("Ctrl").named("Control"))
                .child(text("+").text_sm())
                .child(Kbd::new("B")),
        )
}

pub fn group() -> impl IntoElement {
    view()
        .flex_row()
        .flex_wrap()
        .items_center()
        .justify_center()
        .gap_1()
        .child(muted("Use"))
        .child(
            KbdGroup::new()
                .child(Kbd::new("Ctrl + B"))
                .child(Kbd::new("Ctrl + K")),
        )
        .child(muted("to open the command palette"))
}

pub fn button_example() -> impl IntoElement {
    let accepted = signal(false);
    view()
        .items_center()
        .gap_2()
        .child(
            Button::new("Accept")
                .variant(ButtonVariant::Outline)
                .icon_end(Kbd::new("⏎").named("Enter"))
                .on_press(move || accepted.set(true)),
        )
        .child(when(accepted, || muted("Accepted in preview.")))
}

fn hint(title: &'static str, keys: KbdGroup) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .px_2()
        .py_1()
        .bg(move || theme().foreground)
        .text_color(move || theme().background)
        .rounded(px(8.0))
        .child(text(title).text_xs())
        .child(keys)
}

pub fn tooltip_example() -> impl IntoElement {
    let status = signal("");
    view()
        .gap_3()
        .items_center()
        .child(
            ButtonGroup::new("Document actions")
                .control(move |join| {
                    join.apply(
                        Button::new("Save")
                            .variant(ButtonVariant::Outline)
                            .on_press(move || status.set("Save selected in preview."))
                            .tooltip(tooltip().delay(180.0).child(hint(
                                "Save Changes",
                                KbdGroup::new().child(Kbd::new("S").tone(KbdTone::Tooltip)),
                            ))),
                    )
                })
                .control(move |join| {
                    join.apply(
                        Button::new("Print")
                            .variant(ButtonVariant::Outline)
                            .on_press(move || status.set("Print selected in preview."))
                            .tooltip(
                                tooltip().delay(180.0).child(hint(
                                    "Print Document",
                                    KbdGroup::new()
                                        .child(Kbd::new("Ctrl").tone(KbdTone::Tooltip))
                                        .child(Kbd::new("P").tone(KbdTone::Tooltip)),
                                )),
                            ),
                    )
                }),
        )
        .child(when(
            move || !status.get().is_empty(),
            move || muted(move || status.get()),
        ))
}

pub fn input_group() -> impl IntoElement {
    let target = element_ref();
    on_key("mod+k", move |_| target.focus());
    view().w_full().max_w(px(320.0)).child(
        InputGroup::new(
            input_group_input()
                .label("Search shortcuts")
                .placeholder("Search..."),
        )
        .control_ref(target)
        .addon(
            InputGroupAddon::new(InputGroupAlign::InlineStart)
                .child(muted("⌕").accessibility_elements_hidden(true)),
        )
        .addon(InputGroupAddon::new(InputGroupAlign::InlineEnd).child(search_shortcut())),
    )
}

pub fn rtl() -> impl IntoElement {
    let selected = signal(1_usize);
    let direction = move || {
        if selected.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    view()
        .items_center()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    |(index, language)| {
                        Button::new(language)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || selected.set(index))
                    },
                )),
        )
        .child(DirectionProvider::new(direction, demo))
}
