use components::{rsx, table::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Table::new(
                    "Invoices",
                    [TableColumn::fixed(100.0), TableColumn::flexible(100.0)],
                )
                .header(
                    TableHeader::new().row(
                        TableRow::new()
                            .head(TableHead::new("Invoice"))
                            .head(TableHead::new("Amount").align_end()),
                    ),
                )
                .body(
                    TableBody::new().row(
                        TableRow::new()
                            .cell(TableCell::text("INV001"))
                            .cell(TableCell::text("$250.00").align_end()),
                    ),
                )
                .caption(TableCaption::new("Recent invoices"))
            }
        </View>
    }
}
