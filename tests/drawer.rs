use std::time::Duration;

use nsh_core::{Bounds, Points, Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

const PANEL: &str = "id:drawer-panel";
const HANDLE: &str = "id:drawer-swipe-handle";
const MOTION: Duration = Duration::from_millis(300);

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
    app.screen()
        .layout_nodes()
        .into_iter()
        .find(|node| node.test_id.as_deref() == Some("drawer-panel"))
        .expect("drawer panel")
        .bounds
}

#[test]
fn delivery_can_be_selected_and_confirmed() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:delivery-trigger");
    assert_count(&app, 1);
    assert!(app.shows("Pick a delivery time"));
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
            2 => b.origin.y.0 + b.size.height.0 - 900.0,
            _ => b.origin.x.0 + b.size.width.0 - 1280.0,
        };
        assert!(position.abs() < 1.0, "{label}: {b:?}");
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
    interact(&mut app, "Components");
    assert_count(&app, 1);
    interact(&mut app, "Close");
    assert_count(&app, 0);
}

#[test]
fn swipe_and_snap_handle_gestures() {
    let mut app = start(390.0, 844.0);
    interact(&mut app, "id:swipe-trigger");
    app.drag(HANDLE, point(px(0.0), px(180.0))).unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 0);

    interact(&mut app, "id:snap-trigger");
    app.drag(HANDLE, point(px(0.0), px(-260.0))).unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 1);
    app.drag(HANDLE, point(px(0.0), px(285.0))).unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 1);
    app.drag(HANDLE, point(px(0.0), px(190.0))).unwrap();
    app.wait(MOTION).unwrap();
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
    assert!((mobile_panel.size.width.0 - 390.0).abs() < 1.0);
    assert!((mobile_panel.origin.y.0 + mobile_panel.size.height.0 - 844.0).abs() < 1.0);
}

#[test]
fn modal_backdrop_dismisses_the_drawer() {
    let mut app = start(1280.0, 900.0);
    interact(&mut app, "id:delivery-trigger");
    app.press_at("id:drawer-backdrop", point(px(20.0), px(100.0)))
        .unwrap();
    app.wait(MOTION).unwrap();
    assert_count(&app, 0);
}
