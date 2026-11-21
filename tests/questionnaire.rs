use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

#[test]
fn conditional_submission_excludes_answers_from_a_disabled_step() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(420.0), px(840.0));
    config.environment.os = Os::Web;
    config.environment.reduce_motion = true;
    let mut app =
        Headless::start_at_page(config, "https://foundation.test/#questionnaire").unwrap();
    app.first_render().unwrap();

    let section = "role:region[Conditional Items]";
    for control in [
        "role:radio[Cloud environment]",
        "role:button[Next]",
        "role:radio[Staging]",
        "role:button[Next]",
        "role:button[Previous]",
        "role:button[Previous]",
        "role:radio[Local workspace]",
        "role:button[Next]",
        "role:radio[Before writing files]",
        "role:button[Save execution plan]",
    ] {
        app.press(&format!("{section} >> {control}")).unwrap();
    }
    assert!(app.shows("Execution plan saved. approval: writes · runtime: local"));
}
