use std::time::Duration;

use docs::App;
use nsh_core::{Bounds, Points, Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

const PANEL: &str = "id:drawer-panel";
const HANDLE: &str = "id:drawer-swipe-handle";
const MOTION: Duration = Duration::from_millis(500);

fn start(width: f32, height: f32) -> Headless {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(px(width), px(height)),
        ..EngineConfig::new(Root::of::<App>())
    })
    .unwrap();
    app.first_render().unwrap();
    app
}

fn interact(app: &mut Headless, selector: &str) {
    app.press(selector).unwrap();
    app.wait(MOTION).unwrap();
}

fn assert_count(app: &Headless, count: usize) {
    assert_eq!(app.screen().count(PANEL), Ok(count));
}

fn panel(app: &Headless) -> Bounds<Points> {
    bounds(app, "drawer-panel")
}

fn bounds(app: &Headless, test_id: &str) -> Bounds<Points> {
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some(test_id))
        .unwrap_or_else(|| panic!("missing {test_id}"))
        .bounds
}

#[test]
fn delivery_can_be_selected_and_confirmed() {
    let mut app = start(1280.0, 900.0);
    assert!(!app.shows("We'll prepare your order as soon as possible."));
    interact(&mut app, "id:delivery-trigger");
    assert_count(&app, 1);
    assert!(app.shows("Pick a delivery time"));
    assert!(app.shows("We'll prepare your order as soon as possible."));
    interact(&mut app, "Cancel");
    assert_count(&app, 0);
    interact(&mut app, "id:delivery-trigger");
    interact(&mut app, "5:30 PM – 5:45 PM");
    interact(&mut app, "Confirm Delivery Time");
    assert_count(&app, 0);
    assert!(app.shows("Delivery time confirmed · 5:30 PM – 5:45 PM"));
}

#[test]
fn all_directions_open_and_escape_closes() {
    let mut app = start(1280.0, 900.0);
    for (label, edge) in [
        ("Open Left Drawer", 0),
        ("Top", 1),
        ("Bottom", 2),
        ("Right", 3),
    ] {
        interact(&mut app, label);
        let b = panel(&app);
        let position = match edge {
            0 => b.origin.x.0,
            1 => b.origin.y.0,
            2 => 900.0 - (b.origin.y.0 + b.size.height.0),
            _ => 1280.0 - (b.origin.x.0 + b.size.width.0),
        };
        assert!((position - 8.0).abs() < 1.0, "{label}: {b:?}");
        let (cross_start, cross_end) = if edge == 0 || edge == 3 {
            (b.origin.y.0, 900.0 - b.origin.y.0 - b.size.height.0)
        } else {
            (b.origin.x.0, 1280.0 - b.origin.x.0 - b.size.width.0)
        };
        assert!((cross_start - 8.0).abs() < 1.0, "{label}: {b:?}");
        assert!((cross_end - 8.0).abs() < 1.0, "{label}: {b:?}");
        app.press_key("escape").unwrap();
        app.wait(MOTION).unwrap();
        assert_count(&app, 0);
    }
}

#[test]
fn nesting_keeps_each_parent_mounted() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:nested-trigger");
    for label in [
        "Open Nested Drawer",
        "Open Third Drawer",
        "Open Fourth Drawer",
    ] {
        interact(&mut app, label);
    }
    for layers in (1..=4).rev() {
        assert_count(&app, layers);
        app.press_key("escape").unwrap();
        app.wait(MOTION).unwrap();
    }
    assert_count(&app, 0);
}

#[test]
fn non_modal_passes_background_presses() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:non-modal-trigger");
    interact(&mut app, "role:button[Components]");
    assert_count(&app, 1);
    interact(&mut app, "Close");
    assert_count(&app, 0);
}

#[test]
fn swipe_and_snap_handle_gestures() {
    let mut app = start(390.0, 844.0);
    interact(&mut app, "id:swipe-trigger");
    app.drag(HANDLE, point(px(0.0), px(180.0)), Default::default())
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 0);

    interact(&mut app, "id:snap-trigger");
    app.drag(HANDLE, point(px(0.0), px(-260.0)), Default::default())
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 1);
    app.drag(HANDLE, point(px(0.0), px(285.0)), Default::default())
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 1);
    app.drag(HANDLE, point(px(0.0), px(190.0)), Default::default())
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 0);
}

#[test]
fn short_swipe_settles_without_dismissing_the_drawer() {
    let mut app = start(390.0, 844.0);
    interact(&mut app, "id:swipe-trigger");
    app.drag(HANDLE, point(px(0.0), px(48.0)), Default::default())
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 1);
    interact(&mut app, "Close");
    assert_count(&app, 0);
}

