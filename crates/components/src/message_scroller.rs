//! A keyed conversation viewport with turn anchoring and reader-controlled following.

use std::collections::HashMap;

use nsh_base::theme;
use nsh_core::{keyboard::KeyCapture, prelude::*};

const EDGE: f32 = 2.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OpeningPosition {
    Start,
    End,
    #[default]
    LastAnchor,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageScrollAlign {
    #[default]
    Start,
    Center,
    End,
    Nearest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MessageScrollerItem<T> {
    pub id: SharedString,
    pub scroll_anchor: bool,
    pub value: T,
}

impl<T> MessageScrollerItem<T> {
    pub fn new(id: impl Into<SharedString>, value: T) -> Self {
        Self {
            id: id.into(),
            scroll_anchor: false,
            value,
        }
    }

    pub fn anchor(mut self, anchor: bool) -> Self {
        self.scroll_anchor = anchor;
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MessageScrollState {
    pub can_scroll_start: bool,
    pub can_scroll_end: bool,
    pub current_anchor: Option<SharedString>,
    pub visible_messages: Vec<SharedString>,
}

#[derive(Clone, Copy)]
pub struct MessageScrollerHandle {
    viewport: ElementRef,
    model: Stored<ScrollModel>,
    commands: Signal<u64>,
    state: Signal<MessageScrollState>,
    padding: Signal<f32>,
    pending: Signal<bool>,
}

impl Default for MessageScrollerHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageScrollerHandle {
    pub fn new() -> Self {
        Self {
            viewport: element_ref(),
            model: stored(ScrollModel::default()),
            commands: signal(0),
            state: signal(MessageScrollState::default()),
            padding: signal(0.0),
            pending: signal(false),
        }
    }

    pub fn state(self) -> ReadSignal<MessageScrollState> {
        self.state.read_only()
    }

    pub fn scroll_to_start(self, animated: bool) {
        self.command(Command::Start(animated));
    }

    pub fn scroll_to_end(self, animated: bool) {
        self.command(Command::End(animated));
    }

    /// Queues a target during opening, or jumps to a known message.
    pub fn scroll_to_message(
        self,
        id: impl Into<SharedString>,
        align: MessageScrollAlign,
        animated: bool,
    ) -> bool {
        let id = id.into();
        if self
            .model
            .with(|model| model.opened && !model.ids.iter().any(|known| known == &id))
        {
            return false;
        }
        self.command(Command::Message(id, align, animated));
        true
    }

    fn command(self, command: Command) {
        self.model.with(|model| {
            model.command = Some(command);
        });
        self.commands
            .update(|revision| *revision = revision.wrapping_add(1));
    }

    fn observe_scroll(self, event: &ScrollEvent, auto_scroll: bool) {
        let state = self.model.with(|model| {
            // The platform may move its offset during a prepend. Preserve
            // the offset from the last committed transcript until layout settles.
            if !model.opened
                || (event.content_size.height.0.max(event.size.height.0)
                    - model.geometry.total_height().max(model.geometry.viewport))
                .abs()
                    > EDGE
            {
                return None;
            }
            model.offset = event.offset.y.0.max(0.0);
            if let Some((target, animated)) = model.in_flight {
                if (model.offset - target).abs() <= EDGE {
                    model.in_flight = None;
                    return Some(model.state());
                }
                if animated {
                    return Some(model.state());
                }
                model.in_flight = None;
            }
            if let Some(target) = model.hold_target() {
                if (model.offset - target).abs() <= EDGE {
                    return Some(model.state());
                }
                model.anchor = None;
            }
            model.follow = auto_scroll && model.geometry.bottom() - model.offset <= EDGE;
            Some(model.state())
        });
        if let Some(state) = state {
            self.state.set(state);
        }
    }

    fn reconcile(self, geometry: Geometry, options: Options) {
        let (movement, state, padding, ready) = self.model.with(|model| {
            let movement = model.reconcile(geometry, options);
            (
                movement,
                model.state(),
                model.geometry.padding,
                model.opened,
            )
        });
        self.padding.set(padding);
        self.state.set(state);
        if let Some((target, animated)) = movement {
            self.viewport
                .scroll_to(point(px(0.0), px(target)), animated);
        }
        if ready {
            self.pending.set(false);
        }
    }
}

pub struct MessageScroller<T: 'static> {
    handle: MessageScrollerHandle,
    items: Reactive<Vec<MessageScrollerItem<T>>>,
    initial: OpeningPosition,
    auto_scroll: bool,
    preserve_history: bool,
    peek: Reactive<f32>,
    height: f32,
}

impl<T: Clone + PartialEq + 'static> MessageScroller<T> {
    pub fn new(
        handle: MessageScrollerHandle,
        items: impl IntoReactive<Vec<MessageScrollerItem<T>>>,
    ) -> Self {
        Self {
            handle,
            items: items.into_reactive(),
            initial: OpeningPosition::LastAnchor,
            auto_scroll: false,
            preserve_history: true,
            peek: 32.0.into_reactive(),
            height: 340.0,
        }
    }

    pub fn default_scroll_position(mut self, initial: OpeningPosition) -> Self {
        self.initial = initial;
        self
    }

    pub fn auto_scroll(mut self, enabled: bool) -> Self {
        self.auto_scroll = enabled;
        self
    }

    pub fn preserve_scroll_on_prepend(mut self, enabled: bool) -> Self {
        self.preserve_history = enabled;
        self
    }

    pub fn previous_item_peek(mut self, peek: impl IntoReactive<f32>) -> Self {
        self.peek = peek.into_reactive();
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(120.0);
        self
    }

    pub fn render(
        self,
        render_row: impl Fn(ReadSignal<MessageScrollerItem<T>>) -> AnyElement + 'static,
    ) -> View {
        let handle = self.handle;
        let initial = self.initial;
        let auto_scroll = self.auto_scroll;
        let preserve_history = self.preserve_history;
        let peek = self.peek;
        let items = self.items;
        let frames = stored(HashMap::<SharedString, (f32, f32)>::new());
        let revision = signal(0_u64);
        let viewport = signal(0.0_f32);
        let body = signal(0.0_f32);
        handle.pending.set(initial != OpeningPosition::Start);

        let observed_items = items.clone();
        effect(move || {
            let items = observed_items.get();
            let viewport = viewport.get();
            let body = body.get();
            let peek = peek.get().clamp(0.0, (viewport - 1.0).max(0.0));
            revision.get();
            handle.commands.get();
            let ids = items.iter().map(|item| item.id.clone()).collect::<Vec<_>>();
            handle.model.with(|model| {
                model.ids = ids;
            });
            if viewport <= 0.0 || (items.is_empty() && !handle.model.with(|model| model.opened)) {
                return;
            }
            let geometry = frames.with(|measured| {
                let rows = items
                    .iter()
                    .map(|item| {
                        measured.get(&item.id).map(|&(top, height)| Row {
                            id: item.id.clone(),
                            anchor: item.scroll_anchor,
                            top,
                            height,
                        })
                    })
                    .collect::<Option<Vec<_>>>()?;
                Some(Geometry {
                    rows,
                    viewport,
                    body,
                    padding: 0.0,
                    peek,
                })
            });
            let Some(geometry) = geometry else { return };
            let options = Options {
                initial,
                auto_scroll,
                preserve_history,
                animate: !reduce_motion(),
            };
            let frame = request_animation_frame(move |_| handle.reconcile(geometry, options));
            cleanup(move || frame.cancel());
        });

        let position = handle.state();
        let capture = KeyCapture::keys(
            ["up", "down", "pageup", "pagedown", "home", "end"]
                .into_iter()
                .filter_map(|key| nsh_core::keymap::Keystroke::parse(key, os()).ok()),
        );
        view()
            .relative()
            .w_full()
            .min_w(px(0.0))
            .child(
                scroll()
                    .test_id("message-scroller-viewport")
                    .label("Conversation transcript")
                    .role(AccessibilityRole::Region)
                    .focusable(true)
                    .focus_visible(|style| {
                        style.ring_inset().ring(px(2.0)).ring_color(theme().ring)
                    })
                    .h(px(self.height))
                    .w_full()
                    .min_w(px(0.0))
                    .element_ref(&handle.viewport)
                    .capture_keys(capture)
                    .style_when(handle.pending, |style| style.opacity(0.0))
                    .on_layout(move |bounds: &Bounds<Points>| viewport.set(bounds.size.height.0))
                    .on_scroll(move |event: &ScrollEvent| handle.observe_scroll(event, auto_scroll))
                    .on_scroll_drag_start(move |_: &Point<Points>| {
                        handle.model.with(|model| model.in_flight = None)
                    })
                    .on_scroll_end(move |_: &Point<Points>| {
                        handle.model.with(|model| model.in_flight = None)
                    })
                    .on_wheel(move |_: &WheelEvent| {
                        handle.model.with(|model| model.in_flight = None)
                    })
                    .on_key_down(move |key: &KeyEvent| {
                        if !key.default_prevented {
                            return;
                        }
                        let (offset, height, end) = handle.model.with(|model| {
                            (
                                model.offset,
                                model.geometry.viewport,
                                model.geometry.scroll_end(),
                            )
                        });
                        let target = match key.key.as_ref() {
                            "ArrowUp" => offset - 40.0,
                            "ArrowDown" => offset + 40.0,
                            "PageUp" => offset - height * 0.9,
                            "PageDown" => offset + height * 0.9,
                            "Home" => 0.0,
                            "End" => end,
                            _ => return,
                        }
                        .clamp(0.0, end);
                        handle.model.with(|model| model.in_flight = None);
                        handle.viewport.scroll_to(point(px(0.0), px(target)), false);
                    })
                    .child(
                        view()
                            .min_w(px(0.0))
                            .child(
                                view()
                                    .p_4()
                                    .gap_3()
                                    .min_w(px(0.0))
                                    .on_layout(move |bounds: &Bounds<Points>| {
                                        body.set(bounds.size.height.0)
                                    })
                                    .child(each(
                                        items,
                                        |item| item.id.clone(),
                                        move |item, _| {
                                            let id = item.get_untracked().id;
                                            let measured_id = id.clone();
                                            cleanup(move || {
                                                frames.try_with(|frames| {
                                                    frames.remove(&id);
                                                });
                                            });
                                            view()
                                                .min_w(px(0.0))
                                                .on_layout(move |bounds: &Bounds<Points>| {
                                                    let frame =
                                                        (bounds.origin.y.0, bounds.size.height.0);
                                                    let changed = frames.with(|frames| {
                                                        frames.insert(measured_id.clone(), frame)
                                                            != Some(frame)
                                                    });
                                                    if changed {
                                                        revision.update(|revision| {
                                                            *revision = revision.wrapping_add(1)
                                                        });
                                                    }
                                                })
                                                .child(render_row(item))
                                        },
                                    )),
                            )
                            .child(
                                view()
                                    .h(move || px(handle.padding.get()))
                                    .accessibility_elements_hidden(true),
                            ),
                    ),
            )
            .child(when(
                move || position.with(|state| state.can_scroll_end),
                move || {
                    crate::button::Button::new("Jump to latest")
                        .size(crate::button::ButtonSize::Icon)
                        .variant(crate::button::ButtonVariant::Outline)
                        .icon_start(text("↓").accessibility_elements_hidden(true))
                        .on_press(move || handle.scroll_to_end(true))
                        .absolute()
                        .bottom(px(12.0))
                        .right(px(16.0))
                        .bg(|| theme().background)
                },
            ))
    }
}

enum Command {
    Start(bool),
    End(bool),
    Message(SharedString, MessageScrollAlign, bool),
}

struct Options {
    initial: OpeningPosition,
    auto_scroll: bool,
    preserve_history: bool,
    animate: bool,
}

#[derive(Clone)]
struct Row {
    id: SharedString,
    anchor: bool,
    top: f32,
    height: f32,
}

impl Row {
    fn bottom(&self) -> f32 {
        self.top + self.height
    }
}

#[derive(Default)]
struct Geometry {
    rows: Vec<Row>,
    viewport: f32,
    body: f32,
    padding: f32,
    peek: f32,
}

impl Geometry {
    fn total_height(&self) -> f32 {
        self.body + self.padding
    }

    fn bottom(&self) -> f32 {
        (self.body - self.viewport).max(0.0)
    }

    fn scroll_end(&self) -> f32 {
        (self.total_height() - self.viewport).max(0.0)
    }

    fn row(&self, id: &SharedString) -> Option<&Row> {
        self.rows.iter().find(|row| &row.id == id)
    }
}

#[derive(Default)]
struct ScrollModel {
    ids: Vec<SharedString>,
    opened: bool,
    offset: f32,
    geometry: Geometry,
    anchor: Option<SharedString>,
    follow: bool,
    command: Option<Command>,
    in_flight: Option<(f32, bool)>,
}

impl ScrollModel {
    fn hold_target(&self) -> Option<f32> {
        let anchor = self.geometry.row(self.anchor.as_ref()?)?;
        Some((anchor.top - self.geometry.peek).max(0.0))
    }

    fn state(&self) -> MessageScrollState {
        let visible_messages = self
            .geometry
            .rows
            .iter()
            .filter(|row| {
                row.bottom() > self.offset && row.top < self.offset + self.geometry.viewport
            })
            .map(|row| row.id.clone())
            .collect();
        let current_anchor = self
            .geometry
            .rows
            .iter()
            .rev()
            .find(|row| row.anchor && row.top <= self.offset + self.geometry.peek + EDGE)
            .map(|row| row.id.clone());
        MessageScrollState {
            can_scroll_start: self.offset > EDGE,
            can_scroll_end: self.geometry.scroll_end() - self.offset > EDGE,
            current_anchor,
            visible_messages,
        }
    }

    fn reconcile(&mut self, geometry: Geometry, options: Options) -> Option<(f32, bool)> {
        let previous = std::mem::replace(&mut self.geometry, geometry);
        let was_open = std::mem::replace(&mut self.opened, true);
        let mut target = None;
        let mut animate = false;
        if !was_open {
            let end = self.geometry.bottom();
            target = Some(match options.initial {
                OpeningPosition::Start => 0.0,
                OpeningPosition::End => end,
                OpeningPosition::LastAnchor => self
                    .geometry
                    .rows
                    .iter()
                    .rev()
                    .find(|row| row.anchor)
                    .map_or(end, |row| {
                        if self.geometry.body - row.top <= self.geometry.viewport {
                            end
                        } else {
                            (row.top - self.geometry.peek).clamp(0.0, end)
                        }
                    }),
            });
            self.follow = options.auto_scroll && target == Some(end);
        }
        if let Some(command) = self.command.take() {
            match command {
                Command::Start(animated) => {
                    self.anchor = None;
                    self.follow = false;
                    target = Some(0.0);
                    animate = animated;
                }
                Command::End(animated) => {
                    self.anchor = None;
                    self.follow = options.auto_scroll;
                    target = Some(self.geometry.bottom());
                    animate = animated;
                }
                Command::Message(id, align, animated) => {
                    if let Some(row) = self.geometry.row(&id) {
                        let viewport = self.geometry.viewport;
                        target = Some(match align {
                            MessageScrollAlign::Start => row.top,
                            MessageScrollAlign::Center => row.top + (row.height - viewport) / 2.0,
                            MessageScrollAlign::End => row.bottom() - viewport,
                            MessageScrollAlign::Nearest if row.top < self.offset => row.top,
                            MessageScrollAlign::Nearest
                                if row.bottom() > self.offset + viewport =>
                            {
                                row.bottom() - viewport
                            }
                            MessageScrollAlign::Nearest => self.offset,
                        });
                        self.anchor = None;
                        self.follow = false;
                        animate = animated;
                    }
                }
            }
        } else if was_open {
            if let Some(last_index) = previous
                .rows
                .last()
                .and_then(|last| self.geometry.rows.iter().position(|row| row.id == last.id))
            {
                let incoming = &self.geometry.rows[last_index + 1..];
                if (self.offset >= previous.bottom() - EDGE || self.anchor.is_some())
                    && let Some(anchor) = incoming.iter().rev().find(|row| row.anchor)
                {
                    self.anchor = Some(anchor.id.clone());
                    self.follow = false;
                }
            }

            let prepended = previous.rows.first().is_some_and(|first| {
                self.geometry
                    .rows
                    .first()
                    .is_some_and(|new| new.id != first.id)
                    && self.geometry.row(&first.id).is_some()
            });
            if prepended && options.preserve_history {
                target = previous
                    .rows
                    .iter()
                    .filter(|row| {
                        row.bottom() > self.offset && row.top < self.offset + previous.viewport
                    })
                    .find_map(|row| {
                        self.geometry
                            .row(&row.id)
                            .map(|new| self.offset + new.top - row.top)
                    });
            }

            if let Some(anchor) = self.hold_target() {
                if self.geometry.bottom() > anchor + EDGE && options.auto_scroll {
                    self.anchor = None;
                    self.follow = true;
                    target = Some(self.geometry.bottom());
                } else {
                    self.geometry.padding =
                        (anchor + self.geometry.viewport - self.geometry.body).max(0.0);
                    target = Some(anchor);
                }
            } else if target.is_none() && self.follow {
                target = Some(self.geometry.bottom());
            }
        }

        let Some(target) = target.map(|target| target.clamp(0.0, self.geometry.scroll_end()))
        else {
            self.offset = self.offset.min(self.geometry.scroll_end());
            return None;
        };
        // An entrance or relayout can repeat a request before scroll events
        // arrive. Avoid restarting an animation with the same destination.
        if self
            .in_flight
            .is_some_and(|(pending, _)| (target - pending).abs() <= EDGE)
        {
            return None;
        }
        // A native scroll view can keep its old offset when its content
        // is replaced, even though a new controller starts at zero.
        if was_open && (target - self.offset).abs() < 0.25 {
            return None;
        }
        let animated = animate && options.animate && was_open;
        self.in_flight = Some((target, animated));
        if !animated {
            self.offset = target;
        }
        Some((target, animated))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geometry(specs: &[(&str, bool, f32)]) -> Geometry {
        let mut body = 0.0;
        let rows = specs
            .iter()
            .map(|&(id, anchor, height)| {
                let row = Row {
                    id: id.to_owned().into(),
                    anchor,
                    top: body,
                    height,
                };
                body += height;
                row
            })
            .collect();
        Geometry {
            rows,
            body,
            viewport: 200.0,
            padding: 0.0,
            peek: 32.0,
        }
    }

    fn options(initial: OpeningPosition) -> Options {
        Options {
            initial,
            auto_scroll: true,
            preserve_history: true,
            animate: false,
        }
    }

    #[test]
    fn opening_at_start_applies_zero_to_a_reused_viewport() {
        let mut model = ScrollModel::default();
        let rows = [("a", true, 300.0)];
        assert_eq!(
            model.reconcile(geometry(&rows), options(OpeningPosition::Start)),
            Some((0.0, false))
        );
    }

    #[test]
    fn prepend_preserves_the_visible_row_and_its_pixel_offset() {
        let mut model = ScrollModel::default();
        let opt = || options(OpeningPosition::Start);
        let original = [("a", true, 170.0), ("b", false, 155.0), ("c", true, 250.0)];
        model.reconcile(geometry(&original), opt());
        model.offset = 201.0;
        let mut prepended = vec![("history", true, 116.0)];
        prepended.extend(original);
        assert_eq!(
            model.reconcile(geometry(&prepended), opt()),
            Some((317.0, false))
        );
        assert_eq!(model.state().visible_messages[0].as_ref(), "b");
    }

    #[test]
    fn a_new_turn_keeps_its_peek_until_the_reply_fills_the_viewport() {
        let mut model = ScrollModel::default();
        let opt = || options(OpeningPosition::End);
        let mut rows = vec![("prior", true, 500.0)];
        model.reconcile(geometry(&rows), opt());
        assert_eq!(model.offset, 300.0);
        rows.extend([("new", true, 40.0), ("reply", false, 20.0)]);
        model.reconcile(geometry(&rows), opt());
        assert_eq!(model.offset, 468.0);
        assert_eq!(model.geometry.padding, 108.0);
        rows[2].2 = 300.0;
        model.reconcile(geometry(&rows), opt());
        assert_eq!(model.offset, 640.0);
        assert!(model.follow);
        assert_eq!(model.geometry.padding, 0.0);
    }

    #[test]
    fn explicit_jumps_pause_following_and_scroll_to_end_resumes_it() {
        let mut model = ScrollModel::default();
        let opt = || options(OpeningPosition::End);
        let mut rows = [("a", true, 250.0), ("b", true, 250.0)];
        model.reconcile(geometry(&rows), opt());
        model.command = Some(Command::Message(
            "a".into(),
            MessageScrollAlign::Start,
            false,
        ));
        model.reconcile(geometry(&rows), opt());
        rows[1].2 += 100.0;
        model.reconcile(geometry(&rows), opt());
        assert_eq!(model.offset, 0.0);
        model.command = Some(Command::End(false));
        model.reconcile(geometry(&rows), opt());
        rows[1].2 += 100.0;
        model.reconcile(geometry(&rows), opt());
        assert_eq!(model.offset, 500.0);
    }
}
