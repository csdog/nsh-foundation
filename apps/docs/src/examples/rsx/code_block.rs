use components::{code_block::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                CodeBlock::new(
                    CodeBlockContent::new("let count = 3;").language("rust"),
                )
            }
        </View>
    }
}
