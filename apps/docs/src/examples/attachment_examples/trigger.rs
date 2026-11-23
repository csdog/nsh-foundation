use components::attachment::*;
use nsh_base::{dialog, theme};
use nsh_core::{element::set_clipboard, prelude::*};

pub fn preview() -> impl IntoElement {
    let visible = signal(true);
    let open = signal(false);
    let copied = signal(false);

    view().w_full().max_w(px(384.0)).gap_2()
        .child(when(visible, move || {
            Attachment::new()
                .media(AttachmentMedia::icon(text("▤")))
                .content(AttachmentContent::new(AttachmentTitle::new("research-summary.pdf"))
                    .description(AttachmentDescription::new("Open preview dialog")))
                .trigger(AttachmentTrigger::new("Preview research-summary.pdf", move || open.set(true)))
                .actions(AttachmentActions::new()
                    .child(AttachmentAction::new("Copy link", "⧉", move || {
                        spawn(async move {
                            if let Some(url) = app_url().await {
                                set_clipboard(url);
                                copied.set(true);
                            }
                        });
                    }))
                    .child(AttachmentAction::new("Remove research-summary.pdf", "×", move || {
                        visible.set(false);
                        copied.set(false);
                    })))
        }))
        .child(when(copied, || text("Preview link copied.").text_sm()))
        .child(when(open, move || {
            dialog()
                .label("research-summary.pdf")
                .animation(ModalAnimation::Fade)
                .on_cancel(move || open.set(false))
                .child(view().absolute().inset_0().bg(hsla(0.0, 0.0, 0.0, 0.30))
                    .backdrop_blur(px(8.0)).on_press(move || open.set(false)))
                .child(view().absolute().inset_0().items_center().justify_center().pointer_events(PointerEvents::BoxNone)
                    .child(view().w(move || px((window().width.0 - 32.0).clamp(0.0, 448.0)))
                        .gap_4().p_6().rounded(px(24.0)).bg(move || theme().popover)
                        .text_color(move || theme().popover_foreground).shadow_lg()
                        .test_id("attachment-preview-dialog")
                        .child(text("research-summary.pdf").text_lg().font_weight(FontWeight::SEMIBOLD))
                        .child(text("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").text_sm())
                        .child(view().role(AccessibilityRole::Button).label("Close preview").items_center()
                            .cursor_pointer().p_2().rounded(px(8.0)).bg(move || theme().muted)
                            .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                            .on_press(move || open.set(false)).child(text("Close")))))
        }))
}
