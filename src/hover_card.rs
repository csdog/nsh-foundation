//! Link previews using Native Shell's hover timing and anchored overlays.

use nsh_base::{HoverCardEvent, hover_card};
use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;

use crate::{
    button::{Button, ButtonVariant},
    direction::{Direction, use_direction},
};

const POPUP: &str = "p-4 rounded-3xl bg-popover text-popover-foreground ring-1 ring-foreground/5 dark:ring-foreground/10 shadow-lg transition duration-100";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

impl HoverCardSide {
    fn placement(self, direction: Direction) -> Placement {
        match (self, direction) {
            (Self::Top, _) => Placement::Top,
            (Self::Bottom, _) => Placement::Bottom,
            (Self::Left, _)
            | (Self::InlineStart, Direction::Ltr)
            | (Self::InlineEnd, Direction::Rtl) => Placement::Left,
            (Self::Right, _)
            | (Self::InlineEnd, Direction::Ltr)
            | (Self::InlineStart, Direction::Rtl) => Placement::Right,
        }
    }
}

pub struct HoverCardTrigger {
    surface: View,
    open_delay: f64,
    close_delay: f64,
}

impl HoverCardTrigger {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self::render(Button::new(label).variant(ButtonVariant::Link).surface())
    }

    /// Uses a styled button or link and preserves its own press handler.
    pub fn render(surface: View) -> Self {
        Self {
            surface,
            open_delay: 600.0,
            close_delay: 300.0,
        }
    }

    pub fn delay(mut self, milliseconds: f64) -> Self {
        self.open_delay = milliseconds;
        self
    }

    pub fn close_delay(mut self, milliseconds: f64) -> Self {
        self.close_delay = milliseconds;
        self
    }

    pub fn test_id(mut self, id: &'static str) -> Self {
        self.surface = self.surface.test_id(id);
        self
    }
}

pub struct HoverCardContent {
    side: HoverCardSide,
    align: Align,
    side_offset: f32,
    align_offset: f32,
    width: u16,
    children: Vec<AnyElement>,
}

