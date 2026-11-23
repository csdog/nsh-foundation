use std::rc::Rc;

use chrono::{Datelike, Days, NaiveDate};
use icu_calendar::{
    Date as IcuDate,
    cal::{Iso, Persian},
};
use nsh_base::theme;
use nsh_core::{keyboard::KeyCapture, keymap::Keystroke, prelude::*};

use super::{
    ARABIC_MONTHS, ARABIC_WEEKDAYS, Calendar, Date, MONTHS, WEEKDAYS, digits, local_today,
};

const PERSIAN_MONTHS: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];
const PERSIAN_WEEKDAYS: [&str; 7] = ["ی", "د", "س", "چ", "پ", "ج", "ش"];

#[derive(Clone, Copy)]
struct Month {
    year: i32,
    index: u32,
}

impl Month {
    fn of(date: NaiveDate, persian: bool) -> Self {
        if persian && let Some(date) = to_persian(date) {
            Self {
                year: date.year().extended_year(),
                index: date.month().ordinal.into(),
            }
        } else {
            Self {
                year: date.year(),
                index: date.month(),
            }
        }
    }

    fn start(self, persian: bool) -> Option<NaiveDate> {
        if persian {
            let date = IcuDate::try_new_persian(self.year, self.index as u8, 1).ok()?;
            let iso = date.to_calendar(Iso);
            NaiveDate::from_ymd_opt(
                iso.year().extended_year(),
                iso.month().ordinal.into(),
                iso.day_of_month().0.into(),
            )
        } else {
            NaiveDate::from_ymd_opt(self.year, self.index, 1)
        }
    }

    fn offset(self, delta: i32) -> Self {
        let index = self.index as i32 - 1 + delta;
        Self {
            year: self.year + index.div_euclid(12),
            index: (index.rem_euclid(12) + 1) as u32,
        }
    }
}

fn to_persian(date: NaiveDate) -> Option<IcuDate<Persian>> {
    let iso = IcuDate::try_new_iso(date.year(), date.month() as u8, date.day() as u8).ok()?;
    Some(iso.to_calendar(Persian::new()))
}

impl Calendar {
    pub(super) fn grid(self, value: Signal<Date>) -> View {
        let today = local_today();
        let persian = self.persian;
        let initial = value.get_untracked().start().unwrap_or(today);
        let page = signal(
            Month::of(initial, persian)
                .start(persian)
                .unwrap_or(initial),
        );
        effect(move || {
            if let Some(month) = value
                .get()
                .start()
                .and_then(|date| Month::of(date, persian).start(persian))
            {
                page.set(month);
            }
        });
        let options = Rc::new(self);
        let rtl = options.rtl || persian;
        let number = options.months;
        let nav = move |delta: i32, label: &'static str, glyph: &'static str| {
            view()
                .role(AccessibilityRole::Button)
                .label(label)
                .w(px(30.0))
                .h(px(30.0))
                .rounded(px(12.0))
                .items_center()
                .justify_center()
                .hover(|s| s.bg(theme().muted))
                .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                .on_press(move || {
                    if let Some(month) = Month::of(page.get_untracked(), persian)
                        .offset(delta)
                        .start(persian)
                    {
                        page.set(month);
                    }
                })
                .child(text(glyph).text_sm())
        };
        view()
            .gap_2()
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(nav(-1, "Previous month", if rtl { "›" } else { "‹" }))
                    .child(nav(1, "Next month", if rtl { "‹" } else { "›" })),
            )
            .child(dynamic(move || {
                let month = Month::of(page.get(), persian);
                let mut body = view()
                    .gap_3()
                    .style_when(|| window().width.0 >= 720.0, |s| s.flex_row());
                for offset in 0..number {
                    body = body.child(month_grid(
                        month.offset(offset as i32),
                        options.clone(),
                        value,
                        today,
                    ));
                }
                body
            }))
    }
}