#[test]
fn profile_switches_between_centered_dialog_and_bottom_drawer() {
    let mut desktop = start(1280.0, 900.0);
    interact(&mut desktop, "id:responsive-trigger");
    let desktop_panel = panel(&desktop);
    assert!((desktop_panel.center().x.0 - 640.0).abs() < 1.0);
    assert!((desktop_panel.center().y.0 - 450.0).abs() < 1.0);
    desktop.type_text("Email", "new@example.com").unwrap();
    interact(&mut desktop, "Save changes");
    assert!(desktop.shows("Profile updated"));

    let mut mobile = start(390.0, 844.0);
    interact(&mut mobile, "id:responsive-trigger");
    let mobile_panel = panel(&mobile);
    assert!((mobile_panel.origin.x.0 - 8.0).abs() < 1.0);
    assert!((mobile_panel.size.width.0 - 374.0).abs() < 1.0);
    assert!((mobile_panel.origin.y.0 + mobile_panel.size.height.0 - 836.0).abs() < 1.0);
}

#[test]
fn modal_backdrop_uses_the_rhea_tint_and_dismisses_the_drawer() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:delivery-trigger");
    let backdrop = app
        .screen()
        .view_style("id:drawer-backdrop")
        .unwrap()
        .unwrap();
    assert_eq!(backdrop.background_color, Some(0x0000004d));
    assert!((backdrop.opacity - 1.0).abs() < 0.01);
    app.press_at("id:drawer-backdrop", point(px(20.0), px(100.0)))
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 0);
}

#[test]
fn code_tabs_copy_the_selected_language_and_can_return_to_preview() {
    let mut app = start(1280.0, 900.0);
    let region = "role:region[Delivery time]";
    interact(&mut app, &format!("{region} >> Rust"));
    assert_highlighted_line(&app, "DrawerRoot::new(drawer, move || {");
    interact(&mut app, &format!("{region} >> Copy"));
    assert_eq!(
        app.screen()
            .count(&format!("{region} >> role:button[Copied]")),
        Ok(1),
    );
    assert_eq!(
        app.screen().clipboard().unwrap(),
        include_str!("../src/examples/drawer.rs"),
    );
    interact(&mut app, &format!("{region} >> RSX"));
    assert_eq!(
        app.screen()
            .count(&format!("{region} >> role:button[Copy]")),
        Ok(1),
    );
    assert_highlighted_line(&app, "use components::rsx;");
    interact(&mut app, &format!("{region} >> Copy"));
    let source = app.screen().clipboard().unwrap();
    assert_eq!(source, include_str!("../src/examples/drawer_rsx.rs"));
    assert!(source.contains("<DrawerContent accessibilityLabel=\"Move Goal\">"));
    assert!(source.contains("<View style={{ padding: 16, gap: 8 }}>"));
    interact(&mut app, &format!("{region} >> Run example"));
    interact(&mut app, &format!("{region} >> role:button[Open Drawer]"));
    assert!(app.shows("Set your daily activity goal."));
    interact(&mut app, "Done");
    interact(&mut app, &format!("{region} >> TSX"));
    assert_highlighted_line(&app, "import { Text, View } from \"nsh\";");
    assert!(app.shows("Native Shell"));
    interact(&mut app, &format!("{region} >> Copy"));
    let source = app.screen().clipboard().unwrap();
    assert_eq!(source, include_str!("../src/examples/drawer.tsx"));
    assert!(source.contains("<DrawerTrigger"));
    interact(&mut app, &format!("{region} >> Preview"));
    interact(&mut app, "id:delivery-trigger");
    assert!(app.shows("Pick a delivery time"));
}

#[component]
fn TriggerChildrenExample() -> impl IntoElement {
    components::rsx! {
        <DrawerRoot>
            <DrawerTrigger accessibilityLabel="Open settings drawer">
                <Text>Settings</Text>
            </DrawerTrigger>
            <DrawerTrigger accessibilityLabel="Open drawer without text" />
            <DrawerContent accessibilityLabel="Settings drawer">
                <Text>Drawer content</Text>
            </DrawerContent>
        </DrawerRoot>
    }
}

#[test]
fn named_trigger_renders_its_children_and_opens_the_drawer() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<TriggerChildrenExample>())).unwrap();
    app.first_render().unwrap();
    let trigger = "role:button[Open settings drawer]";
    assert_eq!(
        app.screen().count(&format!("{trigger} >> text:Settings")),
        Ok(1)
    );
    assert!(!app.shows("Open settings drawer"));
    assert!(!app.shows("Open drawer without text"));
    interact(&mut app, "role:button[Open drawer without text]");
    assert!(app.shows("Drawer content"));
}

fn assert_highlighted_line(app: &Headless, expected: &str) {
    assert!(
        app.screen()
            .layout_nodes()
            .into_iter()
            .filter_map(|node| node.text)
            .any(|line| {
                line.text.text.trim() == expected
                    && line
                        .text
                        .spans
                        .windows(2)
                        .any(|pair| pair[0].style.color != pair[1].style.color)
            }),
        "expected syntax colors on {expected:?}",
    );
}

