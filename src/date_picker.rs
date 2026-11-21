//! A composable calendar popover with optional date entry.

use std::rc::Rc;

use chrono::{Datelike, Days, NaiveDate, Weekday};
use nsh_base::{PopoverEvent, YearRange, input, popover, theme};
use nsh_core::{element::LayoutDirection, keymap::When, prelude::*};

use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    calendar::{Calendar, CalendarLanguage, Date, Matcher, local_today},
};

#[derive(Clone, Copy)]
pub enum DateInput {
    /// ISO, US numeric, or English dates with a month name.
    Date,
    /// Explicit dates and common English phrases such as "in 2 days".
    NaturalLanguage,
}

type DateFormat = Rc<dyn Fn(NaiveDate) -> String>;

pub struct DatePicker {
    placeholder: &'static str,
    label: Option<&'static str>,
    trigger_id: Option<&'static str>,
    value: Option<Signal<Date>>,
    initial: Date,
    format: Option<DateFormat>,
    range: bool,
    close_on_select: bool,
    language: CalendarLanguage,
    years: Option<YearRange>,
    disabled: Option<Matcher>,
    input: Option<(DateInput, &'static str, &'static str)>,
}

impl DatePicker {
    pub fn new(placeholder: &'static str) -> Self {
        Self {
            placeholder,
            label: None,
            trigger_id: None,
            value: None,
            initial: Date::default(),
            format: None,
            range: false,
            close_on_select: false,
            language: CalendarLanguage::English,
            years: None,
            disabled: None,
            input: None,
        }
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn test_id(mut self, id: &'static str) -> Self {
        self.trigger_id = Some(id);
        self
    }

    pub fn value(mut self, value: Signal<Date>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl Into<Date>) -> Self {
        self.initial = value.into();
        self
    }

    pub fn range(mut self) -> Self {
        self.range = true;
        self.input = None;
        self
    }

    pub fn close_on_select(mut self) -> Self {
        self.close_on_select = true;
        self
    }

    pub fn language(mut self, language: CalendarLanguage) -> Self {
        self.language = language;
        self
    }

    pub fn year_range(mut self, years: YearRange) -> Self {
        self.years = Some(years);
        self
    }

    pub fn disabled(mut self, matcher: Matcher) -> Self {
        self.disabled = Some(matcher);
        self
    }

    pub fn format(mut self, format: impl Fn(NaiveDate) -> String + 'static) -> Self {
        self.format = Some(Rc::new(format));
        self
    }

    /// Replaces the date button with an editable single-date field and calendar button.
    pub fn input(
        mut self,
        mode: DateInput,
        placeholder: &'static str,
        initial_text: &'static str,
    ) -> Self {
        self.range = false;
        self.input = Some((mode, placeholder, initial_text));
        self
    }
}

impl IntoElement for DatePicker {
    fn into_element(self) -> AnyElement {
        let date = self.value.unwrap_or_else(|| signal(self.initial));
        let open = signal(false);
        let focus_calendar = stored(false);
        let calendar_ref = element_ref();
        let label = self.label;
        let placeholder = self.placeholder;
        let lang = self.language;
        let direction = if lang.rtl() {
            LayoutDirection::Rtl
        } else {
            LayoutDirection::Ltr
        };
        let format = self
            .format
            .unwrap_or_else(|| Rc::new(move |date| format_date(date, lang)));
        let entered = signal(self.input.map_or_else(String::new, |(_, _, initial)| {
            if initial.is_empty() {
                date.get_untracked()
                    .start()
                    .map_or_else(String::new, |date| format(date))
            } else {
                initial.to_owned()
            }
        }));
        let invalid = signal(false);
        let close = self.close_on_select || self.input.is_some();
        let range = self.range;
        let matcher = self.disabled.clone();
        let years = self.years;
        let trigger = if self.input.is_some() {
            Button::new(placeholder)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconSmall)
                .icon_start(calendar_icon())
                .surface()
        } else {
            let format = format.clone();
            Button::new(move || match date.get() {
                Date::Range(Some(start), Some(end)) => {
                    format!("{} – {}", format(start), format(end))
                }
                value => value
                    .start()
                    .map_or_else(|| placeholder.to_owned(), |date| format(date)),
            })
            .variant(ButtonVariant::Outline)
            .icon_start(calendar_icon())
            .surface()
            .w_full()
            .justify_start()
            .direction(direction)
        };
        let trigger = trigger.when_some(self.trigger_id, |trigger, id| trigger.test_id(id));
        let popup = popover()
            .open(move || open.get())
            .align(if self.input.is_some() || lang.rtl() {
                Align::End
            } else {
                Align::Start
            })
            .margin(px(8.0))
            .offset(px(5.0))
            .popup_class("rounded-2xl shadow-lg transition-opacity duration-100")
            .popup_starting_class("opacity-0")
            .popup_ending_class("opacity-0")
            .trigger(trigger)
            .on_open(move || {
                if focus_calendar.replace(false) {
                    calendar_ref.focus();
                }
            })
            .on_open_change(move |event: &PopoverEvent| {
                if let PopoverEvent::OpenChange(next, _) = event {
                    open.set(*next);
                }
            })
            .child(dynamic({
                let format = format.clone();
                move || {
                    let mut calendar = Calendar::new().value(date).language(lang).range(range);
                    if range && window().width.0 >= 720.0 {
                        calendar = calendar.months(2);
                    }
                    if let Some(matcher) = &matcher {
                        calendar = calendar.disabled(matcher.clone());
                    }
                    if let Some(years) = years {
                        calendar = calendar.year_range(years);
                    }
                    let format = format.clone();
                    view()
                        .direction(direction)
                        .role(AccessibilityRole::Dialog)
                        .label(match lang {
                            CalendarLanguage::English => "Calendar",
                            CalendarLanguage::Arabic => "التقويم",
                            CalendarLanguage::Hebrew => "לוח שנה",
                        })
                        .focusable(true)
                        .element_ref(&calendar_ref)
                        .child(calendar.on_change(move |selection| {
                            if let Some(chosen) = selection.start() {
                                entered.set(format(chosen));
                                invalid.set(false);
                            }
                            if close && selection.is_complete() {
                                open.set(false);
                            }
                        }))
                }
            }));
        let control = match self.input {
            None => popup.into_element(),
            Some((mode, hint, _)) => {
                let name = label.unwrap_or(placeholder);
                // Programmatic updates change the field. Values resolved from a
                // draft retain the user's spelling until a calendar selection.
                let resolved = stored(date.get_untracked());
                effect(move || {
                    let current = date.get();
                    if resolved.replace(current) != current {
                        entered.set(current.start().map_or_else(String::new, |day| format(day)));
                        invalid.set(false);
                    }
                });
                let matcher = self.disabled;
                let parse = Rc::new(move |value: &str| {
                    let parsed = match mode {
                        DateInput::Date => parse_date(value),
                        DateInput::NaturalLanguage => parse_natural_date(value, local_today()),
                    }?;
                    (!matcher
                        .as_ref()
                        .is_some_and(|m| m.matched(&parsed) == Some(true)))
                    .then_some(parsed)
                });
                let changed = parse.clone();
                let apply = move |value| {
                    resolved.replace(value);
                    date.set(value);
                    invalid.set(false);
                };
                let update = move |value: &str| {
                    entered.set(value.to_owned());
                    if value.trim().is_empty() {
                        apply(Date::Single(None));
                    } else if let Some(parsed) = changed(value) {
                        apply(parsed.into());
                    }
                };
                let focused = signal(false);
                effect(move || {
                    if focused.get() && !open.get() {
                        on_key_with(
                            "down",
                            KeyOptions {
                                when: When::Always,
                                repeat: false,
                                for_focus: true,
                                ..Default::default()
                            },
                            move |_| {
                                focus_calendar.replace(true);
                                open.set(true);
                            },
                        );
                    }
                });
                let field = view().flex_1().min_w(px(0.0)).child(
                    input()
                        .label(name)
                        .value(move || entered.get())
                        .placeholder(hint)
                        .h(px(34.0))
                        .px_2()
                        .text_color(move || theme().foreground)
                        .on_focus(move || focused.set(true))
                        .on_blur(move || focused.set(false))
                        .on_change(update)
                        .on_submit(move |value: &str| {
                            if let Some(parsed) = parse(value) {
                                apply(parsed.into());
                            } else {
                                invalid.set(!value.trim().is_empty());
                            }
                        }),
                );
                view()
                    .gap_1()
                    .child(
                        view()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .pr_1()
                            .rounded(px(12.0))
                            .border(px(1.0))
                            .border_color(move || {
                                if invalid.get() {
                                    theme().destructive
                                } else {
                                    theme().input
                                }
                            })
                            .child(field)
                            .child(popup),
                    )
                    .child(when(invalid, move || {
                        text(match mode {
                            DateInput::Date => "Enter a valid date.",
                            DateInput::NaturalLanguage => {
                                "Try an exact date, tomorrow, or a phrase like in 2 days."
                            }
                        })
                        .text_sm()
                        .text_color(move || theme().destructive)
                    }))
                    .into_element()
            }
        };
        view()
            .direction(direction)
            .w_full()
            .gap_2()
            .child(label.map(|label| text(label).text_sm().font_weight(FontWeight::MEDIUM)))
            .child(control)
            .into_element()
    }
}

