use docs::App;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[test]
fn invoice_table_matches_headers_and_rows_and_sums_each_visible_set() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Web;
    let mut app = Headless::start_at_page(config, "https://foundation.test/#table").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:table-docs-scroll").unwrap();
    for (preview, rows, total) in [("Table demo", 9, "$2,250.00"), ("Footer", 5, "$750.00")] {
        let table = format!("role:region[{preview}] >> role:table[Recent invoices]");
        assert_eq!(
            app.screen().count(&format!("{table} >> role:columnheader")),
            Ok(4)
        );
        assert_eq!(
            app.screen().count(&format!("{table} >> role:row")),
            Ok(rows)
        );
        assert_eq!(
            app.screen()
                .count(&format!("{table} >> id:table-footer >> role:cell")),
            Ok(2)
        );
        assert!(app.shows(total));
    }
}
