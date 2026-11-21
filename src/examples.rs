use nsh_base::{input, theme};
use nsh_core::{AccessibilityState, Checked, prelude::*};

use crate::{
    drawer::{
        Direction, Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter,
        DrawerHeader, DrawerOptions, DrawerTitle, DrawerTrigger, Modality, SnapPoint,
    },
    ui::{Variant, button, button_surface, muted},
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

pub(crate) fn delivery(
    status: Signal<Option<SharedString>>,
) -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let selection = signal(0_usize);
    let trigger = DrawerTrigger::render(drawer, button_surface("Open Drawer", Variant::Secondary))
        .test_id("delivery-trigger");
    let content = DrawerContent::new(drawer, "Pick a delivery time", move || {
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
            .child(
                DrawerHeader::new()
                    .child(DrawerTitle::new("Pick a delivery time"))
                    .child(DrawerDescription::new(
                        "We'll prepare your order as soon as possible.",
                    )),
            )
            .child(scroll().flex_shrink(1.0).p_4().child(choices))
            .child(
                DrawerFooter::new()
                    .child(button(
                        "Confirm Delivery Time",
                        Variant::Primary,
                        move || {
                            let (label, _) = DELIVERY[selection.get()];
                            status.set(Some(format!("Delivery time confirmed · {label}").into()));
                            drawer.close();
                        },
                    ))
                    .child(DrawerClose::render(
                        drawer,
                        button_surface("Cancel", Variant::Outline),
                    )),
            )
    })
    .options(adaptive);
    (trigger, content)
}

pub(crate) fn positions() -> (impl IntoElement, impl IntoElement) {
    let left = Drawer::new(signal(false));
    let top = Drawer::new(signal(false));
    let bottom = Drawer::new(signal(false));
    let right = Drawer::new(signal(false));
    let openers = view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_2()
        .child(DrawerTrigger::new(left, "Open Left Drawer"))
        .child(DrawerTrigger::new(top, "Top"))
        .child(DrawerTrigger::new(bottom, "Bottom"))
        .child(DrawerTrigger::new(right, "Right"));
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

pub(crate) fn swipe() -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    (
        DrawerTrigger::render(drawer, button_surface("Open Drawer", Variant::Secondary))
            .test_id("swipe-trigger"),
        demo_panel(drawer, "Drawer", "Drawer with a swipe handle.", || {
            DrawerOptions {
                show_swipe_handle: true,
                ..Default::default()
            }
        }),
    )
}

pub(crate) fn nested() -> (impl IntoElement, impl IntoElement) {
    let first = Drawer::new(signal(false));
    (
        DrawerTrigger::render(first, button_surface("Open Drawer", Variant::Secondary))
            .test_id("nested-trigger"),
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
    view().child(
        DrawerContent::new(drawer, titles[level], move || {
            let mut actions = DrawerFooter::new();
            if level + 1 < titles.len() {
                let next = Drawer::nested(drawer);
                let labels = [
                    "Open Nested Drawer",
                    "Open Third Drawer",
                    "Open Fourth Drawer",
                ];
                actions = actions
                    .child(DrawerTrigger::new(next, labels[level]))
                    .child(nested_level(next, level + 1));
            }
            view()
                .flex_1()
                .child(example_header(titles[level], descriptions[level]))
                .child(placeholder(140.0))
                .child(actions.child(DrawerClose::render(
                    drawer,
                    button_surface("Close", Variant::Outline),
                )))
        })
        .options(adaptive),
    )
}

pub(crate) fn custom_size() -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let preview =
        DrawerTrigger::new(drawer, "Open Scrollable Drawer").test_id("custom-size-trigger");
    let content = DrawerContent::new(drawer, "Notification history", move || {
        let mut items = view().gap_3();
        for (title, detail) in [
            ("Order confirmed", "Your order has reached the kitchen."),
            ("Preparing your order", "The kitchen has started cooking."),
            (
                "Driver assigned",
                "Your driver will collect your order shortly.",
            ),
            ("Picked up", "Your driver is on the way."),
            ("Arriving soon", "Your order is two minutes away."),
            ("Delivered", "Your order is at your door."),
        ] {
            items = items.child(
                view()
                    .gap_1()
                    .p_3()
                    .rounded(px(8.0))
                    .bg(move || theme().muted)
                    .child(text(title).text_sm().font_weight(FontWeight::SEMIBOLD))
                    .child(muted(detail)),
            );
        }
        view()
            .flex_1()
            .min_h(px(0.0))
            .child(example_header(
                "Notification history",
                "Scroll the updates. The header and close button stay in place.",
            ))
            .child(
                scroll()
                    .flex_1()
                    .min_h(px(0.0))
                    .p_4()
                    .test_id("drawer-notifications")
                    .child(items),
            )
            .child(DrawerFooter::new().child(DrawerClose::new(drawer, "Done")))
    })
    .options(|| DrawerOptions {
        height: Some(360.0),
        ..Default::default()
    });
    (preview, content)
}

