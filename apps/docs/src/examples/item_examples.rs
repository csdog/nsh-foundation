use nsh_base::{menu_item, theme};
use nsh_core::{
    element::{Image, ImageAsset, ImageSrc},
    prelude::*,
};

use components::{
    aspect_ratio::AspectRatio,
    avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage, AvatarSize},
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuTrigger},
    item::{
        Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia,
        ItemMediaVariant, ItemSize, ItemTitle, ItemVariant,
    },
    ui::muted,
};

const DOCUMENTATION: &str = "https://ui.shadcn.com/docs/components/base/item";
const SOURCE: &str = "https://github.com/shadcn-ui/ui";

fn stage(content: impl IntoElement) -> View {
    view().w_full().max_w(px(448.0)).child(content)
}

fn content(title: &'static str, description: &'static str) -> ItemContent {
    ItemContent::new()
        .title(ItemTitle::new(title))
        .description(ItemDescription::new(description))
}

fn icon(symbol: &'static str) -> Text {
    text(symbol).text_sm().accessibility_elements_hidden(true)
}

fn icon_media(symbol: &'static str) -> ItemMedia {
    ItemMedia::new(icon(symbol)).variant(ItemMediaVariant::Icon)
}

fn action(label: &'static str, on_press: impl Fn() + 'static) -> View {
    Button::new(label)
        .size(ButtonSize::Small)
        .variant(ButtonVariant::Outline)
        .on_press(on_press)
}

fn icon_action(
    label: impl IntoReactive<SharedString>,
    symbol: &'static str,
    on_press: impl Fn() + 'static,
) -> View {
    Button::new(label)
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::IconSmall)
        .radius(99.0)
        .icon_start(icon(symbol))
        .on_press(on_press)
}

fn feedback(status: Signal<String>) -> impl IntoElement {
    when(
        move || !status.with(String::is_empty),
        move || muted(move || status.get()),
    )
}

struct Locale {
    language: &'static str,
    title: &'static str,
    description: &'static str,
    action: &'static str,
    verified: &'static str,
    response: &'static str,
}

const LOCALES: [Locale; 3] = [
    Locale {
        language: "English",
        title: "Basic Item",
        description: "A simple item with title and description.",
        action: "Action",
        verified: "Your profile has been verified.",
        response: "Action selected in preview.",
    },
    Locale {
        language: "Arabic",
        title: "عنصر أساسي",
        description: "عنصر بسيط يحتوي على عنوان ووصف.",
        action: "إجراء",
        verified: "تم التحقق من ملفك الشخصي.",
        response: "تم اختيار الإجراء في المعاينة.",
    },
    Locale {
        language: "Hebrew",
        title: "פריט בסיסי",
        description: "פריט פשוט עם כותרת ותיאור.",
        action: "פעולה",
        verified: "הפרופיל שלך אומת.",
        response: "הפעולה נבחרה בתצוגה המקדימה.",
    },
];

fn overview(locale: &'static Locale, rtl: bool) -> impl IntoElement {
    let status = signal(String::new());
    view()
        .gap_4()
        .child(
            Item::new()
                .variant(ItemVariant::Outline)
                .content(content(locale.title, locale.description))
                .actions(ItemActions::new().child(action(locale.action, move || {
                    status.set(locale.response.into())
                }))),
        )
        .child(
            Item::new()
                .variant(ItemVariant::Outline)
                .size(ItemSize::Small)
                .media(ItemMedia::new(icon("✓")))
                .content(ItemContent::new().title(ItemTitle::new(locale.verified)))
                .actions(ItemActions::new().child(icon(if rtl { "‹" } else { "›" })))
                .href(locale.verified, DOCUMENTATION),
        )
        .child(feedback(status))
}

pub fn demo() -> impl IntoElement {
    stage(overview(&LOCALES[0], false))
}

pub fn variant() -> impl IntoElement {
    stage(
        ItemGroup::new().children(
            [
                (
                    ItemVariant::Default,
                    "Default Variant",
                    "Transparent background with no border.",
                ),
                (
                    ItemVariant::Outline,
                    "Outline Variant",
                    "Outlined style with a visible border.",
                ),
                (
                    ItemVariant::Muted,
                    "Muted Variant",
                    "Muted background for secondary content.",
                ),
            ]
            .map(|(variant, title, description)| {
                Item::new()
                    .variant(variant)
                    .media(icon_media("▣"))
                    .content(content(title, description))
            }),
        ),
    )
}

