use components::{input_otp::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    rsx! {
        <View>
            {
                InputOtp::new("Verification code", 6)
                    .group(InputOtpGroup::slots(0..3))
                    .separator(InputOtpSeparator::new())
                    .group(InputOtpGroup::slots(3..6))
            }
        </View>
    }
}
