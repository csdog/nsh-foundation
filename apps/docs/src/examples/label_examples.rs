use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    checkbox::{Checkbox, CheckboxState},
    direction::{Direction, DirectionProvider},
    label::Label,
};

fn terms(label: &'static str, accepted: Signal<bool>) -> View {
    let control = element_ref();
    let read_state = move || {
        if accepted.get() {
            CheckboxState::True
        } else {
            CheckboxState::False
        }
    };
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .child(
            Checkbox::new(label)
                .icon_only()
                .checked(read_state)
                .on_change(move |checked| accepted.set(checked))
                .surface()
                .element_ref(&control),
        )
        .child(
            Label::new(label)
                .for_control(control)
                .on_activate(move || accepted.update(|checked| *checked = !*checked)),
        )
}

pub fn demo() -> impl IntoElement {
    terms("Accept terms and conditions", signal(false))
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1_usize);
    let accepted = signal(false);
    let dir = move || {
        if language.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    let labels = [
        ("English", "Accept terms and conditions"),
        ("Arabic", "قبول الشروط والأحكام"),
        ("Hebrew", "קבל תנאים והגבלות"),
    ];
    view()
        .items_center()
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .children(
                    labels
                        .into_iter()
                        .enumerate()
                        .map(|(index, (language_name, _))| {
                            Button::new(language_name)
                                .size(ButtonSize::Small)
                                .variant(ButtonVariant::Outline)
                                .on_press(move || language.set(index))
                        }),
                ),
        )
        .child(DirectionProvider::new(dir, move || {
            dynamic(move || terms(labels[language.get()].1, accepted))
        }))
}
