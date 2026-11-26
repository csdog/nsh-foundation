use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <Alert>
            <AlertTitle>Payment received</AlertTitle>
            <AlertDescription>Your receipt is ready.</AlertDescription>
        </Alert>
    }
}
