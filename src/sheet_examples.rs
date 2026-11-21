use nsh_core::{element::LayoutDirection, prelude::*};
use nsh_foundation::{
    button::{Button, ButtonVariant},
    direction::{Direction, DirectionProvider},
    input::input,
    label::Label,
    sheet::{
        Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide,
        SheetTitle, SheetTrigger,
    },
};

struct Labels {
    trigger: &'static str,
    title: &'static str,
    description: &'static str,
    name: &'static str,
    username: &'static str,
    save: &'static str,
    close: &'static str,
}

const ENGLISH: Labels = Labels {
    trigger: "Open",
    title: "Edit profile",
    description: "Make changes to your profile here. Click save when you're done.",
    name: "Name",
    username: "Username",
    save: "Save changes",
    close: "Close",
};

const ARABIC: Labels = Labels {
    trigger: "فتح",
    title: "تعديل الملف الشخصي",
    description: "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر حفظ عند الانتهاء.",
    name: "الاسم",
    username: "اسم المستخدم",
    save: "حفظ التغييرات",
    close: "إغلاق",
};

const HEBREW: Labels = Labels {
    trigger: "פתח",
    title: "עריכת פרופיל",
    description: "בצע שינויים בפרופיל שלך כאן. לחץ שמור כשתסיים.",
    name: "שם",
    username: "שם משתמש",
    save: "שמור שינויים",
    close: "סגור",
};

type Profile = [String; 2];

fn profile_state() -> Signal<Profile> {
    signal(["Pedro Duarte".to_owned(), "@peduarte".to_owned()])
}

fn header(title: &'static str, description: &'static str) -> SheetHeader {
    SheetHeader::new(SheetTitle::new(title)).description(SheetDescription::new(description))
}

fn profile(labels: &'static Labels, direction: LayoutDirection, saved: Signal<Profile>) -> View {
    let sheet = Sheet::new(signal(false));
    let draft = signal(saved.get_untracked());
    effect(move || {
        if sheet.open.get() {
            draft.set(saved.get_untracked());
        }
    });
    let save = move || {
        saved.set(draft.get_untracked());
        sheet.close();
    };
    let rtl = direction == LayoutDirection::Rtl;
    let side = if rtl {
        SheetSide::Left
    } else {
        SheetSide::Right
    };
    view()
        .items_center()
        .child(SheetTrigger::new(sheet, labels.trigger).test_id("sheet-profile-trigger"))
        .child(
            SheetContent::new(sheet, header(labels.title, labels.description))
                .side(side)
                .direction(direction)
                .close_button(Some(labels.close))
                .body(move || {
                    view().gap_6().children(
                        [labels.name, labels.username].into_iter().enumerate().map(
                            move |(index, label)| {
                                let control = element_ref();
                                view()
                                    .gap_3()
                                    .child(
                                        Label::new(label)
                                            .for_control(control)
                                            .surface()
                                            .direction(direction),
                                    )
                                    .child(
                                        input()
                                            .label(label)
                                            .element_ref(&control)
                                            .when(rtl, |input| input.text_right())
                                            .value(move || {
                                                draft.with(|profile| profile[index].clone())
                                            })
                                            .on_change(move |value: &str| {
                                                draft.update(|profile| {
                                                    profile[index] = value.to_owned()
                                                })
                                            })
                                            .on_submit(move |_: &str| save()),
                                    )
                            },
                        ),
                    )
                })
                .footer(move || {
                    SheetFooter::new()
                        .child(Button::new(labels.save).on_press(save))
                        .child(SheetClose::new(sheet, labels.close))
                }),
        )
}

pub fn demo() -> View {
    profile(&ENGLISH, LayoutDirection::Ltr, profile_state())
}

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn sides() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .justify_center()
        .gap_2()
        .children(
            [
                ("Top", SheetSide::Top),
                ("Right", SheetSide::Right),
                ("Bottom", SheetSide::Bottom),
                ("Left", SheetSide::Left),
            ]
            .into_iter()
            .map(|(name, side)| {
                let sheet = Sheet::new(signal(false));
                view().child(SheetTrigger::new(sheet, name)).child(
                    SheetContent::new(sheet, header(ENGLISH.title, ENGLISH.description))
                        .side(side)
                        .max_height_fraction(0.5)
                        .body(|| {
                            view()
                                .gap_2()
                                .pb_2()
                                .children((0..10).map(|_| text(PARAGRAPH).text_sm()))
                        })
                        .footer(move || {
                            SheetFooter::new()
                                .child(Button::new(ENGLISH.save).on_press(move || sheet.close()))
                                .child(SheetClose::new(sheet, "Cancel"))
                        }),
                )
            }),
        )
}

pub fn no_close_button() -> View {
    let sheet = Sheet::new(signal(false));
    view()
        .items_center()
        .child(SheetTrigger::new(sheet, "Open Sheet"))
        .child(
            SheetContent::new(
                sheet,
                header(
                    "No Close Button",
                    "This sheet doesn't have a close button in the top-right corner. Click outside to close.",
                ),
            )
            .close_button(None),
        )
}

pub fn rtl() -> View {
    let language = signal(1_usize);
    let saved = profile_state();
    view()
        .items_center()
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
                            .on_press(move || language.set(index))
                            .accessibility_state(move || AccessibilityState {
                                selected: language.get() == index,
                                ..Default::default()
                            })
                    },
                )),
        )
        .child(dynamic(move || {
            let (direction, layout, labels) = match language.get() {
                0 => (Direction::Ltr, LayoutDirection::Ltr, &ENGLISH),
                1 => (Direction::Rtl, LayoutDirection::Rtl, &ARABIC),
                _ => (Direction::Rtl, LayoutDirection::Rtl, &HEBREW),
            };
            DirectionProvider::new(direction, move || profile(labels, layout, saved))
        }))
}