pub fn size() -> impl IntoElement {
    stage(
        ItemGroup::new().children(
            [
                (
                    ItemSize::Default,
                    "Default Size",
                    "The standard size for most use cases.",
                ),
                (
                    ItemSize::Small,
                    "Small Size",
                    "A compact size for dense layouts.",
                ),
                (
                    ItemSize::ExtraSmall,
                    "Extra Small Size",
                    "The most compact size available.",
                ),
            ]
            .map(|(size, title, description)| {
                Item::new()
                    .variant(ItemVariant::Outline)
                    .size(size)
                    .media(icon_media("▣"))
                    .content(content(title, description))
            }),
        ),
    )
}

pub fn icon_example() -> impl IntoElement {
    let status = signal(String::new());
    stage(
        view()
            .gap_2()
            .child(
                Item::new()
                    .variant(ItemVariant::Outline)
                    .media(icon_media("⚠︎"))
                    .content(content(
                        "Security Alert",
                        "New login detected from unknown device.",
                    ))
                    .actions(ItemActions::new().child(action("Review", move || {
                        status.set("Review selected in preview.".into())
                    }))),
            )
            .child(feedback(status)),
    )
}

struct Person {
    name: &'static str,
    email: &'static str,
    initials: &'static str,
    image: ImageAsset,
}

static PEOPLE: [Person; 3] = [
    Person {
        name: "shadcn",
        email: "shadcn@vercel.com",
        initials: "CN",
        image: asset!("avatar-shadcn.jpg"),
    },
    Person {
        name: "maxleiter",
        email: "maxleiter@vercel.com",
        initials: "LR",
        image: asset!("avatar-maxleiter.png"),
    },
    Person {
        name: "evilrabbit",
        email: "evilrabbit@vercel.com",
        initials: "ER",
        image: asset!("avatar-evilrabbit.png"),
    },
];

fn portrait(person: &Person, size: AvatarSize, grayscale: bool) -> Avatar {
    Avatar::new(person.name)
        .image(AvatarImage::new(person.image))
        .fallback(AvatarFallback::new(person.initials))
        .size(size)
        .grayscale(grayscale)
}

pub fn avatar() -> impl IntoElement {
    let status = signal(String::new());
    stage(
        view()
            .gap_4()
            .child(
                Item::new()
                    .variant(ItemVariant::Outline)
                    .media(ItemMedia::new(portrait(
                        &PEOPLE[2],
                        AvatarSize::Large,
                        false,
                    )))
                    .content(content("Evil Rabbit", "Last seen 5 months ago"))
                    .actions(ItemActions::new().child(icon_action(
                        "Invite Evil Rabbit",
                        "+",
                        move || status.set("Invite selected · Evil Rabbit".into()),
                    ))),
            )
            .child(
                Item::new()
                    .variant(ItemVariant::Outline)
                    .media(ItemMedia::new(dynamic(|| {
                        if window().width.0 < 640.0 {
                            AvatarGroup::new().child(portrait(
                                &PEOPLE[2],
                                AvatarSize::Default,
                                true,
                            ))
                        } else {
                            AvatarGroup::new()
                                .child(portrait(&PEOPLE[0], AvatarSize::Default, true))
                                .child(portrait(&PEOPLE[1], AvatarSize::Default, true))
                                .child(portrait(&PEOPLE[2], AvatarSize::Default, true))
                        }
                    })))
                    .content(content(
                        "No Team Members",
                        "Invite your team to collaborate on this project.",
                    ))
                    .actions(ItemActions::new().child(action("Invite", move || {
                        status.set("Invite team selected.".into())
                    }))),
            )
            .child(feedback(status)),
    )
}

fn photo(source: impl Into<ImageSrc>, label: &'static str) -> Image {
    image(source.into())
        .w_full()
        .h_full()
        .object_fit(ResizeMode::Cover)
        .role(AccessibilityRole::Image)
        .label(label)
}

pub fn image_example() -> impl IntoElement {
    let songs = [
        (
            "Midnight City Lights",
            "Neon Dreams",
            "Electric Nights",
            "3:45",
            asset!("item-midnight.png"),
        ),
        (
            "Coffee Shop Conversations",
            "The Morning Brew",
            "Urban Stories",
            "4:05",
            asset!("item-coffee.png"),
        ),
        (
            "Digital Rain",
            "Cyber Symphony",
            "Binary Beats",
            "3:30",
            asset!("item-rain.png"),
        ),
    ];
    stage(
        ItemGroup::new().children(songs.map(|(title, artist, album, duration, artwork)| {
            Item::new()
                .variant(ItemVariant::Outline)
                .media(
                    ItemMedia::new(photo(artwork, title).grayscale(1.0))
                        .variant(ItemMediaVariant::Image),
                )
                .content(
                    ItemContent::new()
                        .title(ItemTitle::new(format!("{title} - {album}")))
                        .description(ItemDescription::new(artist)),
                )
                .content(ItemContent::new().description(ItemDescription::new(duration)))
                .href(title, DOCUMENTATION)
        })),
    )
}

