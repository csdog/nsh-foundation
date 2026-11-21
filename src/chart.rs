//! Themed charts, tooltips, and legends backed by Native Shell plots.

use std::rc::Rc;

use nsh_base::{
    plot::{
        BandScale, BarCornerRadii, PlotAccessibilityValue, Scale, TickFormat, TooltipPlacement,
        bar, grid, plot, plot_axis,
    },
    theme,
};
use nsh_core::{
    element::{LayoutDirection, RowAccessor, Rows},
    prelude::*,
};

#[derive(Clone)]
pub struct ChartSeries {
    key: &'static str,
    label: SharedString,
    color: Reactive<AdaptiveColor>,
}

impl ChartSeries {
    pub fn new(
        key: &'static str,
        label: impl Into<SharedString>,
        color: impl IntoReactive<AdaptiveColor>,
    ) -> Self {
        Self {
            key,
            label: label.into(),
            color: color.into_reactive(),
        }
    }
}

/// A point's displayed label and values, in the order of the configured series.
pub struct ChartRow {
    label: SharedString,
    tick: SharedString,
    values: Vec<f64>,
}

impl ChartRow {
    pub fn new(label: impl Into<SharedString>, values: impl IntoIterator<Item = f64>) -> Self {
        let label = label.into();
        Self {
            tick: label.clone(),
            label,
            values: values.into_iter().collect(),
        }
    }

    pub fn tick(mut self, label: impl Into<SharedString>) -> Self {
        self.tick = label.into();
        self
    }
}

#[derive(Clone)]
pub struct ChartConfig(Rc<[ChartSeries]>);

impl ChartConfig {
    pub fn new(series: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self(series.into_iter().collect::<Vec<_>>().into())
    }
}

pub struct ChartContainer {
    name: &'static str,
    config: ChartConfig,
    rows: Vec<ChartRow>,
    height: f32,
    active_series: Reactive<Option<usize>>,
    show_grid: bool,
    tick_count: Option<u32>,
    tooltip: Option<ChartIndicator>,
    legend: bool,
    rtl: bool,
}

impl ChartContainer {
    pub fn new(name: &'static str, config: ChartConfig, rows: Vec<ChartRow>) -> Self {
        Self {
            name,
            config,
            rows,
            height: 200.0,
            active_series: None.into_reactive(),
            show_grid: false,
            tick_count: None,
            tooltip: None,
            legend: false,
            rtl: false,
        }
    }

    pub fn height(mut self, points: f32) -> Self {
        self.height = points.max(0.0);
        self
    }

    pub fn grid(mut self) -> Self {
        self.show_grid = true;
        self
    }

    pub fn axis(mut self, ticks: u32) -> Self {
        self.tick_count = Some(ticks.max(1));
        self
    }

    pub fn tooltip(mut self, indicator: ChartIndicator) -> Self {
        self.tooltip = Some(indicator);
        self
    }

    pub fn legend(mut self) -> Self {
        self.legend = true;
        self
    }

