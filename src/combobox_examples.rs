use nsh_base::theme;
use nsh_core::{AccessibilityState, element::LayoutDirection, prelude::*};
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    combobox::{Combobox, ComboboxItem},
};

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

fn frameworks() -> Combobox {
    Combobox::new(
        "Framework",
        FRAMEWORKS.map(|word| ComboboxItem::new(word, word)),
    )
    .placeholder("Select a framework")
}

pub fn demo() -> impl IntoElement {
    frameworks()
}
pub fn basic() -> impl IntoElement {
    frameworks()
}

pub fn multiple() -> impl IntoElement {
    frameworks()
        .multiple(true)
        .placeholder("")
        .default_value(["Next.js"])
}

pub fn clear() -> impl IntoElement {
    frameworks().show_clear(true).default_value(["Next.js"])
}

pub fn invalid() -> impl IntoElement {
    frameworks()
        .invalid(true)
        .surface()
        .test_id("invalid-combobox")
}

pub fn disabled() -> impl IntoElement {
    frameworks().disabled(true)
}

pub fn auto_highlight() -> impl IntoElement {
    frameworks()
}

const TIMEZONES: [(&str, [&str; 6]); 3] = [
    (
        "Americas",
        [
            "(GMT-5) New York",
            "(GMT-8) Los Angeles",
            "(GMT-6) Chicago",
            "(GMT-5) Toronto",
            "(GMT-8) Vancouver",
            "(GMT-3) São Paulo",
        ],
    ),
    (
        "Europe",
        [
            "(GMT+0) London",
            "(GMT+1) Paris",
            "(GMT+1) Berlin",
            "(GMT+1) Rome",
            "(GMT+1) Madrid",
            "(GMT+1) Amsterdam",
        ],
    ),
    (
        "Asia/Pacific",
        [
            "(GMT+9) Tokyo",
            "(GMT+8) Shanghai",
            "(GMT+8) Singapore",
            "(GMT+4) Dubai",
            "(GMT+11) Sydney",
            "(GMT+9) Seoul",
        ],
    ),
];

fn timezones() -> Combobox {
    Combobox::new(
        "Timezone",
        TIMEZONES.into_iter().flat_map(|(group, names)| {
            names.map(|name| ComboboxItem::new(name, name).group(group))
        }),
    )
    .placeholder("Select a timezone")
    .empty("No timezones found.")
}

pub fn groups() -> impl IntoElement {
    timezones()
}

pub fn input_group() -> impl IntoElement {
    timezones().prefix(
        view().size(px(18.0)).items_center().justify_center().child(
            text("◎")
                .text_base()
                .text_color(move || theme().muted_foreground),
        ),
    )
}

pub fn custom() -> impl IntoElement {
    Combobox::new("Country", countries())
        .placeholder("Search countries...")
        .empty("No countries found.")
        .render_item(|item| {
            let [code, _, _, continent] = COUNTRIES[item.index as usize];
            view()
                .min_w(px(0.0))
                .gap_1()
                .child(
                    text(item.title.clone())
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM),
                )
                .child(
                    text(format!("{continent} ({code})"))
                        .text_xs()
                        .text_color(move || theme().muted_foreground),
                )
        })
}

pub fn popup() -> impl IntoElement {
    Combobox::new("Select country", countries())
        .placeholder("Select country")
        .popup()
}

fn countries() -> impl Iterator<Item = ComboboxItem> {
    COUNTRIES
        .iter()
        .map(|&[_, key, title, _]| ComboboxItem::new(key, title))
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1);
    let values = signal(vec![SharedString::from("technology")]);
    let controls = ["English", "العربية", "עברית"]
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || language.set(index))
                .accessibility_state(move || AccessibilityState {
                    selected: language.get() == index,
                    ..Default::default()
                })
                .style_when(
                    move || language.get() == index,
                    |style| style.bg(theme().muted),
                )
        })
        .collect::<Vec<_>>();
    view()
        .w_full()
        .max_w(px(320.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .justify_center()
                .flex_wrap()
                .gap_1()
                .child(controls),
        )
        .child(dynamic(move || {
            let language = language.get();
            let (label, placeholder, empty, labels) = match language {
                1 => (
                    "الفئات",
                    "أضف فئات",
                    "لم يتم العثور على فئات.",
                    [
                        "التكنولوجيا",
                        "التصميم",
                        "الأعمال",
                        "التسويق",
                        "التعليم",
                        "الصحة",
                    ],
                ),
                2 => (
                    "קטגוריות",
                    "הוסף קטגוריות",
                    "לא נמצאו קטגוריות.",
                    ["טכנולוגיה", "עיצוב", "עסקים", "שיווק", "חינוך", "בריאות"],
                ),
                _ => (
                    "Categories",
                    "Add categories",
                    "No categories found.",
                    [
                        "Technology",
                        "Design",
                        "Business",
                        "Marketing",
                        "Education",
                        "Health",
                    ],
                ),
            };
            let direction = if language == 0 {
                LayoutDirection::Ltr
            } else {
                LayoutDirection::Rtl
            };
            let choices = [
                "technology",
                "design",
                "business",
                "marketing",
                "education",
                "health",
            ]
            .into_iter()
            .zip(labels)
            .map(|(key, title)| ComboboxItem::new(key, title));
            view()
                .direction(direction)
                .gap_2()
                .child(
                    text(label)
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .when(language != 0, |label| label.text_right()),
                )
                .child(
                    Combobox::new(label, choices)
                        .placeholder(placeholder)
                        .empty(empty)
                        .multiple(true)
                        .value(values)
                        .direction(direction),
                )
        }))
}

const COUNTRIES: &[[&str; 4]] = &[
    ["ar", "argentina", "Argentina", "South America"],
    ["au", "australia", "Australia", "Oceania"],
    ["br", "brazil", "Brazil", "South America"],
    ["ca", "canada", "Canada", "North America"],
    ["cn", "china", "China", "Asia"],
    ["co", "colombia", "Colombia", "South America"],
    ["eg", "egypt", "Egypt", "Africa"],
    ["fr", "france", "France", "Europe"],
    ["de", "germany", "Germany", "Europe"],
    ["it", "italy", "Italy", "Europe"],
    ["jp", "japan", "Japan", "Asia"],
    ["ke", "kenya", "Kenya", "Africa"],
    ["mx", "mexico", "Mexico", "North America"],
    ["nz", "new-zealand", "New Zealand", "Oceania"],
    ["ng", "nigeria", "Nigeria", "Africa"],
    ["za", "south-africa", "South Africa", "Africa"],
    ["kr", "south-korea", "South Korea", "Asia"],
    ["gb", "united-kingdom", "United Kingdom", "Europe"],
    ["us", "united-states", "United States", "North America"],
];
