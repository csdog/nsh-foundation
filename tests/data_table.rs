use nsh_core::{
    Checked, Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_foundation::App;
use nsh_testing::Headless;

fn page(width: f32) -> Headless {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::Web;
    config.environment.idiom = Idiom::Desktop;
    config.environment.reduce_motion = true;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#data-table").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:data-table-docs-scroll").unwrap();
    app
}

const DEMO: &str = "role:region[Data Table demo]";
const PAGINATION: &str = "role:region[Pagination]";

#[test]
fn sorting_filtering_and_column_visibility_work_with_the_same_rows() {
    let mut app = page(1280.0);
    let emails = |app: &Headless| {
        app.screen()
            .texts()
            .into_iter()
            .filter(|s| s.ends_with("@example.com"))
            .take(5)
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let sort = format!("{DEMO} >> role:button[Email]");
    app.press(&sort).unwrap();
    let ascending = emails(&app);
    assert_eq!(
        ascending.first().map(String::as_str),
        Some("abe45@example.com")
    );
    app.press(&sort).unwrap();
    let descending = emails(&app);
    assert_eq!(
        descending.first().map(String::as_str),
        Some("silas22@example.com")
    );

    app.press(&format!("{DEMO} >> role:button[Columns]"))
        .unwrap();
    let toggle = "role:menuitemcheckbox[Amount]";
    app.press(toggle).unwrap();
    assert_eq!(
        app.screen().accessibility_state(toggle).unwrap().checked,
        Some(Checked::False)
    );
    assert_eq!(
        app.screen()
            .count(&format!("{DEMO} >> role:columnheader[Amount]")),
        Ok(0)
    );
    app.press_key("escape").unwrap();

    let filter = format!("{DEMO} >> label:Filter emails...");
    app.type_text(&filter, "CARMELLA").unwrap();
    app.wait_for(&format!("{DEMO} >> text:carmella@example.com"))
        .unwrap();
    assert_eq!(
        app.screen()
            .count(&format!("{DEMO} >> role:checkbox[Select row]")),
        Ok(1)
    );
    app.press(&format!("{DEMO} >> role:button[Open menu]"))
        .unwrap();
    app.press("role:menuitem[Copy payment ID]").unwrap();
    assert_eq!(app.screen().clipboard().as_deref(), Some("bhqecj4p"));
    app.type_text(&filter, "zzzz").unwrap();
    app.wait_for(&format!("{DEMO} >> text:No results."))
        .unwrap();
}

#[test]
fn page_select_all_preserves_selections_across_pages_and_filter_changes() {
    let mut app = page(390.0);
    let select = format!("{PAGINATION} >> role:checkbox[Select all]");
    let next = format!("{PAGINATION} >> role:button[Next]");
    let previous = format!("{PAGINATION} >> role:button[Previous]");
    let selected = |app: &Headless, count: &str| {
        assert_eq!(
            app.screen().count(&format!("{PAGINATION} >> text:{count}")),
            Ok(1)
        );
    };
    assert_eq!(
        app.screen()
            .count(&format!("{PAGINATION} >> role:checkbox[Select row]")),
        Ok(2)
    );
    app.press(&select).unwrap();
    selected(&app, "2 of 5 row(s) selected.");
    app.press(&next).unwrap();
    assert_eq!(
        app.screen().accessibility_state(&select).unwrap().checked,
        Some(Checked::False)
    );
    app.press(&next).unwrap();
    assert_eq!(app.screen().enabled(&next), Ok(false));
    app.press(&select).unwrap();
    selected(&app, "3 of 5 row(s) selected.");

    let filter = format!("{PAGINATION} >> label:Filter emails...");
    app.type_text(&filter, "KEN").unwrap();
    assert_eq!(app.screen().enabled(&previous), Ok(false));
    selected(&app, "1 of 1 row(s) selected.");
}
