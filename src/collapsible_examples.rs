use nsh_base::{input, theme};
use nsh_core::{AccessibilityState, KeyboardType, element::LayoutDirection, prelude::*};
use nsh_foundation::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardSize, CardTitle},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
};

const ENGLISH: [&str; 8] = [
    "Order #4189",
    "Toggle details",
    "Status",
    "Shipped",
    "Shipping address",
    "100 Market St, San Francisco",
    "Items",
    "2x Studio Headphones",
];

pub fn demo() -> impl IntoElement {
    order(ENGLISH, LayoutDirection::Ltr)
}

fn order(labels: [&'static str; 8], direction: LayoutDirection) -> View {
    let [
        number,
        toggle,
        status,
        shipped,
        address_label,
        address,
        items,
        items_detail,
    ] = labels;
    let open = signal(false);
    let trigger = CollapsibleTrigger::render(move |open| {
        Button::new(toggle)
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Icon)
            .icon_start(
                text(move || SharedString::from(if open.get() { "⌃" } else { "⌄" })).text_lg(),
            )
            .surface()
    });
    let details = view()
        .gap_2()
        .pt_2()
        .child(order_detail(address_label, address, direction))
        .child(order_detail(items, items_detail, direction));
    Collapsible::new(trigger, CollapsibleContent::new(details))
        .open(open)
        .on_open_change(move |value| open.set(value))
        .header(move |trigger| {
            view()
                .gap_2()
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_4()
                        .gap_4()
                        .child(text(number).text_sm().font_weight(FontWeight::SEMIBOLD))
                        .child(trigger),
                )
                .child(
                    order_box()
                        .flex_row()
                        .justify_between()
                        .child(
                            text(status)
                                .text_sm()
                                .text_color(move || theme().muted_foreground),
                        )
                        .child(text(shipped).text_sm().font_weight(FontWeight::MEDIUM)),
                )
        })
        .surface()
        .w_full()
        .max_w(px(350.0))
        .direction(direction)
}

fn order_box() -> View {
    view()
        .px_4()
        .py_2()
        .rounded(px(8.0))
        .border(px(1.0))
        .border_color(move || theme().border)
}

fn order_detail(title: &'static str, message: &'static str, direction: LayoutDirection) -> View {
    let rtl = direction == LayoutDirection::Rtl;
    order_box()
        .gap_1()
        .child(
            text(title)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .when(rtl, |text| text.text_right()),
        )
        .child(
            text(message)
                .text_sm()
                .text_color(move || theme().muted_foreground)
                .when(rtl, |text| text.text_right()),
        )
}

pub fn basic() -> impl IntoElement {
    let feedback = signal(false);
    let trigger = CollapsibleTrigger::render(|open| {
        Button::new("Product details")
            .variant(ButtonVariant::Ghost)
            .icon_end(text(move || {
                SharedString::from(if open.get() { "⌃" } else { "⌄" })
            }))
            .surface()
            .justify_between()
            .style_when(move || open.get(), |style| style.bg(theme().muted))
    });
    let content = view()
        .items_start()
        .gap_2()
        .px(px(10.0))
        .pb(px(10.0))
        .bg(move || theme().muted)
        .child(
            text("This panel can be expanded or collapsed to reveal additional content.").text_sm(),
        )
        .child(
            Button::new("Learn More")
                .size(ButtonSize::ExtraSmall)
                .on_press(move || feedback.set(true)),
        )
        .child(when(feedback, || {
            text("Product details are available in this panel.")
                .text_sm()
                .role(AccessibilityRole::Summary)
        }));
    Card::new()
        .content(CardContent::new(Collapsible::new(
            trigger,
            CollapsibleContent::new(content),
        )))
        .surface()
        .max_w(px(384.0))
}

pub fn settings() -> impl IntoElement {
    let open = signal(false);
    let trigger = CollapsibleTrigger::render(|open| {
        Button::new("More corner radii")
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Icon)
            .icon_start(
                text(move || SharedString::from(if open.get() { "−" } else { "+" })).text_lg(),
            )
            .surface()
    });
    let pair = |left, right| {
        view()
            .flex_row()
            .gap_2()
            .child(radius(left))
            .child(radius(right))
    };
    let panel = pair("Bottom left radius", "Bottom right radius")
        .pt_2()
        .pr(px(40.0));
    let disclosure = Collapsible::new(trigger, CollapsibleContent::new(panel))
        .open(open)
        .on_open_change(move |next| open.set(next))
        .header(move |trigger| {
            view()
                .flex_row()
                .gap_2()
                .items_center()
                .child(
                    pair("Top left radius", "Top right radius")
                        .flex_1()
                        .min_w(px(0.0)),
                )
                .child(trigger)
        });
    Card::new()
        .size(CardSize::Small)
        .header(
            CardHeader::new()
                .title(CardTitle::new("Radius"))
                .description(CardDescription::new(
                    "Set the corner radius of the element.",
                )),
        )
        .content(CardContent::new(disclosure))
        .surface()
        .max_w(px(320.0))
}

fn radius(label: &'static str) -> impl IntoElement {
    let value = signal("0".to_owned());
    input()
        .label(label)
        .keyboard_type(KeyboardType::Decimal)
        .placeholder("0")
        .value(move || value.get())
        .on_change(move |next: &str| value.set(next.to_owned()))
        .flex_1()
        .min_w(px(0.0))
        .h(px(32.0))
        .px_2()
        .rounded(px(12.0))
        .border(px(1.0))
        .border_color(move || theme().input)
        .text_color(move || theme().foreground)
}

