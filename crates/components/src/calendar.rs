//! Rhea-styled date and range selection using local civil dates.

mod grid;

use chrono::{Datelike, FixedOffset, NaiveDate};
use nsh_base::{CalendarEntry, CalendarItemKind, YearRange, calendar, theme};
use nsh_core::{element::LayoutDirection, prelude::*};
use std::rc::Rc;

pub use nsh_base::{Date, Matcher};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CalendarLanguage {
    #[default]
    English,
    Arabic,
    Hebrew,
}

impl CalendarLanguage {
    pub(crate) fn rtl(self) -> bool {
        self != Self::English
    }

    pub(crate) fn month_name(self, month: usize) -> &'static str {
        match self {
            Self::English => MONTHS[month],
            Self::Arabic => ARABIC_MONTHS[month],
            Self::Hebrew => HEBREW_MONTHS[month],
        }
    }
}

pub struct Calendar {
    initial: Date,
    value: Option<Signal<Date>>,
    change: Option<Rc<dyn Fn(Date)>>,
    range: bool,
    months: u32,
    cell_size: Reactive<f32>,
    disabled: Option<Matcher>,
    year_range: Option<YearRange>,
    first_day: u32,
    rtl: bool,
    language: CalendarLanguage,
    persian: bool,
    week_numbers: bool,
}

impl Calendar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_value(mut self, date: impl Into<Date>) -> Self {
        self.initial = date.into();
        self
    }

    pub fn value(mut self, date: Signal<Date>) -> Self {
        self.value = Some(date);
        self
    }

    pub fn on_change(mut self, change: impl Fn(Date) + 'static) -> Self {
        self.change = Some(Rc::new(change));
        self
    }

    pub fn range(mut self, range: bool) -> Self {
        self.range = range;
        self
    }

    pub fn months(mut self, months: u32) -> Self {
        self.months = months.clamp(1, 2);
        self
    }

    pub fn cell_size(mut self, size: impl IntoReactive<f32>) -> Self {
        self.cell_size = size.into_reactive().map(|size| {
            if size.is_finite() {
                size.clamp(24.0, 64.0)
            } else {
                32.0
            }
        });
        self
    }

    pub fn disabled(mut self, matcher: Matcher) -> Self {
        self.disabled = Some(matcher);
        self
    }

    pub fn rtl(self, rtl: bool) -> Self {
        self.language(if rtl {
            CalendarLanguage::Arabic
        } else {
            CalendarLanguage::English
        })
    }

    pub fn language(mut self, language: CalendarLanguage) -> Self {
        self.language = language;
        self.rtl = language.rtl();
        self
    }

    pub fn year_range(mut self, years: YearRange) -> Self {
        self.year_range = Some(years);
        self
    }

    /// Render a Solar Hijri calendar. Selections still carry an ISO civil date.
    pub fn persian(mut self, persian: bool) -> Self {
        self.persian = persian;
        self.first_day = if persian { 6 } else { 0 };
        self
    }

    pub fn week_numbers(mut self, week_numbers: bool) -> Self {
        self.week_numbers = week_numbers;
        self
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            initial: Date::default(),
            value: None,
            change: None,
            range: false,
            months: 1,
            cell_size: Reactive::Static(32.0),
            disabled: None,
            year_range: None,
            first_day: 0,
            rtl: false,
            language: CalendarLanguage::English,
            persian: false,
            week_numbers: false,
        }
    }
}

