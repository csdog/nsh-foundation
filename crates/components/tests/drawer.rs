use std::time::Duration;

use components::{
    div,
    drawer::{Drawer, DrawerCloseProps, DrawerContent, DrawerRoot, drawer_trigger},
    rsx,
};
use nsh_core::{element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn ScopedDrawers() -> impl IntoElement {
    let outer = Drawer::new(signal(false));
    let inner = Drawer::new(signal(false));
    let builds = stored(0);
    rsx! {
        DrawerRoot::new(outer) => {
            div() {
                DrawerRoot::new(inner) => {
                    div() {
                        drawer_trigger().child(text("Open inner")),
                        DrawerContent { label: "Inner Drawer" } => {
                            div() {
                                DrawerCloseProps { label: "Close inner" },
                            }
                        },
                    }
                },
                drawer_trigger().child(text("Open outer")),
                text(format!("Scoped builds: {}", builds.get())),
                DrawerContent { label: "Outer Drawer" } => {{
                    builds.replace(builds.get() + 1);
                    rsx! {
                        div() {
                            text(format!("Scoped body {}", builds.get())),
                            DrawerCloseProps { label: "Close outer" },
                        }
                    }
                }},
            }
        }
    }
}

fn interact(app: &mut Headless, selector: &str) {
    app.press(selector).unwrap();
    app.wait(Duration::from_millis(500)).unwrap();
}

fn assert_count(app: &Headless, count: usize) {
    assert_eq!(app.screen().count("id:drawer-panel"), Ok(count));
}

#[test]
fn drawer_parts_use_the_nearest_root_including_lazy_portal_content() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<ScopedDrawers>())).unwrap();
    app.first_render().unwrap();
    assert!(app.shows("Scoped builds: 0"));
    assert!(app.shows("Open inner"));
    assert!(app.shows("Open outer"));
    assert_count(&app, 0);

    interact(&mut app, "Open inner");
    assert_count(&app, 1);
    assert!(app.shows("Close inner"));
    assert!(!app.shows("Close outer"));
    interact(&mut app, "Close inner");
    assert_count(&app, 0);

    interact(&mut app, "Open outer");
    assert_count(&app, 1);
    assert!(app.shows("Scoped body 1"));
    interact(&mut app, "Close outer");
    assert_count(&app, 0);

    interact(&mut app, "Open outer");
    assert!(app.shows("Scoped body 2"));
}
