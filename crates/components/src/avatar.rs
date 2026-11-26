//! Rhea-styled avatars backed by Native Shell's image loading and fallback.

use nsh_base::{avatar as base_avatar, theme};

pub mod tsx;
use nsh_core::{
    element::{ImageSrc, LayoutDirection},
    prelude::*,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
pub enum AvatarSize {
    Small,
    #[default]
    Default,
    Large,
}

impl AvatarSize {
    fn diameter(self) -> f32 {
        match self {
            Self::Small => 24.0,
            Self::Default => 32.0,
            Self::Large => 40.0,
        }
    }
}

pub struct Avatar {
    name: SharedString,
    image: Option<AvatarImage>,
    fallback: Option<AvatarFallback>,
    badge: Option<AvatarBadge>,
    size: AvatarSize,
    grayscale: bool,
    grouped: bool,
}

impl Avatar {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            image: None,
            fallback: None,
            badge: None,
            size: AvatarSize::Default,
            grayscale: false,
            grouped: false,
        }
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = name.into();
        self
    }

    pub fn image(mut self, image: AvatarImage) -> Self {
        self.image = Some(image);
        self
    }

    pub fn fallback(mut self, fallback: AvatarFallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub fn badge(mut self, badge: AvatarBadge) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl IntoElement for Avatar {
    fn into_element(self) -> AnyElement {
        let size = self.size;
        let label = match self.badge.as_ref().and_then(|badge| badge.status) {
            Some(status) => SharedString::from(format!("{}, {status}", self.name)),
            None => self.name.clone(),
        };
        let mut root = base_avatar()
            .name(self.name)
            .accessibility_label(label)
            .src(self.image.map(|image| image.0))
            .size(px(size.diameter()))
            .flex_shrink_0()
            .rounded_full()
            .image_class("rounded-full object-cover")
            .fallback_class(if size == AvatarSize::Small {
                "rounded-full bg-muted text-muted-foreground items-center justify-center text-xs"
            } else {
                "rounded-full bg-muted text-muted-foreground items-center justify-center text-sm"
            })
            .when(self.grayscale, |root| root.grayscale(1.0))
            .when(self.grouped, |root| {
                root.ring(px(2.0)).ring_color(move || theme().background)
            });
        if let Some(fallback) = self.fallback {
            root = root.fallback(
                text(fallback.0)
                    .text_color(move || theme().muted_foreground)
                    .when(size == AvatarSize::Small, |label| label.text_xs())
                    .when(size != AvatarSize::Small, |label| label.text_sm()),
            );
        }
        if let Some(badge) = self.badge {
            root = root.child(badge.render(size));
        }
        root.into_element()
    }
}

pub struct AvatarImage(ImageSrc);

impl AvatarImage {
    pub fn new(src: impl Into<ImageSrc>) -> Self {
        Self(src.into())
    }
}

pub struct AvatarFallback(SharedString);

impl AvatarFallback {
    pub fn new(initials: impl Into<SharedString>) -> Self {
        Self(initials.into())
    }
}

#[derive(Default)]
pub struct AvatarBadge {
    status: Option<&'static str>,
    icon: Option<AnyElement>,
    color: Option<AdaptiveColor>,
}

impl AvatarBadge {
    pub fn new(status: &'static str) -> Self {
        Self {
            status: Some(status),
            ..Self::default()
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_element());
        self
    }

    pub fn color(mut self, color: AdaptiveColor) -> Self {
        self.color = Some(color);
        self
    }

    fn render(self, size: AvatarSize) -> View {
        let diameter = match size {
            AvatarSize::Small => 8.0,
            AvatarSize::Default => 10.0,
            AvatarSize::Large => 12.0,
        };
        let mut badge = view()
            .absolute()
            .right_0()
            .bottom_0()
            .size(px(diameter))
            .rounded_full()
            .z_index(10)
            .items_center()
            .justify_center()
            .ring(px(2.0))
            .ring_color(move || theme().background)
            .bg(move || self.color.unwrap_or_else(|| theme().primary))
            .text_color(move || theme().primary_foreground)
            .accessibility_elements_hidden(true);
        if let Some(icon) = self.icon.filter(|_| size != AvatarSize::Small) {
            badge = badge.child(icon);
        }
        badge
    }
}

#[derive(Default)]
pub struct AvatarGroup {
    items: Vec<Avatar>,
    overflow: Option<AvatarGroupCount>,
    direction: LayoutDirection,
}

impl AvatarGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, avatar: Avatar) -> Self {
        self.items.push(avatar);
        self
    }

    pub fn count(mut self, count: AvatarGroupCount) -> Self {
        self.overflow = Some(count);
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl IntoElement for AvatarGroup {
    fn into_element(self) -> AnyElement {
        let mut row = view()
            .role(AccessibilityRole::Group)
            .label("Avatars")
            .direction(self.direction)
            .flex_row()
            .items_center();
        let size = self
            .items
            .iter()
            .map(|avatar| avatar.size)
            .reduce(|size, next| {
                if next.diameter() > size.diameter() {
                    next
                } else {
                    size
                }
            })
            .unwrap_or_default();
        let mut overlap = 0.0;
        for mut avatar in self.items {
            avatar.grouped = true;
            row = row.child(view().ml(px(overlap)).child(avatar));
            overlap = -8.0;
        }
        if let Some(count) = self.overflow {
            row = row.child(view().ml(px(overlap)).child(count.render(size)));
        }
        row.into_element()
    }
}

pub struct AvatarGroupCount {
    label: &'static str,
    visual: AnyElement,
}

impl AvatarGroupCount {
    pub fn new(label: &'static str, visual: impl IntoElement) -> Self {
        Self {
            label,
            visual: visual.into_element(),
        }
    }

    fn render(self, size: AvatarSize) -> View {
        view()
            .accessible(true)
            .label(self.label)
            .size(px(size.diameter()))
            .flex_shrink_0()
            .ring(px(2.0))
            .ring_color(move || theme().background)
            .rounded_full()
            .items_center()
            .justify_center()
            .bg(move || theme().muted)
            .text_color(move || theme().muted_foreground)
            .child(
                view()
                    .accessibility_elements_hidden(true)
                    .child(self.visual),
            )
    }
}