fn calendar_icon() -> View {
    view()
        .size(px(14.0))
        .rounded(px(3.0))
        .border(px(1.0))
        .border_color(move || theme().foreground)
        .pt(px(4.0))
        .accessibility_elements_hidden(true)
        .child(view().h(px(1.0)).w_full().bg(move || theme().foreground))
}

pub fn format_date(date: NaiveDate, lang: CalendarLanguage) -> String {
    let day = date.day();
    let month = lang.month_name(date.month0() as usize);
    let year = date.year();
    match lang {
        CalendarLanguage::English => {
            let suffix = match day {
                11..=13 => "th",
                _ => match day % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                },
            };
            format!("{month} {day}{suffix}, {year}")
        }
        CalendarLanguage::Arabic => format!("{day} {month} {year}")
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .and_then(|digit| char::from_u32('٠' as u32 + digit))
                    .unwrap_or(c)
            })
            .collect(),
        CalendarLanguage::Hebrew => format!("{day} ב{month} {year}"),
    }
}

pub fn parse_date(text: &str) -> Option<NaiveDate> {
    let text = text.trim();
    [
        "%Y-%m-%d",
        "%m/%d/%Y",
        "%B %d, %Y",
        "%b %d, %Y",
        "%B %d %Y",
        "%b %d %Y",
    ]
    .into_iter()
    .find_map(|format| NaiveDate::parse_from_str(text, format).ok())
}

