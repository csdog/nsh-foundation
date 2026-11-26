use components::{rsx, typography::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {heading(HeadingLevel::H1, "Release notes")}
            {paragraph("The latest features and fixes.")}
        </View>
    }
}
