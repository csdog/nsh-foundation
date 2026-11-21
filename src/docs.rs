use nsh_base::theme;
use nsh_core::{
    element::{app_url, next_app_url_change, set_app_url_with, set_clipboard, set_color_scheme},
    platform::Appearance,
    prelude::*,
};

use crate::{
    accordion_examples, alert_dialog_examples, alert_examples, aspect_ratio_examples,
    attachment_examples, avatar_examples, badge_examples, breadcrumb_examples, bubble_examples,
    button_examples, button_group_examples, calendar_examples, card_examples, carousel_examples,
    chart_examples, checkbox_examples, examples,
    ui::{Variant, button, heading, muted},
};

const SOURCE: &str = "https://github.com/shadcn-ui/ui/blob/f50887ae68d0fdcbfbe86b57be08af94a4c88a68/apps/v4/registry/bases/base/ui/drawer.tsx";
const REFERENCE: &str = "https://ui.shadcn.com/docs/components/base/drawer";
const SECTIONS: [&str; 13] = [
    "Installation",
    "Usage",
    "Composition",
    "Custom Sizes",
    "Styling",
    "Examples",
    "Position",
    "Swipe Handle",
    "Nested",
    "Non Modal",
    "Snap Points",
    "Responsive",
    "API Reference",
];

const SIMPLE: &str = r#"use nsh_core::prelude::*;
use nsh_foundation::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription,
    DrawerFooter, DrawerHeader, DrawerTitle, DrawerTrigger,
};

fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    view()
        .child(DrawerTrigger::new(drawer, "Open Drawer"))
        .child(DrawerContent::new(drawer, "Drawer", move || {
            view()
                .child(DrawerHeader::new()
                    .child(DrawerTitle::new("Drawer"))
                    .child(DrawerDescription::new("A panel at the edge.")))
                .child(DrawerFooter::new()
                    .child(DrawerClose::new(drawer, "Close")))
        }))
}"#;

const SIZED: &str = r#"DrawerContent::new(drawer, "Notification history", move || {
    view().flex_1().min_h(px(0.0))
        .child(DrawerHeader::new()
            .child(DrawerTitle::new("Notification history")))
        .child(scroll().flex_1().min_h(px(0.0)).child(text("Order confirmed")))
        .child(DrawerFooter::new().child(DrawerClose::new(drawer, "Done")))
})
.options(|| DrawerOptions {
    height: Some(360.0),
    ..Default::default()
})"#;

const FLOATING: &str = r#"DrawerContent::new(drawer, "Floating Drawer", body)
    .options(|| DrawerOptions {
        direction: Direction::Right,
        width: Some(360.0),
        inset: 16.0,
        ..Default::default()
    })"#;

const POSITION: &str = r#"DrawerContent::new(drawer, "Move Goal", body)
    .options(|| DrawerOptions {
        direction: Direction::Left,
        ..Default::default()
    })"#;

const HANDLE: &str = r#"DrawerContent::new(drawer, "Drawer", body)
    .options(|| DrawerOptions {
        show_swipe_handle: true,
        ..Default::default()
    })"#;

const NESTED: &str = r#"DrawerContent::new(parent, "Parent", move || {
    let child = Drawer::nested(parent);
    view()
        .child(DrawerTrigger::new(child, "Open Nested Drawer"))
        .child(DrawerContent::new(child, "Nested Drawer", move || {
            view().child(DrawerFooter::new()
                .child(DrawerClose::new(child, "Close")))
        }))
})"#;

const NON_MODAL: &str = r#"DrawerContent::new(drawer, "Non Modal Drawer", body)
    .options(|| DrawerOptions {
        direction: Direction::Right,
        modality: Modality::NonModal,
        disable_outside_dismissal: true,
        ..Default::default()
    })"#;

const SNAP: &str = r#"DrawerContent::new(drawer, "Snap points", body)
    .options(|| DrawerOptions {
        show_swipe_handle: true,
        snap_points: &[SnapPoint::Rem(31.0), SnapPoint::Fraction(1.0)],
        ..Default::default()
    })"#;

const RESPONSIVE: &str = r#"DrawerContent::new(drawer, "Edit profile", profile_form)
    .options(|| DrawerOptions {
        centered: window().width.0 >= 768.0,
        ..Default::default()
    })"#;

const TSX_DELIVERY: &str = include_str!("../references/drawer-tsx/drawer-demo.tsx");
const TSX_SIZED: &str = include_str!("../references/drawer-tsx/drawer-custom-size.tsx");
const TSX_FLOATING: &str = include_str!("../references/drawer-tsx/drawer-floating.tsx");
const TSX_POSITION: &str = include_str!("../references/drawer-tsx/drawer-sides.tsx");
const TSX_HANDLE: &str = include_str!("../references/drawer-tsx/drawer-swipe-handle.tsx");
const TSX_NESTED: &str = include_str!("../references/drawer-tsx/drawer-nested.tsx");
const TSX_NON_MODAL: &str = include_str!("../references/drawer-tsx/drawer-non-modal.tsx");
const TSX_SNAP: &str = include_str!("../references/drawer-tsx/drawer-snap-points.tsx");
const TSX_RESPONSIVE: &str = include_str!("../references/drawer-tsx/drawer-dialog.tsx");
const TSX_LICENSE: &str = include_str!("../references/drawer-tsx/LICENSE.md");

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum DocsPage {
    #[default]
    Drawer,
    Accordion,
    Alert,
    AlertDialog,
    AspectRatio,
    Attachment,
    Avatar,
    Badge,
    Breadcrumb,
    Bubble,
    Button,
    ButtonGroup,
    Calendar,
    Card,
    Carousel,
    Chart,
    Checkbox,
}

type ComponentRoute = (
    DocsPage,
    &'static str,
    &'static str,
    &'static str,
    fn() -> View,
);

