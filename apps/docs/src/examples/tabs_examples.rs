use components::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardHeader, CardTitle},
    direction::{Direction, DirectionProvider},
    tabs::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger, TabsVariant},
};
use nsh_base::theme;
use nsh_core::prelude::*;

#[derive(Clone, Copy)]
struct Section {
    title: &'static str,
    description: &'static str,
    detail: &'static str,
}

const KEYS: [&str; 4] = ["overview", "analytics", "reports", "settings"];

const ENGLISH: [Section; 4] = [
    Section {
        title: "Overview",
        description: "View your key metrics and recent project activity. Track progress across all your active projects.",
        detail: "You have 12 active projects and 3 pending tasks.",
    },
    Section {
        title: "Analytics",
        description: "Track performance and user engagement metrics. Monitor trends and identify growth opportunities.",
        detail: "Page views are up 25% compared to last month.",
    },
    Section {
        title: "Reports",
        description: "Generate and download your detailed reports. Export data in multiple formats for analysis.",
        detail: "You have 5 reports ready and available to export.",
    },
    Section {
        title: "Settings",
        description: "Manage your account preferences and options. Customize your experience to fit your needs.",
        detail: "Configure notifications, security, and themes.",
    },
];

const ARABIC: [Section; 4] = [
    Section {
        title: "نظرة عامة",
        description: "عرض مقاييسك الرئيسية وأنشطة المشروع الأخيرة. تتبع التقدم عبر جميع مشاريعك النشطة.",
        detail: "لديك ١٢ مشروعًا نشطًا و٣ مهام معلقة.",
    },
    Section {
        title: "التحليلات",
        description: "تتبع مقاييس الأداء ومشاركة المستخدمين. راقب الاتجاهات وحدد فرص النمو.",
        detail: "زادت مشاهدات الصفحة بنسبة ٢٥٪ مقارنة بالشهر الماضي.",
    },
    Section {
        title: "التقارير",
        description: "إنشاء وتنزيل تقاريرك التفصيلية. تصدير البيانات بتنسيقات متعددة للتحليل.",
        detail: "لديك ٥ تقارير جاهزة ومتاحة للتصدير.",
    },
    Section {
        title: "الإعدادات",
        description: "إدارة تفضيلات حسابك وخياراته. تخصيص تجربتك لتناسب احتياجاتك.",
        detail: "تكوين الإشعارات والأمان والسمات.",
    },
];

const HEBREW: [Section; 4] = [
    Section {
        title: "סקירה כללית",
        description: "הצג את המדדים העיקריים שלך ופעילות הפרויקט האחרונה. עקוב אחר התקדמות בכל הפרויקטים הפעילים שלך.",
        detail: "יש לך 12 פרויקטים פעילים ו-3 משימות ממתינות.",
    },
    Section {
        title: "אנליטיקה",
        description: "עקוב אחר ביצועים ומדדי מעורבות משתמשים. עקוב אחר מגמות וזהה הזדמנויות צמיחה.",
        detail: "צפיות בדף עלו ב-25% בהשוואה לחודש שעבר.",
    },
    Section {
        title: "דוחות",
        description: "צור והורד את הדוחות המפורטים שלך. ייצא נתונים בפורמטים מרובים לניתוח.",
        detail: "יש לך 5 דוחות מוכנים וזמינים לייצוא.",
    },
    Section {
        title: "הגדרות",
        description: "נהל את העדפות החשבון והאפשרויות שלך. התאם אישית את החוויה שלך כדי שתתאים לצרכים שלך.",
        detail: "הגדר התראות, אבטחה וערכות נושא.",
    },
];

fn dashboard(
    name: &'static str,
    sections: [Section; 4],
    value: Option<Signal<&'static str>>,
) -> View {
    let list = TabsList::new(name).triggers(
        KEYS.into_iter()
            .zip(sections)
            .map(|(key, section)| TabsTrigger::new(key, section.title)),
    );
    let mut tabs = Tabs::new(list).default_value("overview");
    if let Some(value) = value {
        tabs = tabs.value(value);
    }
    for (key, section) in KEYS.into_iter().zip(sections) {
        tabs = tabs.content(TabsContent::new(key, move || {
            Card::new()
                .header(
                    CardHeader::new()
                        .title(CardTitle::new(section.title))
                        .description(CardDescription::new(section.description)),
                )
                .content(CardContent::new(
                    text(section.detail)
                        .text_sm()
                        .text_color(|| theme().muted_foreground),
                ))
        }));
    }
    tabs.surface().w_full().max_w(px(400.0))
}

pub fn demo() -> View {
    dashboard("Project sections", ENGLISH, None)
}

pub fn line() -> Tabs {
    Tabs::new(
        TabsList::new("Project sections")
            .variant(TabsVariant::Line)
            .triggers([
                TabsTrigger::new("overview", "Overview"),
                TabsTrigger::new("analytics", "Analytics"),
                TabsTrigger::new("reports", "Reports"),
            ]),
    )
    .default_value("overview")
}

pub fn vertical() -> Tabs {
    Tabs::new(TabsList::new("Account sections").triggers([
        TabsTrigger::new("account", "Account"),
        TabsTrigger::new("password", "Password"),
        TabsTrigger::new("notifications", "Notifications"),
    ]))
    .orientation(TabsOrientation::Vertical)
    .default_value("account")
}

pub fn disabled() -> Tabs {
    Tabs::new(TabsList::new("Available sections").triggers([
        TabsTrigger::new("home", "Home"),
        TabsTrigger::new("disabled", "Disabled").disabled(true),
    ]))
    .default_value("home")
}

pub fn icons() -> Tabs {
    let window = view()
        .w(px(14.0))
        .h(px(12.0))
        .rounded(px(2.0))
        .border(px(1.0))
        .border_color(|| theme().foreground)
        .child(
            view()
                .absolute()
                .top(px(3.0))
                .left_0()
                .right_0()
                .h(px(1.0))
                .bg(|| theme().foreground),
        );
    Tabs::new(TabsList::new("Views").triggers([
        TabsTrigger::new("preview", "Preview").icon(window),
        TabsTrigger::new("code", "Code").icon(text("‹›").text_base()),
    ]))
    .default_value("preview")
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let value = signal("overview");
    view()
        .w_full()
        .max_w(px(400.0))
        .gap_4()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_1()
                .children(["English", "Arabic", "Hebrew"].into_iter().enumerate().map(
                    move |(index, label)| {
                        Button::new(label)
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Small)
                            .surface()
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                            .on_press(move || language.set(index))
                    },
                )),
        )
        .child(dynamic(move || {
            let (direction, label, sections) = match language.get() {
                0 => (Direction::Ltr, "Project sections", ENGLISH),
                1 => (Direction::Rtl, "أقسام المشروع", ARABIC),
                _ => (Direction::Rtl, "מדורי הפרויקט", HEBREW),
            };
            DirectionProvider::new(direction, move || dashboard(label, sections, Some(value)))
        }))
}
