use nsh_core::prelude::*;
use nsh_foundation::attachment::*;

pub fn preview() -> impl IntoElement {
    let mut group = AttachmentGroup::new("File attachments", 256.0);
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .media(AttachmentMedia::icon(text("▤")))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("briefing-notes.pdf"))
                        .description(AttachmentDescription::new("PDF · 1.4 MB")),
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Remove briefing-notes.pdf",
                    "×",
                    move || visible.set(false),
                )))
        });
    }
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
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
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Remove workspace.png",
                    "×",
                    move || visible.set(false),
                )))
        });
    }
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .media(AttachmentMedia::icon(text("⊞")))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("customers.csv"))
                        .description(AttachmentDescription::new("CSV · 18 KB")),
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Remove customers.csv",
                    "×",
                    move || visible.set(false),
                )))
        });
    }
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .media(AttachmentMedia::icon(text("⌘")))
                .content(
                    AttachmentContent::new(AttachmentTitle::new("renderer.tsx"))
                        .description(AttachmentDescription::new("TSX · 12 KB")),
                )
                .actions(AttachmentActions::new().child(AttachmentAction::new(
                    "Remove renderer.tsx",
                    "×",
                    move || visible.set(false),
                )))
        });
    }
    view().w_full().max_w(px(448.0)).child(group)
}
