use components::{
    div,
    drawer::{
        Drawer, DrawerRoot, drawer_close, drawer_content, drawer_description, drawer_footer,
        drawer_header, drawer_title, drawer_trigger,
    },
};
use nsh_core::{element::StyleRefinement, prelude::*};

fn body_style(style: StyleRefinement) -> StyleRefinement {
    style.flex_1().gap_2()
}

pub fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    DrawerRoot::new(drawer, move || {
        div()
            .child(drawer_trigger().child(text("Open Drawer")))
            .child(drawer_content().accessibility_label("Move Goal").child(|| {
                div()
                    .style_with(body_style)
                    .child(
                        drawer_header()
                            .child(drawer_title().child(text("Move Goal")))
                            .child(
                                drawer_description().child(text("Set your daily activity goal.")),
                            ),
                    )
                    .child(drawer_footer().child(drawer_close().child(text("Done"))))
            }))
    })
}
