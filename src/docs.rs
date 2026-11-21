use nsh_base::theme;
use nsh_core::{
    element::{set_clipboard, set_color_scheme},
    platform::Appearance,
    prelude::*,
};

use crate::{
    examples,
    ui::{Variant, button, heading, muted},
};

const SOURCE: &str = "https://github.com/shadcn-ui/ui/blob/f50887ae68d0fdcbfbe86b57be08af94a4c88a68/apps/v4/registry/bases/base/ui/drawer.tsx";
const REFERENCE: &str = "https://ui.shadcn.com/docs/components/base/drawer";
const SECTIONS: [&str; 13] = [
    "Installation",
    "Usage",
    "Composition",
    "Custom Sizes",
    "Styling",
    "Examples",
    "Position",
    "Swipe Handle",
    "Nested",
    "Non Modal",
    "Snap Points",
    "Responsive",
    "API Reference",
];

const SIMPLE: &str = r#"use nsh_core::prelude::*;
use nsh_foundation::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription,
    DrawerFooter, DrawerHeader, DrawerTitle, DrawerTrigger,
};

fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    view()
        .child(DrawerTrigger::new(drawer, "Open Drawer"))
        .child(DrawerContent::new(drawer, "Drawer", move || {
            view()
                .child(DrawerHeader::new()
                    .child(DrawerTitle::new("Drawer"))
                    .child(DrawerDescription::new("A panel at the edge.")))
                .child(DrawerFooter::new()
                    .child(DrawerClose::new(drawer, "Close")))
        }))
}"#;

const SIZED: &str = r#"DrawerContent::new(drawer, "Notification history", move || {
    view().flex_1().min_h(px(0.0))
        .child(DrawerHeader::new()
            .child(DrawerTitle::new("Notification history")))
        .child(scroll().flex_1().min_h(px(0.0)).child(text("Order confirmed")))
        .child(DrawerFooter::new().child(DrawerClose::new(drawer, "Done")))
})
.options(|| DrawerOptions {
    height: Some(360.0),
    ..Default::default()
})"#;

const FLOATING: &str = r#"DrawerContent::new(drawer, "Floating Drawer", body)
    .options(|| DrawerOptions {
        direction: Direction::Right,
        width: Some(360.0),
        inset: 16.0,
        ..Default::default()
    })"#;

const POSITION: &str = r#"DrawerContent::new(drawer, "Move Goal", body)
    .options(|| DrawerOptions {
        direction: Direction::Left,
        ..Default::default()
    })"#;

const HANDLE: &str = r#"DrawerContent::new(drawer, "Drawer", body)
    .options(|| DrawerOptions {
        show_swipe_handle: true,
        ..Default::default()
    })"#;

const NESTED: &str = r#"DrawerContent::new(parent, "Parent", move || {
    let child = Drawer::nested(parent);
    view()
        .child(DrawerTrigger::new(child, "Open Nested Drawer"))
        .child(DrawerContent::new(child, "Nested Drawer", move || {
            view().child(DrawerFooter::new()
                .child(DrawerClose::new(child, "Close")))
        }))
})"#;

const NON_MODAL: &str = r#"DrawerContent::new(drawer, "Non Modal Drawer", body)
    .options(|| DrawerOptions {
        direction: Direction::Right,
        modality: Modality::NonModal,
        disable_outside_dismissal: true,
        ..Default::default()
    })"#;

const SNAP: &str = r#"DrawerContent::new(drawer, "Snap points", body)
    .options(|| DrawerOptions {
        show_swipe_handle: true,
        snap_points: &[SnapPoint::Rem(31.0), SnapPoint::Fraction(1.0)],
        ..Default::default()
    })"#;

const RESPONSIVE: &str = r#"DrawerContent::new(drawer, "Edit profile", profile_form)
    .options(|| DrawerOptions {
        centered: window().width.0 >= 768.0,
        ..Default::default()
    })"#;