#[test]
fn custom_size_keeps_the_footer_visible_and_inset_floats_the_panel() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:custom-size-trigger");
    assert!((panel(&app).size.height.0 - 360.0).abs() < 1.0);
    app.scroll("id:drawer-notifications", px(320.0)).unwrap();
    interact(&mut app, "Done");
    assert_count(&app, 0);

    interact(&mut app, "id:floating-trigger");
    let floating = panel(&app);
    assert!((floating.origin.x.0 + floating.size.width.0 - 1264.0).abs() < 1.0);
    assert!((floating.origin.y.0 - 16.0).abs() < 1.0);
    assert!((floating.size.width.0 - 360.0).abs() < 1.0);
}

#[component]
fn PartsExample() -> impl IntoElement {
    use components::drawer::{
        Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
        DrawerTitle, DrawerTrigger,
    };

    let drawer = Drawer::new(signal(false));
    let builds = stored(0);
    view()
        .child(
            DrawerTrigger::render(
                drawer,
                view().p_4().label("Custom opener").child(text("Open")),
            )
            .test_id("parts-trigger"),
        )
        .child(text(move || format!("Body builds: {}", builds.get())))
        .child(DrawerContent::new(drawer, "Compound Drawer", move || {
            builds.replace(builds.get() + 1);
            view()
                .child(
                    DrawerHeader::new()
                        .child(
                            view()
                                .test_id("parts-title")
                                .child(DrawerTitle::new("Compound Drawer")),
                        )
                        .child(DrawerDescription::new(
                            "Only mounted while the drawer is visible.",
                        )),
                )
                .child(
                    view().py_4().child(
                        view()
                            .test_id("parts-body")
                            .label("Reopen during exit")
                            .on_press(move || {
                                drawer.close();
                                set_timeout(Duration::from_millis(100), move || drawer.open());
                            })
                            .child(text(format!("Body build {}", builds.get()))),
                    ),
                )
                .child(
                    DrawerFooter::new().child(DrawerClose::render(
                        drawer,
                        view()
                            .p_4()
                            .label("Custom closer")
                            .test_id("parts-close")
                            .child(text("Done")),
                    )),
                )
        }))
}

#[test]
fn independently_styled_trigger_and_close_compose_with_lazy_content() {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(px(390.0), px(844.0)),
        ..EngineConfig::new(Root::of::<PartsExample>())
    })
    .unwrap();
    app.first_render().unwrap();
    let description = "Only mounted while the drawer is visible.";
    assert!(!app.shows(description));
    assert!(app.shows("Body builds: 0"));
    interact(&mut app, "id:parts-trigger");
    assert_count(&app, 1);
    assert!(app.shows(description));
    let panel = panel(&app);
    let title = bounds(&app, "parts-title");
    let body = bounds(&app, "parts-body");
    let button = bounds(&app, "parts-close");
    for part in [title, button] {
        assert!((part.origin.x.0 - panel.origin.x.0 - 17.0).abs() < 1.0);
    }
    assert!((body.origin.x.0 - panel.origin.x.0 - 1.0).abs() < 1.0);
    assert!((panel.right().0 - body.right().0 - 1.0).abs() < 1.0);
    assert!((title.origin.y.0 - panel.origin.y.0 - 17.0).abs() < 1.0);
    assert!((panel.bottom().0 - button.bottom().0 - 17.0).abs() < 1.0);
    interact(&mut app, "Reopen during exit");
    assert_count(&app, 1);
    assert!(app.shows("Body build 1"));
    interact(&mut app, "Custom closer");
    assert_count(&app, 0);
    assert!(!app.shows(description));
    interact(&mut app, "id:parts-trigger");
    assert!(app.shows("Body build 2"));
}

#[path = "../src/examples/drawer_rsx.rs"]
mod drawer_rsx;

#[component]
fn PublishedRsxExample() -> impl IntoElement {
    drawer_rsx::example()
}

#[test]
fn published_rsx_example_builds_and_opens_the_drawer() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<PublishedRsxExample>())).unwrap();
    app.first_render().unwrap();
    assert_count(&app, 0);

    interact(&mut app, "Open Drawer");
    assert_count(&app, 1);
    assert!(app.shows("Set your daily activity goal."));
    assert!(app.screen().layout_nodes().into_iter().any(|node| {
        node.text.is_some_and(|line| {
            line.text.text == "Move Goal"
                && line
                    .text
                    .spans
                    .iter()
                    .any(|span| span.style.font_size == px(20.0))
        })
    }));
    interact(&mut app, "Done");
    assert_count(&app, 0);
}
