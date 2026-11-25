use docs::App;
use std::time::Duration;

use components::code_block::strip_ansi;
use nsh_core::{Checked, Size, element::Root, platform::Os, prelude::*};
use nsh_engine::{Clock, EngineConfig};
use nsh_testing::Headless;

fn start(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::MacOs;
    config.clock = Clock::manual(std::time::UNIX_EPOCH);
    let mut app = Headless::start_at_page(config, "https://foundation.test/#code-block").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:code-block-docs-scroll").unwrap();
    app
}

#[test]
fn gallery_copies_source_with_blank_lines_and_keeps_ansi_off_the_clipboard() {
    let mut app = start(390.0);
    for name in [
        "Code Block demo",
        "Basic",
        "Header",
        "Actions",
        "Floating Actions",
        "Line Numbers",
        "Start Line",
        "Wrap",
        "Highlighted Lines",
        "Diff",
        "Height",
        "Show More",
        "Max Height",
        "Size",
        "Destructive",
        "Trace",
        "Tabs",
        "Message",
        "Typeset",
        "RTL",
        "Syntax Highlighting",
        "Languages",
        "Terminal Output",
        "Streaming",
        "Markdown",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let demo = "role:region[Code Block demo]";
    let wrap = format!("{demo} >> role:button[Wrap lines]");
    app.press(&wrap).unwrap();
    assert_eq!(
        app.screen().accessibility_state(&wrap).unwrap().pressed,
        Some(Checked::True)
    );
    app.press(&format!("{demo} >> role:button[Copy]")).unwrap();
    let copied = app.screen().clipboard().unwrap().to_owned();
    assert!(copied.starts_with("import { streamText } from \"ai\"\n\n"));
    let success = format!("{demo} >> role:button[Copied]");
    assert_eq!(app.screen().count(&success), Ok(1));
    app.run_for(Duration::from_millis(1100)).unwrap();
    app.press(&success).unwrap();
    assert_eq!(app.screen().clipboard().as_deref(), Some(copied.as_str()));
    app.run_for(Duration::from_secs(1)).unwrap();
    assert_eq!(
        app.screen().count(&format!("{demo} >> role:button[Copy]")),
        Ok(1),
    );
    app.press("role:region[Terminal Output] >> role:button[Copy]")
        .unwrap();
    let terminal = app.screen().clipboard().unwrap();
    assert!(!terminal.contains('\x1b'));
    assert!(terminal.contains("1 failed | 13 passed"));
    assert_eq!(
        strip_ansi("\x1b[32mgreen\x1b[39m\n\nlast\n"),
        "green\n\nlast\n"
    );
}

#[test]
fn expansion_tabs_and_replay_update_the_visible_examples() {
    let mut app = start(1280.0);
    let more = "role:region[Show More] >> role:button[Show more]";
    let less = "role:region[Show More] >> role:button[Show less]";
    assert_eq!(
        app.screen().accessibility_state(more).unwrap().expanded,
        Some(false)
    );
    app.press(more).unwrap();
    assert_eq!(
        app.screen().accessibility_state(less).unwrap().expanded,
        Some(true)
    );
    app.press(less).unwrap();
    app.press("role:region[Tabs] >> role:tab[.env.local]")
        .unwrap();
    app.press("role:region[Tabs] >> role:button[Copy]").unwrap();
    assert_eq!(
        app.screen().clipboard().as_deref(),
        Some("AI_GATEWAY_API_KEY=your-api-key")
    );

    let replay = "role:region[Streaming] >> role:button[Replay]";
    app.press(replay).unwrap();
    assert_eq!(app.screen().enabled(replay), Ok(false));
    app.run_for(Duration::from_secs(8)).unwrap();
    assert_eq!(app.screen().enabled(replay), Ok(true));
}

#[test]
fn documentation_code_tabs_preserve_preview_state() {
    let mut app = start(390.0);
    let demo = "role:region[Code Block demo]";
    let wrap = format!("{demo} >> role:button[Wrap lines]");
    app.press(&wrap).unwrap();

    let rust = format!("{demo} >> role:tab[Rust]");
    app.press(&rust).unwrap();
    assert!(app.screen().accessibility_state(&rust).unwrap().selected);
    assert_eq!(app.screen().count(&wrap), Ok(0));

    let preview = format!("{demo} >> role:tab[Preview]");
    app.press(&preview).unwrap();
    assert!(app.screen().accessibility_state(&preview).unwrap().selected);
    assert_eq!(
        app.screen().accessibility_state(&wrap).unwrap().pressed,
        Some(Checked::True)
    );
}
