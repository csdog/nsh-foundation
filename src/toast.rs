//! Bottom-edge notifications styled for Rhea with Native Shell toast primitives.

use std::{future::Future, rc::Rc, time::Duration};

use nsh_base::{
    Anchor, SwipeDirection, ToastManager, ToastMotion, ToastOptions, ToastTransitionStatus, theme,
    toast as native_toast, toast_stack,
};
use nsh_core::{prelude::*, time::now};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    spinner::Spinner,
};

const TIMEOUT: Duration = Duration::from_secs(5);
const MOTION_MS: f32 = 500.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToastId(u64);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastKind {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

pub struct Toast {
    title: Option<SharedString>,
    description: Option<SharedString>,
    kind: ToastKind,
    timeout: Option<Duration>,
    action: Option<(SharedString, Rc<dyn Fn()>)>,
}

impl Toast {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self::empty().title(title)
    }

    pub fn message(description: impl Into<SharedString>) -> Self {
        Self::empty().description(description)
    }

    fn empty() -> Self {
        Self {
            title: None,
            description: None,
            kind: ToastKind::Default,
            timeout: Some(TIMEOUT),
            action: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }

    /// Pass None for a notification that stays open until closed.
    pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn action(mut self, label: impl Into<SharedString>, press: impl Fn() + 'static) -> Self {
        self.action = Some((label.into(), Rc::new(press)));
        self
    }

    fn label(&self) -> SharedString {
        match (&self.title, &self.description) {
            (Some(title), Some(description)) => format!("{title}. {description}").into(),
            (Some(text), None) | (None, Some(text)) => text.clone(),
            (None, None) => "Notification".into(),
        }
    }
}

#[derive(Clone)]
struct Entry {
    id: ToastId,
    toast: Rc<Toast>,
    phase: ToastTransitionStatus,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.phase == other.phase && Rc::ptr_eq(&self.toast, &other.toast)
    }
}

/// Create a Toaster in the screen owner, pass its handle to triggers, and
/// mount its viewport once. Pending work ends when the screen is disposed.
#[derive(Clone, Copy)]
pub struct Toaster {
    owner: Owner,
    manager: Stored<ToastManager<ToastId, Rc<Toast>>>,
    next: Stored<u64>,
    shown: Signal<Vec<Entry>>,
    motion: ToastMotion,
}

impl Toaster {
    pub fn new() -> Self {
        let duration = if reduce_motion() {
            Duration::ZERO
        } else {
            Duration::from_millis(MOTION_MS as u64)
        };
        let motion = ToastMotion {
            duration,
            exit_duration: duration,
            collapsed_peek: px(12.0),
            expanded_gap: px(12.0),
            collapsed_scale_step: 0.1,
            collapsed_visible: 3,
        };
        Self {
            owner: Owner::current(),
            manager: stored(ToastManager::new(motion)),
            next: stored(0),
            shown: signal(Vec::new()),
            motion,
        }
    }

    pub fn show(self, toast: Toast) -> ToastId {
        let id = ToastId(self.next.with(|next| {
            *next += 1;
            *next
        }));
        self.put(id, toast);
        id
    }

    pub fn close(self, id: ToastId) -> bool {
        let changed = self.manager.with(|manager| manager.dismiss(&id, now()));
        if changed {
            self.sync();
        }
        changed
    }

    /// Updates the same notification as its future settles.
    /// Closing the toast leaves the operation running and suppresses its result.
    pub fn promise<T, E>(
        self,
        future: impl Future<Output = Result<T, E>> + 'static,
        loading: impl Into<SharedString>,
        finish: impl FnOnce(Result<T, E>) -> Toast + 'static,
    ) -> ToastId {
        let id = self.show(Toast::new(loading).kind(ToastKind::Loading).timeout(None));
        self.owner.run(|| {
            spawn(async move {
                let result = future.await;
                let active = self.manager.with(|manager| {
                    manager.iter().any(|(entry, _, phase)| {
                        *entry == id && phase != ToastTransitionStatus::Ending
                    })
                });
                if active {
                    let toast = finish(result);
                    announce_for_accessibility(toast.label().to_string());
                    self.put(id, toast);
                }
            });
        });
        id
    }

    pub fn viewport(self) -> impl IntoElement {
        let expanded = signal(false);
        let focused = signal(false);
        set_interval(Duration::from_millis(32), move || {
            if self.shown.with_untracked(Vec::is_empty) {
                return;
            }
            let paused = expanded.get_untracked() || focused.get_untracked();
            if self
                .manager
                .with(|manager| manager.advance(now(), paused).changed)
            {
                self.sync();
            }
        });
        when(
            move || !self.shown.with(Vec::is_empty),
            move || {
                expanded.set(false);
                focused.set(false);
                let focus_scope = element_ref();
                focus_scope.on_focus_in(move || focused.set(true));
                focus_scope.on_focus_out(move || focused.set(false));

                let available = || window().width.0 - safe_area().left.0 - safe_area().right.0;
                let width = move || (available() - 32.0).clamp(0.0, 384.0);
                overlay()
                    .absolute()
                    .inset_0()
                    .pointer_events(PointerEvents::BoxNone)
                    .focusable(false)
                    .child(
                        view()
                            .absolute()
                            .right(move || {
                                let margin = if window().width.0 < 640.0 {
                                    (available() - width()) * 0.5
                                } else {
                                    16.0
                                };
                                safe_area().right + px(margin)
                            })
                            .bottom(|| safe_area().bottom + px(16.0))
                            .w(move || px(width()))
                            .role(AccessibilityRole::Region)
                            .label("Notifications")
                            .element_ref(&focus_scope)
                            .pointer_events(PointerEvents::BoxNone)
                            .child(
                                toast_stack()
                                    .motion(self.motion)
                                    .on_expanded_change(move |value: &bool| expanded.set(*value))
                                    .placement(Anchor::BottomRight)
                                    .on_cancel(move || {
                                        if let Some(id) = self.shown.with_untracked(|shown| {
                                            shown
                                                .iter()
                                                .rev()
                                                .find(|entry| !ending(entry.phase))
                                                .map(|entry| entry.id)
                                        }) {
                                            self.close(id);
                                        }
                                    })
                                    .child(each(
                                        self.shown,
                                        |entry| entry.id.0,
                                        move |entry, _| render(entry, self),
                                    )),
                            ),
                    )
            },
        )
    }