const COMPONENTS: &[ComponentRoute] = &[
    (
        DocsPage::Accordion,
        "Accordion",
        "#accordion",
        "accordion-docs-scroll",
        accordion_page,
    ),
    (
        DocsPage::Alert,
        "Alert",
        "#alert",
        "alert-docs-scroll",
        alert_page,
    ),
    (
        DocsPage::AlertDialog,
        "Alert Dialog",
        "#alert-dialog",
        "alert-dialog-docs-scroll",
        alert_dialog_page,
    ),
    (
        DocsPage::AspectRatio,
        "Aspect Ratio",
        "#aspect-ratio",
        "aspect-ratio-docs-scroll",
        aspect_ratio_page,
    ),
    (
        DocsPage::Attachment,
        "Attachment",
        "#attachment",
        "attachment-docs-scroll",
        attachment_page,
    ),
    (
        DocsPage::Avatar,
        "Avatar",
        "#avatar",
        "avatar-docs-scroll",
        avatar_page,
    ),
    (
        DocsPage::Badge,
        "Badge",
        "#badge",
        "badge-docs-scroll",
        badge_page,
    ),
    (
        DocsPage::Breadcrumb,
        "Breadcrumb",
        "#breadcrumb",
        "breadcrumb-docs-scroll",
        breadcrumb_page,
    ),
    (
        DocsPage::Bubble,
        "Bubble",
        "#bubble",
        "bubble-docs-scroll",
        bubble_page,
    ),
    (
        DocsPage::Button,
        "Button",
        "#button",
        "button-docs-scroll",
        button_page,
    ),
    (
        DocsPage::ButtonGroup,
        "Button Group",
        "#button-group",
        "button-group-docs-scroll",
        button_group_page,
    ),
    (
        DocsPage::Calendar,
        "Calendar",
        "#calendar",
        "calendar-docs-scroll",
        calendar_page,
    ),
    (
        DocsPage::Card,
        "Card",
        "#card",
        "card-docs-scroll",
        card_page,
    ),
    (
        DocsPage::Carousel,
        "Carousel",
        "#carousel",
        "carousel-docs-scroll",
        carousel_page,
    ),
    (
        DocsPage::Chart,
        "Chart",
        "#chart",
        "chart-docs-scroll",
        chart_page,
    ),
    (
        DocsPage::Checkbox,
        "Checkbox",
        "#checkbox",
        "checkbox-docs-scroll",
        checkbox_page,
    ),
];

impl DocsPage {
    fn label(self) -> &'static str {
        COMPONENTS
            .iter()
            .find(|route| route.0 == self)
            .map_or("Drawer", |route| route.1)
    }

    fn anchor(self) -> &'static str {
        COMPONENTS
            .iter()
            .find(|route| route.0 == self)
            .map_or("#drawer", |route| route.2)
    }

    fn from_url(url: &str) -> Self {
        COMPONENTS
            .iter()
            .find(|route| url.ends_with(route.2))
            .map_or(Self::Drawer, |route| route.0)
    }
}

pub(crate) fn page() -> View {
    use nsh_core::url_scheme::HistoryBehavior;

    let page = signal(DocsPage::Drawer);
    let dark = signal(false);
    spawn(async move {
        if let Some(url) = app_url().await {
            page.set(DocsPage::from_url(&url));
        }
        loop {
            page.set(DocsPage::from_url(&next_app_url_change().await.url));
        }
    });
    let navigate = move |target: DocsPage| {
        if page.get_untracked() != target {
            page.set(target);
            let _ = set_app_url_with(target.anchor(), HistoryBehavior::Push);
        }
    };
    let toggle_theme = move || {
        dark.update(|enabled| *enabled = !*enabled);
        set_color_scheme(if dark.get() {
            Appearance::Dark
        } else {
            Appearance::Light
        });
    };

    let mut root = view().flex_1().child(when(
        move || page.get() == DocsPage::Drawer,
        move || drawer_page(navigate, toggle_theme),
    ));
    for &(target, _, _, scroll_id, article) in COMPONENTS {
        root = root.child(when(
            move || page.get() == target,
            move || component_page(target, article(), navigate, toggle_theme, scroll_id),
        ));
    }
    root
}

fn component_article(page: DocsPage, description: &'static str, reference: &'static str) -> View {
    view()
        .w_full()
        .max_w(px(820.0))
        .self_center()
        .p_5()
        .gap_6()
        .child(muted(format!("Components / {}", page.label())))
        .child(
            text(page.label())
                .role(AccessibilityRole::Header)
                .text_size(px(42.0))
                .font_weight(FontWeight::BOLD),
        )
        .child(muted(description))
        .child(view().items_start().child(button(
            "Base UI reference",
            Variant::Outline,
            move || {
                let _ = open_url(reference);
            },
        )))
}

fn accordion_page() -> View {
    let article = component_article(
        DocsPage::Accordion,
        "Interactive headings that expand and collapse sections of content.",
        "https://ui.shadcn.com/docs/components/base/accordion",
    );
    macro_rules! demo {
        ($name:ident, $title:literal) => {
            example(
                $title,
                view()
                    .w_full()
                    .max_w(px(384.0))
                    .child(accordion_examples::$name::preview()),
                include_str!(concat!("accordion_examples/", stringify!($name), ".rs")),
                include_str!(concat!(
                    "../references/accordion-tsx/accordion-",
                    stringify!($name),
                    ".tsx"
                )),
            )
        };
    }
    article
        .child(demo!(demo, "Accordion demo"))
        .child(heading("Usage"))
        .child(muted("Create an Accordion with the IDs of the items that start open. Each AccordionItem pairs a trigger and a content panel. Pass a signal with with_value to share the selection with the rest of your app."))
        .child(heading("Basic"))
        .child(demo!(basic, "Basic"))
        .child(heading("Multiple"))
        .child(demo!(multiple, "Multiple"))
        .child(heading("Disabled"))
        .child(demo!(disabled, "Disabled"))
        .child(heading("Borders"))
        .child(demo!(borders, "Borders"))
        .child(heading("Card"))
        .child(demo!(card, "Card"))
        .child(heading("RTL"))
        .child(demo!(rtl, "RTL"))
        .child(heading("API Reference"))
        .child(api_row("Accordion", "Set defaults, share state with a signal, allow multiple open items, or set the layout direction."))
        .child(api_row("AccordionItem", "Assign a stable ID and optionally disable the trigger."))
        .child(api_row("AccordionTrigger / AccordionContent", "Set the trigger label and supply any Native Shell elements as panel content."))
}

