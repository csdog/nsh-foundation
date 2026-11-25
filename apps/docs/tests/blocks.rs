use std::time::Duration;

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

    app.press("id:chat-new").unwrap();
    app.wait_for("Start a conversation").unwrap();
    app.type_text("label:Message", "First thread").unwrap();
    app.press("role:button[Send]").unwrap();
    app.wait_for("id:chat-reply").unwrap();
    app.type_text("label:Message", "Original draft").unwrap();
    app.press("role:button[History]").unwrap();
    app.wait_for("id:chat-history-underlay").unwrap();
    app.press("role:button[New chat]").unwrap();
    app.wait_for("Start a conversation").unwrap();
    app.type_text("label:Message", "Second thread").unwrap();
    app.press("role:button[Send]").unwrap();

    app.press("role:button[History]").unwrap();
    app.press("role:button[Search history]").unwrap();
    app.type_text("label:Search chats", "first").unwrap();
    let history = "role:navigation[Chat history]";
    assert_eq!(
        app.screen().count(&format!("{history} >> Second thread")),
        Ok(0)
    );
    app.press(&format!("{history} >> First thread")).unwrap();
    assert!(app.shows("First thread"));
    assert!(!app.shows("Second thread"));
    app.press("role:button[Send]").unwrap();
    app.wait_for("Original draft").unwrap();
}

#[test]
fn desktop_composer_moves_below_the_transcript_and_preserves_the_editor() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(1280.0), px(900.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:chat-composer").unwrap();

    let composer_top = |app: &Headless| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some("chat-composer"))
            .unwrap()
            .bounds
            .origin
            .y
            .0
    };
    app.press("id:chat-new").unwrap();
    app.wait_for("Start a conversation").unwrap();
    let initial = composer_top(&app);
    assert!((260.0..580.0).contains(&initial));
    app.type_text("label:Message", "First message").unwrap();
    app.press("role:button[Send]").unwrap();
    app.wait_for("id:chat-reply").unwrap();
    assert!(composer_top(&app) > 720.0);
    app.press("role:button[Copy reply]").unwrap();
    assert_eq!(
        app.screen().clipboard().as_deref(),
        Some("Message received. This is a local chat demo.")
    );
    app.press("id:chat-new").unwrap();
    app.wait_for("Start a conversation").unwrap();
    assert_eq!(composer_top(&app), initial);
}

#[test]
fn stopping_a_local_reply_keeps_the_message_and_allows_another_send() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:blocks-chat").unwrap();

    app.press("id:chat-new").unwrap();
    app.wait_for("Start a conversation").unwrap();
    app.type_text("label:Message", "Keep this message").unwrap();
    app.press("role:button[Send]").unwrap();
    app.wait_for("id:chat-pending").unwrap();
    app.press("role:button[Stop reply]").unwrap();
    app.wait(Duration::from_millis(700)).unwrap();
    assert!(app.shows("Keep this message"));
    assert_eq!(app.screen().count("id:chat-pending"), Ok(0));
    assert_eq!(app.screen().count("id:chat-reply"), Ok(0));

    app.type_text("label:Message", "Try again").unwrap();
    app.press("role:button[Send]").unwrap();
    app.wait_for("id:chat-reply").unwrap();
    assert_eq!(app.screen().count("id:chat-message"), Ok(2));
    assert_eq!(app.screen().count("id:chat-reply"), Ok(1));
}

#[test]
fn mobile_history_returns_to_the_same_chat_surface() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:chat-main-surface").unwrap();
    let position = |app: &Headless, id| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| node.test_id.as_deref() == Some(id))
            .unwrap()
            .bounds
    };
    app.type_text("label:Message", "Keep this draft").unwrap();
    app.press("role:button[History]").unwrap();
    app.wait_for("id:chat-history-underlay").unwrap();
    let panel = position(&app, "chat-history-underlay");
    let surface = position(&app, "chat-main-surface");
    assert_eq!(panel.origin.x, px(0.0));
    assert_eq!(surface.origin.x, panel.size.width);
    assert_eq!(surface.size.width, px(390.0));
    app.press("id:chat-history-return").unwrap();
    assert_eq!(app.screen().count("id:chat-history-underlay"), Ok(0));

    app.press("role:button[History]").unwrap();
    app.press_key("escape").unwrap();
    assert_eq!(app.screen().count("id:chat-history-underlay"), Ok(0));
    assert_eq!(app.screen().count("id:chat-main-surface"), Ok(1));
    app.press("role:button[Send]").unwrap();
    app.wait_for("Keep this draft").unwrap();
}

#[test]
fn mobile_history_swipes_commit_or_return_to_their_start() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    let edge = "id:chat-history-edge";
    let panel = "id:chat-history-underlay";
    let close = "id:chat-history-return";
    app.wait_for(edge).unwrap();

    app.drag(edge, point(px(14.0), px(0.0)), Default::default())
        .unwrap();
    assert_eq!(app.screen().count(panel), Ok(0));

    app.drag(edge, point(px(150.0), px(0.0)), Default::default())
        .unwrap();
    app.wait_for(panel).unwrap();
    assert_eq!(app.screen().count(close), Ok(1));

    app.drag(close, point(px(-14.0), px(0.0)), Default::default())
        .unwrap();
    assert_eq!(app.screen().count(panel), Ok(1));

    app.drag(close, point(px(-150.0), px(0.0)), Default::default())
        .unwrap();
    assert_eq!(app.screen().count(panel), Ok(0));
    assert_eq!(app.screen().count(edge), Ok(1));
}

#[test]
fn blocks_opens_a_populated_sample_and_allows_switching_to_a_blank_chat() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#blocks").unwrap();
    app.first_render().unwrap();
    app.wait_for("Mobile layout review").unwrap();
    assert_eq!(app.screen().count("id:chat-message"), Ok(3));
    assert_eq!(app.screen().count("id:chat-reply"), Ok(3));

    app.press("role:button[History]").unwrap();
    app.press("role:navigation[Chat history] >> Release notes")
        .unwrap();
    app.wait_for("Draft a short release note for this iteration.")
        .unwrap();
    assert_eq!(app.screen().count("id:chat-reply"), Ok(1));

    app.press("id:chat-new").unwrap();
    app.wait_for("Start a conversation").unwrap();
    assert_eq!(app.screen().count("id:chat-message"), Ok(0));
}
