use nsh_base::{input, theme};
use nsh_core::{AccessibilityState, Checked, prelude::*};

use crate::{
    drawer::{
        Direction, Drawer, DrawerOptions, Modality, SnapPoint, close, footer, header, trigger,
    },
    ui::{Variant, button, muted},
};

pub(crate) const BREAKPOINT: f32 = 768.0;

fn adaptive() -> DrawerOptions {
    let mobile = window().width.0 < BREAKPOINT;
    DrawerOptions {
        direction: if mobile {
            Direction::Down
        } else {
            Direction::Right
        },
        show_swipe_handle: mobile,
        ..Default::default()
    }
}

const DELIVERY: [(&str, &str); 5] = [
    ("Standard delivery", "25–35 min · Driver assigned now"),
    ("5:00 PM – 5:15 PM", "Prep starts at 4:45 PM"),
    ("5:30 PM – 5:45 PM", "Good if you're heading home"),
    ("6:00 PM – 6:15 PM", "Most popular · High demand"),
    ("6:30 PM – 6:45 PM", "Last slot before kitchen closes"),
];

pub(crate) fn delivery(status: Signal<Option<SharedString>>) -> (View, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let selection = signal(0_usize);
    let trigger = button("Open Drawer", Variant::Secondary, move || drawer.open())
        .test_id("delivery-trigger");
    let content = drawer.content("Pick a delivery time", adaptive, move || {
        let mut choices = view()
            .gap_2()
            .role(AccessibilityRole::Radiogroup)
            .label("Delivery time");
        for (index, (label, description)) in DELIVERY.into_iter().enumerate() {
            let row = view()
                .label(label)
                .hint(description)
                .role(AccessibilityRole::Radio)
                .accessibility_state(move || AccessibilityState {
                    checked: Some(if selection.get() == index {
                        Checked::True
                    } else {
                        Checked::False
                    }),
                    ..Default::default()
                })
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2()
                .p_3()
                .rounded(px(10.0))
                .border(px(1.0))
                .border_color(move || {
                    if selection.get() == index {
                        theme().ring
                    } else {
                        theme().border
                    }
                })
                .hover(|style| style.opacity(0.75))
                .on_press(move || selection.set(index))
                .child(
                    view()
                        .flex_1()
                        .gap_1()
                        .child(
                            view()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(text(label).text_sm().font_weight(FontWeight::SEMIBOLD))
                                .child((index == 0).then(|| {
                                    view()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded_full()
                                        .bg(move || theme().secondary)
                                        .child(
                                            text("Fastest")
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM),
                                        )
                                })),
                        )
                        .child(muted(description)),
                )
                .child(
                    view()
                        .size(px(16.0))
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .border(px(1.0))
                        .border_color(move || theme().ring)
                        .child(when(
                            move || selection.get() == index,
                            || {
                                view()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(move || theme().primary)
                            },
                        )),
                );
            choices = choices.child(row);
        }
        view()
            .flex_1()
            .child(header(
                "Pick a delivery time",
                "We'll prepare your order as soon as possible.",
            ))
            .child(scroll().flex_shrink(1.0).px_4().py_2().child(choices))
            .child(
                footer()
                    .child(button(
                        "Confirm Delivery Time",
                        Variant::Primary,
                        move || {
                            let (label, _) = DELIVERY[selection.get()];
                            status.set(Some(format!("Delivery time confirmed · {label}").into()));
                            drawer.close();
                        },
                    ))
                    .child(button("Cancel", Variant::Outline, move || drawer.close())),
            )
    });
    (trigger, content)
}

pub(crate) fn positions() -> (View, impl IntoElement) {
    let left = Drawer::new(signal(false));
    let top = Drawer::new(signal(false));
    let bottom = Drawer::new(signal(false));
    let right = Drawer::new(signal(false));
    let openers = view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_2()
        .child(trigger(left, "Open Left Drawer"))
        .child(trigger(top, "Top"))
        .child(trigger(bottom, "Bottom"))
        .child(trigger(right, "Right"));
    let panels = view()
        .child(demo_panel(
            left,
            "Move Goal",
            "Set your daily activity goal.",
            || DrawerOptions {
                direction: Direction::Left,
                ..Default::default()
            },
        ))
        .child(demo_panel(top, "Top Drawer", "", || DrawerOptions {
            direction: Direction::Up,
            ..Default::default()
        }))
        .child(demo_panel(bottom, "Bottom Drawer", "", || DrawerOptions {
            direction: Direction::Down,
            ..Default::default()
        }))
        .child(demo_panel(right, "Right Drawer", "", || DrawerOptions {
            direction: Direction::Right,
            ..Default::default()
        }));
    (openers, panels)
}

pub(crate) fn swipe() -> (View, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    (
        button("Open Drawer", Variant::Secondary, move || drawer.open()).test_id("swipe-trigger"),
        demo_panel(drawer, "Drawer", "Drawer with a swipe handle.", || {
            DrawerOptions {
                show_swipe_handle: true,
                ..Default::default()
            }
        }),
    )
}

