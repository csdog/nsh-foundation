use docs::App;
use nsh_core::{Bounds, Points, Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn right_to_left_mirrors_the_avatar_without_resizing_the_shapes() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#skeleton").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:skeleton-docs-scroll").unwrap();

    let rtl_shapes = |app: &Headless| {
        let shapes = app
            .screen()
            .layout_nodes()
            .into_iter()
            .filter(|node| node.test_id.as_deref() == Some("skeleton-placeholder"))
            .map(|node| node.bounds)
            .collect::<Vec<Bounds<Points>>>();
        shapes[shapes.len() - 3..].to_vec()
    };

    let arabic = rtl_shapes(&app);
    assert_eq!(arabic[0].size, Size::new(px(48.0), px(48.0)));
    assert!(arabic[0].origin.x > arabic[1].origin.x);
    app.press("role:region[RTL] >> role:button[English]")
        .unwrap();
    let english = rtl_shapes(&app);
    assert_eq!(english[0].size, arabic[0].size);
    assert_eq!(english[1].size, arabic[1].size);
    assert!(english[0].origin.x < english[1].origin.x);
}
