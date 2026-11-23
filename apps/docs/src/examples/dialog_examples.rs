use components::{
    button::{Button, ButtonSize, ButtonVariant},
    dialog::{
        Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle, DialogTrigger,
    },
};
use nsh_base::theme;
use nsh_core::{
    element::{Input, LayoutDirection, input},
    prelude::*,
};

#[derive(Clone, Copy)]
struct Labels {
    open: &'static str,
    title: &'static str,
    description: &'static str,
    name: &'static str,
    username: &'static str,
    cancel: &'static str,
    save: &'static str,
    close: &'static str,
}

const EN: Labels = Labels {
    open: "Open Dialog",
    title: "Edit profile",
    description: "Make changes to your profile here. Click save when you're done.",
    name: "Name",
    username: "Username",
    cancel: "Cancel",
    save: "Save changes",
    close: "Close",
};

const AR: Labels = Labels {
    open: "فتح الحوار",
    title: "تعديل الملف الشخصي",
    description: "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر فوق حفظ عند الانتهاء.",
    name: "الاسم",
    username: "اسم المستخدم",
    cancel: "إلغاء",
    save: "حفظ التغييرات",
    close: "إغلاق",
};

const HE: Labels = Labels {
    open: "פתח דיאלוג",
    title: "ערוך פרופיל",
    description: "בצע שינויים בפרופיל שלך כאן. לחץ על שמור כשתסיים.",
    name: "שם",
    username: "שם משתמש",
    cancel: "בטל",
    save: "שמור שינויים",
    close: "סגור",
};

fn header(title: &'static str, description: &'static str) -> DialogHeader {
    DialogHeader::new(DialogTitle::new(title)).description(DialogDescription::new(description))
}

type Profile = (String, String);

fn initial_profile() -> Profile {
    ("Pedro Duarte".to_owned(), "@peduarte".to_owned())
}

fn input_surface(label: &'static str) -> Input {
    input()
        .label(label)
        .h(px(36.0))
        .rounded(px(10.0))
        .border(px(1.0))
        .border_color(move || theme().input)
        .px_3()
        .text_sm()
        .text_color(move || theme().foreground)
}

fn profile(labels: Labels, direction: LayoutDirection, saved: Signal<Profile>) -> View {
    let state = Dialog::new(signal(false));
    let draft = signal(saved.get_untracked());
    let saved_name = signal(String::new());
    effect(move || {
        if state.open.get() {
            draft.set(saved.get_untracked());
        }
    });
    let commit = move || {
        let profile = draft.get_untracked();
        saved_name.set(profile.0.clone());
        saved.set(profile);
        state.close();
    };
    let rtl = direction == LayoutDirection::Rtl;
    let caption = move |label| {
        text(label)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .when(rtl, |caption| caption.text_right())
    };
    view()
        .direction(direction)
        .items_center()
        .gap_2()
        .child(DialogTrigger::new(state, labels.open).test_id("profile-dialog-trigger"))
        .child(
            DialogContent::new(state, header(labels.title, labels.description))
                .direction(direction)
                .max_width(384.0)
                .close_button(Some(labels.close))
                .body(move || {
                    view()
                        .gap_4()
                        .child(
                            view().gap_2().child(caption(labels.name)).child(
                                input_surface(labels.name)
                                    .value(move || draft.with(|draft| draft.0.clone()))
                                    .on_change(move |value: &str| {
                                        draft.update(|draft| draft.0 = value.to_owned())
                                    })
                                    .on_submit(move |_: &str| commit()),
                            ),
                        )
                        .child(
                            view().gap_2().child(caption(labels.username)).child(
                                input_surface(labels.username)
                                    .value(move || draft.with(|draft| draft.1.clone()))
                                    .on_change(move |value: &str| {
                                        draft.update(|draft| draft.1 = value.to_owned())
                                    })
                                    .on_submit(move |_: &str| commit()),
                            ),
                        )
                })
                .footer(move || {
                    DialogFooter::new()
                        .child(DialogClose::new(state, labels.cancel))
                        .child(Button::new(labels.save).on_press(commit))
                }),
        )
        .child(when(
            move || !saved_name.with(String::is_empty),
            move || {
                text(move || format!("{} · {}", labels.save, saved_name.get()))
                    .text_sm()
                    .text_color(move || theme().muted_foreground)
            },
        ))
}

pub fn demo() -> impl IntoElement {
    profile(EN, LayoutDirection::Ltr, signal(initial_profile()))
}

pub fn close_button() -> impl IntoElement {
    let state = Dialog::new(signal(false));
    view()
        .items_center()
        .child(DialogTrigger::new(state, "Share"))
        .child(
            DialogContent::new(
                state,
                header(
                    "Share link",
                    "Anyone who has this link will be able to view this.",
                ),
            )
            .body(|| {
                view().child(
                    input_surface("Link")
                        .value("https://ui.shadcn.com/docs/installation")
                        .readonly(true),
                )
            })
            .footer(move || {
                DialogFooter::new()
                    .align_start()
                    .child(DialogClose::render(state, Button::new("Close").surface()))
            }),
        )
}

pub fn no_close_button() -> impl IntoElement {
    let state = Dialog::new(signal(false));
    view()
        .items_center()
        .child(DialogTrigger::new(state, "No Close Button"))
        .child(
            DialogContent::new(
                state,
                header(
                    "No Close Button",
                    "This dialog doesn't have a corner close button.",
                ),
            )
            .close_button(None),
        )
}

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn paragraphs() -> View {
    view()
        .gap_4()
        .children((0..10).map(|_| text(PARAGRAPH).text_sm().line_height(px(20.0))))
}

pub fn sticky_footer() -> impl IntoElement {
    let state = Dialog::new(signal(false));
    view()
        .items_center()
        .child(DialogTrigger::new(state, "Sticky Footer"))
        .child(
            DialogContent::new(
                state,
                header(
                    "Sticky Footer",
                    "The footer stays visible while the content scrolls.",
                ),
            )
            .scrollable(paragraphs)
            .footer(move || DialogFooter::new().child(DialogClose::new(state, "Close"))),
        )
}

pub fn scrollable_content() -> impl IntoElement {
    let state = Dialog::new(signal(false));
    view()
        .items_center()
        .child(DialogTrigger::new(state, "Scrollable Content"))
        .child(
            DialogContent::new(
                state,
                header(
                    "Scrollable Content",
                    "This is a dialog with scrollable content.",
                ),
            )
            .scrollable(paragraphs),
        )
}

pub fn rtl() -> impl IntoElement {
    let hebrew = signal(false);
    let saved = signal(initial_profile());
    view()
        .gap_3()
        .items_center()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(
                    Button::new("Arabic")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(false)),
                )
                .child(
                    Button::new("Hebrew")
                        .size(ButtonSize::Small)
                        .variant(ButtonVariant::Outline)
                        .on_press(move || hebrew.set(true)),
                ),
        )
        .child(dynamic(move || {
            profile(
                if hebrew.get() { HE } else { AR },
                LayoutDirection::Rtl,
                saved,
            )
        }))
}
