use components::carousel::{
    CarouselContent, CarouselItem, CarouselNext, CarouselPrevious, carousel_root,
};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    carousel_root()
        .accessibility_label("Featured places")
        .previous(CarouselPrevious::new())
        .content(
            CarouselContent::new()
                .item(CarouselItem::new(view().child(text("Mountain"))))
                .item(CarouselItem::new(view().child(text("Coast"))))
                .item(CarouselItem::new(view().child(text("Forest")))),
        )
        .next(CarouselNext::new())
}
