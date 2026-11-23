use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent},
    chart::{
        ChartConfig, ChartContainer, ChartIndicator, ChartRow, ChartSeries, ChartTooltipContent,
        ChartTooltipRow, format_value,
    },
};
use nsh_base::theme;
use nsh_core::{AccessibilityState, prelude::*};

const MONTHLY: [(f64, f64); 6] = [
    (186.0, 80.0),
    (305.0, 200.0),
    (237.0, 120.0),
    (73.0, 190.0),
    (209.0, 130.0),
    (214.0, 140.0),
];

const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];

const DAILY: [(u32, u32); 30] = [
    (222, 150),
    (97, 180),
    (167, 120),
    (242, 260),
    (373, 290),
    (301, 340),
    (245, 180),
    (409, 320),
    (59, 110),
    (261, 190),
    (327, 350),
    (292, 210),
    (342, 380),
    (137, 220),
    (120, 170),
    (138, 190),
    (446, 360),
    (364, 410),
    (243, 180),
    (89, 150),
    (137, 200),
    (224, 170),
    (138, 230),
    (387, 290),
    (215, 250),
    (75, 130),
    (383, 420),
    (122, 180),
    (315, 240),
    (454, 380),
];

pub fn demo() -> impl IntoElement {
    let selected = signal(0);
    let rows = DAILY
        .iter()
        .enumerate()
        .map(|(index, (desktop, mobile))| {
            let day = index + 1;
            ChartRow::new(
                format!("Apr {day}, 2024"),
                [f64::from(*desktop), f64::from(*mobile)],
            )
            .tick(format!("Apr {day}"))
        })
        .collect();
    let totals = [
        DAILY.iter().map(|v| v.0).sum::<u32>(),
        DAILY.iter().map(|v| v.1).sum::<u32>(),
    ];
    let mut controls = view().flex_row().min_w(px(0.0));
    for (index, name) in ["Desktop", "Mobile"].into_iter().enumerate() {
        controls = controls.child(
            view()
                .role(AccessibilityRole::Button)
                .label(name)
                .focusable(true)
                .flex_1()
                .min_w(px(0.0))
                .px_4()
                .py_3()
                .gap_1()
                .border_l(px(1.0))
                .border_color(move || theme().border)
                .style_when(
                    move || selected.get() == index,
                    |style| style.bg(theme().muted.opacity(0.5)),
                )
                .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
                .accessibility_state(move || AccessibilityState {
                    selected: selected.get() == index,
                    ..Default::default()
                })
                .on_press(move || selected.set(index))
                .child(
                    text(name)
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                )
                .child(
                    text(format_value(f64::from(totals[index])))
                        .text_xl()
                        .font_weight(FontWeight::BOLD),
                ),
        );
    }
    let visitors = ChartContainer::new(
        "Daily page views",
        config("Page Views", "Page Views", true),
        rows,
    )
    .active_series(move || Some(selected.get()))
    .height(224.0)
    .grid()
    .axis(9)
    .tooltip(ChartIndicator::Dot);
    Card::new().spacing(0.0).content(CardContent::new(
        view()
            .child(
                view()
                    .border_b(px(1.0))
                    .border_color(move || theme().border)
                    .style_when(move || window().width.0 >= 640.0, |style| style.flex_row())
                    .child(
                        view()
                            .flex_1()
                            .p_4()
                            .gap_1()
                            .child(
                                text("Bar Chart - Interactive")
                                    .text_base()
                                    .font_weight(FontWeight::MEDIUM),
                            )
                            .child(
                                text("April 2024 · 30 days")
                                    .text_sm()
                                    .text_color(move || theme().muted_foreground),
                            ),
                    )
                    .child(controls),
            )
            .child(view().px_3().pt_4().child(visitors)),
    ))
}

pub fn basic() -> impl IntoElement {
    monthly("Monthly visitors", MONTHS, "Desktop", "Mobile", false)
}

pub fn with_grid() -> impl IntoElement {
    monthly(
        "Monthly visitors with grid",
        MONTHS,
        "Desktop",
        "Mobile",
        false,
    )
    .grid()
}

pub fn with_axis() -> impl IntoElement {
    monthly(
        "Monthly visitors with axis",
        MONTHS,
        "Desktop",
        "Mobile",
        false,
    )
    .grid()
    .axis(6)
}

pub fn with_tooltip() -> impl IntoElement {
    monthly(
        "Monthly visitors with tooltip",
        MONTHS,
        "Desktop",
        "Mobile",
        false,
    )
    .grid()
    .axis(6)
    .tooltip(ChartIndicator::Dot)
}

