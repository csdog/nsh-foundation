use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Carousel accessibilityLabel="Featured places">
            <CarouselPrevious />
            <CarouselContent>
                <CarouselItem><Text>Mountain</Text></CarouselItem>
                <CarouselItem><Text>Coast</Text></CarouselItem>
                <CarouselItem><Text>Forest</Text></CarouselItem>
            </CarouselContent>
            <CarouselNext />
        </Carousel>
    }
}
