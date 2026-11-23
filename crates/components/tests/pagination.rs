use components::pagination::{
    Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious,
};
use nsh_core::{AccessibilityCurrent, element::Root, platform::Os, prelude::*};
use nsh_engine::EngineConfig;
use nsh_testing::Headless;

#[component]
fn PageLinks() -> impl IntoElement {
    Pagination::new(
        PaginationContent::new()
            .item(PaginationItem::new(PaginationPrevious::new(
                PaginationLink::new("Previous", "https://example.com/previous").disabled(true),
            )))
            .item(PaginationItem::new(
                PaginationLink::new("1", "https://example.com/page-1")
                    .label("Page 1")
                    .current(true),
            )),
    )
}

#[test]
fn current_page_link_opens_and_keyboard_skips_the_disabled_destination() {
    let mut config = EngineConfig::new(Root::of::<PageLinks>());
    config.environment.os = Os::Web;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();

    let current = app
        .screen()
        .accessibility_state("role:link[Page 1]")
        .unwrap();
    assert_eq!(current.current, Some(AccessibilityCurrent::Page));

    app.press_key("tab").unwrap();
    app.press_key("enter").unwrap();
    assert_eq!(app.screen().opened_urls(), ["https://example.com/page-1"]);
}
