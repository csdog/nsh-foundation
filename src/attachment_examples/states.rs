use std::time::Duration;

use nsh_core::prelude::*;
use nsh_foundation::attachment::*;

pub fn preview() -> impl IntoElement {
    view()
        .w_full()
        .max_w(px(384.0))
        .gap_2()
        .child(state_card(
            "selected-file.pdf",
            AttachmentState::Idle,
            "Ready to upload",
            "◷",
            "Remove selected-file.pdf",
        ))
        .child(state_card(
            "design-system.zip",
            AttachmentState::Uploading,
            "Uploading · 64%",
            "↥",
            "Cancel upload",
        ))
        .child(state_card(
            "market-research.pdf",
            AttachmentState::Processing,
            "Processing document",
            "◌",
            "Remove market-research.pdf",
        ))
        .child(retry_card())
        .child(state_card(
            "uploaded-report.pdf",
            AttachmentState::Done,
            "Uploaded · 1.8 MB",
            "✓",
            "Remove uploaded-report.pdf",
        ))
}

fn state_card(
    name: &'static str,
    state: AttachmentState,
    detail: &'static str,
    icon: &'static str,
    label: &'static str,
) -> impl IntoElement {
    let visible = signal(true);
    when(visible, move || {
        Attachment::new()
            .state(state)
            .media(AttachmentMedia::icon(text(icon)))
            .content(
                AttachmentContent::new(AttachmentTitle::new(name))
                    .description(AttachmentDescription::new(detail)),
            )
            .actions(AttachmentActions::new().child(AttachmentAction::new(
                label,
                "×",
                move || visible.set(false),
            )))
    })
}

fn retry_card() -> impl IntoElement {
    use AttachmentState::{Done, Error, Uploading};
    let visible = signal(true);
    let state = signal(Error);
    when(visible, move || {
        let actions = AttachmentActions::new()
            .child(
                AttachmentAction::new("Retry upload", "↻", move || {
                    state.set(Uploading);
                    set_timeout(Duration::from_millis(900), move || state.set(Done));
                })
                .visible(move || state.get() == Error),
            )
            .child(AttachmentAction::new(
                "Remove financial-model.xlsx",
                "×",
                move || visible.set(false),
            ));
        Attachment::new()
            .state(state)
            .media(AttachmentMedia::icon(text(move || match state.get() {
                Error => "!",
                Uploading => "↥",
                _ => "✓",
            })))
            .content(
                AttachmentContent::new(AttachmentTitle::new("financial-model.xlsx")).description(
                    AttachmentDescription::new(move || match state.get() {
                        Error => "Upload failed · File exceeds 25 MB",
                        Uploading => "Retrying sample upload…",
                        _ => "Uploaded · Sample file",
                    }),
                ),
            )
            .actions(actions)
    })
}