fn month_grid(month: Month, options: Rc<Calendar>, value: Signal<Date>, today: NaiveDate) -> View {
    let Some(start) = month.start(options.persian) else {
        return view();
    };
    let Some(end) = month.offset(1).start(options.persian) else {
        return view();
    };
    let start_offset = (start.weekday().num_days_from_sunday() + 7 - options.first_day) % 7;
    let Some(first) = start.checked_sub_days(Days::new(start_offset.into())) else {
        return view();
    };
    let days = ((end - first).num_days() + 6) / 7 * 7;
    let dates = Rc::new(
        (0..days)
            .filter_map(|index| first.checked_add_days(Days::new(index as u64)))
            .collect::<Vec<_>>(),
    );
    let refs = Rc::new(dates.iter().map(|_| element_ref()).collect::<Vec<_>>());
    let cursor = signal(
        value
            .get_untracked()
            .start()
            .filter(|day| *day >= start && *day < end)
            .unwrap_or(start),
    );
    let keys = KeyCapture::keys(
        ["left", "right", "up", "down", "home", "end"]
            .into_iter()
            .filter_map(|key| Keystroke::parse(key, os()).ok()),
    );
    let rtl = options.rtl || options.persian;
    let cell = options.cell_size.clone();
    let columns = if options.week_numbers { 8.0 } else { 7.0 };
    let width = move || {
        px(cell
            .get()
            .min(((window().width.0 - 104.0) / columns).max(24.0)))
    };
    let weekdays = if options.persian {
        PERSIAN_WEEKDAYS
    } else if options.rtl {
        ARABIC_WEEKDAYS
    } else {
        WEEKDAYS
    };
    let months = if options.persian {
        PERSIAN_MONTHS
    } else if options.rtl {
        ARABIC_MONTHS
    } else {
        MONTHS
    };
    let year = formatted(month.year.to_string(), &options);
    let heading = format!("{} {}", months[month.index as usize - 1], year);
    let mut header = view().flex_row().items_center();
    if options.week_numbers {
        header = header.child(
            view()
                .w(width.clone())
                .items_center()
                .child(text("Wk").text_xs()),
        );
    }
    for offset in 0..7 {
        header = header.child(
            view()
                .w(width.clone())
                .items_center()
                .child(text(weekdays[((options.first_day + offset) % 7) as usize]).text_xs()),
        );
    }
    let mut panel = view()
        .role(AccessibilityRole::Group)
        .label(heading.clone())
        .gap_1()
        .child(
            text(heading)
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_center(),
        )
        .child(header);

    for (week_index, days) in dates.chunks(7).enumerate() {
        let mut row = view().flex_row().items_center();
        if options.week_numbers {
            let number = week_number(days[0]);
            row = row.child(
                view()
                    .w(width.clone())
                    .items_center()
                    .label(format!("Week {number}"))
                    .child(
                        text(format!("{number}"))
                            .text_xs()
                            .text_color(move || theme().muted_foreground),
                    ),
            );
        }
        for (day_index, &day) in days.iter().enumerate() {
            let index = week_index * 7 + day_index;
            let disabled = options
                .disabled
                .as_ref()
                .and_then(|m| m.matched(&day))
                .unwrap_or(false);
            let outside = day < start || day >= end;
            let refs_for_key = refs.clone();
            let dates_for_key = dates.clone();
            let options_for_key = options.clone();
            let change = options.change.clone();
            let range = options.range;
            let display = if options.persian {
                to_persian(day).map_or(day.day(), |date| date.day_of_month().0.into())
            } else {
                day.day()
            };
            let selected = move || {
                let current = value.get();
                current.is_active(&day) || current.is_in_range(&day)
            };
            row = row.child(
                view()
                    .role(AccessibilityRole::Button)
                    .label(day.to_string())
                    .element_ref(&refs[index])
                    .w(width.clone())
                    .h(width.clone())
                    .rounded(px(12.0))
                    .items_center()
                    .justify_center()
                    .disabled(disabled)
                    .disabled_style(|s| s.opacity(0.4))
                    .focusable(move || cursor.get() == day)
                    .accessibility_state(move || AccessibilityState {
                        selected: selected(),
                        ..Default::default()
                    })
                    .text_color(move || {
                        if selected() {
                            theme().primary_foreground
                        } else if outside {
                            theme().muted_foreground
                        } else {
                            theme().foreground
                        }
                    })
                    .style_when(selected, |s| s.bg(theme().primary))
                    .style_when(
                        move || day == today,
                        |s| s.border(px(1.0)).border_color(theme().border),
                    )
                    .focus_visible(|s| s.ring(px(2.0)).ring_color(theme().ring))
                    .capture_keys(keys.clone())
                    .on_key_down(move |key: &KeyEvent| {
                        if !key.default_prevented {
                            return;
                        }
                        let delta = match key.key.as_ref() {
                            "ArrowLeft" => {
                                if rtl {
                                    1
                                } else {
                                    -1
                                }
                            }
                            "ArrowRight" => {
                                if rtl {
                                    -1
                                } else {
                                    1
                                }
                            }
                            "ArrowUp" => -7,
                            "ArrowDown" => 7,
                            "Home" => -(day_index as isize),
                            "End" => 6 - day_index as isize,
                            _ => return,
                        };
                        let mut target = index as isize + delta;
                        while let Some(&date) = dates_for_key.get(target as usize) {
                            if !options_for_key
                                .disabled
                                .as_ref()
                                .and_then(|m| m.matched(&date))
                                .unwrap_or(false)
                            {
                                cursor.set(date);
                                refs_for_key[target as usize].focus();
                                break;
                            }
                            target += if delta == 0 { 1 } else { delta.signum() };
                        }
                    })
                    .on_press(move || {
                        let picked = match (range, value.get_untracked()) {
                            (true, Date::Range(Some(start), None)) if start <= day => {
                                Date::Range(Some(start), Some(day))
                            }
                            (true, _) => Date::Range(Some(day), None),
                            (false, _) => day.into(),
                        };
                        cursor.set(day);
                        value.set(picked);
                        if let Some(change) = &change {
                            change(picked);
                        }
                    })
                    .child(text(formatted(display.to_string(), &options)).text_sm()),
            );
        }
        panel = panel.child(row);
    }
    panel
}

fn formatted(value: String, options: &Calendar) -> String {
    if options.persian {
        digits(value, "۰۱۲۳۴۵۶۷۸۹")
    } else if options.rtl {
        digits(value, "٠١٢٣٤٥٦٧٨٩")
    } else {
        value
    }
}

// A Sunday-start week belongs to the year of its last day.
fn week_number(first: NaiveDate) -> u32 {
    let last = first.checked_add_days(Days::new(6)).unwrap_or(first);
    (last.ordinal() - 1) / 7 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persian_month_boundary_round_trips() {
        let date = NaiveDate::from_ymd_opt(2025, 6, 12).unwrap();
        let persian = to_persian(date).unwrap();
        assert_eq!(
            (
                persian.year().extended_year(),
                persian.month().ordinal,
                persian.day_of_month().0
            ),
            (1404, 3, 22)
        );
        assert_eq!(
            Month::of(date, true).offset(1).start(true),
            NaiveDate::from_ymd_opt(2025, 6, 22)
        );
    }

    #[test]
    fn year_boundary_is_week_one() {
        assert_eq!(
            week_number(NaiveDate::from_ymd_opt(2025, 12, 28).unwrap()),
            1
        );
    }
}
