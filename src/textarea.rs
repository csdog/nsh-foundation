//! Rhea-styled multiline Native Shell input.

use nsh_base::theme;
use nsh_core::{element::Input, prelude::*};

use crate::{
    direction::use_direction,
    input::{styled_input, text_input_base},
};

/// A multiline input that grows with its contents. Use Native Shell's input
/// methods to set its accessible name, placeholder, value, and events.
pub fn textarea() -> Input {
    invalid_textarea(false)
}

/// Applies reactive validation styling and the accessible invalid state.
pub fn invalid_textarea(invalid: impl IntoReactive<bool>) -> Input {
    styled_input(text_input_base(), invalid)
        .multiline(true)
        .min_h(px(64.0))
        .py_2()
        .direction(use_direction().map(LayoutDirection::from))
        .line_height(px(20.0))
        .style_when(
            || window().width.0 < 768.0,
            |style| style.line_height(px(24.0)),
        )
        .placeholder_color(|| {
            let color = theme().muted_foreground;
            if dark() { color.dark } else { color.light }
        })
}