pub(crate) fn nested() -> (View, impl IntoElement) {
    let first = Drawer::new(signal(false));
    (
        button("Open Drawer", Variant::Secondary, move || first.open()).test_id("nested-trigger"),
        nested_level(first, 0),
    )
}

fn nested_level(drawer: Drawer, level: usize) -> View {
    let titles = ["Drawer", "Nested Drawer", "Third Drawer", "Fourth Drawer"];
    let descriptions = [
        "Open another drawer from the same direction.",
        "The parent drawer stays mounted behind this one.",
        "Two drawers are stacked behind this one.",
        "This is the frontmost drawer in the stack.",
    ];
    view().child(drawer.content(titles[level], adaptive, move || {
        let mut actions = footer();
        if level + 1 < titles.len() {
            let next = Drawer::nested(drawer);
            let labels = [
                "Open Nested Drawer",
                "Open Third Drawer",
                "Open Fourth Drawer",
            ];
            actions = actions
                .child(trigger(next, labels[level]))
                .child(nested_level(next, level + 1));
        }
        view()
            .flex_1()
            .child(header(titles[level], descriptions[level]))
            .child(placeholder(140.0))
            .child(actions.child(button("Close", Variant::Outline, move || drawer.close())))
    }))
}

pub(crate) fn non_modal() -> (View, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    (
        trigger(drawer, "Non Modal").test_id("non-modal-trigger"),
        demo_panel(drawer, "Non Modal Drawer", "", || DrawerOptions {
            direction: Direction::Right,
            modality: Modality::NonModal,
            disable_outside_dismissal: true,
            ..Default::default()
        }),
    )
}

pub(crate) fn snap() -> (View, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let open = trigger(drawer, "Open Snap Drawer").test_id("snap-trigger");
    let content = drawer.content(
        "Snap points",
        || DrawerOptions {
            show_swipe_handle: true,
            snap_points: &[SnapPoint::Rem(31.0), SnapPoint::Fraction(1.0)],
            ..Default::default()
        },
        move || {
            let available = || window().height.0.min(496.0);
            view()
                .h(move || px(available() - 30.0))
                .child(header(
                    "Snap points",
                    "Drag the drawer to snap between a compact peek and a near full-height view.",
                ))
                .child(placeholder(200.0))
                .child(footer().child(close(drawer, "Close")))
        },
    );
    (open, content)
}

pub(crate) fn responsive(status: Signal<Option<SharedString>>) -> (View, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let saved_email = signal("shadcn@example.com".to_owned());
    let saved_username = signal("@shadcn".to_owned());
    let content = drawer.content(
        "Edit profile",
        || DrawerOptions {
            centered: window().width.0 >= BREAKPOINT,
            ..Default::default()
        },
        move || {
            let email = signal(saved_email.get_untracked());
            let username = signal(saved_username.get_untracked());
            let save = move || {
                saved_email.set(email.get());
                saved_username.set(username.get());
                status.set(Some("Profile updated".into()));
                drawer.close();
            };
            view()
                .child(header(
                    "Edit profile",
                    "Make changes to your profile here. Click save when you're done.",
                ))
                .child(
                    view()
                        .p_4()
                        .gap_4()
                        .child(field("Email", email, save))
                        .child(field("Username", username, save))
                        .child(button("Save changes", Variant::Primary, save))
                        .child(button("Cancel", Variant::Outline, move || drawer.close())),
                )
        },
    );
    (
        trigger(drawer, "Edit Profile").test_id("responsive-trigger"),
        content,
    )
}

fn field(name: &'static str, value: Signal<String>, submit: impl Fn() + Copy + 'static) -> View {
    view()
        .gap_2()
        .child(text(name).text_sm().font_weight(FontWeight::MEDIUM))
        .child(
            input()
                .label(name)
                .value(move || value.get())
                .h(px(36.0))
                .p_2()
                .rounded(px(8.0))
                .border(px(1.0))
                .border_color(move || theme().input)
                .text_color(move || theme().foreground)
                .on_change(move |text: &str| value.set(text.to_owned()))
                .on_submit(move |_: &str| submit()),
        )
}

fn demo_panel(
    drawer: Drawer,
    title: &'static str,
    description: &'static str,
    options: impl Fn() -> DrawerOptions + 'static,
) -> impl IntoElement {
    drawer.content(title, options, move || {
        view()
            .flex_1()
            .child(header(title, description))
            .child(placeholder(240.0))
            .child(footer().child(close(drawer, "Close")))
    })
}

fn placeholder(height: f32) -> View {
    view().flex_1().p_4().child(
        view()
            .min_h(px(height))
            .flex_1()
            .rounded(px(16.0))
            .bg(move || theme().muted),
    )
}
