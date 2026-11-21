use nsh_core::{
    Size,
    element::Root,
    platform::{Idiom, Os},
    prelude::*,
};
use nsh_engine::EngineConfig;
use nsh_foundation::{
    App,
    chart::{ChartConfig, ChartContainer, ChartIndicator, ChartRow, ChartSeries},
};
use nsh_testing::Headless;

fn config(root: Root, width: f32) -> EngineConfig {
    let mut config = EngineConfig::new(root);
    config.viewport = Size::new(px(width), px(900.0));
    config.environment.os = Os::MacOs;
    config.environment.idiom = Idiom::Desktop;
    config
}

#[test]
fn gallery_switches_the_interactive_series_and_keeps_the_tooltip() {
    let mut app = Headless::start_at_page(
        config(Root::of::<App>(), 390.0),
        "https://foundation.test/#chart",
    )
    .unwrap();
    app.first_render().unwrap();
    app.wait_for("id:chart-docs-scroll").unwrap();

    for name in [
        "Interactive Bar Chart",
        "Basic",
        "Grid",
        "Axis",
        "Add Tooltip",
        "Legend",
        "Tooltip styles",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }
    let chart = "role:region[Interactive Bar Chart]";
    let desktop = format!("{chart} >> role:button[Desktop]");
    let mobile = format!("{chart} >> role:button[Mobile]");
    assert_eq!(app.screen().selected(&desktop), Ok(true));
    app.hover("label:Daily page views").unwrap();
    app.wait_for(&format!("{chart} >> id:chart-tooltip-content"))
        .unwrap();

    app.press(&mobile).unwrap();
    assert_eq!(app.screen().selected(&desktop), Ok(false));
    assert_eq!(app.screen().selected(&mobile), Ok(true));
    app.hover("label:Daily page views").unwrap();
    app.wait_for(&format!("{chart} >> id:chart-tooltip-content"))
        .unwrap();
    assert!(app.shows("12,486"));
}

#[component]
fn AccessiblePlot() -> impl IntoElement {
    let config = ChartConfig::new([ChartSeries::new("visitors", "Visitors", || {
        nsh_base::theme().chart_1
    })]);
    ChartContainer::new(
        "RTL traffic",
        config,
        vec![
            ChartRow::new("Monday", [140.0]).tick("Mon"),
            ChartRow::new("Tuesday", [180.0]).tick("Tue"),
            ChartRow::new("Wednesday", [250.0]).tick("Wed"),
        ],
    )
    .rtl(true)
    .axis(3)
    .grid()
    .tooltip(ChartIndicator::Dashed)
    .legend()
}

#[test]
fn keyboard_steps_through_the_reversed_plot_and_dismisses_the_tooltip() {
    let mut app = Headless::start(config(Root::of::<AccessiblePlot>(), 1000.0)).unwrap();
    app.first_render().unwrap();
    assert!(app.shows("Mon"));
    let x = |label| {
        app.screen()
            .layout_nodes()
            .into_iter()
            .find(|node| {
                node.text
                    .as_ref()
                    .is_some_and(|text| &*text.text.text == label)
            })
            .unwrap()
            .bounds
            .origin
            .x
            .0
    };
    assert!(x("Mon") > x("Wed"));
    app.press_key("tab").unwrap();
    app.press_key("home").unwrap();
    // The native tooltip's animated translate is omitted from headless bounds.
    let tooltip_has = |app: &Headless, label| {
        app.screen().layout_nodes().into_iter().any(|node| {
            node.text
                .as_ref()
                .is_some_and(|text| &*text.text.text == label)
        })
    };
    assert!(tooltip_has(&app, "Monday"));
    app.press_key("left").unwrap();
    assert!(tooltip_has(&app, "Tuesday"));
    app.press_key("escape").unwrap();
    app.wait_for_gone(
        "id:chart-tooltip-content",
        std::time::Duration::from_secs(2),
    )
    .unwrap();
}
