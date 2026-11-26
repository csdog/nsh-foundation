use components::{native_select::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                NativeSelect::new("Status")
                    .option(NativeSelectOption::new("", "Select status"))
                    .option(NativeSelectOption::new("todo", "Todo"))
                    .option(NativeSelectOption::new("done", "Done"))
            }
        </View>
    }
}