#[derive(Clone, Copy)]
enum File {
    Directory(&'static str, &'static [File]),
    Leaf(&'static str),
}

use File::{Directory, Leaf};

const TREE: &[File] = &[
    Directory(
        "components",
        &[
            Directory(
                "ui",
                &[
                    Leaf("button.tsx"),
                    Leaf("card.tsx"),
                    Leaf("dialog.tsx"),
                    Leaf("input.tsx"),
                    Leaf("select.tsx"),
                    Leaf("table.tsx"),
                ],
            ),
            Leaf("login-form.tsx"),
            Leaf("register-form.tsx"),
        ],
    ),
    Directory("lib", &[Leaf("utils.ts"), Leaf("cn.ts"), Leaf("api.ts")]),
    Directory(
        "hooks",
        &[
            Leaf("use-media-query.ts"),
            Leaf("use-debounce.ts"),
            Leaf("use-local-storage.ts"),
        ],
    ),
    Directory("types", &[Leaf("index.d.ts"), Leaf("api.d.ts")]),
    Directory(
        "public",
        &[Leaf("favicon.ico"), Leaf("logo.svg"), Leaf("images")],
    ),
    Leaf("app.tsx"),
    Leaf("layout.tsx"),
    Leaf("globals.css"),
    Leaf("package.json"),
    Leaf("tsconfig.json"),
    Leaf("README.md"),
    Leaf(".gitignore"),
];

pub fn file_tree() -> impl IntoElement {
    let selected = signal("");
    let tab = signal("Explorer");
    let tabs = ["Explorer", "Outline"].map(|label| {
        Button::new(label)
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Small)
            .on_press(move || tab.set(label))
            .role(AccessibilityRole::Tab)
            .accessibility_state(move || AccessibilityState {
                selected: tab.get() == label,
                ..Default::default()
            })
            .style_when(
                move || tab.get() == label,
                |style| style.bg(theme().background),
            )
            .flex_1()
    });
    Card::new()
        .size(CardSize::Small)
        .spacing(8.0)
        .content(CardContent::new(
            view()
                .gap_2()
                .child(
                    view()
                        .role(AccessibilityRole::Tablist)
                        .label("Files")
                        .flex_row()
                        .gap_1()
                        .rounded(px(12.0))
                        .p_1()
                        .bg(move || theme().muted)
                        .child(tabs),
                )
                .child(entries(TREE, selected)),
        ))
        .surface()
        .max_w(px(256.0))
}

fn entries(files: &'static [File], selected: Signal<&'static str>) -> View {
    let mut list = view().gap_1();
    for &file in files {
        list = match file {
            Directory(name, children) => {
                let trigger = CollapsibleTrigger::render(move |open| labeled_trigger(name, open));
                list.child(Collapsible::new(
                    trigger,
                    CollapsibleContent::new(entries(children, selected).pl(px(16.0)).pt_1()),
                ))
            }
            Leaf(name) => list.child(
                Button::new(name)
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small)
                    .icon_start(text("▤").text_color(move || theme().muted_foreground))
                    .on_press(move || selected.set(name))
                    .justify_start()
                    .accessibility_state(move || AccessibilityState {
                        selected: selected.get() == name,
                        ..Default::default()
                    })
                    .style_when(
                        move || selected.get() == name,
                        |style| style.bg(theme().muted),
                    ),
            ),
        };
    }
    list
}

fn labeled_trigger(label: &'static str, open: Memo<bool>) -> View {
    view()
        .label(label)
        .flex_row()
        .items_center()
        .gap_2()
        .px_2()
        .h(px(28.0))
        .rounded(px(8.0))
        .hover(|style| style.bg(theme().muted))
        .focus_visible(|style| style.ring(px(2.0)).ring_color(theme().ring))
        .child(text(move || SharedString::from(if open.get() { "⌄" } else { "›" })).text_base())
        .child(text(label).text_sm())
}

pub fn rtl() -> impl IntoElement {
    let selection = signal(1);
    let controls = ["English", "العربية", "עברית"]
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            Button::new(label)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || selection.set(index))
                .accessibility_state(move || AccessibilityState {
                    selected: selection.get() == index,
                    ..Default::default()
                })
                .style_when(
                    move || selection.get() == index,
                    |style| style.bg(theme().muted),
                )
        })
        .collect::<Vec<_>>();
    view()
        .w_full()
        .items_center()
        .gap_3()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .child(controls),
        )
        .child(dynamic(move || match selection.get() {
            1 => order(
                [
                    "الطلب #4189",
                    "تبديل التفاصيل",
                    "الحالة",
                    "تم الشحن",
                    "عنوان الشحن",
                    "100 Market St, San Francisco",
                    "العناصر",
                    "2x سماعات الاستوديو",
                ],
                LayoutDirection::Rtl,
            ),
            2 => order(
                [
                    "הזמנה #4189",
                    "הצג פרטים",
                    "סטטוס",
                    "נשלח",
                    "כתובת משלוח",
                    "100 Market St, San Francisco",
                    "פריטים",
                    "2x אוזניות אולפן",
                ],
                LayoutDirection::Rtl,
            ),
            _ => order(ENGLISH, LayoutDirection::Ltr),
        }))
}
