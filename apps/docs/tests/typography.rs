use docs::App;
use nsh_core::{element::Root, platform::Os};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn headings_have_distinct_levels_and_comparison_keeps_table_semantics() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#typography").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:typography-docs-scroll").unwrap();

    for level in 1..=4 {
        let heading = format!("role:region[h{level}] >> id:typography-heading");
        assert_eq!(
            app.screen().accessibility_level(&heading).unwrap(),
            Some(level)
        );
    }
    let table = "role:region[table] >> role:table";
    assert_eq!(
        app.screen().count(&format!("{table} >> role:columnheader")),
        Ok(2)
    );
    assert_eq!(app.screen().count(&format!("{table} >> role:cell")), Ok(6));
}
