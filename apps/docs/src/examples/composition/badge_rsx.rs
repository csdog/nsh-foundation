use components::{badge::BadgeVariant, rsx};
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Badge variant={BadgeVariant::Outline}>
            <Text>In progress</Text>
        </Badge>
    }
}
