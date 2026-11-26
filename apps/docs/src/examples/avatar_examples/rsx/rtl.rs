use components::{avatar::AvatarGroupCount, rsx};
use nsh_core::element::LayoutDirection;
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View style={|style| style.flex_row().items_center().gap_4()} direction={LayoutDirection::Rtl}>
            <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")}>
                <AvatarFallback>CN</AvatarFallback>
            </Avatar>
            <Avatar name="@evilrabbit" src={asset!("avatar-evilrabbit.png")}>
                <AvatarFallback>ER</AvatarFallback>
                <AvatarBadge status="Online" color={AdaptiveColor::rgb(0x16a34a, 0x166534)} />
            </Avatar>
            <AvatarGroup direction={LayoutDirection::Rtl}
                count={AvatarGroupCount::new("ثلاثة أشخاص آخرون", text("+٣").text_sm())}
            >
                <Avatar name="@shadcn" src={asset!("avatar-shadcn.jpg")} grayscale>
                    <AvatarFallback>CN</AvatarFallback>
                </Avatar>
                <Avatar name="@maxleiter" src={asset!("avatar-maxleiter.png")} grayscale>
                    <AvatarFallback>LR</AvatarFallback>
                </Avatar>
                <Avatar name="@evilrabbit" src={asset!("avatar-evilrabbit.png")} grayscale>
                    <AvatarFallback>ER</AvatarFallback>
                </Avatar>
            </AvatarGroup>
        </View>
    }
}