fn alert_page() -> View {
    macro_rules! demo {
        ($name:ident, $title:literal) => {
            example(
                $title,
                view()
                    .w_full()
                    .max_w(px(448.0))
                    .child(alert_examples::$name::preview()),
                include_str!(concat!("alert_examples/", stringify!($name), ".rs")),
                include_str!(concat!(
                    "../references/alert-tsx/alert-",
                    stringify!($name),
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Alert,
        "A callout that draws attention to a message or action.",
        "https://ui.shadcn.com/docs/components/base/alert",
    )
    .child(demo!(demo, "Alert demo"))
    .child(heading("Usage"))
    .child(muted(
        "Compose an Alert with a title and description. Add an optional icon or action.",
    ))
    .child(heading("Basic"))
    .child(demo!(basic, "Basic"))
    .child(heading("Destructive"))
    .child(demo!(destructive, "Destructive"))
    .child(heading("Action"))
    .child(demo!(action, "Action"))
    .child(heading("Custom Colors"))
    .child(demo!(colors, "Custom Colors"))
    .child(heading("RTL"))
    .child(demo!(rtl, "RTL"))
    .child(heading("API Reference"))
    .child(api_row(
        "Alert",
        "Set the variant, adaptive colors, or layout direction.",
    ))
    .child(api_row(
        "AlertTitle / AlertDescription",
        "Set the text or render custom description content.",
    ))
    .child(api_row(
        "AlertAction",
        "Add an action control beside the message.",
    ))
}

fn alert_dialog_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                alert_dialog_examples::$module::preview(),
                include_str!(concat!(
                    "alert_dialog_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../references/alert-dialog-tsx/alert-dialog-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::AlertDialog,
        "A modal dialog that asks the user to confirm an important action.",
        "https://ui.shadcn.com/docs/components/base/alert-dialog",
    )
    .child(demo!(demo, "demo", "Alert Dialog demo"))
    .child(heading("Usage"))
    .child(muted("Share one AlertDialog state between the trigger and content. Compose the header and footer, and pass a callback to AlertDialogAction to handle confirmation."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Small"))
    .child(demo!(small, "small", "Small"))
    .child(heading("Media"))
    .child(demo!(media, "media", "Media"))
    .child(heading("Small with Media"))
    .child(demo!(small_media, "small-media", "Small with Media"))
    .child(heading("Destructive"))
    .child(demo!(destructive, "destructive", "Destructive"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("AlertDialogContent", "Default and Small sizes. Set direction for right-to-left layouts."))
    .child(api_row("AlertDialogHeader / AlertDialogFooter", "Compose the title, description, optional media, cancel, and confirm actions."))
}

fn aspect_ratio_page() -> View {
    macro_rules! demo {
        ($name:ident, $title:literal) => {
            example(
                $title,
                aspect_ratio_examples::$name::preview(),
                include_str!(concat!("aspect_ratio_examples/", stringify!($name), ".rs")),
                include_str!(concat!(
                    "../references/aspect-ratio-tsx/aspect-ratio-",
                    stringify!($name),
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::AspectRatio,
        "A frame that keeps its proportions as the available width changes.",
        "https://ui.shadcn.com/docs/components/base/aspect-ratio",
    )
    .child(demo!(demo, "Aspect Ratio demo"))
    .child(heading("Usage"))
    .child(muted("Pass the ratio as width divided by height, then place the frame in a container with a defined width. Use object_fit(ResizeMode::Cover) to fill it with an image while preserving the image’s proportions."))
    .child(heading("Square"))
    .child(demo!(square, "Square"))
    .child(heading("Portrait"))
    .child(demo!(portrait, "Portrait"))
    .child(heading("RTL"))
    .child(demo!(rtl, "RTL"))
    .child(heading("API Reference"))
    .child(api_row("AspectRatio::new(ratio, child)", "Use a positive width-to-height ratio. The ratio can be bound to a signal."))
}

fn attachment_page() -> View {
    macro_rules! demo {
        ($name:ident, $title:literal) => {
            example(
                $title,
                attachment_examples::$name::preview(),
                include_str!(concat!("attachment_examples/", stringify!($name), ".rs")),
                include_str!(concat!(
                    "../references/attachment-tsx/attachment-",
                    stringify!($name),
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Attachment,
        "File cards with previews, status, and independent actions.",
        "https://ui.shadcn.com/docs/components/base/attachment",
    )
    .child(demo!(demo, "Attachment demo"))
    .child(heading("Usage"))
    .child(muted("Compose AttachmentMedia, AttachmentContent, and optional AttachmentActions. Set the state, size, and orientation on Attachment."))
    .child(heading("Image"))
    .child(demo!(image, "Image"))
    .child(heading("States"))
    .child(demo!(states, "States"))
    .child(heading("Sizes"))
    .child(demo!(sizes, "Sizes"))
    .child(heading("Group"))
    .child(demo!(group, "Group"))
    .child(heading("Trigger"))
    .child(demo!(trigger, "Trigger"))
    .child(heading("Accessibility"))
    .child(muted("Label the card trigger and each icon-only action. Use Left and Right to scroll a focused group. Describe failures in text."))
    .child(heading("API Reference"))
    .child(api_row("Attachment", "Set idle, uploading, processing, error, or done. Use default, small, or extra-small sizes."))
    .child(api_row("AttachmentGroup", "Arrange equal-width cards in a horizontal scrolling group."))
}

fn avatar_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                avatar_examples::$module::preview(),
                include_str!(concat!("avatar_examples/", stringify!($module), ".rs")),
                include_str!(concat!("../references/avatar-tsx/avatar-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Avatar,
        "A profile image with a fallback, optional badge, and grouping.",
        "https://ui.shadcn.com/docs/components/base/avatar",
    )
    .child(demo!(demo, "demo", "Avatar demo"))
    .child(heading("Usage"))
    .child(muted("Compose AvatarImage and AvatarFallback inside Avatar. Set the accessible name on Avatar and add an optional AvatarBadge."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Badge"))
    .child(demo!(badge, "badge", "Badge"))
    .child(heading("Badge with Icon"))
    .child(demo!(badge_icon, "badge-icon", "Badge with Icon"))
    .child(heading("Avatar Group"))
    .child(demo!(group, "group", "Avatar Group"))
    .child(heading("Avatar Group Count"))
    .child(demo!(group_count, "group-count", "Avatar Group Count"))
    .child(heading("Avatar Group with Icon"))
    .child(demo!(group_count_icon, "group-count-icon", "Avatar Group with Icon"))
    .child(heading("Sizes"))
    .child(demo!(size, "size", "Sizes"))
    .child(heading("Dropdown"))
    .child(demo!(dropdown, "dropdown", "Dropdown"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Avatar", "Use a name with an optional image, fallback, and status badge."))
    .child(api_row("AvatarGroup", "Overlap avatars and supply an accessible count or icon for additional members."))
}

fn badge_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                badge_examples::$module::preview(),
                include_str!(concat!("badge_examples/", stringify!($module), ".rs")),
                include_str!(concat!("../references/badge-tsx/badge-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Badge,
        "A compact label for a status, category, or link.",
        "https://ui.shadcn.com/docs/components/base/badge",
    )
    .child(demo!(demo, "demo", "Badge demo"))
    .child(heading("Usage"))
    .child(muted("Create a badge with a label. Set its variant or add an optional leading or trailing icon."))
    .child(heading("Variants"))
    .child(demo!(variants, "variants", "Variants"))
    .child(heading("With Icon"))
    .child(demo!(icon, "icon", "With Icon"))
    .child(heading("With Spinner"))
    .child(demo!(spinner, "spinner", "With Spinner"))
    .child(heading("Link"))
    .child(demo!(link, "link", "Link"))
    .child(heading("Custom Colors"))
    .child(demo!(colors, "colors", "Custom Colors"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Badge", "Set the variant or adaptive colors. Add icon_start or icon_end, or set href to make the badge a link."))
}

fn breadcrumb_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                breadcrumb_examples::$module::preview(),
                include_str!(concat!("breadcrumb_examples/", stringify!($module), ".rs")),
                include_str!(concat!(
                    "../references/breadcrumb-tsx/breadcrumb-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Breadcrumb,
        "A navigation trail that shows the current page and its parent pages.",
        "https://ui.shadcn.com/docs/components/base/breadcrumb",
    )
    .child(demo!(demo, "demo", "Breadcrumb demo"))
    .child(heading("Usage"))
    .child(muted("Build a BreadcrumbList from BreadcrumbItem entries and mark the current location with BreadcrumbPage. Add separators between entries."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Custom separator"))
    .child(demo!(separator, "separator", "Custom separator"))
    .child(heading("Dropdown"))
    .child(demo!(dropdown, "dropdown", "Dropdown"))
    .child(heading("Collapsed"))
    .child(demo!(ellipsis, "ellipsis", "Collapsed"))
    .child(heading("Link component"))
    .child(demo!(link, "link", "Link component"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("BreadcrumbLink", "Open a URL or supply a custom surface and navigation callback."))
    .child(api_row("BreadcrumbSeparator / BreadcrumbEllipsis", "Customize the separator or represent hidden levels."))
}

fn bubble_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                bubble_examples::$module::preview(),
                include_str!(concat!("bubble_examples/", stringify!($module), ".rs")),
                include_str!(concat!("../references/bubble-tsx/bubble-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Bubble,
        "A conversational surface for messages, actions, and reactions.",
        "https://ui.shadcn.com/docs/components/base/bubble",
    )
    .child(demo!(demo, "demo", "Bubble demo"))
    .child(heading("Usage"))
    .child(muted("Wrap a message or control in BubbleContent. Set its variant and alignment on Bubble. Use BubbleGroup for consecutive messages."))
    .child(heading("Variants"))
    .child(demo!(variants, "variants", "Variants"))
    .child(heading("Alignment"))
    .child(demo!(alignment, "alignment", "Alignment"))
    .child(heading("Bubble Group"))
    .child(demo!(group_demo, "group-demo", "Bubble Group"))
    .child(heading("Links and Buttons"))
    .child(demo!(link_button, "link-button", "Links and Buttons"))
    .child(heading("Reactions"))
    .child(demo!(reactions, "reactions", "Reactions"))
    .child(heading("Show More / Collapsible"))
    .child(demo!(collapsible, "collapsible", "Show More / Collapsible"))
    .child(heading("Tooltip"))
    .child(demo!(tooltip, "tooltip", "Tooltip"))
    .child(heading("Popover"))
    .child(demo!(popover, "popover", "Popover"))
    .child(heading("API Reference"))
    .child(api_row("Bubble / BubbleContent", "Set the appearance and alignment. Use text, a button, a link, or a custom surface."))
    .child(api_row("BubbleReactions", "Position reactions at the top or bottom. Give an emoji-only row a descriptive label."))
}

fn button_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                button_examples::$module::preview(),
                include_str!(concat!("button_examples/", stringify!($module), ".rs")),
                include_str!(concat!("../references/button-tsx/button-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Button,
        "A control that runs an action or opens a destination.",
        "https://ui.shadcn.com/docs/components/base/button",
    )
    .child(demo!(demo, "demo", "Button demo"))
    .child(heading("Usage"))
    .child(muted("Use Button::new with a label, then attach a callback with on_press or a destination with href."))
    .child(heading("Cursor"))
    .child(muted("Buttons use the default cursor. Call pointer_cursor to show a pointer on hover."))
    .child(heading("Size"))
    .child(demo!(size, "size", "Size"))
    .child(heading("Default"))
    .child(demo!(default, "default", "Default"))
    .child(heading("Outline"))
    .child(demo!(outline, "outline", "Outline"))
    .child(heading("Secondary"))
    .child(demo!(secondary, "secondary", "Secondary"))
    .child(heading("Ghost"))
    .child(demo!(ghost, "ghost", "Ghost"))
    .child(heading("Destructive"))
    .child(demo!(destructive, "destructive", "Destructive"))
    .child(heading("Link"))
    .child(demo!(link, "link", "Link"))
    .child(heading("Icon"))
    .child(demo!(icon, "icon", "Icon"))
    .child(heading("With Icon"))
    .child(demo!(with_icon, "with-icon", "With Icon"))
    .child(heading("Rounded"))
    .child(demo!(rounded, "rounded", "Rounded"))
    .child(heading("Spinner"))
    .child(demo!(spinner, "spinner", "Spinner"))
    .child(heading("Button Group"))
    .child(demo!(group_demo, "group-demo", "Button Group"))
    .child(heading("As Link"))
    .child(demo!(render, "render", "As Link"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Button", "Set a variant or size, customize the radius, or supply leading and trailing icons."))
    .child(api_row("disabled / loading", "Set either state with a bool or bind it to a signal. Loading adds a spinner and blocks activation."))
}

fn button_group_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                button_group_examples::$module::preview(),
                include_str!(concat!(
                    "button_group_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../references/button-group-tsx/button-group-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::ButtonGroup,
        "Join related buttons, inputs, and menu triggers in one labelled control group.",
        "https://ui.shadcn.com/docs/components/base/button-group",
    )
    .child(example(
        "Button Group demo",
        button_examples::group_demo::preview(),
        include_str!("button_examples/group_demo.rs"),
        include_str!("../references/button-group-tsx/button-group-demo.tsx"),
    ))
    .child(heading("Usage"))
    .child(muted("Pass an accessible label to ButtonGroup. Add native controls with child and use control to style the visible trigger of a compound control. Tab moves between individually focusable controls."))
    .child(heading("Orientation"))
    .child(demo!(orientation, "orientation", "Orientation"))
    .child(heading("Size"))
    .child(demo!(size, "size", "Size"))
    .child(heading("Nested"))
    .child(demo!(nested, "nested", "Nested"))
    .child(heading("Separator"))
    .child(demo!(separator, "separator", "Separator"))
    .child(heading("Split"))
    .child(demo!(split, "split", "Split"))
    .child(heading("Input"))
    .child(demo!(input, "input", "Input"))
    .child(heading("Input Group"))
    .child(demo!(input_group, "input-group", "Input Group"))
    .child(heading("Dropdown Menu"))
    .child(demo!(dropdown, "dropdown", "Dropdown Menu"))
    .child(heading("Select"))
    .child(demo!(select, "select", "Select"))
    .child(heading("Popover"))
    .child(demo!(popover, "popover", "Popover"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("ButtonGroup", "Join controls horizontally or vertically. Use separate to leave space between subgroups."))
    .child(api_row("ButtonGroupSeparator / ButtonGroupText", "Add a divider or styled label among a group's controls."))
}

fn calendar_page() -> View {
    macro_rules! demo {
        ($module:ident, $name:literal, $title:literal) => {
            example(
                $title,
                calendar_examples::$module::preview(),
                include_str!(concat!("calendar_examples/", stringify!($module), ".rs")),
                include_str!(concat!(
                    "../references/calendar-tsx/calendar-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Calendar,
        "An interactive calendar for selecting dates and date ranges.",
        "https://ui.shadcn.com/docs/components/base/calendar",
    )
    .child(demo!(demo, "demo", "Calendar demo"))
    .child(heading("Usage"))
    .child(muted("Use Calendar::new for a single date or range(true) for a date range. Bind a signal to value to share the selection."))
    .child(heading("Persian / Hijri / Jalali Calendar"))
    .child(demo!(hijri, "hijri", "Persian / Hijri / Jalali Calendar"))
    .child(heading("Selected Date (With TimeZone)"))
    .child(demo!(timezone, "timezone", "Selected Date (With TimeZone)"))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Range Calendar"))
    .child(demo!(range, "range", "Range Calendar"))
    .child(heading("Month and Year Selector"))
    .child(demo!(caption, "caption", "Month and Year Selector"))
    .child(heading("Presets"))
    .child(demo!(presets, "presets", "Presets"))
    .child(heading("Date and Time Picker"))
    .child(demo!(time, "time", "Date and Time Picker"))
    .child(heading("Booked dates"))
    .child(demo!(booked_dates, "booked-dates", "Booked dates"))
    .child(heading("Custom Cell Size"))
    .child(demo!(custom_days, "custom-days", "Custom Cell Size"))
    .child(heading("Week Numbers"))
    .child(demo!(week_numbers, "week-numbers", "Week Numbers"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Calendar", "Select single dates or ranges. Configure months, cell size, unavailable dates, RTL, Persian display and week numbers."))
    .child(api_row("local_today / date_at_offset", "Read the device's local calendar date or interpret a timestamp at a fixed offset."))
}

fn card_page() -> View {
    macro_rules! demo {
        ($module:ident, $function:ident, $name:literal, $title:literal) => {
            example(
                $title,
                card_examples::$module::$function(),
                include_str!(concat!("card_examples/", stringify!($module), ".rs")),
                include_str!(concat!("../references/card-tsx/card-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Card,
        "Group a title, description, content, and actions in a surface.",
        "https://ui.shadcn.com/docs/components/base/card",
    )
    .child(demo!(login, preview, "demo", "Card demo"))
    .child(heading("Composition"))
    .child(muted("Use CardHeader for CardTitle, CardDescription, and CardAction. Put the main body in CardContent and the actions in CardFooter."))
    .child(heading("Size"))
    .child(demo!(small, preview, "small", "Small"))
    .child(heading("Spacing"))
    .child(demo!(login, spacing, "spacing", "Spacing"))
    .child(demo!(edge_to_edge, preview, "edge-to-edge", "Edge to Edge"))
    .child(heading("Image"))
    .child(demo!(image, preview, "image", "Image"))
    .child(heading("RTL"))
    .child(demo!(login, rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Card", "Set a size, direction, or shared spacing signal. Use media to place a full-width cover before the header."))
    .child(api_row("CardHeader / CardTitle / CardDescription / CardAction", "Compose the card heading. The optional action sits at the end of the header."))
    .child(api_row("CardContent", "Add a main section. Use edge_to_edge to span the card and flush_footer to remove the gap above the footer."))
    .child(api_row("CardFooter", "Add an action row or a stack of controls."))
}

fn carousel_page() -> View {
    macro_rules! demo {
        ($example:ident, $name:literal, $title:literal) => {
            example(
                $title,
                carousel_examples::$example(),
                include_str!("carousel_examples.rs"),
                include_str!(concat!(
                    "../references/carousel-tsx/carousel-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Carousel,
        "Browse a series of slides using touch, the keyboard, or navigation buttons.",
        "https://ui.shadcn.com/docs/components/base/carousel",
    )
    .child(demo!(demo, "demo", "Carousel demo"))
    .child(heading("Composition"))
    .child(muted("Add CarouselItem slides to CarouselContent. Place CarouselPrevious and CarouselNext around the viewport."))
    .child(heading("Sizes"))
    .child(demo!(sizes, "size", "Sizes"))
    .child(heading("Spacing"))
    .child(demo!(spacing, "spacing", "Spacing"))
    .child(heading("Orientation"))
    .child(demo!(orientation, "orientation", "Orientation"))
    .child(heading("Options"))
    .child(muted("Use looping(true) to wrap the slides. Slides snap to the start of the viewport."))
    .child(heading("API"))
    .child(demo!(api, "api", "API"))
    .child(heading("Events"))
    .child(muted("Use on_change for selections made with the carousel. Set or read a bound index signal for programmatic navigation and the current slide."))
    .child(heading("Plugins"))
    .child(demo!(autoplay, "plugin", "Autoplay"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Carousel", "Set orientation, direction, looping, a controlled index, or an autoplay delay."))
    .child(api_row("CarouselContent / CarouselItem", "Set the track spacing, vertical viewport height, and each slide's share of the viewport."))
    .child(api_row("CarouselPrevious / CarouselNext", "The controls disable automatically at the ends and accept localized labels."))
}

fn chart_page() -> View {
    macro_rules! demo {
        ($example:ident, $name:literal, $title:literal) => {
            example(
                $title,
                chart_examples::$example(),
                include_str!("chart_examples.rs"),
                include_str!(concat!("../references/chart-tsx/chart-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Chart,
        "Build accessible, themed charts with Native Shell plots.",
        "https://ui.shadcn.com/docs/components/base/chart",
    )
    .child(demo!(demo, "demo", "Interactive Bar Chart"))
    .child(heading("Your First Chart"))
    .child(demo!(basic, "example", "Basic"))
    .child(heading("Add a Grid"))
    .child(demo!(with_grid, "example-grid", "Grid"))
    .child(heading("Add an Axis"))
    .child(demo!(with_axis, "example-axis", "Axis"))
    .child(heading("Add Tooltip"))
    .child(demo!(with_tooltip, "example-tooltip", "Add Tooltip"))
    .child(heading("Add Legend"))
    .child(demo!(with_legend, "example-legend", "Legend"))
    .child(heading("Chart Config"))
    .child(muted("ChartConfig maps each series key to a label and color. Pass values in series order and use tick to shorten the axis label."))
    .child(heading("Theming"))
    .child(muted("Use reactive theme colors such as theme().chart_1 or pass a custom color. The series color also supplies the legend and tooltip marker."))
    .child(heading("Tooltip"))
    .child(demo!(tooltip_styles, "tooltip", "Tooltip styles"))
    .child(api_row("ChartTooltipContent / ChartLegendContent", "Display series names and values with dot, line, dashed, or hidden indicators. Use formatted to supply display text for a tooltip value."))
    .child(heading("Accessibility"))
    .child(muted("Tab to the plot. The arrow keys move between points, Home and End go to either edge, and Escape clears the selected point."))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
}

fn checkbox_page() -> View {
    macro_rules! demo {
        ($example:ident, $name:literal, $title:literal) => {
            example(
                $title,
                checkbox_examples::$example(),
                include_str!("checkbox_examples.rs"),
                include_str!(concat!(
                    "../references/checkbox-tsx/checkbox-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Checkbox,
        "Select an option or indicate a partial selection.",
        "https://ui.shadcn.com/docs/components/base/checkbox",
    )
    .child(demo!(demo, "demo", "Checkbox demo"))
    .child(heading("Checked State"))
    .child(muted("Use default_checked to set the initial value. To control a checkbox, pass checked and update the state in on_change. Pressing a mixed checkbox requests a checked value."))
    .child(heading("Invalid State"))
    .child(demo!(invalid, "invalid", "Invalid State"))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Description"))
    .child(demo!(description, "description", "Description"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Group"))
    .child(demo!(group, "group", "Group"))
    .child(heading("Table"))
    .child(demo!(table, "table", "Table"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Checkbox", "An accessible toggle with a label, optional description, and a checked, unchecked, or mixed value. Use icon_only for a labeled box inside a table or another compact layout."))
}

fn component_page(
    page: DocsPage,
    article: View,
    navigate: impl Fn(DocsPage) + Copy + 'static,
    toggle_theme: impl Fn() + 'static,
    scroll_id: &'static str,
) -> View {
    let menu = signal(false);
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .items_center()
                .justify_between()
                .min_h(px(58.0))
                .px_4()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(text("nsh-foundation").font_weight(FontWeight::SEMIBOLD))
                .child(
                    view()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap_1()
                        .child(button("Components", Variant::Ghost, move || {
                            menu.update(|open| *open = !*open)
                        }))
                        .child(button("Theme", Variant::Ghost, toggle_theme)),
                ),
        )
        .child(when(menu, move || {
            scroll()
                .max_h(px(240.0))
                .flex_shrink_0()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(
                    component_links(page, move |target| {
                        menu.set(false);
                        navigate(target);
                    })
                    .px_4()
                    .py_2()
                    .items_start(),
                )
        }))
        .child(
            scroll()
                .flex_1()
                .min_h(px(0.0))
                .test_id(scroll_id)
                .child(article),
        )
}

fn component_links(page: DocsPage, navigate: impl Fn(DocsPage) + Copy + 'static) -> View {
    let mut links = view()
        .role(AccessibilityRole::Navigation)
        .label("Components");
    for target in COMPONENTS
        .iter()
        .map(|route| route.0)
        .chain([DocsPage::Drawer])
    {
        if target != page {
            links = links.child(button(target.label(), Variant::Ghost, move || {
                navigate(target)
            }));
        }
    }
    links
}

fn drawer_page(
    navigate: impl Fn(DocsPage) + Copy + 'static,
    toggle_theme: impl Fn() + Copy + 'static,
) -> View {
    let scroll_ref = element_ref();
    let locations = stored([0.0_f32; SECTIONS.len()]);
    let menu = signal(false);
    let selected = signal(None::<usize>);
    let status = signal(None::<SharedString>);
    let show_tsx_license = signal(false);

    let go = move |index: usize| {
        selected.set(Some(index));
        let y = locations.with(|items| items[index]);
        scroll_ref.scroll_to(
            Point {
                x: px(0.0),
                y: px((y - 18.0).max(0.0)),
            },
            false,
        );
        menu.set(false);
    };

    let (delivery, delivery_panel) = examples::delivery(status);
    let (custom_size, custom_size_panel) = examples::custom_size();
    let (floating, floating_panel) = examples::floating();
    let (positions, position_panels) = examples::positions();
    let (swipe, swipe_panel) = examples::swipe();
    let (nested, nested_panel) = examples::nested();
    let (non_modal, non_modal_panel) = examples::non_modal();
    let (snap, snap_panel) = examples::snap();
    let (responsive, responsive_panel) = examples::responsive(status);

    let article = view()
        .max_w(px(820.0))
        .w_full()
        .self_center()
        .px(move || px(if window().width.0 < 760.0 { 22.0 } else { 36.0 }))
        .pt_8()
        .pb_16()
        .gap_8()
        .child(
            view().gap_3()
                .child(view().flex_row().items_center().gap_2()
                    .child(muted("Components"))
                    .child(muted("/"))
                    .child(text("Drawer").text_sm().font_weight(FontWeight::MEDIUM)))
                .child(
                    text("Drawer")
                        .role(AccessibilityRole::Header)
                        .text_size(px(42.0))
                        .font_weight(FontWeight::BOLD),
                )
                .child(muted("A swipeable panel that opens from an edge of the screen."))
                .child(view().flex_row().gap_2()
                    .child(button("Base UI reference", Variant::Outline, || { let _ = open_url(REFERENCE); }))
                    .child(button("Source", Variant::Ghost, || { let _ = open_url(SOURCE); }))),
        )
        .child(example("Delivery time", delivery, SIMPLE, TSX_DELIVERY))
        .child(delivery_panel)
        .child(
            section("Installation", 0, locations)
                .child(muted("Build the Native Shell Rust project for the Web target."))
                .child(code_block("nsh package --target web --out dist")),
        )
        .child(
            section("Usage", 1, locations)
                .child(muted("Pass the same Drawer to DrawerTrigger and DrawerContent. Build the content from DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, and DrawerClose."))
                .child(code_block(SIMPLE))
                .child(muted("The TSX tabs show React examples for shadcn/ui with Base UI. Use them in a project with the referenced components installed. The delivery example also needs Sonner’s Toaster."))
                .child(view().items_start().child(button("TSX license", Variant::Outline, move || {
                    show_tsx_license.update(|open| *open = !*open);
                })))
                .child(when(show_tsx_license, || code_block(TSX_LICENSE))),
        )
        .child(section("Composition", 2, locations)
            .child(muted("The trigger can sit anywhere in the layout. DrawerContent supplies the window layer and backdrop, and mounts its children when the drawer opens."))
            .child(code_block("Drawer\n├── DrawerTrigger\n└── DrawerContent\n    ├── DrawerPortal + DrawerOverlay\n    ├── DrawerHeader\n    │   ├── DrawerTitle\n    │   └── DrawerDescription\n    └── DrawerFooter\n        └── DrawerClose")))
        .child(section("Custom Sizes", 3, locations)
            .child(muted("Use height for vertical drawers and width for side drawers. Put long content in a scroll view."))
            .child(example("Custom Sizes", custom_size, SIZED, TSX_SIZED))
            .child(custom_size_panel))
        .child(section("Styling", 4, locations)
            .child(muted("Edge drawers sit 8 points from the viewport by default. Set inset to override the gap. The header and footer provide 16 points of side padding. Use p_4() on a custom body to align it with them."))
            .child(example("Styling", floating, FLOATING, TSX_FLOATING))
            .child(floating_panel))
        .child(section("Examples", 5, locations))
        .child(section("Position", 6, locations)
            .child(muted("Open the drawer from any of the four screen edges."))
            .child(example("Position", positions, POSITION, TSX_POSITION))
            .child(position_panels))
        .child(section("Swipe Handle", 7, locations)
            .child(muted("Drag the handle toward the edge to dismiss the drawer."))
            .child(example("Swipe Handle", swipe, HANDLE, TSX_HANDLE))
            .child(swipe_panel))
        .child(section("Nested", 8, locations)
            .child(muted("Open up to four drawers. Close the topmost drawer to return to the previous one."))
            .child(example("Nested", nested, NESTED, TSX_NESTED))
            .child(nested_panel))
        .child(section("Non Modal", 9, locations)
            .child(muted("Scroll and interact with this page while the drawer stays open."))
            .child(example("Non Modal", non_modal, NON_MODAL, TSX_NON_MODAL)))
        .child(section("Snap Points", 10, locations)
            .child(muted("Drag the drawer between a compact peek and its full height."))
            .child(example("Snap Points", snap, SNAP, TSX_SNAP))
            .child(snap_panel))
        .child(section("Responsive", 11, locations)
            .child(muted("Edit the profile in a desktop dialog or a mobile drawer."))
            .child(example("Responsive", responsive, RESPONSIVE, TSX_RESPONSIVE))
            .child(responsive_panel))
        .child(section("API Reference", 12, locations)
            .child(muted("Use Drawer::new with an open signal. Drawer::nested links a child to a parent that remains mounted."))
            .child(api_row("DrawerTrigger / DrawerClose", "Use new for the default button or render to attach drawer behavior to your own control."))
            .child(api_row("DrawerContent", "Creates the window layer and backdrop. Set placement, size, and behavior with options."))
            .child(api_row("DrawerHeader / DrawerTitle / DrawerDescription / DrawerFooter", "Compose the visible content with ordinary Native Shell children and styles."))
            .child(api_row("DrawerPortal / DrawerOverlay / DrawerSwipeHandle", "Customize the window layer or backdrop, or style the generated drag handle."))
            .child(api_row("direction", "Edge of the viewport. Defaults to Down."))
            .child(api_row("modality", "Modal traps focus. NonModal leaves the page interactive."))
            .child(api_row("disable_outside_dismissal", "Keeps the drawer open on outside presses."))
            .child(api_row("height / width / inset", "Panel dimensions and distance from the viewport edges, in points."))
            .child(api_row("snap_points / snap_index", "Vertical detents and the active detent signal."))
            .child(api_row("centered", "Displays a centered dialog for the responsive example.")));

    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(58.0))
                .flex_shrink_0()
                .px_5()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(text("◩").text_lg())
                        .child(text("nsh-foundation").font_weight(FontWeight::SEMIBOLD))
                        .child(when(
                            || window().width.0 >= 760.0,
                            move || button("Components", Variant::Ghost, move || go(5)),
                        )),
                )
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(when(
                            || window().width.0 < 1220.0,
                            move || {
                                button("Contents", Variant::Ghost, move || {
                                    menu.update(|open| *open = !*open)
                                })
                            },
                        ))
                        .child(
                            button("Accordion", Variant::Ghost, move || {
                                navigate(DocsPage::Accordion)
                            })
                            .test_id("open-accordion-docs"),
                        )
                        .child(button("Theme", Variant::Ghost, toggle_theme)),
                ),
        )
        .child(when(menu, move || {
            view()
                .p_3()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(
                    component_links(DocsPage::Drawer, navigate)
                        .flex_row()
                        .flex_wrap(),
                )
                .child(section_links(go, selected))
        }))
        .child(
            view()
                .flex_1()
                .min_h(px(0.0))
                .flex_row()
                .child(when(
                    || window().width.0 >= 940.0,
                    move || sidebar(go, navigate),
                ))
                .child(
                    scroll()
                        .flex_1()
                        .min_w(px(0.0))
                        .element_ref(&scroll_ref)
                        .test_id("drawer-docs-scroll")
                        .on_scroll(move |event: &ScrollEvent| {
                            let next = locations.with(|positions| {
                                positions.iter().rposition(|position| {
                                    *position > 0.0 && *position <= event.offset.y.0 + 32.0
                                })
                            });
                            selected.set(next);
                        })
                        .child(article),
                )
                .child(when(
                    || window().width.0 >= 1220.0,
                    move || {
                        view()
                            .w(px(196.0))
                            .flex_shrink_0()
                            .pt_8()
                            .pr_5()
                            .gap_2()
                            .child(
                                text("On this page")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD),
                            )
                            .child(section_links(go, selected))
                    },
                )),
        )
        .child(non_modal_panel)
        .child(when(
            move || status.get().is_some(),
            move || {
                view()
                    .absolute()
                    .right_4()
                    .bottom_4()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .rounded(px(10.0))
                    .border(px(1.0))
                    .border_color(move || theme().border)
                    .bg(move || theme().popover)
                    .shadow_lg()
                    .role(AccessibilityRole::Alert)
                    .child(text(move || status.get().unwrap_or_default()).text_sm())
                    .child(button("Dismiss", Variant::Ghost, move || status.set(None)))
            },
        ))
}

fn section(name: &'static str, index: usize, locations: Stored<[f32; SECTIONS.len()]>) -> View {
    view()
        .gap_3()
        .pt_5()
        .border_t(px(1.0))
        .border_color(move || theme().border)
        .on_layout(move |bounds: &Bounds<Points>| {
            locations.with(|items| items[index] = bounds.origin.y.0);
        })
        .child(heading(name))
}

fn section_links(go: impl Fn(usize) + Copy + 'static, selected: Signal<Option<usize>>) -> View {
    let mut links = view()
        .role(AccessibilityRole::Navigation)
        .label("Drawer sections");
    for (index, name) in SECTIONS.into_iter().enumerate() {
        links = links.child(
            view()
                .role(AccessibilityRole::Link)
                .label(name)
                .cursor_pointer()
                .py_1()
                .px_2()
                .rounded(px(6.0))
                .style_when(
                    move || selected.get() == Some(index),
                    |style| style.bg(theme().muted),
                )
                .on_press(move || go(index))
                .child(text(name).text_sm().text_color(move || {
                    if selected.get() == Some(index) {
                        theme().foreground
                    } else {
                        theme().muted_foreground
                    }
                })),
        );
    }
    links
}

fn sidebar(
    go: impl Fn(usize) + Copy + 'static,
    navigate: impl Fn(DocsPage) + Copy + 'static,
) -> View {
    view()
        .w(px(215.0))
        .flex_shrink_0()
        .p_5()
        .gap_4()
        .border_r(px(1.0))
        .border_color(move || theme().border)
        .child(
            text("Documentation")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD),
        )
        .child(
            view().gap_2().child(muted("Getting Started")).child(
                view()
                    .items_start()
                    .child(button("Installation", Variant::Ghost, move || go(0))),
            ),
        )
        .child(
            view()
                .gap_2()
                .child(muted("Components"))
                .child(component_links(DocsPage::Drawer, navigate).items_start())
                .child(view().items_start().child(button(
                    "Drawer",
                    Variant::Secondary,
                    move || go(5),
                ))),
        )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExampleTab {
    Preview,
    Rust,
    Tsx,
}

fn example(
    label: &'static str,
    contents: impl IntoElement,
    rust: &'static str,
    tsx: &'static str,
) -> View {
    let selected = signal(ExampleTab::Preview);
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .border(px(1.0))
        .border_color(move || theme().border)
        .rounded(px(12.0))
        .overflow_hidden()
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_1()
                .px_3()
                .py_2()
                .border_b(px(1.0))
                .border_color(move || theme().border)
                .child(tab("Preview", selected, ExampleTab::Preview))
                .child(tab("Rust", selected, ExampleTab::Rust))
                .child(tab("TSX", selected, ExampleTab::Tsx)),
        )
        .child(
            view()
                .style_when(
                    move || selected.get() != ExampleTab::Preview,
                    |style| style.hidden(),
                )
                .min_h(px(248.0))
                .p_4()
                .items_center()
                .justify_center()
                .child(contents),
        )
        .child(when(
            move || selected.get() == ExampleTab::Rust,
            move || code_block(rust),
        ))
        .child(when(
            move || selected.get() == ExampleTab::Tsx,
            move || {
                view()
                    .child(
                        view()
                            .px_3()
                            .py_2()
                            .child(muted("shadcn/ui · React + Base UI")),
                    )
                    .child(code_panel(tsx, true))
            },
        ))
}

fn tab(label: &'static str, selected: Signal<ExampleTab>, value: ExampleTab) -> View {
    button(label, Variant::Ghost, move || selected.set(value))
        .accessibility_state(move || AccessibilityState {
            selected: selected.get() == value,
            ..Default::default()
        })
        .bg(move || {
            if selected.get() == value {
                theme().muted
            } else {
                hsla(0.0, 0.0, 0.0, 0.0).into()
            }
        })
}

fn code_block(value: &'static str) -> View {
    code_panel(value, false)
}

fn code_panel(value: &'static str, scrollable: bool) -> View {
    let copied = signal(false);
    let reset = stored(None::<Spawned<()>>);
    view()
        .p_3()
        .gap_2()
        .rounded(px(8.0))
        .bg(move || theme().muted)
        .child(
            view().items_end().child(
                button("Copy", Variant::Outline, move || {
                    set_clipboard(value);
                    copied.set(true);
                    if let Some(task) = reset.replace(None) {
                        task.cancel();
                    }
                    reset.replace(Some(set_timeout(
                        std::time::Duration::from_secs(2),
                        move || copied.set(false),
                    )));
                })
                .child(when(copied, || muted("Copied"))),
            ),
        )
        .child(if scrollable {
            scroll().h(px(340.0)).child(code_text(value)).into_element()
        } else {
            code_text(value).into_element()
        })
}

fn code_text(value: &'static str) -> Text {
    text(value)
        .text_sm()
        .font_family("monospace")
        .selectable(true)
}

fn api_row(name: &'static str, description: &'static str) -> View {
    view()
        .gap_1()
        .p_3()
        .rounded(px(8.0))
        .bg(move || theme().muted)
        .child(text(name).text_sm().font_family("monospace"))
        .child(muted(description))
}