impl HoverCardContent {
    pub fn new() -> Self {
        Self {
            side: HoverCardSide::Bottom,
            align: Align::Center,
            side_offset: 4.0,
            align_offset: 4.0,
            width: 288,
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between the trigger and card in points.
    pub fn side_offset(mut self, points: f32) -> Self {
        self.side_offset = points;
        self
    }

    /// Shifts the card along the edge of the trigger, in points.
    pub fn align_offset(mut self, points: f32) -> Self {
        self.align_offset = points;
        self
    }

    pub fn width(mut self, points: u16) -> Self {
        self.width = points;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }
}

impl Default for HoverCardContent {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HoverCard {
    trigger: HoverCardTrigger,
    content: HoverCardContent,
    open: Option<Signal<bool>>,
    default_open: bool,
}

impl HoverCard {
    pub fn new(trigger: HoverCardTrigger, content: HoverCardContent) -> Self {
        Self {
            trigger,
            content,
            open: None,
            default_open: false,
        }
    }

    /// Controls visibility with a Rust signal that also receives hover and dismissal changes.
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

impl IntoElement for HoverCard {
    fn into_element(self) -> AnyElement {
        let HoverCardContent {
            side,
            align,
            side_offset,
            align_offset,
            width,
            children,
        } = self.content;
        let open = self.open;
        let direction = use_direction();
        let side_direction = direction.clone();
        let motion_direction = direction.clone();
        let margin = 8.0 + align_offset.abs();
        let position = signal(None::<(Placement, Points)>);
        let horizontal = side.placement(Direction::Ltr).is_horizontal();
        let cross_direction = direction.clone();
        let cross_offset = move || {
            px(if !horizontal && cross_direction.get() == Direction::Rtl {
                -align_offset
            } else {
                align_offset
            })
        };
        let motion = move || {
            let actual = position
                .get()
                .map(|(side, _)| side)
                .unwrap_or_else(|| side.placement(motion_direction.get()));
            let slide = match actual {
                Placement::Top => "translate-y-2",
                Placement::Bottom => "-translate-y-2",
                Placement::Left => "translate-x-2",
                Placement::Right => "-translate-x-2",
            };
            SharedString::from(format!("opacity-0 scale-95 {slide}"))
        };
        hover_card()
            .open(move || open.map(|open| open.get()))
            .default_open(self.default_open)
            .on_open_change(move |event: &HoverCardEvent| {
                if let (Some(open), HoverCardEvent::OpenChange(next, _)) = (open, event) {
                    open.set(*next);
                }
            })
            .on_position(move |event: &HoverCardEvent| {
                if let HoverCardEvent::Position(side, _, room) = event {
                    position.set(Some((*side, room.width)));
                }
            })
            .on_close(move || position.set(None))
            .open_delay(self.trigger.open_delay)
            .close_delay(self.trigger.close_delay)
            .placement(move || side.placement(side_direction.get()))
            .align(align)
            .offset(px(side_offset))
            .margin(px(margin))
            .popup_class(POPUP)
            .popup_style(move |style| {
                let style =
                    style
                        .direction(direction.map(LayoutDirection::from))
                        .w(move || {
                            px(f32::from(width).min((window().width.0 - margin * 2.0).max(0.0)))
                        });
                if horizontal {
                    style
                        .max_w(move || {
                            position
                                .get()
                                .map_or(px(f32::from(width)), |(_, width)| width)
                        })
                        .translate_y(cross_offset)
                } else {
                    style.translate_x(cross_offset)
                }
            })
            .popup_starting_class(motion.clone())
            .popup_ending_class(motion)
            .trigger(self.trigger.surface)
            .child(view().test_id("hover-card-panel").children(children))
            .into_element()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use nsh_core::{
        Size,
        element::Root,
        platform::{Idiom, Os},
        time::Clock,
    };
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;

    #[component]
    fn Preview() -> impl IntoElement {
        let open = signal(false);
        view()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                HoverCard::new(
                    HoverCardTrigger::new("Preview")
                        .delay(140.0)
                        .close_delay(100.0)
                        .test_id("trigger"),
                    HoverCardContent::new().child(text("Preview contents")),
                )
                .open(open),
            )
            .child(Button::new("Outside").on_press(|| {}))
            .child(text(move || if open.get() { "Opened" } else { "Closed" }))
    }

    fn start(idiom: Idiom, width: f32) -> Headless {
        let mut config = EngineConfig::new(Root::of::<Preview>());
        config.viewport = Size::new(px(width), px(600.0));
        config.environment.os = Os::Web;
        config.environment.idiom = idiom;
        config.environment.reduce_motion = true;
        config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();
        app
    }

    #[test]
    fn desktop_hover_opens_after_the_delay_and_card_hover_cancels_close() {
        let mut app = start(Idiom::Desktop, 1000.0);
        app.hover("id:trigger").unwrap();
        app.wait(Duration::from_millis(45)).unwrap();
        assert!(!app.shows("Preview contents"));
        app.wait_for("Preview contents").unwrap();
        assert!(app.shows("Opened"));
        app.hover("id:hover-card-panel").unwrap();
        app.wait(Duration::from_millis(170)).unwrap();
        assert!(app.shows("Preview contents"));
        app.hover("id:trigger").unwrap();
        app.press_key("escape").unwrap();
        app.wait_for_gone("id:hover-card-panel", Duration::from_secs(1))
            .unwrap();
        assert!(app.shows("Closed"));
    }

    #[test]
    fn touch_trigger_can_toggle_the_controlled_card() {
        let mut app = start(Idiom::Phone, 390.0);
        app.press("id:trigger").unwrap();
        app.wait_for("Preview contents").unwrap();
        app.press("id:trigger").unwrap();
        app.wait_for_gone("id:hover-card-panel", Duration::from_secs(1))
            .unwrap();
        assert!(app.shows("Closed"));
    }
}