pub(crate) fn page() -> View {
    let scroll_ref = element_ref();
    let locations = stored([0.0_f32; SECTIONS.len()]);
    let menu = signal(false);
    let dark = signal(false);
    let selected = signal(None::<usize>);
    let status = signal(None::<SharedString>);

    let go = move |index: usize| {
        selected.set(Some(index));
        let y = locations.with(|items| items[index]);
        scroll_ref.scroll_to(
            Point {
                x: px(0.0),
                y: px((y - 18.0).max(0.0)),
            },
            false,
        );
        menu.set(false);
    };

    let (delivery, delivery_panel) = examples::delivery(status);
    let (custom_size, custom_size_panel) = examples::custom_size();
    let (floating, floating_panel) = examples::floating();
    let (positions, position_panels) = examples::positions();
    let (swipe, swipe_panel) = examples::swipe();
    let (nested, nested_panel) = examples::nested();
    let (non_modal, non_modal_panel) = examples::non_modal();
    let (snap, snap_panel) = examples::snap();
    let (responsive, responsive_panel) = examples::responsive(status);

    let article = view()
        .max_w(px(820.0))
        .w_full()
        .self_center()
        .px(move || px(if window().width.0 < 760.0 { 22.0 } else { 36.0 }))
        .pt_8()
        .pb_16()
        .gap_8()
        .child(
            view().gap_3()
                .child(view().flex_row().items_center().gap_2()
                    .child(muted("Components"))
                    .child(muted("/"))
                    .child(text("Drawer").text_sm().font_weight(FontWeight::MEDIUM)))
                .child(
                    text("Drawer")
                        .role(AccessibilityRole::Header)
                        .text_size(px(42.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(muted("A swipeable panel that opens from an edge of the screen."))
                .child(view().flex_row().gap_2()
                    .child(button("Base UI reference", Variant::Outline, || { let _ = open_url(REFERENCE); }))
                    .child(button("Source", Variant::Ghost, || { let _ = open_url(SOURCE); }))),
        )
        .child(example("Delivery time", delivery, SIMPLE))
        .child(delivery_panel)
        .child(
            section("Installation", 0, locations)
                .child(muted("Build the Native Shell Rust project for the Web target."))
                .child(code_block("nsh package --target web --out dist")),
        )
        .child(
            section("Usage", 1, locations)
                .child(muted("Pass the same Drawer to DrawerTrigger and DrawerContent. Build the content from DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, and DrawerClose."))
                .child(code_block(SIMPLE)),
        )
        .child(section("Composition", 2, locations)
            .child(muted("The trigger can sit anywhere in the layout. DrawerContent supplies the window layer and backdrop, and mounts its children when the drawer opens."))
            .child(code_block("Drawer\n├── DrawerTrigger\n└── DrawerContent\n    ├── DrawerPortal + DrawerOverlay\n    ├── DrawerHeader\n    │   ├── DrawerTitle\n    │   └── DrawerDescription\n    └── DrawerFooter\n        └── DrawerClose")))
        .child(section("Custom Sizes", 3, locations)
            .child(muted("Use height for vertical drawers and width for side drawers. Put long content in a scroll view."))
            .child(example("Custom Sizes", custom_size, SIZED))
            .child(custom_size_panel))
        .child(section("Styling", 4, locations)
            .child(muted("Drawers use the popover, foreground, border, and muted theme tokens. Set inset to float the surface away from the viewport edge."))
            .child(example("Styling", floating, FLOATING))
            .child(floating_panel))
        .child(section("Examples", 5, locations))
        .child(section("Position", 6, locations)
            .child(muted("Open the drawer from any of the four screen edges."))
            .child(example("Position", positions, POSITION))
            .child(position_panels))
        .child(section("Swipe Handle", 7, locations)
            .child(muted("Drag the handle toward the edge to dismiss the drawer."))
            .child(example("Swipe Handle", swipe, HANDLE))
            .child(swipe_panel))
        .child(section("Nested", 8, locations)
            .child(muted("Open up to four drawers. Close the topmost drawer to return to the previous one."))
            .child(example("Nested", nested, NESTED))
            .child(nested_panel))
        .child(section("Non Modal", 9, locations)
            .child(muted("Scroll and interact with this page while the drawer stays open."))
            .child(example("Non Modal", non_modal, NON_MODAL)))
        .child(section("Snap Points", 10, locations)
            .child(muted("Drag the drawer between a compact peek and its full height."))
            .child(example("Snap Points", snap, SNAP))
            .child(snap_panel))
        .child(section("Responsive", 11, locations)
            .child(muted("Edit the profile in a desktop dialog or a mobile drawer."))
            .child(example("Responsive", responsive, RESPONSIVE))
            .child(responsive_panel))
        .child(section("API Reference", 12, locations)
            .child(muted("Use Drawer::new with an open signal. Drawer::nested links a child to a parent that remains mounted."))
            .child(api_row("DrawerTrigger / DrawerClose", "Use new for the default button or render to attach drawer behavior to your own control."))
            .child(api_row("DrawerContent", "Creates the window layer and backdrop. Set placement, size, and behavior with options."))
            .child(api_row("DrawerHeader / DrawerTitle / DrawerDescription / DrawerFooter", "Compose the visible content with ordinary Native Shell children and styles."))
            .child(api_row("DrawerPortal / DrawerOverlay / DrawerSwipeHandle", "Customize the window layer or backdrop, or style the generated drag handle."))
            .child(api_row("direction", "Edge of the viewport. Defaults to Down."))
            .child(api_row("modality", "Modal traps focus. NonModal leaves the page interactive."))
            .child(api_row("disable_outside_dismissal", "Keeps the drawer open on outside presses."))
            .child(api_row("height / width / inset", "Panel dimensions and distance from the viewport edges, in points."))
            .child(api_row("snap_points / snap_index", "Vertical detents and the active detent signal."))
            .child(api_row("centered", "Displays a centered dialog for the responsive example.")));

    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(58.0))
                .flex_shrink_0()
                .px_5()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(text("◩").text_lg())
                        .child(text("nsh-foundation").font_weight(FontWeight::SEMIBOLD))
                        .child(when(
                            || window().width.0 >= 760.0,
                            move || button("Components", Variant::Ghost, move || go(5)),
                        )),
                )
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(when(
                            || window().width.0 < 1220.0,
                            move || {
                                button("Contents", Variant::Ghost, move || {
                                    menu.update(|open| *open = !*open)
                                })
                            },
                        ))
                        .child(button("Theme", Variant::Ghost, move || {
                            dark.update(|enabled| *enabled = !*enabled);
                            set_color_scheme(if dark.get() {
                                Appearance::Dark
                            } else {
                                Appearance::Light
                            });
                        })),
                ),
        )
        .child(when(menu, move || {
            view()
                .p_3()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(section_links(go, selected))
        }))
        .child(
            view()
                .flex_1()
                .min_h(px(0.0))
                .flex_row()
                .child(when(|| window().width.0 >= 940.0, move || sidebar(go)))
                .child(
                    scroll()
                        .flex_1()
                        .min_w(px(0.0))
                        .element_ref(&scroll_ref)
                        .test_id("drawer-docs-scroll")
                        .on_scroll(move |event: &ScrollEvent| {
                            let next = locations.with(|positions| {
                                positions.iter().rposition(|position| {
                                    *position > 0.0 && *position <= event.offset.y.0 + 32.0
                                })
                            });
                            selected.set(next);
                        })
                        .child(article),
                )
                .child(when(
                    || window().width.0 >= 1220.0,
                    move || {
                        view()
                            .w(px(196.0))
                            .flex_shrink_0()
                            .pt_8()
                            .pr_5()
                            .gap_2()
                            .child(
                                text("On this page")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD),
                            )
                            .child(section_links(go, selected))
                    },
                )),
        )
        .child(non_modal_panel)
        .child(when(
            move || status.get().is_some(),
            move || {
                view()
                    .absolute()
                    .right_4()
                    .bottom_4()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .rounded(px(10.0))
                    .border(px(1.0))
                    .border_color(move || theme().border)
                    .bg(move || theme().popover)
                    .shadow_lg()
                    .role(AccessibilityRole::Alert)
                    .child(text(move || status.get().unwrap_or_default()).text_sm())
                    .child(button("Dismiss", Variant::Ghost, move || status.set(None)))
            },
        ))
}