    pub fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }

    /// Selects a single series. None shows all configured series.
    pub fn active_series(mut self, index: impl IntoReactive<Option<usize>>) -> Self {
        self.active_series = index.into_reactive();
        self
    }

    pub fn surface(self) -> View {
        let hovered = signal(None::<u32>);
        let rows: Rc<[ChartRow]> = self.rows.into();
        let series = self.config.0.clone();
        let data = Rows::from_columns(
            rows.len(),
            series.iter().enumerate().map(|(i, s)| {
                (
                    s.key.into(),
                    rows.iter()
                        .map(|row| row.values.get(i).copied().unwrap_or(f64::NAN))
                        .collect(),
                )
            }),
        );
        let active = self.active_series;
        let series_count = series.len();
        let selected_index = computed(move || active.get().filter(|index| *index < series_count));
        let selected = move || selected_index.get();
        let direction = if self.rtl {
            LayoutDirection::Rtl
        } else {
            LayoutDirection::Ltr
        };
        let mut band = BandScale::default();
        band.padding_inner = percentage(if rows.len() > 12 { 0.18 } else { 0.25 });
        band.padding_outer = percentage(0.15);
        band.group_padding = percentage(0.12);
        band.reverse = self.rtl;

        let hover_row = rows.clone();
        let accessible = self.config.0.clone();
        let mut plot = plot()
            .data(data)
            .x_scale(Scale::Band(band))
            .accessibility_label(self.name)
            .accessibility_value(move || {
                let row = hover_row.get(hovered.get()? as usize)?;
                let mut description = PlotAccessibilityValue::default();
                let mut parts = vec![row.label.to_string()];
                for (i, series) in accessible.iter().enumerate() {
                    if selected().is_some_and(|chosen| chosen != i) {
                        continue;
                    }
                    if let Some(&value) = row.values.get(i).filter(|value| value.is_finite()) {
                        parts.push(format!("{} {}", series.label, format_value(value)));
                    }
                }
                description.text = Some(parts.join(", ").into());
                Some(description)
            })
            .on_hover_change(move |index: &Option<u32>| hovered.set(*index))
            .w_full()
            .h(px(self.height))
            .focus_visible(|style| {
                style
                    .rounded(px(4.0))
                    .ring(px(2.0))
                    .ring_color(theme().ring)
            })
            .when(self.show_grid, |plot| {
                plot.child(
                    grid()
                        .ticks(4)
                        .stroke_width(px(1.0))
                        .stroke_color(move || theme().border),
                )
            });

        let marks = self.config.0.clone();
        let radius = BarCornerRadii::all(px(if rows.len() > 12 { 1.0 } else { 4.0 }));
        // Keep the viewport and keyboard focus while changing the visible marks.
        plot = plot.child(dynamic(move || {
            let selection = selected();
            marks
                .iter()
                .enumerate()
                .filter(|(i, _)| selection.is_none_or(|index| index == *i))
                .map(|(_, series)| {
                    bar()
                        .value(RowAccessor::key(series.key))
                        .fill_color(series.color.clone())
                        .corner_radii(radius)
                })
                .collect::<Vec<_>>()
        }));
        if let Some(ticks) = self.tick_count {
            let tick_data = rows.clone();
            plot = plot.child(
                plot_axis()
                    .ticks(move || {
                        if window().width.0 < 640.0 {
                            ticks.min(6)
                        } else {
                            ticks
                        }
                    })
                    .format(TickFormat::from_fn(move |value, _| {
                        tick_data
                            .get(value as usize)
                            .map_or_else(SharedString::default, |row| row.tick.clone())
                    }))
                    .label_class("pt-2 text-xs text-muted-foreground"),
            );
        }
        if let Some(indicator) = self.tooltip {
            let marks = self.config.0.clone();
            plot = plot
                .cross_line_class("w-full bg-muted/50")
                .tooltip_placement(TooltipPlacement::Pointer)
                .tooltip_offset(px(8.0))
                .tooltip_content(dynamic(move || {
                    let row = rows.get(hovered.get()? as usize)?;
                    let entries = marks
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| selected().is_none_or(|index| index == *i))
                        .filter_map(|(i, series)| {
                            let value = row.values.get(i).filter(|v| v.is_finite())?;
                            Some(ChartTooltipRow::new(series.clone(), *value))
                        })
                        .collect::<Vec<_>>();
                    Some(
                        ChartTooltipContent::new(row.label.clone(), entries)
                            .indicator(indicator)
                            .direction(direction),
                    )
                }));
        }
        let legend = self
            .legend
            .then(|| ChartLegendContent::new(self.config).direction(direction));
        view()
            .w_full()
            .min_w(px(0.0))
            .gap_3()
            .child(
                view()
                    .when(self.tick_count.is_some(), |pane| pane.pb(px(26.0)))
                    .child(plot),
            )
            .child(legend)
    }
}