    fn put(self, id: ToastId, toast: Toast) {
        self.manager.with(|manager| {
            let options = ToastOptions::timeout(toast.timeout);
            manager.push(id, Rc::new(toast), options, now());
        });
        self.sync();
    }

    fn sync(self) {
        self.shown.set(self.manager.with(|manager| {
            manager
                .visible(self.motion.collapsed_visible)
                .map(|(&id, toast, phase)| Entry {
                    id,
                    toast: toast.clone(),
                    phase,
                })
                .collect()
        }));
    }
}

impl Default for Toaster {
    fn default() -> Self {
        Self::new()
    }
}

fn ending(phase: ToastTransitionStatus) -> bool {
    phase == ToastTransitionStatus::Ending
}

fn render(entry: ReadSignal<Entry>, toaster: Toaster) -> impl IntoElement {
    let id = entry.with(|entry| entry.id);
    let offset = animated_value(80.0);
    let opacity = animated_value(0.0);
    effect(move || {
        let closed = entry.with(|entry| ending(entry.phase));
        let y = if closed { 100.0 } else { 0.0 };
        let alpha = if closed { 0.0 } else { 1.0 };
        if reduce_motion() {
            offset.set(0.0);
            opacity.set(alpha);
        } else {
            let easing = Easing::Bezier(0.22, 1.0, 0.36, 1.0);
            let _ = offset.timing(y, MOTION_MS, easing);
            let _ = opacity.timing(alpha, MOTION_MS, easing);
        }
    });
    native_toast()
        .transition_status(move || entry.with(|entry| entry.phase))
        .accessibility_label(move || Some(entry.with(|entry| entry.toast.label())))
        .swipe_direction(Some(SwipeDirection::Down))
        .on_dismiss_request(move || {
            toaster.close(id);
        })
        .child(
            view()
                .test_id("toast-card")
                .bind_translate_y(&offset)
                .bind_opacity(&opacity)
                .pointer_events(move || {
                    if entry.with(|entry| ending(entry.phase)) {
                        PointerEvents::None
                    } else {
                        PointerEvents::Auto
                    }
                })
                .flex_row()
                .items_center()
                .gap_3()
                .p_4()
                .rounded(px(16.0))
                .border(px(1.0))
                .border_color(|| theme().border)
                .bg(|| theme().popover)
                .text_color(|| theme().popover_foreground)
                .shadow_lg()
                .child(dynamic(move || {
                    status_icon(entry.with(|entry| entry.toast.kind))
                }))
                .child(
                    view()
                        .min_w(px(0.0))
                        .flex_1()
                        .gap_1()
                        .child(when(
                            move || entry.with(|entry| entry.toast.title.is_some()),
                            move || {
                                text(move || {
                                    entry
                                        .with(|entry| entry.toast.title.clone().unwrap_or_default())
                                })
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                            },
                        ))
                        .child(when(
                            move || entry.with(|entry| entry.toast.description.is_some()),
                            move || {
                                text(move || {
                                    entry.with(|entry| {
                                        entry.toast.description.clone().unwrap_or_default()
                                    })
                                })
                                .text_sm()
                                .text_color(|| theme().muted_foreground)
                            },
                        )),
                )
                .child(when(
                    move || entry.with(|entry| entry.toast.action.is_some()),
                    move || {
                        Button::new(move || {
                            entry.with(|entry| {
                                entry
                                    .toast
                                    .action
                                    .as_ref()
                                    .map(|(label, _)| label.clone())
                                    .unwrap_or_default()
                            })
                        })
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Small)
                        .on_press(move || {
                            let press = entry.with(|entry| {
                                entry.toast.action.as_ref().map(|(_, press)| press.clone())
                            });
                            if toaster.close(id)
                                && let Some(press) = press
                            {
                                press();
                            }
                        })
                    },
                ))
                .child(
                    Button::new("Close toast")
                        .icon_start(text("×").text_lg())
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::IconSmall)
                        .on_press(move || {
                            toaster.close(id);
                        }),
                ),
        )
}

fn status_icon(kind: ToastKind) -> Option<AnyElement> {
    let glyph = match kind {
        ToastKind::Default => return None,
        ToastKind::Loading => return Some(Spinner::new().decorative().into_element()),
        ToastKind::Success => "✓",
        ToastKind::Info => "i",
        ToastKind::Warning => "!",
        ToastKind::Error => "×",
    };
    Some(
        view()
            .size(px(16.0))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .border(px(1.0))
            .border_color(move || {
                if kind == ToastKind::Error {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            })
            .text_color(move || {
                if kind == ToastKind::Error {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            })
            .rounded_full()
            .accessibility_elements_hidden(true)
            .test_id("toast-kind")
            .child(text(glyph).text_xs().font_weight(FontWeight::SEMIBOLD))
            .into_element(),
    )
}
