//! Styled, single-axis viewports backed by Native Shell scrolling.

use std::time::Duration;

use nsh_base::{ScrollbarMode, ScrollbarMotion, scrollbar, theme};
use nsh_core::{
    TouchEvent,
    element::{LayoutDirection, watch_scroll},
    geometry::Axis,
    keymap::Keystroke,
    prelude::*,
    time::{Instant, now},
};

use crate::direction::{Direction, use_direction};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollAreaOrientation {
    Horizontal,
    #[default]
    Vertical,
}

impl ScrollAreaOrientation {
    fn horizontal(self) -> bool {
        self == Self::Horizontal
    }
}

pub struct ScrollBar {
    orientation: ScrollAreaOrientation,
    mode: ScrollbarMode,
}

impl ScrollBar {
    pub fn new(orientation: ScrollAreaOrientation) -> Self {
        Self {
            orientation,
            mode: ScrollbarMode::Hover,
        }
    }

    pub fn mode(mut self, mode: ScrollbarMode) -> Self {
        self.mode = mode;
        self
    }

    fn render(
        self,
        viewport: ElementRef,
        hovered: Signal<bool>,
        direction: Reactive<Direction>,
    ) -> impl IntoElement {
        let horizontal = self.orientation.horizontal();
        let mode = move || match self.mode {
            ScrollbarMode::Hover if hovered.get() => ScrollbarMode::Always,
            other => other,
        };
        let mut motion = ScrollbarMotion::default();
        motion.idle = 650.0;
        motion.enter = 100.0;
        motion.exit = 140.0;
        scrollbar()
            .scroll(viewport)
            .axis(if horizontal {
                Axis::Horizontal
            } else {
                Axis::Vertical
            })
            .motion(motion)
            .mode(mode)
            .min_thumb_length(px(24.0))
            .thumb_style(|style| style.rounded_full().bg(move || theme().border))
            .direction(LayoutDirection::Ltr)
            .test_id(if horizontal {
                "scroll-area-horizontal-bar"
            } else {
                "scroll-area-vertical-bar"
            })
            .absolute()
            .p(px(1.0))
            .when(horizontal, |bar| {
                bar.left(px(0.0)).right(px(0.0)).bottom(px(0.0)).h(px(10.0))
            })
            .when(!horizontal, |bar| {
                bar.top(px(0.0))
                    .bottom(px(0.0))
                    .w(px(10.0))
                    .style_with(move |style| {
                        if direction.get() == Direction::Rtl {
                            style.left(px(0.0))
                        } else {
                            style.right(px(0.0))
                        }
                    })
            })
    }
}

pub struct ScrollArea {
    label: Reactive<SharedString>,
    width: f32,
    height: f32,
    bar: ScrollBar,
    body: AnyElement,
    viewport: Option<ElementRef>,
}

impl ScrollArea {
    pub fn new(label: impl IntoReactive<SharedString>, body: impl IntoElement) -> Self {
        Self {
            label: label.into_reactive(),
            width: 192.0,
            height: 288.0,
            bar: ScrollBar::new(ScrollAreaOrientation::Vertical),
            body: body.into_element(),
            viewport: None,
        }
    }

    pub fn width(mut self, points: f32) -> Self {
        self.width = points;
        self
    }

    pub fn height(mut self, points: f32) -> Self {
        self.height = points;
        self
    }

    /// Sets the scrolling axis and the appearance of its indicator.
    pub fn scroll_bar(mut self, bar: ScrollBar) -> Self {
        self.bar = bar;
        self
    }

    pub fn viewport_ref(mut self, viewport: ElementRef) -> Self {
        self.viewport = Some(viewport);
        self
    }
}

