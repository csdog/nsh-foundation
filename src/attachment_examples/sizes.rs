use nsh_core::prelude::*;
use nsh_foundation::attachment::*;

pub fn preview() -> impl IntoElement {
    let mut stack = view().w_full().max_w(px(384.0)).gap_2();
    for (size, title) in [
        (AttachmentSize::Default, "Default attachment"),
        (AttachmentSize::Small, "Small attachment"),
        (AttachmentSize::ExtraSmall, "Extra small attachment"),
    ] {
        let content = AttachmentContent::new(AttachmentTitle::new(title));
        let content = if size == AttachmentSize::ExtraSmall {
            content
        } else {
            content.description(AttachmentDescription::new("PDF · 2.4 MB"))
        };
        stack = stack.child(
            Attachment::new()
                .size(size)
                .media(AttachmentMedia::icon(text("▤")))
                .content(content),
        );
    }
    stack
}
