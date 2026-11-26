//! Native Shell TSX binding for the Foundation avatar.

use nsh_core::{element::ImageSrc, prelude::*};

use super::{AvatarFallback, AvatarImage, AvatarSize};

/// Registers Avatar in an app native module.
pub fn register(registry: &mut Registry) {
    registry.register::<Avatar>();
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
}

/// A person's image or initials.
#[component]
pub fn Avatar(props: AvatarProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let mut avatar = super::Avatar::new(props.name.get()).size(props.size.get());
    if let Some(src) = props.src.get() {
        avatar = avatar.image(AvatarImage::new(src));
    }
    if let Some(initials) = props.fallback.get() {
        avatar = avatar.fallback(AvatarFallback::new(initials));
    }
    avatar
}
