use components::{attachment::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Attachment::new().content(
                    AttachmentContent::new(AttachmentTitle::new("Report.pdf"))
                        .description(AttachmentDescription::new("PDF document")),
                )
            }
        </View>
    }
}
