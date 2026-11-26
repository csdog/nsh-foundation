use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <DrawerRoot>
            <DrawerTrigger>
                <Text>Open Drawer</Text>
            </DrawerTrigger>
            <DrawerContent accessibilityLabel="Move Goal">
                <View style={{ padding: 16, gap: 8 }}>
                    <Text style={{ fontSize: 20 }}>Move Goal</Text>
                    <Text>Set your daily activity goal.</Text>
                    <DrawerClose>
                        <Text class="text-primary-foreground">Done</Text>
                    </DrawerClose>
                </View>
            </DrawerContent>
        </DrawerRoot>
    }
}