fn section(name: &'static str, index: usize, locations: Stored<[f32; SECTIONS.len()]>) -> View {
    view()
        .gap_3()
        .pt_5()
        .border_t(px(1.0))
        .border_color(move || theme().border)
        .on_layout(move |bounds: &Bounds<Points>| {
            locations.with(|items| items[index] = bounds.origin.y.0);
        })
        .child(heading(name))
}

fn section_links(go: impl Fn(usize) + Copy + 'static, selected: Signal<Option<usize>>) -> View {
    let mut links = view()
        .role(AccessibilityRole::Navigation)
        .label("Drawer sections");
    for (index, name) in SECTIONS.into_iter().enumerate() {
        links = links.child(
            view()
                .role(AccessibilityRole::Link)
                .label(name)
                .cursor_pointer()
                .py_1()
                .px_2()
                .rounded(px(6.0))
                .style_when(
                    move || selected.get() == Some(index),
                    |style| style.bg(theme().muted),
                )
                .on_press(move || go(index))
                .child(text(name).text_sm().text_color(move || {
                    if selected.get() == Some(index) {
                        theme().foreground
                    } else {
                        theme().muted_foreground
                    }
                })),
        );
    }
    links
}

fn sidebar(go: impl Fn(usize) + Copy + 'static) -> View {
    view()
        .w(px(215.0))
        .flex_shrink_0()
        .p_5()
        .gap_4()
        .border_r(px(1.0))
        .border_color(move || theme().border)
        .child(
            text("Documentation")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            view().gap_2().child(muted("Getting Started")).child(
                view()
                    .items_start()
                    .child(button("Installation", Variant::Ghost, move || go(0))),
            ),
        )
        .child(
            view()
                .gap_2()
                .child(muted("Components"))
                .child(view().items_start().child(button(
                    "Drawer",
                    Variant::Secondary,
                    move || go(5),
                ))),
        )
}