impl IntoElement for ScrollArea {
    fn into_element(self) -> AnyElement {
        let direction = use_direction();
        let viewport = self.viewport.unwrap_or_else(element_ref);
        let horizontal = self.bar.orientation.horizontal();
        let hovered = signal(false);
        let metrics = stored(ScrollEvent::default());
        let pending = stored(None::<(Point<Points>, Instant)>);
        let laid_out = stored(false);
        let initial_direction = direction.clone();
        watch_scroll(
            move || viewport.node(),
            move |event| {
                metrics.replace(*event);
                if pending.get().is_some_and(|(target, _)| {
                    (target.x.0 - event.offset.x.0).abs() < 1.0
                        && (target.y.0 - event.offset.y.0).abs() < 1.0
                }) {
                    pending.replace(None);
                }
                if horizontal
                    && event.size.width.0 > 0.0
                    && event.content_size.width.0 > event.size.width.0
                    && !laid_out.replace(true)
                    && initial_direction.get() == Direction::Rtl
                {
                    viewport.scroll_to(
                        point(event.content_size.width - event.size.width, px(0.0)),
                        false,
                    );
                }
            },
        );
        let keyboard_direction = direction.clone();
        let axes = if horizontal {
            ["left", "right"]
        } else {
            ["up", "down"]
        };
        let keys = KeyCapture::keys(
            axes.into_iter()
                .chain(["home", "end", "pageup", "pagedown"])
                .filter_map(|key| Keystroke::parse(key, os()).ok()),
        );
        let screen = scroll()
            .when(horizontal, |viewport| viewport.horizontal())
            .scrollbar(false)
            .size_full()
            .role(AccessibilityRole::Group)
            .label(self.label)
            .focusable(true)
            .focus_visible(|style| {
                style
                    .ring_inset()
                    .ring(px(3.0))
                    .ring_color(theme().ring.opacity(0.5))
            })
            .element_ref(&viewport)
            .test_id("scroll-area-viewport")
            .on_blur(move || {
                pending.replace(None);
            })
            .capture_keys(keys)
            .on_key_down(move |event: &KeyEvent| {
                if !event.default_prevented {
                    return;
                }
                let event_metrics = metrics.get();
                // Repeated keys can arrive before the native scroll reports
                // the offset commanded by the previous key.
                let base = pending
                    .get()
                    .filter(|(_, issued)| {
                        now().saturating_duration_since(*issued) < Duration::from_millis(600)
                    })
                    .map_or(event_metrics.offset, |(target, _)| target);
                let (offset, size, content) = if horizontal {
                    (
                        base.x,
                        event_metrics.size.width,
                        event_metrics.content_size.width,
                    )
                } else {
                    (
                        base.y,
                        event_metrics.size.height,
                        event_metrics.content_size.height,
                    )
                };
                let max = (content - size).max(px(0.0));
                let rtl = horizontal && keyboard_direction.get() == Direction::Rtl;
                let page = size * 0.9;
                let next = match event.key.as_ref() {
                    "ArrowLeft" | "ArrowUp" => offset - px(40.0),
                    "ArrowRight" | "ArrowDown" => offset + px(40.0),
                    "PageUp" => {
                        if rtl {
                            offset + page
                        } else {
                            offset - page
                        }
                    }
                    "PageDown" => {
                        if rtl {
                            offset - page
                        } else {
                            offset + page
                        }
                    }
                    "Home" => {
                        if rtl {
                            max
                        } else {
                            px(0.0)
                        }
                    }
                    "End" => {
                        if rtl {
                            px(0.0)
                        } else {
                            max
                        }
                    }
                    _ => return,
                }
                .clamp(px(0.0), max);
                let next = if horizontal {
                    point(next, px(0.0))
                } else {
                    point(px(0.0), next)
                };
                pending.replace(Some((next, now())));
                viewport.scroll_to(next, !reduce_motion());
            })
            .child(self.body);
        view()
            .relative()
            .w(px(self.width))
            .max_w(percentage(1.0))
            .h(px(self.height))
            .min_w(px(0.0))
            .rounded(px(6.0))
            .border(px(1.0))
            .border_color(move || theme().border)
            .overflow_hidden()
            .direction(direction.clone().map(LayoutDirection::from))
            .on_wheel(move |_: &WheelEvent| {
                pending.replace(None);
            })
            .on_touch(move |_: &TouchEvent| {
                pending.replace(None);
            })
            .on_mouse_enter(move |_: &MouseEvent| hovered.set(true))
            .on_mouse_leave(move |_: &MouseEvent| hovered.set(false))
            .child(screen)
            .child(self.bar.render(viewport, hovered, direction))
            .into_element()
    }
}
