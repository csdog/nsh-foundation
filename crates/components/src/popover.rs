//! Styled popovers backed by Native Shell's anchored overlay and dismissal.

use nsh_base::{PopoverEvent, popover, theme};
use nsh_core::{element::LayoutDirection, prelude::*};

use crate::{
    button::{Button, ButtonVariant},
    direction::{Direction, use_direction},
};

const STYLE: &str = "p-4 gap-4 rounded-3xl bg-popover text-popover-foreground ring-1 ring-foreground/5 dark:ring-foreground/10 shadow-lg transition duration-100";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

impl PopoverSide {
    fn physical(self, direction: Direction) -> Placement {
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

pub struct PopoverTrigger(View);

impl PopoverTrigger {
    pub fn new(label: impl IntoReactive<SharedString>) -> Self {
        Self::render(Button::new(label).variant(ButtonVariant::Outline).surface())
    }

    pub fn render(surface: View) -> Self {
        Self(surface)
    }
}

pub struct PopoverContent {
    label: SharedString,
    side: PopoverSide,
    align: Align,
    width: f32,
    side_offset: f32,
    align_offset: f32,
    initial_focus: Option<ElementRef>,
    children: Vec<AnyElement>,
}

impl Default for PopoverContent {
    fn default() -> Self {
        Self {
            label: "Popover".into(),
            side: PopoverSide::Bottom,
            align: Align::Center,
            width: 288.0,
            side_offset: 4.0,
            align_offset: 0.0,
            initial_focus: None,
            children: Vec::new(),
        }
    }
}

impl PopoverContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    /// Focus this element when the desktop popover opens.
    pub fn initial_focus(mut self, target: ElementRef) -> Self {
        self.initial_focus = Some(target);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }
}

pub struct Popover {
    trigger: PopoverTrigger,
    content: PopoverContent,
    open: Option<Signal<bool>>,
    default_open: bool,
}

impl Popover {
    pub fn new(trigger: PopoverTrigger, content: PopoverContent) -> Self {
        Self {
            trigger,
            content,
            open: None,
            default_open: false,
        }
    }

    /// Share visibility with the caller, including changes from native dismissal.
    pub fn open(mut self, open: Signal<bool>) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

impl IntoElement for Popover {
    fn into_element(self) -> AnyElement {
        let PopoverContent {
            label,
            side,
            align,
            width,
            side_offset,
            align_offset,
            initial_focus,
            children,
        } = self.content;
        let panel = element_ref();
        let open = self.open;
        let direction = use_direction();
        let placement_direction = direction.clone();
        let placement = move || side.physical(placement_direction.get());
        let horizontal = side.physical(Direction::Ltr).is_horizontal();
        let position = signal(None::<(Placement, Points)>);
        let animation_placement = placement.clone();
        let motion = move || {
            let side = position
                .get()
                .map_or_else(&animation_placement, |(side, _)| side);
            if reduce_motion() {
                return SharedString::from("");
            }
            SharedString::from(match side {
                Placement::Top => "opacity-0 scale-95 translate-y-2",
                Placement::Bottom => "opacity-0 scale-95 -translate-y-2",
                Placement::Left => "opacity-0 scale-95 translate-x-2",
                Placement::Right => "opacity-0 scale-95 -translate-x-2",
            })
        };
        let margin = 8.0 + align_offset.abs();
        let cross_direction = direction.clone();
        let cross = move || {
            px(if !horizontal && cross_direction.get() == Direction::Rtl {
                -align_offset
            } else {
                align_offset
            })
        };
        let desktop = is_desktop();
        popover()
            .open(move || open.map(|state| state.get()))
            .default_open(self.default_open)
            .on_open_change(move |event: &PopoverEvent| {
                if let (Some(open), PopoverEvent::OpenChange(next, _)) = (open, event) {
                    open.set(*next);
                }
            })
            .on_open(move || {
                if desktop {
                    initial_focus
                        .filter(|target| target.node().is_some())
                        .unwrap_or(panel)
                        .focus();
                }
            })
            .on_position(move |event: &PopoverEvent| {
                if let PopoverEvent::Position(side, _, available) = event {
                    let next = Some((*side, available.width));
                    if position.get_untracked() != next {
                        position.set(next);
                    }
                }
            })
            .on_close(move || position.set(None))
            .placement(placement)
            .align(align)
            .offset(px(side_offset))
            .margin(px(margin))
            .popup_class(STYLE)
            .popup_style(move |style| {
                let style = style
                    .direction(direction.map(LayoutDirection::from))
                    .w(move || px(width.min((window().width.0 - margin * 2.0).max(0.0))));
                if horizontal {
                    style
                        .max_w(move || position.get().map_or(px(width), |(_, room)| room))
                        .translate_y(cross)
                } else {
                    style.translate_x(cross)
                }
            })
            .popup_starting_class(motion.clone())
            .popup_ending_class(motion)
            .trigger(self.trigger.0)
            .child(
                view()
                    .role(AccessibilityRole::Dialog)
                    .label(label)
                    .test_id("popover-panel")
                    .element_ref(&panel)
                    .focusable(initial_focus.is_none())
                    .min_w(px(0.0))
                    .gap_4()
                    .children(children),
            )
            .into_element()
    }
}

pub struct PopoverHeader;

impl PopoverHeader {
    pub fn new() -> View {
        view().gap_1()
    }
}

pub struct PopoverTitle;

impl PopoverTitle {
    pub fn new(title: impl IntoReactive<SharedString>) -> Text {
        text(title).text_base().font_weight(FontWeight::MEDIUM)
    }
}

pub struct PopoverDescription;

impl PopoverDescription {
    pub fn new(description: impl IntoReactive<SharedString>) -> Text {
        text(description)
            .text_sm()
            .text_color(move || theme().muted_foreground)
    }
}
