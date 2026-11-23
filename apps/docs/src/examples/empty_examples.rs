use nsh_base::{input, theme};
use nsh_core::{SvgPaint, prelude::*};

use components::{
    avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage, AvatarSize},
    button::{Button, ButtonSize, ButtonVariant},
    direction::{Direction, DirectionProvider},
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    ui::muted,
};

struct ProjectCopy {
    title: &'static str,
    description: &'static str,
    create: &'static str,
    import: &'static str,
    learn: &'static str,
    response: &'static str,
}

const ENGLISH: ProjectCopy = ProjectCopy {
    title: "No Projects Yet",
    description: "You haven't created any projects yet. Get started by creating your first project.",
    create: "Create Project",
    import: "Import Project",
    learn: "Learn More",
    response: "Selected",
};

const ARABIC: ProjectCopy = ProjectCopy {
    title: "لا توجد مشاريع بعد",
    description: "لم تقم بإنشاء أي مشاريع بعد. ابدأ بإنشاء مشروعك الأول.",
    create: "إنشاء مشروع",
    import: "استيراد مشروع",
    learn: "تعرف على المزيد",
    response: "تم الاختيار",
};

const HEBREW: ProjectCopy = ProjectCopy {
    title: "אין פרויקטים עדיין",
    description: "עדיין לא יצרת פרויקטים. התחל על ידי יצירת הפרויקט הראשון שלך.",
    create: "צור פרויקט",
    import: "ייבא פרויקט",
    learn: "למד עוד",
    response: "נבחר",
};

fn icon(symbol: &'static str) -> Text {
    text(symbol)
        .text_lg()
        .text_center()
        .accessibility_elements_hidden(true)
}

const FOLDER: &str = "M2 6h5l2 2h9v9H2z M8 10l-2 2 2 2 M12 10l2 2-2 2";
const CLOUD: &str = "M5 15a3 3 0 1 1 0-6 4.5 4.5 0 0 1 8.7-1A3.5 3.5 0 1 1 15 15z";
const BELL: &str = "M3 14h14l-2-2V8a5 5 0 0 0-10 0v4z M8 16a2 2 0 0 0 4 0";

fn icon_header(title: &'static str, detail: &'static str, drawing: &'static str) -> EmptyHeader {
    EmptyHeader::new()
        .child(
            EmptyMedia::new().variant(EmptyMediaVariant::Icon).child(
                canvas(
                    Painter::new()
                        .stroke_style_from_view(SvgPaint::CurrentColor)
                        .line_width(px(1.5))
                        .stroke_svg(drawing),
                )
                .size(px(20.0))
                .accessibility_elements_hidden(true),
            ),
        )
        .child(EmptyTitle::new(title))
        .child(EmptyDescription::new(detail))
}

fn feedback(status: Signal<String>) -> impl IntoElement {
    when(
        move || !status.with(String::is_empty),
        move || muted(move || status.get()).text_center(),
    )
}

fn projects(copy: &'static ProjectCopy, rtl: bool) -> impl IntoElement {
    let status = signal(String::new());
    let select = move |action| status.set(format!("{} · {action}", copy.response));
    Empty::new()
        .child(icon_header(copy.title, copy.description, FOLDER))
        .child(
            EmptyContent::new()
                .flex_row()
                .flex_wrap()
                .justify_center()
                .gap_2()
                .child(Button::new(copy.create).on_press(move || select(copy.create)))
                .child(
                    Button::new(copy.import)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || select(copy.import)),
                ),
        )
        .child(
            Button::new(copy.learn)
                .variant(ButtonVariant::Link)
                .size(ButtonSize::Small)
                .icon_end(icon(if rtl { "↖" } else { "↗" }))
                .href("https://ui.shadcn.com/docs/components/base/empty"),
        )
        .child(feedback(status))
}

pub fn demo() -> impl IntoElement {
    projects(&ENGLISH, false)
}

pub fn outline() -> impl IntoElement {
    let status = signal(String::new());
    Empty::new()
        .border(px(1.0))
        .border_color(move || theme().border)
        .border_dashed()
        .child(icon_header(
            "Cloud Storage Empty",
            "Upload files to your cloud storage to access them anywhere.",
            CLOUD,
        ))
        .child(
            EmptyContent::new().child(
                Button::new("Upload Files")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Small)
                    .on_press(move || status.set("Upload Files selected".into())),
            ),
        )
        .child(feedback(status))
}

