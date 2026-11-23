use nsh_base::theme;
use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    resizable::{
        ResizableHandle as Handle, ResizableOrientation as Orientation, ResizablePanel as Panel,
        ResizablePanelGroup as Group,
    },
};

fn frame(group: Group) -> View {
    view()
        .w_full()
        .max_w(px(384.0))
        .h(px(202.0))
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .overflow_hidden()
        .child(group)
}

fn panel(label: &'static str, size: f32) -> Panel {
    Panel::new(
        view()
            .size_full()
            .items_center()
            .justify_center()
            .child(text(label).text_sm().font_weight(FontWeight::SEMIBOLD)),
    )
    .default_size(size)
}

fn nested(labels: [&'static str; 3], saved: Option<[Signal<Vec<f32>>; 2]>) -> View {
    let inner = Group::new(Orientation::Vertical)
        .panel(panel(labels[1], 25.0))
        .handle(
            Handle::new()
                .with_handle(true)
                .label("Resize top and bottom"),
        )
        .panel(panel(labels[2], 75.0));
    let inner = match saved {
        Some([_, vertical]) => inner.layout(vertical),
        None => inner,
    };
    let outer = Group::new(Orientation::Horizontal)
        .panel(panel(labels[0], 50.0))
        .handle(Handle::new().with_handle(true).label("Resize side panels"))
        .panel(Panel::new(inner).default_size(50.0));
    frame(match saved {
        Some([horizontal, _]) => outer.layout(horizontal),
        None => outer,
    })
}

pub fn demo() -> View {
    nested(["One", "Two", "Three"], None)
}

pub fn vertical() -> View {
    frame(
        Group::new(Orientation::Vertical)
            .panel(panel("Header", 25.0))
            .handle(Handle::new().label("Resize header and content"))
            .panel(panel("Content", 75.0)),
    )
}

pub fn handle() -> View {
    frame(
        Group::new(Orientation::Horizontal)
            .panel(panel("Sidebar", 25.0))
            .handle(
                Handle::new()
                    .with_handle(true)
                    .label("Resize sidebar and content"),
            )
            .panel(panel("Content", 75.0)),
    )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let saved = [signal(vec![50.0, 50.0]), signal(vec![25.0, 75.0])];
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || language.set(index))
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                    },
                )),
        )
        .child(dynamic(move || {
            let index = language.get();
            let direction = if index == 0 {
                Direction::Ltr
            } else {
                Direction::Rtl
            };
            let labels = [
                ["One", "Two", "Three"],
                ["واحد", "اثنان", "ثلاثة"],
                ["אחד", "שניים", "שלושה"],
            ][index];
            DirectionProvider::new(direction, move || nested(labels, Some(saved)))
        }))
}
