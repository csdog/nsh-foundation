use components::attachment::*;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let mut group = AttachmentGroup::new("Image attachments", 120.0);
    {
        group = group.child(
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(
                    image(asset!("attachment-workspace.jpg"))
                        .label("Workspace")
                        .w_full()
                        .h_full()
                        .object_fit(ResizeMode::Cover),
                ))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("workspace.png"))
                        .description(AttachmentDescription::new("PNG · 820 KB")),
                ),
        );
    }
    {
        group = group.child(
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(
                    image(asset!("attachment-desk.jpg"))
                        .label("Desk")
                        .w_full()
                        .h_full()
                        .object_fit(ResizeMode::Cover),
                ))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("desk-reference.jpg"))
                        .description(AttachmentDescription::new("JPG · 1.1 MB")),
                ),
        );
    }
    {
        group = group.child(
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(
                    image(asset!("attachment-office.jpg"))
                        .label("Office")
                        .w_full()
                        .h_full()
                        .object_fit(ResizeMode::Cover),
                ))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("office-reference.jpg"))
                        .description(AttachmentDescription::new("JPG · 940 KB")),
                ),
        );
    }
    let uploading = signal(true);
    let document = signal(true);
    view()
        .w_full()
        .max_w(px(448.0))
        .gap_2()
        .child(group)
        .child(when(uploading, move || {
            Attachment::new()
                .state(AttachmentState::Uploading)
                .media(AttachmentMedia::icon(text("↥")))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("sales-dashboard.pdf"))
                        .description(AttachmentDescription::new("Uploading · 64%")),
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Cancel upload",
                    "×",
                    move || uploading.set(false),
                )))
        }))
        .child(when(document, move || {
            Attachment::new()
                .state(AttachmentState::Done)
                .media(AttachmentMedia::icon(text("⌘")))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("message-renderer.tsx"))
                        .description(AttachmentDescription::new("TypeScript · 12 KB")),
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Remove message-renderer.tsx",
                    "×",
                    move || document.set(false),
                )))
        }))
}