impl IntoElement for ChartContainer {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChartIndicator {
    #[default]
    Dot,
    Line,
    Dashed,
    Hidden,
}

pub struct ChartTooltipRow {
    series: ChartSeries,
    value: SharedString,
}

impl ChartTooltipRow {
    pub fn new(series: ChartSeries, value: f64) -> Self {
        Self {
            series,
            value: format_value(value).into(),
        }
    }

    pub fn formatted(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }
}

pub struct ChartTooltipContent {
    label: SharedString,
    entries: Vec<ChartTooltipRow>,
    indicator: ChartIndicator,
    hide_label: bool,
    direction: LayoutDirection,
}

impl ChartTooltipContent {
    pub fn new(label: impl Into<SharedString>, entries: Vec<ChartTooltipRow>) -> Self {
        Self {
            label: label.into(),
            entries,
            indicator: ChartIndicator::Dot,
            hide_label: false,
            direction: LayoutDirection::Inherit,
        }
    }

    pub fn indicator(mut self, indicator: ChartIndicator) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn hide_label(mut self) -> Self {
        self.hide_label = true;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn surface(self) -> View {
        let mut tooltip = view()
            .test_id("chart-tooltip-content")
            .direction(self.direction)
            .min_w(px(128.0))
            .rounded(px(12.0))
            .border(px(1.0))
            .border_color(move || theme().border.opacity(0.5))
            .bg(move || theme().background)
            .shadow_xl()
            .px(px(10.0))
            .py(px(6.0))
            .gap_1();
        if !self.hide_label {
            tooltip = tooltip.child(text(self.label).text_xs().font_weight(FontWeight::MEDIUM));
        }
        for item in self.entries {
            tooltip = tooltip.child(
                view()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(marker(item.series.color, self.indicator))
                    .child(
                        text(item.series.label)
                            .text_xs()
                            .flex_1()
                            .text_color(move || theme().muted_foreground),
                    )
                    .child(text(item.value).text_xs().font_weight(FontWeight::MEDIUM)),
            );
        }
        tooltip
    }
}

impl IntoElement for ChartTooltipContent {
    fn into_element(self) -> AnyElement {
        self.surface().into_element()
    }
}

pub struct ChartLegendContent {
    config: ChartConfig,
    direction: LayoutDirection,
}

impl ChartLegendContent {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            config,
            direction: LayoutDirection::Inherit,
        }
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl IntoElement for ChartLegendContent {
    fn into_element(self) -> AnyElement {
        let legend = view()
            .direction(self.direction)
            .flex_row()
            .flex_wrap()
            .justify_center()
            .gap_4();
        self.config
            .0
            .iter()
            .fold(legend, |legend, item| {
                legend.child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .child(marker(item.color.clone(), ChartIndicator::Dot))
                        .child(text(item.label.clone()).text_xs()),
                )
            })
            .into_element()
    }
}

fn marker(color: Reactive<AdaptiveColor>, style: ChartIndicator) -> Option<View> {
    let marker = view().h(px(10.0)).flex_shrink_0().rounded(px(2.0));
    match style {
        ChartIndicator::Dot => Some(marker.w(px(10.0)).bg(color)),
        ChartIndicator::Line => Some(marker.w(px(4.0)).bg(color)),
        ChartIndicator::Dashed => Some(
            marker
                .w(px(6.0))
                .border(px(1.0))
                .border_dashed()
                .border_color(color),
        ),
        ChartIndicator::Hidden => None,
    }
}

pub fn format_value(value: f64) -> String {
    let raw = value.to_string();
    let (sign, body) = raw
        .strip_prefix('-')
        .map_or(("", raw.as_str()), |body| ("-", body));
    let (whole, fraction) = body
        .split_once('.')
        .map_or((body, ""), |(whole, _)| (whole, &body[whole.len()..]));
    if !whole.bytes().all(|digit| digit.is_ascii_digit()) {
        return raw;
    }
    let mut formatted = String::from(sign);
    for (i, digit) in whole.char_indices() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(digit);
    }
    formatted.push_str(fraction);
    formatted
}
