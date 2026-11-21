use chrono::{Datelike, NaiveTime};
use nsh_base::theme;
use nsh_core::{element::input, prelude::*};
use nsh_foundation::calendar::{Calendar, Date, date_this_year, local_today};

pub fn preview() -> impl IntoElement {
    let date = signal(Date::from(date_this_year(local_today().month(), 12)));
    let start = signal(String::from("10:30:00"));
    let end = signal(String::from("12:30:00"));
    let field = |label: &'static str, value: Signal<String>| {
        view().gap_1().child(text(label).text_sm()).child(
            input()
                .label(label)
                .value(value)
                .w(px(136.0))
                .h(px(32.0))
                .px_2()
                .rounded(px(12.0))
                .border(px(1.0))
                .border_color(move || theme().border)
                .bg(move || theme().input.opacity(0.5))
                .text_color(move || theme().foreground)
                .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                .on_change(move |text: &str| value.set(text.to_owned())),
        )
    };
    let parsed = |time: &str| {
        NaiveTime::parse_from_str(time, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(time, "%H:%M"))
    };
    view()
        .items_center()
        .gap_3()
        .child(Calendar::new().value(date))
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap_2()
                .child(field("Start Time", start))
                .child(field("End Time", end)),
        )
        .child(
            text(move || {
                let (Ok(start), Ok(end)) = (parsed(&start.get()), parsed(&end.get())) else {
                    return "Enter times as HH:MM or HH:MM:SS.".to_owned();
                };
                if end < start {
                    return "End Time must be after Start Time.".to_owned();
                }
                format!(
                    "{} · {start}–{end}",
                    date.get().format("%Y-%m-%d").unwrap_or_default()
                )
            })
            .text_xs(),
        )
}
