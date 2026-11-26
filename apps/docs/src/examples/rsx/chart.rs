use components::{chart::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let config = ChartConfig::new([ChartSeries::new(
        "orders",
        "Orders",
        AdaptiveColor::rgb(0x2563eb, 0x60a5fa),
    )]);
    rsx! {
        <View>
            {
                ChartContainer::new(
                    "Weekly orders",
                    config,
                    vec![
                        ChartRow::new("Monday", [12.0]),
                        ChartRow::new("Tuesday", [18.0]),
                    ],
                )
                .surface()
            }
        </View>
    }
}
