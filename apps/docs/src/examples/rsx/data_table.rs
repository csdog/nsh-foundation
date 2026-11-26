use components::{data_table::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                DataTable::new(
                    "Orders",
                    vec!["A-001", "A-002"],
                    vec![DataTableColumn::new("Order", 160.0, |row: &&str| {
                        text(*row)
                    })],
                )
            }
        </View>
    }
}
