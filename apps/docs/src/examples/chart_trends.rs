use components::{
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    chart::{ChartConfig, ChartContainer, ChartIndicator, ChartKind, ChartRow, ChartSeries},
};
use nsh_core::prelude::*;

pub fn line() -> impl IntoElement {
    monthly_trend(
        "Line Chart",
        "Monthly desktop visitors",
        ChartKind::Line,
        ChartIndicator::Dot,
    )
}

pub fn area() -> impl IntoElement {
    monthly_trend(
        "Area Chart",
        "Monthly desktop visitors, measured from zero",
        ChartKind::Area,
        ChartIndicator::Line,
    )
}

fn monthly_trend(
    title: &'static str,
    description: &'static str,
    kind: ChartKind,
    indicator: ChartIndicator,
) -> Card {
    let rows = [
        ("January", "Jan", 186.0),
        ("February", "Feb", 305.0),
        ("March", "Mar", 237.0),
        ("April", "Apr", 73.0),
        ("May", "May", 209.0),
        ("June", "Jun", 214.0),
    ]
    .into_iter()
    .map(|(label, tick, value)| ChartRow::new(label, [value]).tick(tick))
    .collect();

    let config = ChartConfig::new([ChartSeries::new(
        "desktop",
        "Desktop",
        AdaptiveColor::new(rgb(0x2563eb), rgb(0x60a5fa)),
    )]);
    Card::new()
        .header(
            CardHeader::new()
                .title(CardTitle::new(title))
                .description(CardDescription::new(description)),
        )
        .content(CardContent::new(
            ChartContainer::new(title, config, rows)
                .kind(kind)
                .grid()
                .axis(6)
                .tooltip(indicator),
        ))
}