pub fn group() -> impl IntoElement {
    let status = signal(String::new());
    stage(
        view()
            .gap_3()
            .child(ItemGroup::new().children(PEOPLE.iter().map(|person| {
                Item::new()
                    .variant(ItemVariant::Outline)
                    .media(ItemMedia::new(portrait(person, AvatarSize::Default, true)))
                    .content(content(person.name, person.email))
                    .actions(ItemActions::new().child(icon_action(
                        format!("Invite {}", person.name),
                        "+",
                        move || status.set(format!("Invite selected · {}", person.name)),
                    )))
            })))
            .child(feedback(status)),
    )
}

pub fn header() -> impl IntoElement {
    let models = [
        (
            "v0-1.5-sm",
            "Everyday tasks and UI generation.",
            asset!("item-model-small.jpg"),
        ),
        (
            "v0-1.5-lg",
            "Advanced thinking or reasoning.",
            asset!("item-model-large.jpg"),
        ),
        (
            "v0-2.0-mini",
            "Open Source model for everyone.",
            asset!("item-model-mini.jpg"),
        ),
    ];
    let wide = || window().width.0 >= 650.0;
    view().w_full().max_w(px(576.0)).child(
        ItemGroup::new()
            .style_when(wide, |style| style.flex_row().items_stretch())
            .children(models.map(|(name, description, artwork)| {
                Item::new()
                    .variant(ItemVariant::Outline)
                    .header(
                        ItemHeader::new()
                            .child(AspectRatio::new(1.0, photo(artwork, name)).radius(5.0)),
                    )
                    .content(content(name, description))
                    .surface()
                    .style_when(wide, |style| style.flex_1())
            })),
    )
}

pub fn link_example() -> impl IntoElement {
    stage(
        view()
            .gap_4()
            .child(
                Item::new()
                    .content(content(
                        "Visit our documentation",
                        "Learn how to get started with our components.",
                    ))
                    .actions(ItemActions::new().child(icon("›")))
                    .href("Visit our documentation", DOCUMENTATION),
            )
            .child(
                Item::new()
                    .variant(ItemVariant::Outline)
                    .content(content(
                        "External resource",
                        "Open the shadcn/ui source repository.",
                    ))
                    .actions(ItemActions::new().child(icon("↗")))
                    .href("External resource", SOURCE),
            ),
    )
}

pub fn dropdown() -> impl IntoElement {
    let status = signal(String::new());
    let options = PEOPLE.iter().map(|person| {
        menu_item()
            .accessibility_label(person.name)
            .w_full()
            .px_2()
            .py_1()
            .rounded(px(8.0))
            .focus(|style| style.bg(theme().accent))
            .hover(|style| style.bg(theme().accent))
            .on_select(move || status.set(format!("Selected · {}", person.name)))
            .child(
                Item::new()
                    .size(ItemSize::ExtraSmall)
                    .media(ItemMedia::new(portrait(person, AvatarSize::Small, true)))
                    .content(content(person.name, person.email))
                    .surface()
                    .p_0(),
            )
    });
    view()
        .w_full()
        .items_center()
        .gap_3()
        .child(DropdownMenu::new(
            DropdownMenuTrigger::render(
                Button::new("Select")
                    .variant(ButtonVariant::Outline)
                    .icon_end(icon("⌄"))
                    .surface(),
            ),
            DropdownMenuContent::new()
                .align(Align::End)
                .width(216)
                .children(options),
        ))
        .child(feedback(status))
}

pub fn rtl() -> impl IntoElement {
    let chosen = signal(1_usize);
    let direction = move || {
        if chosen.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    stage(
        view()
            .gap_4()
            .child(
                view()
                    .flex_row()
                    .flex_wrap()
                    .justify_center()
                    .gap_2()
                    .children(
                        LOCALES.iter().enumerate().map(|(index, locale)| {
                            action(locale.language, move || chosen.set(index))
                        }),
                    ),
            )
            .child(DirectionProvider::new(direction, move || {
                dynamic(move || overview(&LOCALES[chosen.get()], direction() == Direction::Rtl))
            })),
    )
}
