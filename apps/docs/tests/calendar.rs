use docs::App;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{FixedOffset, NaiveDate, TimeZone};
use components::calendar::date_at_offset;
use nsh_core::{Size, element::Root, platform::Os, prelude::*};
use nsh_engine::{Clock, EngineConfig};
use nsh_testing::Headless;

#[test]
fn calendar_gallery_selects_ranges_and_supports_booked_and_persian_days() {
    let mut config = EngineConfig::new(Root::of::<App>());
    config.viewport = Size::new(px(390.0), px(844.0));
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    config.clock = Clock::manual(UNIX_EPOCH + Duration::from_secs(1_795_262_400));
    let mut app = Headless::start_at_page(config, "https://foundation.test/#calendar").unwrap();
    app.first_render().unwrap();
    app.wait_for("id:calendar-docs-scroll").unwrap();

    for name in [
        "Calendar demo",
        "Persian / Hijri / Jalali Calendar",
        "Selected Date (With TimeZone)",
        "Basic",
        "Range Calendar",
        "Month and Year Selector",
        "Presets",
        "Date and Time Picker",
        "Booked dates",
        "Custom Cell Size",
        "Week Numbers",
        "RTL",
    ] {
        assert_eq!(app.screen().count(&format!("role:region[{name}]")), Ok(1));
    }

    assert_eq!(
        app.screen()
            .enabled("role:region[Booked dates] >> label:2026-01-12"),
        Ok(false)
    );
    app.press("role:region[Range Calendar] >> label:2026-01-20")
        .unwrap();
    assert!(
        app.screen()
            .selected("role:region[Range Calendar] >> label:2026-01-20")
            .unwrap()
    );
    app.press("role:region[Range Calendar] >> label:2026-01-23")
        .unwrap();
    assert!(app.shows("2026-01-20 - 2026-01-23"));

    let persian = "role:region[Persian / Hijri / Jalali Calendar]";
    assert!(app.shows("خرداد ۱۴۰۴"));
    assert!(
        app.screen()
            .selected(&format!("{persian} >> label:2025-06-12"))
            .unwrap()
    );
    app.press(&format!("{persian} >> role:button[Next month]"))
        .unwrap();
    assert!(app.shows("تیر ۱۴۰۴"));
    assert!(app.shows("Wk"));

    app.press("role:region[Presets] >> role:button[Tomorrow]")
        .unwrap();
    assert!(app.shows("Selected: 2026-11-22"));
}

#[test]
fn dates_are_interpreted_on_the_correct_side_of_midnight() {
    let instant = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2026, 1, 1, 1, 0, 0)
        .unwrap();
    assert_eq!(
        date_at_offset(instant.timestamp_millis(), -8 * 3600),
        NaiveDate::from_ymd_opt(2025, 12, 31)
    );
    assert_eq!(
        date_at_offset(instant.timestamp_millis(), 14 * 3600),
        NaiveDate::from_ymd_opt(2026, 1, 1)
    );
}

#[component]
fn PersianKeyboard() -> impl IntoElement {
    let value = signal(components::calendar::Date::from(
        NaiveDate::from_ymd_opt(2025, 6, 12).unwrap(),
    ));
    view()
        .child(
            components::calendar::Calendar::new()
                .persian(true)
                .value(value),
        )
        .child(text(move || {
            format!(
                "ISO date: {}",
                value.get().format("%Y-%m-%d").unwrap_or_default()
            )
        }))
}

#[test]
fn persian_grid_left_arrow_advances_one_civil_day() {
    let mut config = EngineConfig::new(nsh_testing::windowed_root::<PersianKeyboard>());
    config.environment.os = Os::Linux;
    config.environment.reduce_motion = true;
    let mut app = Headless::start(config).unwrap();
    app.first_render().unwrap();
    for _ in 0..3 {
        app.press_key("tab").unwrap();
    }
    app.press_key("left").unwrap();
    app.press_key("enter").unwrap();
    assert!(app.shows("ISO date: 2025-06-13"));
}