impl IntoElement for Calendar {
    fn into_element(self) -> AnyElement {
        let value = self.value.unwrap_or_else(|| signal(self.initial));
        let direction = if self.rtl || self.persian {
            LayoutDirection::Rtl
        } else {
            LayoutDirection::Inherit
        };
        let width = self.cell_size.clone();
        let columns = if self.week_numbers { 8.0 } else { 7.0 };
        let months = self.months;
        let frame = view()
            .direction(direction)
            .gap_3()
            .min_w(px(0.0))
            .max_w(percentage(1.0))
            .p_3()
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(move || theme().border)
            .bg(move || theme().card)
            .text_color(move || theme().card_foreground)
            .w(move || {
                let month_width = width.get() * columns;
                px(if window().width.0 >= 720.0 && months == 2 {
                    2.0 * month_width + 40.0
                } else {
                    month_width + 26.0
                })
            });
        if self.persian || self.week_numbers {
            return frame.child(self.grid(value)).into_element();
        }
        let language = self.language;
        let cell = self.cell_size;
        let change = self.change;
        frame.child(calendar().range(self.range)
            .value(move || value.get())
            .number_of_months(months)
            .first_day_of_week(self.first_day)
            .disabled_matcher(self.disabled)
            .year_range(self.year_range)
            .header_class("mb-2")
            .body_class(move || Some(SharedString::from(if window().width.0 < 720.0 { "flex-col gap-4" } else { "flex-row gap-4" })))
            .item_class(move || {
                let max_cell = ((window().width.0 - 104.0) / 7.0).max(24.0);
                let size = cell.get().min(max_cell).floor();
                Some(SharedString::from(format!("min-w-[{size}px] h-[{size}px] rounded-2xl px-1 text-sm hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring aria-selected:bg-primary aria-selected:text-primary-foreground disabled:opacity-40 data-outside:text-muted-foreground data-today:border data-today:border-border data-range-middle:rounded-none data-range-middle:bg-secondary data-range-middle:text-secondary-foreground data-range-start:bg-primary data-range-start:text-primary-foreground data-range-end:bg-primary data-range-end:text-primary-foreground")))
            })
            .previous_accessibility_label(match language {
                CalendarLanguage::English => "Previous month",
                CalendarLanguage::Arabic => "الشهر السابق",
                CalendarLanguage::Hebrew => "החודש הקודם",
            })
            .next_accessibility_label(match language {
                CalendarLanguage::English => "Next month",
                CalendarLanguage::Arabic => "الشهر التالي",
                CalendarLanguage::Hebrew => "החודש הבא",
            })
            .render_item(move |entry: &CalendarEntry| text(native_label(entry, language)).text_sm())
            .on_change(move |date: &Date| {
                value.set(*date);
                if let Some(change) = &change { change(*date); }
            })
        ).into_element()
    }
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const ARABIC_MONTHS: [&str; 12] = [
    "يناير",
    "فبراير",
    "مارس",
    "أبريل",
    "مايو",
    "يونيو",
    "يوليو",
    "أغسطس",
    "سبتمبر",
    "أكتوبر",
    "نوفمبر",
    "ديسمبر",
];
const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
const ARABIC_WEEKDAYS: [&str; 7] = ["أح", "اث", "ثل", "أر", "خم", "جم", "سب"];
const HEBREW_WEEKDAYS: [&str; 7] = ["א׳", "ב׳", "ג׳", "ד׳", "ה׳", "ו׳", "ש׳"];
const HEBREW_MONTHS: [&str; 12] = [
    "ינואר",
    "פברואר",
    "מרץ",
    "אפריל",
    "מאי",
    "יוני",
    "יולי",
    "אוגוסט",
    "ספטמבר",
    "אוקטובר",
    "נובמבר",
    "דצמבר",
];

fn native_label(entry: &CalendarEntry, language: CalendarLanguage) -> String {
    let rtl = language.rtl();
    match entry.kind {
        CalendarItemKind::Previous => if rtl { "›" } else { "‹" }.to_owned(),
        CalendarItemKind::Next => if rtl { "‹" } else { "›" }.to_owned(),
        CalendarItemKind::MonthToggle | CalendarItemKind::Month => language
            .month_name((entry.value.saturating_sub(1) as usize).min(11))
            .to_owned(),
        CalendarItemKind::Weekday => {
            let weekdays = match language {
                CalendarLanguage::English => WEEKDAYS,
                CalendarLanguage::Arabic => ARABIC_WEEKDAYS,
                CalendarLanguage::Hebrew => HEBREW_WEEKDAYS,
            };
            weekdays[(entry.value as usize).min(6)].to_owned()
        }
        _ if language == CalendarLanguage::Arabic => digits(entry.value.to_string(), "٠١٢٣٤٥٦٧٨٩"),
        _ => entry.value.to_string(),
    }
}

fn digits(value: String, alphabet: &str) -> String {
    let digits: Vec<_> = alphabet.chars().collect();
    value
        .chars()
        .map(|ch| ch.to_digit(10).map_or(ch, |digit| digits[digit as usize]))
        .collect()
}

/// Interpret a timestamp using an explicit fixed offset.
pub fn date_at_offset(unix_ms: i64, seconds_east: i32) -> Option<NaiveDate> {
    let instant = chrono::DateTime::from_timestamp_millis(unix_ms)?;
    let offset = FixedOffset::east_opt(seconds_east)?;
    instant
        .naive_utc()
        .checked_add_offset(offset)
        .map(|time| time.date())
}

/// The calendar date in the device or browser's local time zone.
pub fn local_today() -> NaiveDate {
    let millis = nsh_core::time::unix_ms();
    #[cfg(not(target_arch = "wasm32"))]
    {
        chrono::DateTime::from_timestamp_millis(millis as i64)
            .unwrap_or_default()
            .with_timezone(&chrono::Local)
            .date_naive()
    }
    #[cfg(target_arch = "wasm32")]
    {
        #[link(wasm_import_module = "nsh")]
        unsafe extern "C" {
            fn local_offset(ms: f64) -> f64;
        }
        // Native Shell's Web host returns seconds east of UTC for this timestamp.
        let offset = unsafe { local_offset(millis) };
        date_at_offset(millis as i64, offset as i32).unwrap_or_default()
    }
}

/// A date in the current year, using today if the month or day is invalid.
pub fn date_this_year(month: u32, day: u32) -> NaiveDate {
    let today = local_today();
    NaiveDate::from_ymd_opt(today.year(), month, day).unwrap_or(today)
}
