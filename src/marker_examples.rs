use std::time::Duration;

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};

fn stage(gap: f32) -> View {
    view().w_full().max_w(px(384.0)).gap(px(gap)).py(px(48.0))
}

fn marker(label: &'static str) -> Marker {
    Marker::new(MarkerContent::new(label))
}

fn icon(symbol: &'static str) -> MarkerIcon {
    MarkerIcon::new(text(symbol).text_sm())
}

fn spinner() -> MarkerIcon {
    MarkerIcon::new(
        view()
            .w(px(14.0))
            .h(px(14.0))
            .rounded_full()
            .border(px(1.5))
            .border_color(|| theme().muted_foreground.opacity(0.3))
            .border_t_color(|| theme().muted_foreground)
            .style_when(move || !reduce_motion(), |style| style.animate_spin()),
    )
}

fn divider(label: &'static str) -> Marker {
    marker(label).variant(MarkerVariant::Separator)
}

pub fn demo() -> View {
    stage(32.0)
        .child(marker("Switched to a new branch").icon(icon("⑂")))
        .child(
            Marker::new(MarkerContent::new("Thinking...").shimmer())
                .status()
                .icon(spinner()),
        )
        .child(divider("Conversation compacted"))
        .child(marker("Explored 4 files").icon(icon("⌕")))
}

pub fn variants() -> View {
    stage(32.0)
        .child(marker("A default marker for inline notes."))
        .child(divider("A separator marker"))
        .child(marker("A border marker for row boundaries.").variant(MarkerVariant::Border))
}

pub fn status() -> View {
    stage(32.0)
        .child(marker("Compacting conversation").status().icon(spinner()))
        .child(divider("Running tests").status().icon(spinner()))
}

pub fn shimmer() -> View {
    stage(32.0)
        .child(Marker::new(MarkerContent::new("Thinking...").shimmer()).status())
        .child(
            Marker::new(MarkerContent::new("Reading 4 files").shimmer())
                .variant(MarkerVariant::Separator)
                .status(),
        )
}

pub fn separator() -> View {
    stage(32.0)
        .child(divider("Today"))
        .child(divider("Worked for 42s"))
        .child(divider("Conversation compacted"))
}

pub fn border() -> View {
    stage(12.0)
        .child(
            marker("Switched to release-candidate")
                .variant(MarkerVariant::Border)
                .icon(icon("⑂")),
        )
        .child(
            marker("Reviewed 8 related files")
                .variant(MarkerVariant::Border)
                .icon(icon("⌕")),
        )
        .child(
            marker("Opened implementation notes")
                .variant(MarkerVariant::Border)
                .icon(icon("▤")),
        )
}

pub fn with_icon() -> View {
    stage(48.0)
        .child(marker("Switched to a new branch").icon(icon("⑂")))
        .child(divider("Explored 4 files").icon(icon("⌕")))
        .child(marker("Syncing completed").icon(icon("✓")).stacked())
}

pub fn link_button() -> View {
    let feedback = signal(false);
    let revision = signal(0_u32);
    stage(32.0)
        .child(
            marker("View the pull request")
                .icon(icon("⑂"))
                .href("https://github.com/shadcn-ui/ui/pulls"),
        )
        .child(
            marker("Revert this change")
                .icon(icon("↶"))
                .on_press(move || {
                    let current = revision.get_untracked().wrapping_add(1);
                    revision.set(current);
                    let message = "You clicked the revert button";
                    feedback.set(true);
                    announce_for_accessibility(message);
                    set_timeout(Duration::from_secs(3), move || {
                        if revision.get_untracked() == current {
                            feedback.set(false);
                        }
                    });
                }),
        )
        .child(when(feedback, || {
            view()
                .rounded_md()
                .border(px(1.0))
                .border_color(|| theme().border)
                .bg(|| theme().popover)
                .p_3()
                .child(text("You clicked the revert button").text_sm())
        }))
}