pub fn background() -> impl IntoElement {
    let status = signal(String::new());
    Empty::new()
        .bg(move || theme().muted.opacity(0.3))
        .child(icon_header(
            "No Notifications",
            "You're all caught up. New notifications will appear here.",
            BELL,
        ))
        .child(
            EmptyContent::new().child(
                Button::new("Refresh")
                    .variant(ButtonVariant::Outline)
                    .icon_start(icon("⟳"))
                    .on_press(move || status.set("Still no notifications".into())),
            ),
        )
        .child(feedback(status))
}

fn portrait(
    name: &'static str,
    initials: &'static str,
    source: impl Into<nsh_core::element::ImageSrc>,
) -> Avatar {
    Avatar::new(name)
        .image(AvatarImage::new(source))
        .fallback(AvatarFallback::new(initials))
        .size(AvatarSize::Large)
        .grayscale(true)
}

pub fn avatar() -> impl IntoElement {
    let status = signal(String::new());
    Empty::new()
        .child(
            EmptyHeader::new()
                .child(EmptyMedia::new().child(portrait("shadcn", "LR", asset!("avatar-shadcn.jpg"))))
                .child(EmptyTitle::new("User Offline"))
                .child(EmptyDescription::new(
                    "This user is currently offline. You can leave a message to notify them or try again later.",
                )),
        )
        .child(
            EmptyContent::new().child(
                Button::new("Leave Message")
                    .size(ButtonSize::Small)
                    .on_press(move || status.set("Leave Message selected".into())),
            ),
        )
        .child(feedback(status))
}

pub fn avatar_group() -> impl IntoElement {
    let status = signal(String::new());
    Empty::new()
        .child(
            EmptyHeader::new()
                .child(
                    EmptyMedia::new().child(
                        AvatarGroup::new()
                            .child(portrait("@shadcn", "CN", asset!("avatar-shadcn.jpg")))
                            .child(portrait("@maxleiter", "LR", asset!("avatar-maxleiter.png")))
                            .child(portrait(
                                "@evilrabbit",
                                "ER",
                                asset!("avatar-evilrabbit.png"),
                            )),
                    ),
                )
                .child(EmptyTitle::new("No Team Members"))
                .child(EmptyDescription::new(
                    "Invite your team to collaborate on this project.",
                )),
        )
        .child(
            EmptyContent::new().child(
                Button::new("Invite Members")
                    .size(ButtonSize::Small)
                    .icon_start(icon("+"))
                    .on_press(move || status.set("Invite Members selected".into())),
            ),
        )
        .child(feedback(status))
}

pub fn input_group() -> impl IntoElement {
    let status = signal(String::new());
    Empty::new()
        .child(
            EmptyHeader::new()
                .child(EmptyTitle::new("404 - Not Found"))
                .child(EmptyDescription::new(
                    "The page you're looking for doesn't exist. Try searching for what you need below.",
                )),
        )
        .child(
            EmptyContent::new()
                .child(
                    view()
                        .w_full()
                        .max_w(px(288.0))
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .rounded(px(12.0))
                        .border(px(1.0))
                        .border_color(move || theme().border)
                        .child(icon("⌕"))
                        .child(
                            input()
                                .label("Search pages")
                                .placeholder("Try searching for pages...")
                                .flex_1()
                                .min_w(px(0.0))
                                .h(px(32.0))
                                .border(px(0.0))
                                .bg(AdaptiveColor::TRANSPARENT)
                                .on_submit(move |value: &str| {
                                    let query = value.trim();
                                    if !query.is_empty() {
                                        status.set(format!("Search preview · {query}"));
                                    }
                                }),
                        )
                        .child(
                            text("/")
                                .text_xs()
                                .rounded(px(4.0))
                                .px_1()
                                .bg(move || theme().muted),
                        ),
                )
                .child(EmptyDescription::render(
                    view()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .child(muted("Need help?"))
                        .child(
                            Button::new("Contact support")
                                .variant(ButtonVariant::Link)
                                .size(ButtonSize::Small)
                                .href("https://github.com/shadcn-ui/ui/discussions"),
                        ),
                )),
        )
        .child(feedback(status))
}

pub fn rtl() -> impl IntoElement {
    let language = signal(1_usize);
    let direction = move || {
        if language.get() == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        }
    };
    let mut options = view().flex_row().flex_wrap().justify_center().gap_2();
    for (index, name) in ["English", "Arabic", "Hebrew"].into_iter().enumerate() {
        options = options.child(
            Button::new(name)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Small)
                .on_press(move || language.set(index)),
        );
    }
    view()
        .w_full()
        .items_center()
        .child(options)
        .child(DirectionProvider::new(direction, move || {
            dynamic(move || {
                projects(
                    [&ENGLISH, &ARABIC, &HEBREW][language.get()],
                    direction() == Direction::Rtl,
                )
            })
        }))
}
