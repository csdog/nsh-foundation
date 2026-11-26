//! Native Shell TSX binding for the Foundation avatar.

use nsh_core::{Hsla, element::ImageSrc, prelude::*};

use super::{AvatarBadge, AvatarFallback, AvatarSize, group_row};

#[derive(Clone, Copy)]
struct GroupedAvatar;

/// Registers Avatar in an app native module.
pub fn register(registry: &mut Registry) {
    registry
        .register::<Avatar>()
        .register::<AvatarGroup>()
        .register::<AvatarGroupCount>();
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarProps {
    /// The person's accessible name.
    pub name: Prop<SharedString>,
    /// The picture URL or image asset.
    pub src: Prop<Option<ImageSrc>>,
    /// The fallback initials, if they differ from the name's initials.
    pub fallback: Prop<Option<SharedString>>,
    /// The diameter preset.
    #[prop(default)]
    pub size: Prop<AvatarSize>,
    /// Desaturates the avatar.
    #[prop(default)]
    pub grayscale: Prop<bool>,
    /// The accessible status of the badge.
    pub badge_status: Prop<Option<SharedString>>,
    /// The badge color.
    pub badge_color: Prop<Option<Hsla>>,
    /// An optional text glyph in the badge.
    pub badge_icon: Prop<Option<SharedString>>,
}

/// A person's image or initials.
#[component]
pub fn Avatar(props: AvatarProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let mut avatar = super::Avatar::new(props.name.get())
        .size(props.size.get())
        .grayscale(props.grayscale.get());
    avatar.grouped = use_context::<GroupedAvatar>().is_some();
    if let Some(src) = props.src.get() {
        avatar = avatar.src(src);
    }
    if let Some(initials) = props.fallback.get() {
        avatar = avatar.fallback(AvatarFallback::new(initials));
    }
    if let Some(status) = props.badge_status.get() {
        let mut badge = AvatarBadge::new(status);
        if let Some(color) = props.badge_color.get() {
            badge = badge.color(color.into());
        }
        if let Some(icon) = props.badge_icon.get() {
            badge = badge.icon(text(icon).text_size(px(8.0)));
        }
        avatar = avatar.badge(badge);
    }
    avatar
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarGroupProps {
    /// The avatars and optional count.
    pub children: Prop<Children>,
}

/// An overlapping row of avatars.
#[component]
pub fn AvatarGroup(props: AvatarGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_context(GroupedAvatar);
    group_row().children_each(props.children, move |child, at| {
        view()
            .ml(move || px(if at.index() == 0 { 0.0 } else { -8.0 }))
            .child(child)
    })
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarGroupCountProps {
    /// The accessible name for the remaining avatars.
    pub accessibility_label: Prop<SharedString>,
    /// The visible count or icon.
    pub children: Prop<Children>,
    /// The count's diameter preset.
    #[prop(default)]
    pub size: Prop<AvatarSize>,
}

/// The overflow indicator for an avatar group.
#[component]
pub fn AvatarGroupCount(props: AvatarGroupCountProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    super::AvatarGroupCount::new(props.accessibility_label.get(), props.children)
        .render(props.size.get())
}
