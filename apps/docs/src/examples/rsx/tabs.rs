use components::{rsx, tabs::*};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                Tabs::new(
                    TabsList::new("Account")
                        .trigger(TabsTrigger::new("account", "Account"))
                        .trigger(TabsTrigger::new("password", "Password")),
                )
                .default_value("account")
                .content(TabsContent::new("account", || {
                    text("Make changes to your account.")
                }))
                .content(TabsContent::new("password", || {
                    text("Change your password.")
                }))
            }
        </View>
    }
}
