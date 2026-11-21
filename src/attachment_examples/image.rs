use nsh_core::prelude::*;
use nsh_foundation::attachment::*;

pub fn preview() -> impl IntoElement {
    let mut group = AttachmentGroup::new("Image attachments", 120.0);
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(image(asset!("attachment-workspace.jpg")).label("Workspace").w_full().h_full().object_fit(ResizeMode::Cover)))
                .content(AttachmentContent::new(AttachmentTitle::new("workspace.png")).description(AttachmentDescription::new("PNG · 820 KB")))
                .trigger(AttachmentTrigger::new("Open workspace.png", || { let _ = open_url("https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"); }))
                .actions(AttachmentActions::new().child(AttachmentAction::new("Remove workspace.png", "×", move || visible.set(false))))
        });
    }
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(image(asset!("attachment-desk.jpg")).label("Desk").w_full().h_full().object_fit(ResizeMode::Cover)))
                .content(AttachmentContent::new(AttachmentTitle::new("desk-reference.jpg")).description(AttachmentDescription::new("JPG · 1.1 MB")))
                .trigger(AttachmentTrigger::new("Open desk-reference.jpg", || { let _ = open_url("https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80"); }))
                .actions(AttachmentActions::new().child(AttachmentAction::new("Remove desk-reference.jpg", "×", move || visible.set(false))))
        });
    }
    {
        let visible = signal(true);
        group = group.child_when(visible, move || {
            Attachment::new()
                .orientation(AttachmentOrientation::Vertical)
                .media(AttachmentMedia::image(image(asset!("attachment-office.jpg")).label("Office").w_full().h_full().object_fit(ResizeMode::Cover)))
                .content(AttachmentContent::new(AttachmentTitle::new("office-reference.jpg")).description(AttachmentDescription::new("JPG · 940 KB")))
                .trigger(AttachmentTrigger::new("Open office-reference.jpg", || { let _ = open_url("https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80"); }))
                .actions(AttachmentActions::new().child(AttachmentAction::new("Remove office-reference.jpg", "×", move || visible.set(false))))
        });
    }
    view().w_full().max_w(px(448.0)).child(group)
}