/// Resolves common English calendar phrases against a supplied local civil date.
pub fn parse_natural_date(text: &str, today: NaiveDate) -> Option<NaiveDate> {
    if let Some(date) = parse_date(text) {
        return Some(date);
    }
    let text = text.to_ascii_lowercase();
    let words: Vec<_> = text.split_whitespace().collect();
    match words.as_slice() {
        ["today"] => Some(today),
        ["tomorrow"] => today.succ_opt(),
        ["yesterday"] => today.pred_opt(),
        ["day", "after", "tomorrow"] | ["the", "day", "after", "tomorrow"] => {
            today.checked_add_days(Days::new(2))
        }
        ["next", "week"] => today.checked_add_days(Days::new(7)),
        ["last", "week"] => today.checked_sub_days(Days::new(7)),
        ["next", weekday] => {
            let weekday: Weekday = weekday.parse().ok()?;
            let distance =
                (weekday.num_days_from_monday() + 6 - today.weekday().num_days_from_monday()) % 7
                    + 1;
            today.checked_add_days(Days::new(distance.into()))
        }
        ["in", count, unit] | [count, unit, "from", "now"] => {
            today.checked_add_days(Days::new(days(count, unit)?))
        }
        [count, unit, "ago"] => today.checked_sub_days(Days::new(days(count, unit)?)),
        _ => None,
    }
}

fn days(count: &str, unit: &str) -> Option<u64> {
    let count: u64 = count.parse().ok()?;
    match unit {
        "day" | "days" => Some(count),
        "week" | "weeks" => count.checked_mul(7),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_input_accepts_leap_days_and_rejects_invalid_dates() {
        let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29);
        for input in [
            "2024-02-29",
            "2/29/2024",
            "February 29, 2024",
            "Feb 29 2024",
        ] {
            assert_eq!(parse_date(input), leap_day);
        }
        assert_eq!(parse_date("February 29, 2025"), None);
        assert_eq!(parse_date("2024-04-31"), None);
    }

    #[test]
    fn natural_dates_cross_months_and_keep_weekday_order() {
        let today = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        for (phrase, offset) in [
            ("Tomorrow", 1),
            ("  In  2 days ", 2),
            ("next week", 7),
            ("next Thursday", 7),
            ("next Friday", 1),
            ("2 weeks ago", -14),
            ("3 days from now", 3),
        ] {
            assert_eq!(
                parse_natural_date(phrase, today),
                today.checked_add_signed(chrono::Duration::days(offset))
            );
        }
        assert_eq!(
            parse_natural_date("in 18446744073709551615 weeks", today),
            None
        );
        assert_eq!(parse_natural_date("pick a random date", today), None);
    }
}
