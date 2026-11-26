use components::{resizable::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                ResizablePanelGroup::new(ResizableOrientation::Horizontal)
                    .panel(ResizablePanel::new(text("One")).default_size(50.0))
                    .handle(ResizableHandle::new().with_handle(true))
                    .panel(ResizablePanel::new(text("Two")).default_size(50.0))
            }
        </View>
    }
}
