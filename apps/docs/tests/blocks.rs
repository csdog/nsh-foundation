use docs::App;
use nsh_core::{Size, element::Root, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn mobile_blocks_chat_keeps_drafts_and_messages_in_their_own_threads() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(320.0), px(620.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:blocks-chat").unwrap();

    app.type_text("label:Message", "First thread").unwrap();
    app.press("role:button[Send]").unwrap();
    app.wait_for("id:chat-reply").unwrap();
    app.type_text("label:Message", "Original draft").unwrap();
    app.press("role:button[History]").unwrap();
    app.press("role:button[New chat]").unwrap();
    app.wait_for("Start a conversation").unwrap();
    app.type_text("label:Message", "Second thread").unwrap();
    app.press("role:button[Send]").unwrap();

    app.press("role:button[History]").unwrap();
    app.press("role:button[First thread]").unwrap();
    assert!(app.shows("First thread"));
    assert!(!app.shows("Second thread"));
    app.press("role:button[Send]").unwrap();
    app.wait_for("Original draft").unwrap();
}