fn example(label: &'static str, contents: impl IntoElement, source: &'static str) -> View {
    let show_code = signal(false);
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .border(px(1.0))
        .border_color(move || theme().border)
        .rounded(px(12.0))
        .overflow_hidden()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .px_3()
                .py_2()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(tab("Preview", show_code, false))
                .child(tab("Code", show_code, true)),
        )
        .child(
            view()
                .style_when(show_code, |style| style.hidden())
                .min_h(px(248.0))
                .p_4()
                .items_center()
                .justify_center()
                .child(contents),
        )
        .child(when(show_code, move || code_block(source)))
}

fn tab(label: &'static str, selected: Signal<bool>, value: bool) -> View {
    button(label, Variant::Ghost, move || selected.set(value))
        .accessibility_state(move || AccessibilityState {
            selected: selected.get() == value,
            ..Default::default()
        })
        .bg(move || {
            if selected.get() == value {
                theme().muted
            } else {
                hsla(0.0, 0.0, 0.0, 0.0).into()
            }
        })
}

fn code_block(value: &'static str) -> View {
    let copied = signal(false);
    let reset = stored(None::<Spawned<()>>);
    view()
        .p_3()
        .gap_2()
        .rounded(px(8.0))
        .bg(move || theme().muted)
        .child(
            view().items_end().child(
                button("Copy", Variant::Outline, move || {
                    set_clipboard(value);
                    copied.set(true);
                    if let Some(task) = reset.replace(None) {
                        task.cancel();
                    }
                    reset.replace(Some(set_timeout(
                        std::time::Duration::from_secs(2),
                        move || copied.set(false),
                    )));
                })
                .child(when(copied, || muted("Copied"))),
            ),
        )
        .child(
            text(value)
                .text_sm()
                .font_family("monospace")
                .selectable(true),
        )
}

fn api_row(name: &'static str, description: &'static str) -> View {
    view()
        .gap_1()
        .p_3()
        .rounded(px(8.0))
        .bg(move || theme().muted)
        .child(text(name).text_sm().font_family("monospace"))
        .child(muted(description))
}
