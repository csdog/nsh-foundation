use chrono::{Datelike, Days, NaiveDate, NaiveTime};
use nsh_base::{TimeOfDay, TimePrecision, YearRange, theme, time_field};
use nsh_core::prelude::*;
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    calendar::{CalendarLanguage, Date, date_this_year, local_today},
    date_picker::{DateInput, DatePicker, parse_natural_date},
};

pub fn demo() -> impl IntoElement {
    sized(
        DatePicker::new("Pick a date").test_id("date-picker-demo"),
        192.0,
    )
}

pub fn basic() -> impl IntoElement {
    sized(
        DatePicker::new("Pick a date")
            .label("Date")
            .test_id("date-picker-basic"),
        176.0,
    )
}

pub fn range() -> impl IntoElement {
    let start = date_this_year(1, 20);
    let end = start.checked_add_days(Days::new(20)).unwrap_or(start);
    sized(
        DatePicker::new("Pick a date range")
            .label("Date Picker Range")
            .test_id("date-picker-range")
            .range()
            .default_value((start, end))
            .format(|date| date.format("%b %-d, %Y").to_string()),
        294.0,
    )
}

pub fn dob() -> impl IntoElement {
    sized(
        DatePicker::new("Select date")
            .label("Date of birth")
            .test_id("date-picker-dob")
            .close_on_select()
            .year_range(YearRange::new(1900, local_today().year() + 1))
            .format(|date| date.format("%-m/%-d/%Y").to_string()),
        192.0,
    )
}

pub fn input() -> impl IntoElement {
    sized(
        DatePicker::new("Select date")
            .label("Subscription Date")
            .test_id("date-picker-input")
            .default_value(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
            .format(input_format)
            .input(DateInput::Date, "June 01, 2025", ""),
        224.0,
    )
}

pub fn time() -> impl IntoElement {
    let time = NaiveTime::from_hms_opt(10, 30, 0).unwrap();
    view()
        .w_full()
        .max_w(px(320.0))
        .flex_row()
        .items_start()
        .gap_3()
        .child(
            view().flex_1().min_w(px(0.0)).child(
                DatePicker::new("Select date")
                    .label("Date")
                    .test_id("date-picker-time")
                    .close_on_select(),
            ),
        )
        .child(
            view()
                .w(px(112.0))
                .gap_2()
                .child(text("Time").text_sm().font_weight(FontWeight::MEDIUM))
                .child(
                    time_field()
                        .default_value(TimeOfDay(time))
                        .precision(TimePrecision::Second)
                        .accessibility_label("Time")
                        .h(px(36.0))
                        .px_2()
                        .rounded(px(10.0))
                        .border(px(1.0))
                        .border_color(move || theme().input)
                        .segment_class("text-sm px-1 rounded-md focus:bg-muted")
                        .separator_class("text-sm text-muted-foreground"),
                ),
        )
}

pub fn natural_language() -> impl IntoElement {
    let initial = "In 2 days";
    let date = signal(Date::Single(parse_natural_date(initial, local_today())));
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_2()
        .child(
            DatePicker::new("Select date")
                .label("Schedule Date")
                .test_id("date-picker-natural-language")
                .value(date)
                .format(input_format)
                .input(DateInput::NaturalLanguage, "Tomorrow or next week", initial),
        )
        .child(
            text(move || match date.get().start() {
                Some(date) => format!("Your post will be published on {}.", input_format(date)),
                None => "Choose a date for your post.".to_owned(),
            })
            .text_sm()
            .text_color(move || theme().muted_foreground),
        )
}

pub fn rtl() -> impl IntoElement {
    let hebrew = signal(false);
    let date = signal(Date::default());
    view()
        .w_full()
        .max_w(px(220.0))
        .gap_3()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Arabic")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(false)),
                )
                .child(
                    Button::new("Hebrew")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(true)),
                ),
        )
        .child(dynamic(move || {
            let (placeholder, language) = if hebrew.get() {
                ("בחר תאריך", CalendarLanguage::Hebrew)
            } else {
                ("اختر تاريخًا", CalendarLanguage::Arabic)
            };
            DatePicker::new(placeholder)
                .value(date)
                .language(language)
                .test_id("date-picker-rtl")
        }))
}

fn sized(picker: DatePicker, width: f32) -> View {
    view().w_full().max_w(px(width)).child(picker)
}

fn input_format(date: NaiveDate) -> String {
    date.format("%B %d, %Y").to_string()
}