pub fn with_legend() -> impl IntoElement {
    monthly(
        "Monthly visitors with legend",
        MONTHS,
        "Desktop",
        "Mobile",
        false,
    )
    .grid()
    .axis(6)
    .tooltip(ChartIndicator::Dot)
    .legend()
}

pub fn tooltip_styles() -> impl IntoElement {
    let group = |label, content| {
        view()
            .items_start()
            .gap_2()
            .p_4()
            .child(text(label).text_sm())
            .child(content)
    };
    view()
        .w_full()
        .max_w(px(460.0))
        .justify_center()
        .flex_row()
        .flex_wrap()
        .child(group(
            "Label",
            ChartTooltipContent::new(
                "Page Views",
                vec![
                    ChartTooltipRow::new(
                        ChartSeries::new("desktop", "Desktop", || theme().chart_1),
                        186.0,
                    ),
                    ChartTooltipRow::new(
                        ChartSeries::new("mobile", "Mobile", || theme().chart_2),
                        80.0,
                    ),
                ],
            ),
        ))
        .child(group(
            "Name",
            ChartTooltipContent::new(
                "Browser",
                vec![
                    ChartTooltipRow::new(
                        ChartSeries::new("chrome", "Chrome", || theme().chart_3),
                        1286.0,
                    ),
                    ChartTooltipRow::new(
                        ChartSeries::new("firefox", "Firefox", || theme().chart_4),
                        1000.0,
                    ),
                ],
            )
            .hide_label()
            .indicator(ChartIndicator::Dashed),
        ))
        .child(group(
            "Line",
            ChartTooltipContent::new(
                "Page Views",
                vec![ChartTooltipRow::new(
                    ChartSeries::new("desktop", "Desktop", || theme().chart_3),
                    12486.0,
                )],
            )
            .indicator(ChartIndicator::Line),
        ))
        .child(group(
            "Indicator",
            ChartTooltipContent::new(
                "Browser",
                vec![ChartTooltipRow::new(
                    ChartSeries::new("chrome", "Chrome", || theme().chart_1),
                    1286.0,
                )],
            )
            .hide_label(),
        ))
}

pub fn rtl() -> impl IntoElement {
    let selection = signal(1);
    let mut options = view().flex_row().flex_wrap().justify_center().gap_1();
    for (index, name) in ["English", "العربية", "עברית"].into_iter().enumerate() {
        options = options.child(
            Button::new(name)
                .size(ButtonSize::Small)
                .variant(ButtonVariant::Outline)
                .on_press(move || selection.set(index))
                .style_when(
                    move || selection.get() == index,
                    |style| style.bg(theme().muted),
                )
                .accessibility_state(move || AccessibilityState {
                    selected: selection.get() == index,
                    ..Default::default()
                }),
        );
    }
    view()
        .w_full()
        .gap_4()
        .child(options)
        .child(dynamic(move || {
            let language = selection.get();
            let (months, desktop, mobile) = match language {
                1 => (
                    ["يناير", "فبراير", "مارس", "أبريل", "مايو", "يونيو"],
                    "سطح المكتب",
                    "الجوال",
                ),
                2 => (
                    ["ינואר", "פברואר", "מרץ", "אפריל", "מאי", "יוני"],
                    "מחשב",
                    "נייד",
                ),
                _ => (MONTHS, "Desktop", "Mobile"),
            };
            monthly("Localized monthly visitors", months, desktop, mobile, true)
                .rtl(language != 0)
                .grid()
                .axis(6)
                .tooltip(ChartIndicator::Dot)
                .legend()
        }))
}

fn monthly(
    title: &'static str,
    months: [&'static str; 6],
    desktop: &'static str,
    mobile: &'static str,
    themed: bool,
) -> ChartContainer {
    let rows = months
        .into_iter()
        .zip(MONTHLY)
        .map(|(month, (desktop, mobile))| {
            ChartRow::new(month, [desktop, mobile]).tick(month.chars().take(3).collect::<String>())
        })
        .collect();
    ChartContainer::new(title, config(desktop, mobile, themed), rows)
}

fn config(desktop: &'static str, mobile: &'static str, themed: bool) -> ChartConfig {
    ChartConfig::new([
        ChartSeries::new("desktop", desktop, move || {
            if themed {
                theme().chart_2
            } else {
                rgb(0x2563eb).into()
            }
        }),
        ChartSeries::new("mobile", mobile, move || {
            if themed {
                theme().chart_1
            } else {
                rgb(0x60a5fa).into()
            }
        }),
    ])
}
