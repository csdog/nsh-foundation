use components::{rsx, sidebar::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                SidebarProvider::new(|state: SidebarState| {
                    Sidebar::new(state, |parts| {
                        sidebar_header().child(
                            SidebarMenuButton::new(parts, "Home")
                                .icon(text("⌂"))
                                .surface(),
                        )
                    })
                    .collapsible(SidebarCollapsible::Icon)
                    .layout(sidebar_inset().child(SidebarTrigger::new(state)))
                })
            }
        </View>
    }
}
