use nsh_base::theme;
use nsh_core::prelude::*;

use components::{
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    navigation_menu::{
        NavigationMenu, NavigationMenuContent as Content, NavigationMenuItem as Item,
        NavigationMenuLink as Link, NavigationMenuList as List, NavigationMenuTrigger as Trigger,
    },
};

const START_LINKS: [&str; 3] = [
    "https://ui.shadcn.com/docs",
    "https://ui.shadcn.com/docs/installation",
    "https://ui.shadcn.com/docs/typography",
];
const COMPONENT_LINKS: [&str; 6] = [
    "https://ui.shadcn.com/docs/components/base/alert-dialog",
    "https://ui.shadcn.com/docs/components/base/hover-card",
    "https://ui.shadcn.com/docs/components/base/progress",
    "https://ui.shadcn.com/docs/components/base/scroll-area",
    "https://ui.shadcn.com/docs/components/base/tabs",
    "https://ui.shadcn.com/docs/components/base/tooltip",
];

struct Locale {
    direction: Direction,
    triggers: [&'static str; 4],
    start: [(&'static str, &'static str); 3],
    components: [(&'static str, &'static str); 6],
    statuses: [&'static str; 3],
}

fn describe<const N: usize>(
    content: Content,
    entries: [(&'static str, &'static str); N],
    urls: [&'static str; N],
) -> Content {
    entries
        .into_iter()
        .zip(urls)
        .fold(content, |current, ((label, description), url)| {
            current.link(Link::new(label, url).description(description))
        })
}

fn sample(locale: &'static Locale) -> NavigationMenu {
    let [start, components, with_icons, docs] = locale.triggers;
    let mut list = List::new().item(Item::new(
        Trigger::new(start),
        describe(Content::new(), locale.start, START_LINKS),
    ));
    if window().width.0 >= 768.0 {
        list = list.item(Item::new(
            Trigger::new(components),
            describe(
                Content::new().width(560.0).columns(2),
                locale.components,
                COMPONENT_LINKS,
            ),
        ));
    }
    let status_links = locale
        .statuses
        .into_iter()
        .zip(["ⓘ", "◌", "✓"])
        .fold(Content::new().width(180.0), |content, (label, icon)| {
            content.link(Link::new(label, "#navigation-menu").icon(icon))
        });
    NavigationMenu::new(
        "Example navigation",
        list.item(Item::new(Trigger::new(with_icons), status_links))
            .item(Item::link(Link::new(docs, START_LINKS[0]))),
    )
}

pub fn demo() -> View {
    view()
        .w_full()
        .min_h(px(350.0))
        .items_center()
        .pt_6()
        .child(dynamic(|| sample(&ENGLISH)))
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    view()
        .w_full()
        .min_h(px(350.0))
        .items_center()
        .pt_3()
        .gap_3()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    |(index, name)| {
                        Button::new(name)
                            .size(ButtonSize::Small)
                            .variant(ButtonVariant::Outline)
                            .on_press(move || language.set(index))
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                            .style_when(
                                move || language.get() == index,
                                |style| style.bg(theme().muted),
                            )
                    },
                )),
        )
        .child(dynamic(move || {
            let locale = [&ENGLISH, &ARABIC, &HEBREW][language.get()];
            DirectionProvider::new(locale.direction, move || dynamic(move || sample(locale)))
        }))
}

const ENGLISH: Locale = Locale {
    direction: Direction::Ltr,
    triggers: ["Getting started", "Components", "With Icon", "Docs"],
    start: [
        (
            "Introduction",
            "Re-usable components built with Tailwind CSS.",
        ),
        (
            "Installation",
            "How to install dependencies and structure your app.",
        ),
        ("Typography", "Styles for headings, paragraphs, lists...etc"),
    ],
    components: [
        (
            "Alert Dialog",
            "A modal dialog that interrupts the user with important content and expects a response.",
        ),
        (
            "Hover Card",
            "For sighted users to preview content available behind a link.",
        ),
        (
            "Progress",
            "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
        ),
        ("Scroll-area", "Visually or semantically separates content."),
        (
            "Tabs",
            "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
        ),
        (
            "Tooltip",
            "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
        ),
    ],
    statuses: ["Backlog", "To Do", "Done"],
};

const ARABIC: Locale = Locale {
    direction: Direction::Rtl,
    triggers: ["البدء", "المكونات", "مع أيقونة", "الوثائق"],
    start: [
        (
            "مقدمة",
            "مكونات قابلة لإعادة الاستخدام مبنية باستخدام Tailwind CSS.",
        ),
        ("التثبيت", "كيفية تثبيت التبعيات وتنظيم تطبيقك."),
        ("الطباعة", "أنماط للعناوين والفقرات والقوائم...إلخ"),
    ],
    components: [
        (
            "حوار التنبيه",
            "حوار نافذة يقطع المستخدم بمحتوى مهم ويتوقع استجابة.",
        ),
        (
            "بطاقة التحويم",
            "للمستخدمين المبصرين لمعاينة المحتوى المتاح خلف الرابط.",
        ),
        (
            "التقدم",
            "يعرض مؤشرًا يوضح تقدم إتمام المهمة، عادةً يتم عرضه كشريط تقدم.",
        ),
        ("منطقة التمرير", "يفصل المحتوى بصريًا أو دلاليًا."),
        (
            "التبويبات",
            "مجموعة من أقسام المحتوى المتعددة الطبقات—المعروفة بألواح التبويب—التي يتم عرضها واحدة في كل مرة.",
        ),
        (
            "تلميح",
            "نافذة منبثقة تعرض معلومات متعلقة بعنصر عندما يتلقى العنصر التركيز على لوحة المفاتيح أو عند تحويم الماوس فوقه.",
        ),
    ],
    statuses: ["قائمة الانتظار", "المهام", "منجز"],
};

const HEBREW: Locale = Locale {
    direction: Direction::Rtl,
    triggers: ["התחלה", "רכיבים", "עם אייקון", "תיעוד"],
    start: [
        ("הקדמה", "רכיבים לשימוש חוזר שנבנו עם Tailwind CSS."),
        ("התקנה", "כיצד להתקין תלויות ולבנות את האפליקציה שלך."),
        ("טיפוגרפיה", "סגנונות לכותרות, פסקאות, רשימות...וכו'"),
    ],
    components: [
        (
            "דיאלוג התראה",
            "דיאלוג מודאלי שמפריע למשתמש עם תוכן חשוב ומצפה לתגובה.",
        ),
        (
            "כרטיס ריחוף",
            "למשתמשים רואים כדי להציג תצוגה מקדימה של תוכן זמין מאחורי קישור.",
        ),
        (
            "התקדמות",
            "מציג אינדיקטור המציג את התקדמות ההשלמה של משימה, בדרך כלל מוצג כסרגל התקדמות.",
        ),
        ("אזור גלילה", "מפריד תוכן חזותית או סמנטית."),
        (
            "כרטיסיות",
            "קבוצה של חלקי תוכן מרובדים—המכונים לוחות כרטיסיות—המוצגים אחד בכל פעם.",
        ),
        (
            "טולטיפ",
            "חלון קופץ המציג מידע הקשור לאלמנט כאשר האלמנט מקבל מיקוד מקלדת או כאשר העכבר מרחף מעליו.",
        ),
    ],
    statuses: ["רשימת המתנה", "לעשות", "הושלם"],
};