pub(crate) fn floating() -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    (
        DrawerTrigger::new(drawer, "Open Floating Drawer").test_id("floating-trigger"),
        demo_panel(
            drawer,
            "Floating Drawer",
            "A 16 point inset on every side.",
            || DrawerOptions {
                direction: Direction::Right,
                width: Some(360.0),
                inset: 16.0,
                ..Default::default()
            },
        ),
    )
}

pub(crate) fn non_modal() -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    (
        DrawerTrigger::new(drawer, "Non Modal").test_id("non-modal-trigger"),
        demo_panel(drawer, "Non Modal Drawer", "", || DrawerOptions {
            direction: Direction::Right,
            modality: Modality::NonModal,
            disable_outside_dismissal: true,
            ..Default::default()
        }),
    )
}

pub(crate) fn snap() -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let open = DrawerTrigger::new(drawer, "Open Snap Drawer").test_id("snap-trigger");
    let content = DrawerContent::new(drawer, "Snap points", move || {
        view()
            .h(move || {
                let height = window().height.0;
                let visible = if drawer.snap_index.get() == 0 {
                    height.min(496.0)
                } else {
                    height
                };
                px((visible - 30.0).max(0.0))
            })
            .child(example_header(
                "Snap points",
                "Drag the drawer to snap between a compact peek and a near full-height view.",
            ))
            .child(placeholder(200.0))
            .child(DrawerFooter::new().child(DrawerClose::new(drawer, "Close")))
    })
    .options(|| DrawerOptions {
        show_swipe_handle: true,
        snap_points: &[SnapPoint::Rem(31.0), SnapPoint::Fraction(1.0)],
        ..Default::default()
    });
    (open, content)
}

pub(crate) fn responsive(
    status: Signal<Option<SharedString>>,
) -> (impl IntoElement, impl IntoElement) {
    let drawer = Drawer::new(signal(false));
    let saved_email = signal("shadcn@example.com".to_owned());
    let saved_username = signal("@shadcn".to_owned());
    let content = DrawerContent::new(drawer, "Edit profile", move || {
        let email = signal(saved_email.get_untracked());
        let username = signal(saved_username.get_untracked());
        let save = move || {
            saved_email.set(email.get());
            saved_username.set(username.get());
            status.set(Some("Profile updated".into()));
            drawer.close();
        };
        view()
            .child(example_header(
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
                    .child(DrawerClose::render(
                        drawer,
                        button_surface("Cancel", Variant::Outline),
                    )),
            )
    })
    .options(|| DrawerOptions {
        centered: window().width.0 >= BREAKPOINT,
        ..Default::default()
    });
    (
        DrawerTrigger::new(drawer, "Edit Profile").test_id("responsive-trigger"),
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
    DrawerContent::new(drawer, title, move || {
        view()
            .flex_1()
            .child(example_header(title, description))
            .child(placeholder(240.0))
            .child(DrawerFooter::new().child(DrawerClose::new(drawer, "Close")))
    })
    .options(options)
}

fn example_header(title: &'static str, description: &'static str) -> DrawerHeader {
    let header = DrawerHeader::new().child(DrawerTitle::new(title));
    if description.is_empty() {
        header
    } else {
        header.child(DrawerDescription::new(description))
    }
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
