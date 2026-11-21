use nsh_core::prelude::*;
use nsh_foundation::alert::{Alert, AlertDescription, AlertTitle};

pub fn preview() -> impl IntoElement {
    view().w_full().gap_4()
    .child(Alert::new().icon(text("✓").text_xs().text_center().w(px(16.0)).h(px(16.0)).border(px(1.0)).rounded_full()).title(AlertTitle::new("Payment successful")).description(AlertDescription::new("Your payment of $29.99 has been processed. A receipt has been sent to your email address.")))
    .child(Alert::new().icon(text("i").text_xs().text_center().w(px(16.0)).h(px(16.0)).border(px(1.0)).rounded_full()).title(AlertTitle::new("New feature available")).description(AlertDescription::new("We've added dark mode support. You can enable it in your account settings.")))
}
