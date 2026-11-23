use components::badge::Badge;
use nsh_core::prelude::*;

pub fn preview() -> impl IntoElement {
    let mut row = view().flex_row().flex_wrap().gap_2();
    for (label, background, foreground) in [
        (
            "Blue",
            AdaptiveColor::rgb(0xeff6ff, 0x172554),
            AdaptiveColor::rgb(0x1d4ed8, 0x93c5fd),
        ),
        (
            "Green",
            AdaptiveColor::rgb(0xf0fdf4, 0x052e16),
            AdaptiveColor::rgb(0x15803d, 0x86efac),
        ),
        (
            "Sky",
            AdaptiveColor::rgb(0xf0f9ff, 0x082f49),
            AdaptiveColor::rgb(0x0369a1, 0x7dd3fc),
        ),
        (
            "Purple",
            AdaptiveColor::rgb(0xfaf5ff, 0x3b0764),
            AdaptiveColor::rgb(0x7e22ce, 0xd8b4fe),
        ),
        (
            "Red",
            AdaptiveColor::rgb(0xfef2f2, 0x450a0a),
            AdaptiveColor::rgb(0xb91c1c, 0xfca5a5),
        ),
    ] {
        row = row.child(Badge::new(label).colors(background, foreground));
    }
    row
}
