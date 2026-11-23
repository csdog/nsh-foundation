//! Rhea tooltips attached to Native Shell controls.

use std::time::Duration;

use nsh_base::theme;
use nsh_core::{
    TransformKind,
    element::{__hint_tooltip, ElNode, Overlay},
    prelude::*,
};

use crate::direction::{Direction, use_direction};

const MOTION_MS: f32 = 120.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
    InlineStart,
    InlineEnd,
}

impl TooltipSide {
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

pub struct TooltipContent {
    content: AnyElement,
    side: TooltipSide,
    align: Align,
    delay: Duration,
    offset: f32,
}

impl TooltipContent {
    pub fn new(message: impl IntoReactive<SharedString>) -> Self {
        Self::render(text(message).text_xs())
    }

    pub fn render(content: impl IntoElement) -> Self {
        Self {
            content: content.into_element(),
            side: TooltipSide::Top,
            align: Align::Center,
            delay: Duration::ZERO,
            offset: 4.0,
        }
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn side_offset(mut self, points: f32) -> Self {
        self.offset = points.max(0.0);
        self
    }

    /// Attach to a focusable control. For a disabled button attach to a
    /// focusable wrapper so the explanation remains reachable.
    pub fn attach<E: Interactive + Accessible>(self, trigger: E) -> E {
        let description = element_ref();
        let visible = signal(false);
        trigger
            .accessibility_described_by(move || visible.get().then_some(description))
            .tooltip(Hint(self.overlay(description, visible)))
    }

    fn overlay(self, description: ElementRef, visible: Signal<bool>) -> Overlay {
        let direction = use_direction();
        let preferred = direction.map(move |direction| self.side.placement(direction));
        let actual = signal(preferred.get());
        let mounted = signal(false);
        let transition = signal(0u64);
        let opacity = animated_value(0.0);
        let scale = animated_value(0.95);
        let square = px(8.0);

        overlay()
            .hint(self.delay)
            .hint_open(move || Some(mounted.get()))
            .placement(preferred)
            .align(self.align)
            .offset(px(self.offset))
            .margin(px(8.0))
            .pointer_events(PointerEvents::None)
            .on_hint_show(move |_: &HintReason| {
                transition.update(|id| *id += 1);
                visible.set(true);
                mounted.set(true);
                if reduce_motion() {
                    opacity.set(1.0);
                    scale.set(1.0);
                } else {
                    let _ = opacity.timing(1.0, MOTION_MS, Easing::EaseOut);
                    let _ = scale.timing(1.0, MOTION_MS, Easing::EaseOut);
                }
            })
            .on_hint_hide(move |_: &HintReason| {
                transition.update(|id| *id += 1);
                let id = transition.get_untracked();
                visible.set(false);
                if reduce_motion() {
                    mounted.set(false);
                    opacity.set(0.0);
                } else {
                    let _ = opacity.timing(0.0, MOTION_MS, Easing::EaseIn);
                    let _ = scale.timing(0.95, MOTION_MS, Easing::EaseIn);
                    set_timeout(Duration::from_millis(MOTION_MS as u64), move || {
                        if transition.try_get() == Some(id) {
                            mounted.try_set(false);
                        }
                    });
                }
            })
            .on_position(move |position: &OverlayPosition| actual.set(position.placement))
            .child(
                view()
                    .relative()
                    .element_ref(&description)
                    .max_w(|| px((window().width.0 - 16.0).clamp(0.0, 320.0)))
                    .px_3()
                    .py(px(6.0))
                    .rounded_xl()
                    .bg(|| theme().foreground)
                    .text_color(|| theme().background)
                    .bind_opacity(&opacity)
                    .bind_transform(TransformKind::ScaleX, &scale)
                    .bind_transform(TransformKind::ScaleY, &scale)
                    .test_id("tooltip-content")
                    .child(
                        view()
                            .absolute()
                            .size(square)
                            .rotate(deg(45.0))
                            .rounded(px(1.5))
                            .bg(|| theme().foreground)
                            .accessibility_elements_hidden(true)
                            .test_id("tooltip-arrow")
                            .style_with(move |style| match actual.get() {
                                Placement::Top => style
                                    .bottom(px(-3.0))
                                    .left(percentage(0.5))
                                    .translate_x(px(-4.0)),
                                Placement::Bottom => style
                                    .top(px(-3.0))
                                    .left(percentage(0.5))
                                    .translate_x(px(-4.0)),
                                Placement::Left => style
                                    .right(px(-3.0))
                                    .top(percentage(0.5))
                                    .translate_y(px(-4.0)),
                                Placement::Right => style
                                    .left(px(-3.0))
                                    .top(percentage(0.5))
                                    .translate_y(px(-4.0)),
                            }),
                    )
                    .child(self.content),
            )
    }
}

struct Hint(Overlay);

impl IntoTooltip for Hint {
    fn apply_tooltip(self, node: &mut ElNode) {
        __hint_tooltip(node, self.0.into_element());
    }
}
