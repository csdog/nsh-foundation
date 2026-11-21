use nsh_base::theme;
use nsh_core::{element::set_color_scheme, platform::Appearance, prelude::*};

use crate::{
    examples,
    ui::{Variant, button, heading, muted},
};

const SOURCE: &str = "https://github.com/shadcn-ui/ui/blob/f50887ae68d0fdcbfbe86b57be08af94a4c88a68/apps/v4/registry/bases/base/ui/drawer.tsx";
const REFERENCE: &str = "https://ui.shadcn.com/docs/components/base/drawer";
const SECTIONS: [&str; 12] = [
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
];

pub(crate) fn page() -> View {
    let scroll_ref = element_ref();
    let locations = stored([0.0_f32; SECTIONS.len()]);
    let menu = signal(false);
    let dark = signal(false);
    let selected = signal(None::<usize>);
    let status = signal(None::<SharedString>);
    let code = signal(false);
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
    let (positions, position_panels) = examples::positions();
    let (swipe, swipe_panel) = examples::swipe();
    let (nested, nested_panel) = examples::nested();
    let (non_modal, non_modal_panel) = examples::non_modal();
    let (snap, snap_panel) = examples::snap();
    let (responsive, responsive_panel) = examples::responsive(status);

    let article = view()
        .max_w(px(770.0))
        .w_full()
        .self_center()
        .px(move || px(if window().width.0 < 760.0 { 22.0 } else { 36.0 }))
        .pt_8()
        .pb_16()
        .gap_7()
        .child(
            view().gap_3()
                .child(view().flex_row().items_center().gap_2()
                    .child(muted("Components"))
                    .child(muted("/"))
                    .child(text("Drawer").text_sm().font_weight(FontWeight::MEDIUM)))
                .child(
                    text("Drawer")
                        .role(AccessibilityRole::Header)
                        .text_size(px(38.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(muted("A swipeable panel that opens from an edge of the screen."))
                .child(view().flex_row().gap_2()
                    .child(button("Base UI reference", Variant::Outline, || { let _ = open_url(REFERENCE); }))
                    .child(button("Source", Variant::Ghost, || { let _ = open_url(SOURCE); }))),
        )
        .child(
            view().gap_2()
                .child(view().flex_row().gap_2()
                    .child(button("Preview", Variant::Ghost, move || code.set(false)))
                    .child(button("Rust", Variant::Ghost, move || code.set(true))))
                .child(view().style_when(code, |style| style.hidden()).child(stage(delivery)))
                .child(when(code, || code_block(
                    "let drawer = Drawer::new(signal(false));\ntrigger(drawer, \"Open Drawer\");\ndrawer.content(\"Pick a delivery time\", options, body);"
                )))
                .child(delivery_panel),
        )
        .child(
            section("Installation", 0, locations)
                .child(muted("Build the Native Shell Rust project for the Web target."))
                .child(code_block("nsh package --target web --out dist")),
        )
        .child(
            section("Usage", 1, locations)
                .child(muted("Give Drawer a shared open signal and compose its header, body, and footer."))
                .child(code_block(
                    "let drawer = Drawer::new(signal(false));\nlet opener = trigger(drawer, \"Open\");\nlet panel = drawer.content(\"Drawer\", DrawerOptions::default, body);"
                )),
        )
        .child(section("Composition", 2, locations)
            .child(muted("The trigger opens the surface. Place the heading, scrollable content, and actions inside it."))
            .child(code_block("Drawer → Trigger, Content
Content → Header, Body, Footer")))
        .child(section("Custom Sizes", 3, locations)
            .child(muted("Use height for vertical drawers and width for side drawers. Put long content in a scroll view."))
            .child(code_block("DrawerOptions { height: Some(320.0), ..Default::default() }")))
        .child(section("Styling", 4, locations)
            .child(muted("Drawers read the popover, foreground, border, and muted tokens from the Native Shell theme. Use inset to float the surface away from the viewport edge.")))
        .child(section("Examples", 5, locations))
        .child(section("Position", 6, locations)
            .child(muted("Open the drawer from any of the four screen edges."))
            .child(stage(positions))
            .child(position_panels))
        .child(section("Swipe Handle", 7, locations)
            .child(muted("Drag the handle toward the edge to dismiss the drawer."))
            .child(stage(swipe))
            .child(swipe_panel))
        .child(section("Nested", 8, locations)
            .child(muted("Open up to four drawers. Close the topmost drawer to return to the previous one."))
            .child(stage(nested))
            .child(nested_panel))
        .child(section("Non Modal", 9, locations)
            .child(muted("Scroll and interact with this page while the drawer stays open."))
            .child(stage(non_modal)))
        .child(section("Snap Points", 10, locations)
            .child(muted("Drag the drawer between a compact peek and its full height."))
            .child(stage(snap))
            .child(snap_panel))
        .child(section("Responsive", 11, locations)
            .child(muted("Edit the profile in a desktop dialog or a mobile drawer."))
            .child(stage(responsive))
            .child(responsive_panel));

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
                            || window().width.0 < 760.0,
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

fn stage(contents: View) -> View {
    view()
        .min_h(px(260.0))
        .p_4()
        .rounded(px(12.0))
        .border(px(1.0))
        .border_color(move || theme().border)
        .items_center()
        .justify_center()
        .child(contents)
}

fn code_block(value: &'static str) -> View {
    view()
        .p_4()
        .rounded(px(10.0))
        .bg(move || theme().muted)
        .child(
            text(value)
                .text_sm()
                .font_family("monospace")
                .selectable(true),
        )
}
