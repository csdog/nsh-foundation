use components::rsx;
use nsh_base::{menu, menu_item, menu_separator, theme};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    let selected = signal(None::<&'static str>);
    let choice = |label: &'static str, destructive| {
        menu_item()
            .accessibility_label(label)
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .hover(|style| style.bg(theme().muted))
            .focus(|style| style.bg(theme().muted))
            .on_select(move || selected.set(Some(label)))
            .child(text(label).text_sm().text_color(move || {
                if destructive {
                    theme().destructive
                } else {
                    theme().popover_foreground
                }
            }))
    };
    rsx! {
        <View style={|style| style.items_center().gap_2()}>
            {
                menu()
                    .offset(px(6.0))
                    .margin(px(8.0))
                    .popup_class("w-32 p-1 bg-popover text-popover-foreground rounded-lg border shadow-md")
                    .trigger(rsx! {
                        <View role={AccessibilityRole::Button} accessibilityLabel="Account menu"
                            style={|style| style.rounded_full()}>
                            <Avatar name="shadcn" src={asset!("avatar-shadcn.jpg")}>
                                <AvatarFallback>CN</AvatarFallback>
                            </Avatar>
                        </View>
                    })
                    .child(choice("Profile", false))
                    .child(choice("Billing", false))
                    .child(choice("Settings", false))
                    .child(menu_separator().h(px(1.0)).my_1().bg(move || theme().border))
                    .child(choice("Log out", true))
            }
            {when(move || selected.get().is_some(), move || {
                text(move || format!("Selected in this preview: {}", selected.get().unwrap_or_default())).text_sm()
            })}
        </View>
    }
}
