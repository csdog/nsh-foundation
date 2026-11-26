use crate::tsx_examples::{self, TsxExample};
use nsh_base::{PopoverEvent, popover, theme};
use nsh_core::{
    RevealOptions, ScrollAlignment,
    element::{app_url, next_app_url_change, set_app_url_with, set_color_scheme},
    platform::Appearance,
    prelude::*,
};

use crate::demo_source::DemoSource;
use crate::examples::{
    accordion_examples, alert_dialog_examples, alert_examples, aspect_ratio_examples,
    attachment_examples, avatar_examples, avatar_rsx, avatar_rust, badge_examples,
    breadcrumb_examples, bubble_examples, button_examples, button_group_examples,
    calendar_examples, card_examples, carousel_examples, chart_examples, chart_trends,
    checkbox_examples, code_block_examples, collapsible_examples, combobox_examples,
    command_examples, composition, context_menu_examples, data_table_examples,
    date_picker_examples, dialog_examples, drawer, drawer_rsx, drawer_rust, dropdown_menu_examples,
    empty_examples, field_examples, hover_card_examples, input_examples, input_group_examples,
    input_otp_examples, item_examples, kbd_examples, label_examples, marker_examples,
    menubar_examples, message_examples, message_scroller_examples, native_select_examples,
    navigation_menu_examples, pagination_examples, popover_examples, progress_examples,
    questionnaire_examples, radio_group_examples, resizable_examples, scroll_area_examples,
    select_examples, separator_examples, sheet_examples, sidebar_examples, skeleton_examples,
    slider_examples, spinner_examples, switch_examples, table_examples, tabs_examples,
    textarea_examples, toast_examples, toggle_examples, toggle_group_examples, tooltip_examples,
    trace_examples, typography_examples,
};
use components::{
    button::{Button, ButtonSize, ButtonVariant},
    code_block::{
        CodeBlock, CodeBlockActions, CodeBlockContent, CodeBlockCopyButton, CodeBlockHeader,
    },
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    drawer::Drawer,
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
    toast::Toaster,
    ui::{Variant, button, button_surface, heading, muted},
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

const DRAWER_SOURCE: &str = include_str!("examples/drawer.rs");

const TSX_SIZED: &str = include_str!("../../../references/drawer-tsx/drawer-custom-size.tsx");
const TSX_FLOATING: &str = include_str!("../../../references/drawer-tsx/drawer-floating.tsx");
const TSX_POSITION: &str = include_str!("../../../references/drawer-tsx/drawer-sides.tsx");
const TSX_HANDLE: &str = include_str!("../../../references/drawer-tsx/drawer-swipe-handle.tsx");
const TSX_NESTED: &str = include_str!("../../../references/drawer-tsx/drawer-nested.tsx");
const TSX_NON_MODAL: &str = include_str!("../../../references/drawer-tsx/drawer-non-modal.tsx");
const TSX_SNAP: &str = include_str!("../../../references/drawer-tsx/drawer-snap-points.tsx");
const TSX_RESPONSIVE: &str = include_str!("../../../references/drawer-tsx/drawer-dialog.tsx");

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum DocsPage {
    #[default]
    Drawer,
    Blocks,
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
    CodeBlock,
    Collapsible,
    Combobox,
    Command,
    ContextMenu,
    DataTable,
    DatePicker,
    Dialog,
    Direction,
    DropdownMenu,
    Empty,
    Field,
    HoverCard,
    Input,
    InputGroup,
    InputOtp,
    Item,
    Kbd,
    Label,
    Marker,
    Menubar,
    Message,
    MessageScroller,
    NativeSelect,
    NavigationMenu,
    Pagination,
    Popover,
    Progress,
    Questionnaire,
    RadioGroup,
    Resizable,
    ScrollArea,
    Select,
    Separator,
    Sheet,
    Sidebar,
    Skeleton,
    Slider,
    Spinner,
    Switch,
    Table,
    Tabs,
    Textarea,
    Toast,
    Toggle,
    ToggleGroup,
    Tooltip,
    Trace,
    Typography,
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
    (
        DocsPage::CodeBlock,
        "Code Block",
        "#code-block",
        "code-block-docs-scroll",
        code_block_page,
    ),
    (
        DocsPage::Collapsible,
        "Collapsible",
        "#collapsible",
        "collapsible-docs-scroll",
        collapsible_page,
    ),
    (
        DocsPage::Combobox,
        "Combobox",
        "#combobox",
        "combobox-docs-scroll",
        combobox_page,
    ),
    (
        DocsPage::Command,
        "Command",
        "#command",
        "command-docs-scroll",
        command_page,
    ),
    (
        DocsPage::ContextMenu,
        "Context Menu",
        "#context-menu",
        "context-menu-docs-scroll",
        context_menu_page,
    ),
    (
        DocsPage::DataTable,
        "Data Table",
        "#data-table",
        "data-table-docs-scroll",
        data_table_page,
    ),
    (
        DocsPage::DatePicker,
        "Date Picker",
        "#date-picker",
        "date-picker-docs-scroll",
        date_picker_page,
    ),
    (
        DocsPage::Dialog,
        "Dialog",
        "#dialog",
        "dialog-docs-scroll",
        dialog_page,
    ),
    (
        DocsPage::Direction,
        "Direction",
        "#direction",
        "direction-docs-scroll",
        direction_page,
    ),
    (
        DocsPage::DropdownMenu,
        "Dropdown Menu",
        "#dropdown-menu",
        "dropdown-menu-docs-scroll",
        dropdown_menu_page,
    ),
    (
        DocsPage::Empty,
        "Empty",
        "#empty",
        "empty-docs-scroll",
        empty_page,
    ),
    (
        DocsPage::Field,
        "Field",
        "#field",
        "field-docs-scroll",
        field_page,
    ),
    (
        DocsPage::HoverCard,
        "Hover Card",
        "#hover-card",
        "hover-card-docs-scroll",
        hover_card_page,
    ),
    (
        DocsPage::Input,
        "Input",
        "#input",
        "input-docs-scroll",
        input_page,
    ),
    (
        DocsPage::InputGroup,
        "Input Group",
        "#input-group",
        "input-group-docs-scroll",
        input_group_page,
    ),
    (
        DocsPage::InputOtp,
        "Input OTP",
        "#input-otp",
        "input-otp-docs-scroll",
        input_otp_page,
    ),
    (
        DocsPage::Item,
        "Item",
        "#item",
        "item-docs-scroll",
        item_page,
    ),
    (DocsPage::Kbd, "Kbd", "#kbd", "kbd-docs-scroll", kbd_page),
    (
        DocsPage::Label,
        "Label",
        "#label",
        "label-docs-scroll",
        label_page,
    ),
    (
        DocsPage::Marker,
        "Marker",
        "#marker",
        "marker-docs-scroll",
        marker_page,
    ),
    (
        DocsPage::Menubar,
        "Menubar",
        "#menubar",
        "menubar-docs-scroll",
        menubar_page,
    ),
    (
        DocsPage::Message,
        "Message",
        "#message",
        "message-docs-scroll",
        message_page,
    ),
    (
        DocsPage::MessageScroller,
        "Message Scroller",
        "#message-scroller",
        "message-scroller-docs-scroll",
        message_scroller_page,
    ),
    (
        DocsPage::NativeSelect,
        "Native Select",
        "#native-select",
        "native-select-docs-scroll",
        native_select_page,
    ),
    (
        DocsPage::NavigationMenu,
        "Navigation Menu",
        "#navigation-menu",
        "navigation-menu-docs-scroll",
        navigation_menu_page,
    ),
    (
        DocsPage::Pagination,
        "Pagination",
        "#pagination",
        "pagination-docs-scroll",
        pagination_page,
    ),
    (
        DocsPage::Popover,
        "Popover",
        "#popover",
        "popover-docs-scroll",
        popover_page,
    ),
    (
        DocsPage::Progress,
        "Progress",
        "#progress",
        "progress-docs-scroll",
        progress_page,
    ),
    (
        DocsPage::Questionnaire,
        "Questionnaire",
        "#questionnaire",
        "questionnaire-docs-scroll",
        questionnaire_page,
    ),
    (
        DocsPage::RadioGroup,
        "Radio Group",
        "#radio-group",
        "radio-group-docs-scroll",
        radio_group_page,
    ),
    (
        DocsPage::Resizable,
        "Resizable",
        "#resizable",
        "resizable-docs-scroll",
        resizable_page,
    ),
    (
        DocsPage::ScrollArea,
        "Scroll Area",
        "#scroll-area",
        "scroll-area-docs-scroll",
        scroll_area_page,
    ),
    (
        DocsPage::Select,
        "Select",
        "#select",
        "select-docs-scroll",
        select_page,
    ),
    (
        DocsPage::Separator,
        "Separator",
        "#separator",
        "separator-docs-scroll",
        separator_page,
    ),
    (
        DocsPage::Sheet,
        "Sheet",
        "#sheet",
        "sheet-docs-scroll",
        sheet_page,
    ),
    (
        DocsPage::Sidebar,
        "Sidebar",
        "#sidebar",
        "sidebar-docs-scroll",
        sidebar_page,
    ),
    (
        DocsPage::Skeleton,
        "Skeleton",
        "#skeleton",
        "skeleton-docs-scroll",
        skeleton_page,
    ),
    (
        DocsPage::Slider,
        "Slider",
        "#slider",
        "slider-docs-scroll",
        slider_page,
    ),
    (
        DocsPage::Spinner,
        "Spinner",
        "#spinner",
        "spinner-docs-scroll",
        spinner_page,
    ),
    (
        DocsPage::Switch,
        "Switch",
        "#switch",
        "switch-docs-scroll",
        switch_page,
    ),
    (
        DocsPage::Table,
        "Table",
        "#table",
        "table-docs-scroll",
        table_page,
    ),
    (
        DocsPage::Tabs,
        "Tabs",
        "#tabs",
        "tabs-docs-scroll",
        tabs_page,
    ),
    (
        DocsPage::Textarea,
        "Textarea",
        "#textarea",
        "textarea-docs-scroll",
        textarea_page,
    ),
    (
        DocsPage::Toast,
        "Toast",
        "#toast",
        "toast-docs-scroll",
        toast_page,
    ),
    (
        DocsPage::Toggle,
        "Toggle",
        "#toggle",
        "toggle-docs-scroll",
        toggle_page,
    ),
    (
        DocsPage::ToggleGroup,
        "Toggle Group",
        "#toggle-group",
        "toggle-group-docs-scroll",
        toggle_group_page,
    ),
    (
        DocsPage::Tooltip,
        "Tooltip",
        "#tooltip",
        "tooltip-docs-scroll",
        tooltip_page,
    ),
    (
        DocsPage::Trace,
        "Trace",
        "#trace",
        "trace-docs-scroll",
        trace_page,
    ),
    (
        DocsPage::Typography,
        "Typography",
        "#typography",
        "typography-docs-scroll",
        typography_page,
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
        if self == Self::Blocks {
            return "#blocks";
        }
        COMPONENTS
            .iter()
            .find(|route| route.0 == self)
            .map_or("#drawer", |route| route.2)
    }

    fn from_url(url: &str) -> Self {
        if url.ends_with(Self::Blocks.anchor()) {
            return Self::Blocks;
        }
        COMPONENTS
            .iter()
            .find(|route| url.ends_with(route.2))
            .map_or(Self::Drawer, |route| route.0)
    }
}

pub(crate) fn page() -> View {
    use nsh_core::url_scheme::HistoryBehavior;

    let page = signal(None::<DocsPage>);
    let sidebar_offset = stored(Point::<Points>::default());
    spawn(async move {
        let initial = app_url().await;
        page.set(Some(
            initial
                .as_deref()
                .map_or(DocsPage::Drawer, DocsPage::from_url),
        ));
        loop {
            page.set(Some(DocsPage::from_url(&next_app_url_change().await.url)));
        }
    });
    let navigate = move |target: DocsPage| {
        if page.get_untracked() != Some(target) {
            page.set(Some(target));
            let _ = set_app_url_with(target.anchor(), HistoryBehavior::Push);
        }
    };
    let toggle_theme = move || {
        set_color_scheme(if dark() {
            Appearance::Light
        } else {
            Appearance::Dark
        });
    };

    view()
        .flex_1()
        .child(when(
            move || page.get().is_some_and(|page| page != DocsPage::Blocks),
            move || docs_layout(page, navigate, toggle_theme, sidebar_offset),
        ))
        .child(when(
            move || page.get() == Some(DocsPage::Blocks),
            move || crate::blocks::page(move || navigate(DocsPage::Drawer), toggle_theme),
        ))
}

fn docs_layout(
    page: Signal<Option<DocsPage>>,
    navigate: impl Fn(DocsPage) + Copy + 'static,
    toggle_theme: impl Fn() + Copy + 'static,
    sidebar_offset: Stored<Point<Points>>,
) -> View {
    let sections = DrawerSections {
        scroll_ref: element_ref(),
        locations: stored([0.0; SECTIONS.len()]),
        selected: signal(None),
    };
    let non_modal = Drawer::new(signal(false));
    let is_drawer = move || page.get() == Some(DocsPage::Drawer);
    let mut articles = view()
        .flex_1()
        .min_h(px(0.0))
        .min_w(px(0.0))
        .child(when(is_drawer, move || drawer_page(sections, non_modal)));
    for &(target, _, _, scroll_id, article) in COMPONENTS {
        articles = articles.child(when(
            move || page.get() == Some(target),
            move || {
                scroll()
                    .flex_1()
                    .min_w(px(0.0))
                    .test_id(scroll_id)
                    .child(article())
            },
        ));
    }
    view()
        .flex_1()
        .bg(|| theme().background)
        .text_color(|| theme().foreground)
        .child(dynamic(move || {
            let current = page.get().unwrap_or(DocsPage::Drawer);
            if current == DocsPage::Drawer {
                drawer_header(navigate, toggle_theme, sections)
            } else {
                docs_header(
                    move || mobile_navigation(current, navigate, |_| None),
                    || None,
                    move || navigate(DocsPage::Drawer),
                    move || navigate(DocsPage::Blocks),
                    toggle_theme,
                )
            }
        }))
        .child(
            view()
                .flex_1()
                .min_h(px(0.0))
                .flex_row()
                .child(when(desktop_navigation, move || {
                    sidebar(page, navigate, sidebar_offset)
                }))
                .child(articles),
        )
        .child(when(is_drawer, move || drawer::non_modal_panel(non_modal)))
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
                include_str!(concat!(
                    "examples/accordion_examples/",
                    stringify!($name),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/accordion-tsx/accordion-",
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
                include_str!(concat!(
                    "examples/alert_examples/",
                    stringify!($name),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/alert-tsx/alert-",
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
                    "examples/alert_dialog_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/alert-dialog-tsx/alert-dialog-",
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
                include_str!(concat!(
                    "examples/aspect_ratio_examples/",
                    stringify!($name),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/aspect-ratio-tsx/aspect-ratio-",
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
                include_str!(concat!(
                    "examples/attachment_examples/",
                    stringify!($name),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/attachment-tsx/attachment-",
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
        ($rust:ident, $rsx:ident, $tsx:ident, $title:literal) => {
            example_with_rsx(
                $title,
                avatar_examples::$rust::example(),
                avatar_examples::$rust::CODE,
                Some(tsx_examples::$tsx::EXAMPLE),
                Some(avatar_examples::$rsx::CODE),
            )
        };
    }
    component_article(
        DocsPage::Avatar,
        "A profile image with a fallback, optional badge, and grouping.",
        "https://ui.shadcn.com/docs/components/base/avatar",
    )
    .child(example_with_rsx(
        "Avatar demo",
        avatar_rsx::example(),
        avatar_rust::CODE,
        Some(tsx_examples::avatar::EXAMPLE),
        Some(avatar_rsx::CODE),
    ))
    .child(demo!(demo_rust, demo, avatar_demo, "Avatar group preview"))
    .child(heading("Usage"))
    .child(muted("Compose AvatarImage and AvatarFallback inside Avatar. Set the accessible name on Avatar and add an optional AvatarBadge."))
    .child(heading("Basic"))
    .child(demo!(basic_rust, basic, avatar_basic, "Basic"))
    .child(heading("Badge"))
    .child(demo!(badge_rust, badge, avatar_badge, "Badge"))
    .child(heading("Badge with Icon"))
    .child(demo!(badge_icon_rust, badge_icon, avatar_badge_icon, "Badge with Icon"))
    .child(heading("Avatar Group"))
    .child(demo!(group_rust, group, avatar_group, "Avatar Group"))
    .child(heading("Avatar Group Count"))
    .child(demo!(group_count_rust, group_count, avatar_group_count, "Avatar Group Count"))
    .child(heading("Avatar Group with Icon"))
    .child(demo!(group_count_icon_rust, group_count_icon, avatar_group_count_icon, "Avatar Group with Icon"))
    .child(heading("Sizes"))
    .child(demo!(size_rust, size, avatar_size, "Sizes"))
    .child(heading("Dropdown"))
    .child(demo!(dropdown_rust, dropdown, avatar_dropdown, "Dropdown"))
    .child(heading("RTL"))
    .child(demo!(rtl_rust, rtl, avatar_rtl, "RTL"))
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
                include_str!(concat!(
                    "examples/badge_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/badge-tsx/badge-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Badge,
        "A compact label for a status, category, or link.",
        "https://ui.shadcn.com/docs/components/base/badge",
    )
    .child(example_with_rsx(
        "Badge composition",
        composition::badge_rsx::example(),
        composition::badge_rust::CODE,
        Some(tsx_examples::badge::EXAMPLE),
        Some(composition::badge_rsx::CODE),
    ))
    .child(heading("Usage"))
    .child(muted("Use badge().child(...) to supply visible content. Set the variant or add an optional leading or trailing icon."))
    .child(muted("To use this TSX example, call components::badge::tsx::register in your nsh-foundation/native module."))
    .child(demo!(demo, "demo", "Badge demo"))
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
                include_str!(concat!(
                    "examples/breadcrumb_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/breadcrumb-tsx/breadcrumb-",
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
                include_str!(concat!(
                    "examples/bubble_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/bubble-tsx/bubble-",
                    $name,
                    ".tsx"
                )),
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
                include_str!(concat!(
                    "examples/button_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/button-tsx/button-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Button,
        "A control that runs an action or opens a destination.",
        "https://ui.shadcn.com/docs/components/base/button",
    )
    .child(example_with_rsx(
        "Button composition",
        composition::button_rsx::example(),
        composition::button_rust::CODE,
        Some(tsx_examples::button::EXAMPLE),
        Some(composition::button_rsx::CODE),
    ))
    .child(heading("Usage"))
    .child(muted("Use button().child(...) for visible content. Set accessibility_label for icon-only buttons. Apply style_with before attaching an on_press callback or href."))
    .child(muted("To use this TSX example, call components::button::tsx::register in your nsh-foundation/native module."))
    .child(demo!(demo, "demo", "Button demo"))
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
                    "examples/button_group_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/button-group-tsx/button-group-",
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
        include_str!("examples/button_examples/group_demo.rs"),
        include_str!("../../../references/button-group-tsx/button-group-demo.tsx"),
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
                include_str!(concat!(
                    "examples/calendar_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!(
                    "../../../references/calendar-tsx/calendar-",
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
                include_str!(concat!(
                    "examples/card_examples/",
                    stringify!($module),
                    ".rs"
                )),
                include_str!(concat!("../../../references/card-tsx/card-", $name, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Card,
        "Group a title, description, content, and actions in a surface.",
        "https://ui.shadcn.com/docs/components/base/card",
    )
    .child(example_with_rsx(
        "Card composition",
        composition::card_rsx::example(),
        composition::card_rust::CODE,
        Some(crate::native_examples::source("Card composition").unwrap().tsx),
        Some(composition::card_rsx::CODE),
    ))
    .child(heading("Composition"))
    .child(muted("Use CardHeader for CardTitle, CardDescription, and CardAction. Put the main body in CardContent and the actions in CardFooter."))
    .child(demo!(login, preview, "demo", "Card demo"))
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
                include_str!("examples/carousel_examples.rs"),
                include_str!(concat!(
                    "../../../references/carousel-tsx/carousel-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Carousel,
        "Drag or swipe to change slides. You can also scroll along the carousel, use the arrow keys, or press the navigation buttons.",
        "https://ui.shadcn.com/docs/components/base/carousel",
    )
    .child(example_with_rsx(
        "Carousel composition",
        view().max_w(px(360.0)).w_full().child(composition::carousel_rsx::example()),
        composition::carousel_rust::CODE,
        Some(crate::native_examples::source("Carousel composition").unwrap().tsx),
        Some(composition::carousel_rsx::CODE),
    ))
    .child(heading("Composition"))
    .child(muted("Use carousel_root().accessibility_label(...) to name the carousel. Add CarouselItem slides to CarouselContent, and place CarouselPrevious and CarouselNext around the viewport."))
    .child(demo!(demo, "demo", "Carousel demo"))
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
                include_str!("examples/chart_examples.rs"),
                include_str!(concat!(
                    "../../../references/chart-tsx/chart-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Chart,
        "Build accessible, themed charts with Native Shell plots.",
        "https://ui.shadcn.com/docs/components/base/chart",
    )
    .child(demo!(demo, "demo", "Interactive Bar Chart"))
    .child(heading("Line Chart"))
    .child(example(
        "Line Chart",
        chart_trends::line(),
        include_str!("examples/chart_trends.rs"),
        include_str!("../../../references/chart-tsx/chart-line-default.tsx"),
    ))
    .child(heading("Area Chart"))
    .child(example(
        "Area Chart",
        chart_trends::area(),
        include_str!("examples/chart_trends.rs"),
        include_str!("../../../references/chart-tsx/chart-area-default.tsx"),
    ))
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
    .child(muted("ChartConfig maps each series key to a label and color. Pass values in series order and use tick to shorten the axis label. Choose Bar, Line, or Area with ChartContainer::kind."))
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
                include_str!("examples/checkbox_examples.rs"),
                include_str!(concat!(
                    "../../../references/checkbox-tsx/checkbox-",
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

fn code_block_page() -> View {
    macro_rules! demos {
        ($page:expr; $(($function:ident, $name:literal, $title:literal)),+ $(,)?) => {
            $page$(.child(heading($title)).child(example(
                $title,
                code_block_examples::$function(),
                include_str!("examples/code_block_examples/mod.rs"),
                include_str!(concat!("../../../references/code-block-tsx/code-block-", $name, ".tsx")),
            )))+
        };
    }
    let page = component_article(
        DocsPage::CodeBlock,
        "Display source code, terminal output, and diffs.",
        "https://ui.shadcn.com/docs/components/base/code-block",
    )
    .child(example(
        "Code Block demo",
        code_block_examples::demo(),
        include_str!("examples/code_block_examples/mod.rs"),
        include_str!("../../../references/code-block-tsx/code-block-demo.tsx"),
    ))
    .child(heading("Composition"))
    .child(muted("Put the source in CodeBlockContent. Add a CodeBlockHeader with a title and actions, place the actions directly on the block to float them, or add a CodeBlockFooter for an expansion button."));
    demos!(page;
        (basic, "basic", "Basic"),
        (header, "header", "Header"),
        (actions, "actions", "Actions"),
        (floating, "floating-actions", "Floating Actions"),
        (line_numbers, "line-numbers", "Line Numbers"),
        (start_line, "start-line", "Start Line"),
        (wrap, "wrap", "Wrap"),
        (highlighted, "highlighted-lines", "Highlighted Lines"),
        (diff, "diff", "Diff"),
        (height, "height", "Height"),
        (show_more, "show-more", "Show More"),
        (show_more_scroll, "show-more-scroll", "Max Height"),
        (size, "size", "Size"),
        (destructive, "destructive", "Destructive"),
        (trace, "trace", "Trace"),
        (tabs, "tabs", "Tabs"),
        (chat, "chat", "Message"),
        (typeset, "typeset", "Typeset"),
        (rtl, "rtl", "RTL"),
        (highlighting, "highlighting", "Syntax Highlighting"),
        (languages, "languages", "Languages"),
        (ansi, "ansi", "Terminal Output"),
        (streaming, "streaming", "Streaming"),
        (markdown, "markdown", "Markdown"),
    )
    .child(heading("API Reference"))
    .child(api_row("CodeBlock / CodeBlockHeader / CodeBlockTitle", "Compose the surface and header. Choose a default or compact size and a default or destructive appearance."))
    .child(api_row("CodeBlockContent / CodeBlockLine", "Set wrapping, start line, syntax language, and height. Supply individual lines for diff markers and emphasis."))
    .child(api_row("CodeBlockActions / CodeBlockCopyButton / CodeBlockFooter", "Add actions, copy source text, or place controls under the content."))
    .child(muted("Syntax highlighting uses bundled Syntect grammars. The Terminal Output example understands basic ANSI text colors and removes escape codes when copying."))
}

fn collapsible_page() -> View {
    macro_rules! demo {
        ($example:ident, $name:literal, $title:literal) => {
            example(
                $title,
                collapsible_examples::$example(),
                include_str!("examples/collapsible_examples.rs"),
                include_str!(concat!(
                    "../../../references/collapsible-tsx/collapsible-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Collapsible,
        "Reveal or hide a section of content.",
        "https://ui.shadcn.com/docs/components/base/collapsible",
    )
    .child(demo!(demo, "demo", "Collapsible demo"))
    .child(heading("Composition"))
    .child(muted("Pair a CollapsibleTrigger with CollapsibleContent. Use header to place the trigger beside a title or a persistent summary."))
    .child(heading("Controlled State"))
    .child(muted("Set default_open for an initial value. Bind open to a signal and update it in on_open_change to control the panel."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Settings Panel"))
    .child(demo!(settings, "settings", "Settings Panel"))
    .child(heading("File Tree"))
    .child(demo!(file_tree, "file-tree", "File Tree"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Collapsible / CollapsibleTrigger / CollapsibleContent", "Compose a disclosure, style the trigger using its open state, and provide the content that expands. Use disabled to block activation or reveal(false) to switch instantly."))
}

fn combobox_page() -> View {
    macro_rules! demos {
        ($page:expr; $(($function:ident, $name:literal, $title:literal)),+ $(,)?) => {
            $page$(.child(heading($title)).child(example(
                $title,
                combobox_examples::$function(),
                include_str!("examples/combobox_examples.rs"),
                include_str!(concat!("../../../references/combobox-tsx/combobox-", $name, ".tsx")),
            )))+
        };
    }
    let page = component_article(
        DocsPage::Combobox,
        "Filter a list and select one or more options.",
        "https://ui.shadcn.com/docs/components/base/combobox",
    )
    .child(example(
        "Combobox demo",
        combobox_examples::demo(),
        include_str!("examples/combobox_examples.rs"),
        include_str!("../../../references/combobox-tsx/combobox-demo.tsx"),
    ))
    .child(heading("Usage"))
    .child(muted("Supply options as ComboboxItem values. Use group to add section labels and render_item to show additional information."))
    .child(heading("Multiple Selection"))
    .child(muted("Set multiple(true) to keep several items. Each chosen item has a remove button. Bind a value signal to read or update the selection from Rust."));
    demos!(page;
        (basic, "basic", "Basic"),
        (multiple, "multiple", "Multiple"),
        (clear, "clear", "Clear Button"),
        (groups, "groups", "Groups"),
        (custom, "custom", "Custom Items"),
        (invalid, "invalid", "Invalid"),
        (disabled, "disabled", "Disabled"),
        (auto_highlight, "auto-highlight", "Auto Highlight"),
        (popup, "popup", "Popup"),
        (input_group, "input-group", "Input Group"),
        (rtl, "rtl", "RTL"),
    )
    .child(heading("API Reference"))
    .child(api_row("Combobox", "Type to filter and highlight the first result. Use the arrow keys to move among results, Enter to select, and Escape to close."))
    .child(api_row("ComboboxItem", "Give each option a unique value and a label. Items can be grouped or disabled."))
    .child(api_row("Popup and input addons", "Use popup() for a searchable list behind a button, or prefix() to add content before the editable input."))
}

fn command_page() -> View {
    macro_rules! demo {
        ($example:ident, $title:literal) => {
            example(
                $title,
                command_examples::$example(),
                include_str!("examples/command_examples.rs"),
                include_str!(concat!(
                    "../../../references/command-tsx/command-",
                    stringify!($example),
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Command,
        "Search and run commands from a palette.",
        "https://ui.shadcn.com/docs/components/base/command",
    )
    .child(demo!(demo, "Command demo"))
    .child(heading("Composition"))
    .child(muted("Create a Command with a list of CommandItem values. Set group, keywords, disabled, and shortcut on each item. Use on_select to run the action for the selected id."))
    .child(heading("Dialog"))
    .child(demo!(dialog, "Dialog"))
    .child(heading("Basic"))
    .child(demo!(basic, "Basic"))
    .child(heading("Shortcuts"))
    .child(demo!(shortcuts, "Shortcuts"))
    .child(heading("Groups"))
    .child(demo!(groups, "Groups"))
    .child(heading("Scrollable"))
    .child(demo!(scrollable, "Scrollable"))
    .child(heading("RTL"))
    .child(demo!(rtl, "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Command", "Use the arrows to navigate and Enter to select. Escape clears the search before dismissing an open dialog."))
    .child(api_row("CommandDialog", "Open the palette from a button or bind a shortcut such as mod+j."))
    .child(api_row("CommandItem", "Supply labels, grouping, keywords, disabled state and shortcut hints. Bind any item shortcuts in the application that owns the actions."))
}

fn context_menu_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                context_menu_examples::$example(),
                include_str!("examples/context_menu_examples.rs"),
                include_str!(concat!(
                    "../../../references/context-menu-tsx/context-menu-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::ContextMenu,
        "Show actions for an area with a right click or a touch hold.",
        "https://ui.shadcn.com/docs/components/base/context-menu",
    )
    .child(demo!(demo, "demo", "Context Menu demo"))
    .child(heading("Usage"))
    .child(muted("Right click or hold the trigger on a touchscreen. With the trigger focused, use Shift+F10 or the Menu key. Use the arrow keys to navigate and Escape to dismiss."))
    .child(heading("Composition"))
    .child(muted("Create a ContextMenu with a trigger view. Add ContextMenuItem rows, labels, separators, and submenus as children. Keep checkbox and radio values in Rust signals."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Submenu"))
    .child(demo!(submenu, "submenu", "Submenu"))
    .child(heading("Shortcuts"))
    .child(demo!(shortcuts, "shortcuts", "Shortcuts"))
    .child(heading("Groups"))
    .child(demo!(groups, "groups", "Groups"))
    .child(heading("Icons"))
    .child(demo!(icons, "icons", "Icons"))
    .child(heading("Checkboxes"))
    .child(demo!(checkboxes, "checkboxes", "Checkboxes"))
    .child(heading("Radio"))
    .child(demo!(radio_items, "radio", "Radio"))
    .child(heading("Destructive"))
    .child(demo!(destructive, "destructive", "Destructive"))
    .child(heading("Sides"))
    .child(demo!(sides, "sides", "Sides"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("ContextMenu", "Pass in a trigger view. An optional placement chooses the preferred popup side. Native Shell may flip the popup to fit or show it as a sheet on compact touch devices."))
    .child(api_row("ContextMenuItem", "Attach an on_select callback. Use checkbox or radio for checked state. Shortcut labels describe application key bindings."))
    .child(api_row("Labels, separators and submenus", "Use these parts to compose groups or reveal additional actions without leaving the current context."))
}

fn data_table_page() -> View {
    const RUST: &str = include_str!("examples/data_table_examples.rs");
    component_article(
        DocsPage::DataTable,
        "Build interactive tables with typed rows, sorting, filtering, and pagination.",
        "https://ui.shadcn.com/docs/components/base/data-table",
    )
    .child(example(
        "Data Table demo",
        data_table_examples::demo(),
        RUST,
        include_str!("../../../references/data-table-tsx/data-table-demo.tsx"),
    ))
    .child(heading("Set up Table Features"))
    .child(muted("Pass your Rust records and column definitions to DataTable::new. The table keeps page, filter, sort, visibility, and selection state."))
    .child(heading("Basic Table"))
    .child(code_block("let columns = vec![\n    DataTableColumn::new(\"Email\", 224.0, |row: &Payment| {\n        text(row.email.to_ascii_lowercase()).text_sm()\n    }).flexible(),\n];\nlet table = DataTable::new(\"Payments\", payments, columns);"))
    .child(heading("Cell Formatting"))
    .child(muted("Give each column a renderer. The payments example lowercases emails and formats amounts as US dollars. Call align_end() to align a column's header and cells."))
    .child(heading("Row Actions"))
    .child(muted("Render an action menu in a column. The sample menu copies the payment ID and displays customer or payment details inline."))
    .child(heading("Pagination"))
    .child(muted("This preview uses two rows per page so you can try Previous, Next, and page-level selection."))
    .child(example(
        "Pagination",
        data_table_examples::pagination(),
        RUST,
        include_str!("../../../references/data-table-tsx/data-table-pagination.tsx"),
    ))
    .child(heading("Sorting"))
    .child(muted("Attach a comparator with sortable(). Clicking the Email header switches between ascending and descending order."))
    .child(heading("Filtering"))
    .child(muted("Call filter() with a field accessor. The input matches text without regard to case and returns to the first page on each edit."))
    .child(heading("Visibility"))
    .child(muted("The Columns menu controls which fields are shown. Use always_visible() for controls that should stay available."))
    .child(heading("Row Selection"))
    .child(muted("The header checkbox controls the current page. The footer counts selected rows across all matching pages."))
    .child(heading("Reusable Components"))
    .child(muted("Use the same typed column definitions in multiple tables. Each table keeps its own sort order, pagination, and column visibility."))
    .child(heading("RTL"))
    .child(example(
        "RTL",
        data_table_examples::rtl(),
        RUST,
        include_str!("../../../references/data-table-tsx/data-table-rtl.tsx"),
    ))
}

fn date_picker_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                date_picker_examples::$example(),
                include_str!("examples/date_picker_examples.rs"),
                include_str!(concat!(
                    "../../../references/date-picker-tsx/date-picker-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::DatePicker,
        "Select a date or a range with a calendar popover.",
        "https://ui.shadcn.com/docs/components/base/date-picker",
    )
    .child(demo!(demo, "demo", "Date Picker demo"))
    .child(heading("Usage"))
    .child(code_block("let date = signal(Date::default());\nlet picker = DatePicker::new(\"Pick a date\").value(date);"))
    .child(heading("Composition"))
    .child(muted("DatePicker combines a trigger, Native Shell Popover, and Calendar. Supply a Rust signal to use the selection elsewhere."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Range Picker"))
    .child(demo!(range, "range", "Range Picker"))
    .child(heading("Date of Birth"))
    .child(demo!(dob, "dob", "Date of Birth"))
    .child(heading("Input"))
    .child(muted("Enter an ISO date, a US numeric date, or an English month name. Press Down Arrow to open the calendar."))
    .child(demo!(input, "input", "Input"))
    .child(heading("Time Picker"))
    .child(demo!(time, "time", "Time Picker"))
    .child(heading("Natural Language Picker"))
    .child(muted("Try tomorrow, next week, next Friday, or in 2 days. Relative phrases use today's date on your device."))
    .child(demo!(natural_language, "natural-language", "Natural Language Picker"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
}

fn dialog_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                dialog_examples::$example(),
                include_str!("examples/dialog_examples.rs"),
                include_str!(concat!(
                    "../../../references/dialog-tsx/dialog-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Dialog,
        "Open a modal window for a focused task.",
        "https://ui.shadcn.com/docs/components/base/dialog",
    )
    .child(demo!(demo, "demo", "Dialog demo"))
    .child(heading("Usage"))
    .child(code_block("let state = Dialog::new(signal(false));\nlet trigger = DialogTrigger::new(state, \"Open Dialog\");\nlet title = DialogTitle::new(\"Edit profile\");\nlet content = DialogContent::new(state, DialogHeader::new(title));"))
    .child(heading("Composition"))
    .child(muted("Share the same Dialog state between DialogTrigger, DialogContent, and DialogClose. Compose a header from DialogTitle and DialogDescription."))
    .child(heading("Custom Close Button"))
    .child(demo!(close_button, "close-button", "Custom Close Button"))
    .child(heading("No Close Button"))
    .child(demo!(no_close_button, "no-close-button", "No Close Button"))
    .child(heading("Sticky Footer"))
    .child(demo!(sticky_footer, "sticky-footer", "Sticky Footer"))
    .child(heading("Scrollable Content"))
    .child(demo!(scrollable_content, "scrollable-content", "Scrollable Content"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("DialogContent", "Mounts the modal, backdrop, and optional corner action. Configure Escape and outside-click dismissal independently."))
    .child(api_row("DialogFooter", "Keeps actions visible while the body scrolls. On narrow screens the actions stack."))
    .child(api_row("DialogClose", "Use render() to give an existing button the close action."))
}

fn direction_page() -> View {
    component_article(
        DocsPage::Direction,
        "Set the layout direction for an application or a section.",
        "https://ui.shadcn.com/docs/components/base/direction",
    )
    .child(example(
        "Direction demo",
        card_examples::login::direction(),
        include_str!("examples/card_examples/login.rs"),
        include_str!("../../../references/card-tsx/card-rtl.tsx"),
    ))
    .child(heading("Usage"))
    .child(muted("Wrap a view in DirectionProvider. Supply Direction::Ltr, Direction::Rtl, or a Rust signal to switch direction at runtime. An inner provider can override direction for its own children."))
    .child(code_block("let direction = signal(Direction::Rtl);\n\nDirectionProvider::new(direction, || {\n    view().child(text(\"Content in this direction scope\"))\n})"))
    .child(heading("use_direction"))
    .child(muted("Read the nearest provider from a component inside the scope. Without a provider the value follows the Native Shell surface."))
    .child(code_block("let direction = use_direction();\ntext(move || format!(\"Current direction: {}\", direction.get().as_str()))"))
}

fn dropdown_menu_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                dropdown_menu_examples::$example(),
                include_str!("examples/dropdown_menu_examples.rs"),
                include_str!(concat!(
                    "../../../references/dropdown-menu-tsx/dropdown-menu-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::DropdownMenu,
        "Open a menu of actions from a button.",
        "https://ui.shadcn.com/docs/components/base/dropdown-menu",
    )
    .child(demo!(demo, "demo", "Dropdown Menu demo"))
    .child(heading("Usage"))
    .child(code_block("DropdownMenu::new(\n    DropdownMenuTrigger::new(\"Open\"),\n    DropdownMenuContent::new().child(\n        DropdownMenuItem::new(\"Profile\").on_select(open_profile),\n    ),\n)"))
    .child(heading("Composition"))
    .child(muted("Use a button or custom trigger with one content surface. Compose labels, groups, separators, nested submenus, and checked items."))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Submenu"))
    .child(demo!(submenu, "submenu", "Submenu"))
    .child(heading("Shortcuts"))
    .child(demo!(shortcuts, "shortcuts", "Shortcuts"))
    .child(heading("Icons"))
    .child(demo!(icons, "icons", "Icons"))
    .child(heading("Checkboxes"))
    .child(demo!(checkboxes, "checkboxes", "Checkboxes"))
    .child(heading("Checkboxes Icons"))
    .child(demo!(checkboxes_icons, "checkboxes-icons", "Checkboxes Icons"))
    .child(heading("Radio Group"))
    .child(demo!(radio_group, "radio-group", "Radio Group"))
    .child(heading("Radio Icons"))
    .child(demo!(radio_icons, "radio-icons", "Radio Icons"))
    .child(heading("Destructive"))
    .child(demo!(destructive, "destructive", "Destructive"))
    .child(heading("Avatar"))
    .child(demo!(avatar, "avatar", "Avatar"))
    .child(heading("Complex"))
    .child(demo!(complex, "complex", "Complex"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("DropdownMenuContent", "Choose placement, alignment, and width. The native menu keeps the popup within available space and can show a sheet on touch devices."))
    .child(api_row("DropdownMenuItem", "Use on_select to run an action. A Rust signal can supply checked or radio state. Set close_on_select(false) to keep the menu open while editing settings."))
    .child(api_row("DropdownMenuSubmenu", "Add nested items or another submenu. Use the arrow keys to navigate, Enter to select, and Escape to go back or dismiss."))
}

fn empty_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                empty_examples::$example(),
                include_str!("examples/empty_examples.rs"),
                include_str!(concat!(
                    "../../../references/empty-tsx/empty-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Empty,
        "Display an empty state and guide users to their next action.",
        "https://ui.shadcn.com/docs/components/base/empty",
    )
    .child(demo!(demo, "demo", "Empty demo"))
    .child(heading("Usage"))
    .child(code_block("Empty::new()\n    .child(EmptyHeader::new()\n        .child(EmptyTitle::new(\"No data\"))\n        .child(EmptyDescription::new(\"No data found\")))\n    .child(EmptyContent::new().child(Button::new(\"Add data\")))"))
    .child(heading("Composition"))
    .child(muted("Group the media, title, and description in EmptyHeader. Add buttons, inputs, or links to EmptyContent."))
    .child(heading("Outline"))
    .child(demo!(outline, "outline", "Outline"))
    .child(heading("Background"))
    .child(demo!(background, "background", "Background"))
    .child(heading("Avatar"))
    .child(demo!(avatar, "avatar", "Avatar"))
    .child(heading("Avatar Group"))
    .child(demo!(avatar_group, "avatar-group", "Avatar Group"))
    .child(heading("InputGroup"))
    .child(demo!(input_group, "input-group", "InputGroup"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Empty", "Centers the empty state. Use Native Shell style methods to set the border, background, and padding."))
    .child(api_row("EmptyHeader", "Groups centered media, title, and description."))
    .child(api_row("EmptyMedia", "Holds arbitrary media. EmptyMediaVariant::Icon adds a muted 40-point tile."))
    .child(api_row("EmptyTitle", "Displays a centered title from static or reactive text."))
    .child(api_row("EmptyDescription", "Displays centered muted text. Use render to compose links or richer content."))
    .child(api_row("EmptyContent", "Holds actions and fields. Apply flex_row and flex_wrap for a responsive row."))
}

fn field_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                field_examples::$example(),
                include_str!("examples/field_examples.rs"),
                include_str!(concat!(
                    "../../../references/field-tsx/field-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Field,
        "Combine controls with labels, descriptions, and validation messages.",
        "https://ui.shadcn.com/docs/components/base/field",
    )
    .child(demo!(demo, "demo", "Field demo"))
    .child(heading("Usage"))
    .child(code_block(r#"let control = element_ref();
Field::new()
    .child(FieldLabel::new("Email").for_control(control))
    .child(input().label("Email").element_ref(&control))
    .child(FieldDescription::new("Use your work email."))"#))
    .child(heading("Composition"))
    .child(muted("Start with a FieldSet and FieldLegend for named sections. Use FieldGroup to space related fields and FieldContent to group text next to a control."))
    .child(heading("Input"))
    .child(demo!(input_example, "input", "Input"))
    .child(heading("Textarea"))
    .child(demo!(textarea, "textarea", "Textarea"))
    .child(heading("Select"))
    .child(demo!(select_example, "select", "Select"))
    .child(heading("Slider"))
    .child(demo!(slider_example, "slider", "Slider"))
    .child(heading("Fieldset"))
    .child(demo!(fieldset, "fieldset", "Fieldset"))
    .child(heading("Checkbox"))
    .child(demo!(checkbox, "checkbox", "Checkbox"))
    .child(heading("Radio"))
    .child(demo!(radio_example, "radio", "Radio"))
    .child(heading("Switch"))
    .child(demo!(switch_example, "switch", "Switch"))
    .child(heading("Choice Card"))
    .child(demo!(choice_card, "choice-card", "Choice Card"))
    .child(heading("Field Group"))
    .child(demo!(group, "group", "Field Group"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("Responsive Layout"))
    .child(demo!(responsive, "responsive", "Responsive Layout"))
    .child(heading("Validation and Errors"))
    .child(muted("Submit the checkout or profile preview with a blank field to see the validation state. Set the invalid state on both the Field and its control. Associate the control with its help or error text using accessibility_described_by."))
    .child(code_block(r#"FieldError::messages(move || validate(email.get()))"#))
    .child(heading("API Reference"))
    .child(api_row("Field", "Choose Vertical, Horizontal, or Responsive orientation. Responsive fields switch at 448 points of available field width."))
    .child(api_row("FieldSet / FieldLegend / FieldGroup", "Name sections and lay out related fields. FieldLegend supports Legend and Label variants."))
    .child(api_row("FieldLabel / FieldTitle", "Display a field label or title. FieldLabel::for_control focuses the matching control when pressed."))
    .child(api_row("FieldContent / FieldDescription", "Group secondary content and provide help text or links."))
    .child(api_row("FieldSeparator", "Separate sections with an optional visible label."))
    .child(api_row("FieldError", "Render a validation alert, or use messages to show a deduplicated list."))
}

fn hover_card_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                hover_card_examples::$example(),
                include_str!("examples/hover_card_examples.rs"),
                include_str!(concat!(
                    "../../../references/hover-card-tsx/hover-card-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::HoverCard,
        "Preview content from a trigger without leaving the page.",
        "https://ui.shadcn.com/docs/components/base/hover-card",
    )
    .child(demo!(demo, "demo", "Hover Card demo"))
    .child(heading("Usage"))
    .child(code_block(r#"HoverCard::new(
    HoverCardTrigger::new("Hover Here"),
    HoverCardContent::new().child(text("Preview content")),
)"#))
    .child(heading("Composition"))
    .child(muted("Pass one HoverCardTrigger and one HoverCardContent to HoverCard. Put essential information in the trigger or linked content so keyboard users can reach it."))
    .child(heading("Trigger Delays"))
    .child(code_block("HoverCardTrigger::new(\"Hover Here\").delay(100.0).close_delay(200.0)"))
    .child(heading("Positioning"))
    .child(code_block("HoverCardContent::new().side(HoverCardSide::Top).align(Align::Start)"))
    .child(heading("Basic"))
    .child(demo!(demo, "demo", "Basic"))
    .child(heading("Sides"))
    .child(demo!(sides, "sides", "Sides"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("HoverCard", "Use open(signal) to control the card or default_open(true) to start open. Escape and an outside press dismiss it."))
    .child(api_row("HoverCardTrigger", "Set open and close delays in milliseconds. Desktop uses pointer hover. A tap toggles the preview on touch layouts."))
    .child(api_row("HoverCardContent", "Choose physical or logical side, alignment, offsets, and width. Native Shell flips placement when available space requires it."))
}

fn input_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                input_examples::$example(),
                include_str!("examples/input_examples.rs"),
                include_str!(concat!(
                    "../../../references/input-tsx/input-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Input,
        "Enter text, passwords, and contact details in a native control.",
        "https://ui.shadcn.com/docs/components/base/input",
    )
    .child(demo!(demo, "demo", "Input demo"))
    .child(heading("Usage"))
    .child(code_block(r#"use components::input::input;

input()
    .label("Email")
    .placeholder("name@example.com")
    .keyboard_type(KeyboardType::Email)"#))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Field"))
    .child(demo!(field, "field", "Field"))
    .child(heading("Field Group"))
    .child(demo!(field_group, "fieldgroup", "Field Group"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("File"))
    .child(demo!(file, "file", "File"))
    .child(heading("Inline"))
    .child(demo!(inline, "inline", "Inline"))
    .child(heading("Grid"))
    .child(demo!(grid, "grid", "Grid"))
    .child(heading("Required"))
    .child(demo!(required, "required", "Required"))
    .child(heading("Badge"))
    .child(demo!(badge, "badge", "Badge"))
    .child(heading("Input Group"))
    .child(demo!(input_group, "input-group", "Input Group"))
    .child(heading("Button Group"))
    .child(demo!(button_group, "button-group", "Button Group"))
    .child(heading("Form"))
    .child(demo!(form, "form", "Form"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("input", "Configure the native control with label, placeholder, value, masked, disabled, keyboard_type and input events."))
    .child(api_row("invalid_input", "Pass a reactive boolean to bind error styling and the accessible invalid state. Use Field to place labels, descriptions and errors."))
    .child(api_row("file_input", "Open the system picker with a FilePickRequest. Receive selected files in a callback. The preview displays the selected filename."))
}

fn input_group_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                input_group_examples::$example(),
                include_str!("examples/input_group_examples.rs"),
                include_str!(concat!(
                    "../../../references/input-group-tsx/input-group-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::InputGroup,
        "Add labels, actions, and status to inputs and textareas.",
        "https://ui.shadcn.com/docs/components/base/input-group",
    )
    .child(demo!(demo, "demo", "Input Group demo"))
    .child(heading("Usage"))
    .child(code_block(r#"InputGroup::new(input_group_input().label("Website").placeholder("example.com"))
    .addon(InputGroupAddon::new(InputGroupAlign::InlineStart)
        .child(input_group_text("https://")))"#))
    .child(heading("Composition"))
    .child(muted("Pass a native input or textarea to InputGroup, then add text, icons, or buttons with InputGroupAddon. Use action for a child that should receive focus itself."))
    .child(heading("Align"))
    .child(demo!(inline_start, "inline-start", "Inline Start"))
    .child(demo!(inline_end, "inline-end", "Inline End"))
    .child(demo!(block_start, "block-start", "Block Start"))
    .child(demo!(block_end, "block-end", "Block End"))
    .child(heading("Icon"))
    .child(demo!(icons, "icon", "Icon"))
    .child(heading("Text"))
    .child(demo!(text_addons, "text", "Text"))
    .child(heading("Button"))
    .child(demo!(buttons, "button", "Button"))
    .child(heading("Kbd"))
    .child(demo!(kbd, "kbd", "Kbd"))
    .child(heading("Dropdown"))
    .child(demo!(dropdown, "dropdown", "Dropdown"))
    .child(heading("Spinner"))
    .child(demo!(spinners, "spinner", "Spinner"))
    .child(heading("Textarea"))
    .child(demo!(textarea, "textarea", "Textarea"))
    .child(heading("Custom Input"))
    .child(demo!(custom, "custom", "Custom Input"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("InputGroup", "Group a native Input with addons, set disabled or invalid state, and share its focus handle with a FieldLabel."))
    .child(api_row("InputGroupAddon", "Choose InlineStart, InlineEnd, BlockStart, or BlockEnd. The inline sides follow the layout direction. Press a text addon to focus the field."))
    .child(api_row("input_group_input / input_group_textarea", "Create a native control styled for the group. Configure labels, values, keyboard and events on the returned control."))
    .child(api_row("input_group_text / input_group_button", "Use the supplied text and button styles for adornments and interactive actions."))
}

fn input_otp_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                input_otp_examples::$example(),
                include_str!("examples/input_otp_examples.rs"),
                include_str!(concat!(
                    "../../../references/input-otp-tsx/input-otp-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::InputOtp,
        "Enter a one-time verification code with one keyboard field and visible slots.",
        "https://ui.shadcn.com/docs/components/base/input-otp",
    )
    .child(demo!(demo, "demo", "Input OTP demo"))
    .child(heading("Usage"))
    .child(code_block(r#"InputOtp::new("Verification code", 6)
    .group(InputOtpGroup::slots(0..3))
    .separator(InputOtpSeparator::new())
    .group(InputOtpGroup::slots(3..6))"#))
    .child(heading("Composition"))
    .child(muted("Use InputOtpGroup to lay out adjacent cells. Add indexed InputOtpSlot values or use slots with a range. InputOtpSeparator divides groups."))
    .child(heading("Pattern"))
    .child(demo!(pattern, "pattern", "Pattern"))
    .child(heading("Separator"))
    .child(demo!(separator, "separator", "Separator"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Controlled"))
    .child(demo!(controlled, "controlled", "Controlled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("Four Digits"))
    .child(demo!(four_digits, "four-digits", "Four Digits"))
    .child(heading("Alphanumeric"))
    .child(demo!(alphanumeric, "alphanumeric", "Alphanumeric"))
    .child(heading("Form"))
    .child(demo!(form, "form", "Form"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("InputOtp", "Choose the number of slots and bind a Rust signal with value. Use on_change, on_complete, or on_submit to respond to edits."))
    .child(api_row("InputOtpPattern", "Choose Digits, Alphanumeric, or a custom character predicate. The built-in patterns normalize supported localized digits and strip separators from pasted codes."))
    .child(api_row("InputOtpSlot", "Index the character the slot shows. Set invalid on a slot or the complete input to display validation styling."))
}

fn item_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                item_examples::$example(),
                include_str!("examples/item_examples.rs"),
                include_str!(concat!(
                    "../../../references/item-tsx/item-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Item,
        "Compose a row with optional media, details, and actions.",
        "https://ui.shadcn.com/docs/components/base/item",
    )
    .child(demo!(demo, "demo", "Item demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Item::new()
    .variant(ItemVariant::Outline)
    .content(ItemContent::new()
        .title(ItemTitle::new("Title"))
        .description(ItemDescription::new("Details")))
    .actions(ItemActions::new().child(Button::new("Action")))"#))
    .child(heading("Composition"))
    .child(muted("Use ItemMedia, ItemContent, and ItemActions for the row. Add a full-width ItemHeader or ItemFooter, and use ItemGroup with ItemSeparator to organize related rows."))
    .child(heading("Item and Field"))
    .child(muted("Use Item to present content and actions. Use Field to connect a form control to its label, description, and validation feedback."))
    .child(heading("Variant"))
    .child(demo!(variant, "variant", "Variant"))
    .child(heading("Size"))
    .child(demo!(size, "size", "Size"))
    .child(heading("Icon"))
    .child(demo!(icon_example, "icon", "Icon"))
    .child(heading("Avatar"))
    .child(demo!(avatar, "avatar", "Avatar"))
    .child(heading("Image"))
    .child(demo!(image_example, "image", "Image"))
    .child(heading("Group"))
    .child(demo!(group, "group", "Group"))
    .child(heading("Header"))
    .child(demo!(header, "header", "Header"))
    .child(heading("Link"))
    .child(demo!(link_example, "link", "Link"))
    .child(heading("Dropdown"))
    .child(demo!(dropdown, "dropdown", "Dropdown"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Item", "Choose Default, Outline, or Muted and one of three sizes. Use href to make the whole row a native link."))
    .child(api_row("ItemGroup / ItemSeparator", "Group related rows and add optional divider lines."))
    .child(api_row("ItemMedia", "Render an avatar, an icon, or an image. The image frame scales with ItemSize."))
    .child(api_row("ItemContent / ItemTitle / ItemDescription", "Compose the main text. Add another content block for trailing metadata."))
    .child(api_row("ItemActions / ItemHeader / ItemFooter", "Place controls in the row, or use a full-width section above or below it."))
}

fn kbd_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                kbd_examples::$example(),
                include_str!("examples/kbd_examples.rs"),
                include_str!(concat!("../../../references/kbd-tsx/kbd-", $source, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Kbd,
        "Display a keyboard key or a combination of keys.",
        "https://ui.shadcn.com/docs/components/base/kbd",
    )
    .child(demo!(demo, "demo", "Kbd demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Kbd::new("Ctrl").named("Control")"#))
    .child(heading("Composition"))
    .child(code_block(r#"KbdGroup::new()
    .child(Kbd::new("Ctrl"))
    .child(Kbd::new("B"))"#))
    .child(heading("Group"))
    .child(demo!(group, "group", "Group"))
    .child(heading("Button"))
    .child(demo!(button_example, "button", "Button"))
    .child(heading("Tooltip"))
    .child(demo!(tooltip_example, "tooltip", "Tooltip"))
    .child(heading("Input Group"))
    .child(demo!(input_group, "input-group", "Input Group"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Kbd", "Display a key label. Use named for a spoken name and tone to match its surrounding tooltip or input group."))
    .child(api_row("KbdGroup", "Lay out several keys together. Add text separators as needed. The group follows the surrounding layout direction."))
}

fn label_page() -> View {
    component_article(
        DocsPage::Label,
        "Name a control and let users focus or activate it through its visible label.",
        "https://ui.shadcn.com/docs/components/base/label",
    )
    .child(example(
        "Label demo",
        label_examples::demo(),
        include_str!("examples/label_examples.rs"),
        include_str!("../../../references/label-tsx/label-demo.tsx"),
    ))
    .child(heading("Usage"))
    .child(code_block(r#"let target = element_ref();
view()
    .child(Label::new("Email").for_control(target))
    .child(input().label("Email").element_ref(&target))"#))
    .child(heading("Label in Field"))
    .child(muted("Use FieldLabel with FieldDescription and FieldError when a form control needs help or validation messages."))
    .child(example(
        "Label in Field",
        field_examples::demo(),
        include_str!("examples/field_examples.rs"),
        include_str!("../../../references/field-tsx/field-demo.tsx"),
    ))
    .child(heading("RTL"))
    .child(example(
        "RTL",
        label_examples::rtl(),
        include_str!("examples/label_examples.rs"),
        include_str!("../../../references/label-tsx/label-rtl.tsx"),
    ))
    .child(heading("API Reference"))
    .child(api_row("Label", "Create a text label with new or supply formatted content with render."))
    .child(api_row("for_control", "Focus the native control when its label is pressed. Supply the same accessible name on the control."))
    .child(api_row("on_activate / disabled", "Attach a secondary action for controls such as checkboxes. Share the control's disabled state to suppress label activation."))
}

fn marker_page() -> View {
    macro_rules! demo {
        ($name:ident, $source:literal, $label:literal) => {
            example(
                $label,
                marker_examples::$name(),
                include_str!("examples/marker_examples.rs"),
                include_str!(concat!(
                    "../../../references/marker-tsx/marker-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Marker,
        "Display an inline status, bordered row, or labeled separator in a conversation.",
        "https://ui.shadcn.com/docs/components/base/marker",
    )
    .child(demo!(demo, "demo", "Marker demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Marker::new(MarkerContent::new("Explored 4 files"))
    .icon(MarkerIcon::new(text("✓")))"#))
    .child(heading("Variants"))
    .child(demo!(variants, "variants", "Variants"))
    .child(heading("Status"))
    .child(demo!(status, "status", "Status"))
    .child(heading("Shimmer"))
    .child(demo!(shimmer, "shimmer", "Shimmer"))
    .child(heading("Separator"))
    .child(demo!(separator, "separator", "Separator"))
    .child(heading("Border"))
    .child(demo!(border, "border", "Border"))
    .child(heading("With Icon"))
    .child(demo!(with_icon, "icon", "With Icon"))
    .child(heading("Links and Buttons"))
    .child(demo!(link_button, "link-button", "Links and Buttons"))
    .child(heading("API Reference"))
    .child(api_row("Marker", "Choose a layout variant. Use stacked for a vertical icon and label, href for a link, and on_press for a button."))
    .child(api_row("MarkerIcon", "Add a decorative icon. Assistive technologies read the adjacent content."))
    .child(api_row("MarkerContent", "Display static or reactive text. Use shimmer for a moving highlight and status on Marker to announce text changes."))
}

fn menubar_page() -> View {
    macro_rules! demo {
        ($name:ident, $source:literal, $label:literal) => {
            example(
                $label,
                menubar_examples::$name(),
                include_str!("examples/menubar_examples.rs"),
                include_str!(concat!(
                    "../../../references/menubar-tsx/menubar-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Menubar,
        "Organize application commands in a persistent row of menus.",
        "https://ui.shadcn.com/docs/components/base/menubar",
    )
    .child(demo!(demo, "demo", "Menubar demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Menubar::new("Application menu").menu(MenubarMenu::new(
    MenubarTrigger::new("File"),
    MenubarContent::new().child(
        MenubarItem::new("New Tab").on_select(open_tab),
    ),
))"#))
    .child(heading("Checkbox"))
    .child(demo!(checkboxes, "checkbox", "Checkbox"))
    .child(heading("Radio"))
    .child(demo!(radio, "radio", "Radio"))
    .child(heading("Submenu"))
    .child(demo!(submenu, "submenu", "Submenu"))
    .child(heading("With Icons"))
    .child(demo!(icons, "icons", "With Icons"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Menubar", "Group application menus in one keyboard stop. Use the arrow keys to move between menus."))
    .child(api_row("MenubarMenu / MenubarTrigger / MenubarContent", "Pair each top-level label with one menu. Its popup anchors to the trigger or uses the native sheet on touch screens."))
    .child(api_row("MenubarItem / MenubarSub", "Use ordinary actions, separators, nested menus, checkboxes, or radio choices. Store checked values in Rust signals and define actual keyboard shortcuts in the application."))
}

fn message_page() -> View {
    macro_rules! demo {
        ($name:ident, $source:literal, $label:literal) => {
            example(
                $label,
                message_examples::$name(),
                include_str!("examples/message_examples.rs"),
                include_str!(concat!(
                    "../../../references/message-tsx/message-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Message,
        "Lay out a conversation message with its avatar, sender, and actions.",
        "https://ui.shadcn.com/docs/components/base/message",
    )
    .child(demo!(demo, "demo", "Message demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Message::new(
    MessageContent::new()
        .child(Bubble::new(BubbleContent::text("How can I help you today?")))
        .footer(MessageFooter::text("Delivered")),
)
.avatar(MessageAvatar::new(Avatar::new("Assistant")))
.align(MessageAlign::End)"#))
    .child(heading("Avatar"))
    .child(demo!(avatars, "avatar", "Avatar"))
    .child(heading("Group"))
    .child(demo!(group, "group", "Group"))
    .child(heading("Header and Footer"))
    .child(demo!(header_footer, "header-footer", "Header and Footer"))
    .child(heading("Actions"))
    .child(demo!(actions, "actions", "Actions"))
    .child(heading("Attachment"))
    .child(demo!(attachment, "attachment", "Attachment"))
    .child(heading("Accessibility"))
    .child(muted("Name icon-only actions and use a status Marker to announce changing progress."))
    .child(heading("API Reference"))
    .child(api_row("Message / MessageGroup", "Choose start or end alignment. Group consecutive rows with the same sender."))
    .child(api_row("MessageAvatar", "Attach an avatar or reserve its gutter with empty. The avatar sits at the bottom of the message surface above its footer."))
    .child(api_row("MessageContent / MessageHeader / MessageFooter", "Compose one or more surfaces with optional sender metadata and actions. Use flush for a ghost bubble."))
}

fn message_scroller_page() -> View {
    macro_rules! demo {
        ($name:ident, $source:literal, $label:literal) => {
            example(
                $label,
                message_scroller_examples::$name(),
                include_str!("examples/message_scroller_examples.rs"),
                include_str!(concat!(
                    "../../../references/message-scroller-tsx/message-scroller-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::MessageScroller,
        "Keep a conversation readable as messages stream in, history loads, or the reader jumps between turns.",
        "https://ui.shadcn.com/docs/components/base/message-scroller",
    )
    .child(demo!(demo, "demo", "Message Scroller demo"))
    .child(heading("Usage"))
    .child(code_block(r#"let handle = MessageScrollerHandle::new();
MessageScroller::new(handle, messages)
    .auto_scroll(true)
    .render(|item| {
        text(move || item.with(|item| item.value.clone())).into_element()
    })"#))
    .child(heading("Anchoring Turns"))
    .child(demo!(anchoring, "anchoring", "Anchoring Turns"))
    .child(heading("Group Chat"))
    .child(demo!(group_chat, "group-chat", "Group Chat"))
    .child(heading("Keeping Context Visible"))
    .child(demo!(previous_context, "previous-context", "Keeping Context Visible"))
    .child(heading("Following the Live Edge"))
    .child(demo!(streaming, "streaming", "Following the Live Edge"))
    .child(heading("Opening Saved Threads"))
    .child(demo!(opening_position, "opening-position", "Opening Saved Threads"))
    .child(heading("Loading Earlier Messages"))
    .child(demo!(load_history, "load-history", "Loading Earlier Messages"))
    .child(heading("Animating New Messages"))
    .child(demo!(animation, "animation", "Animating New Messages"))
    .child(heading("Jumping to Messages"))
    .child(demo!(commands, "commands", "Jumping to Messages"))
    .child(heading("Tracking the Reader’s Position"))
    .child(demo!(visibility, "visibility", "Tracking the Reader’s Position"))
    .child(heading("Reading Scroll State"))
    .child(demo!(scrollable, "scrollable", "Reading Scroll State"))
    .child(heading("API Reference"))
    .child(api_row("MessageScroller", "Provide a keyed transcript, opening position, previous-item peek and optional follow-output. Native scrolling stays available via touch, wheel and keyboard."))
    .child(api_row("MessageScrollerItem", "Give each row a stable ID. Set anchor on any message or marker that begins a turn."))
    .child(api_row("MessageScrollerHandle", "Scroll to the start, end or a named row. Read the visible messages, current anchor and available scroll directions."))
}

fn native_select_page() -> View {
    macro_rules! demo {
        ($name:ident, $source:literal, $label:literal) => {
            example(
                $label,
                native_select_examples::$name(),
                include_str!("examples/native_select_examples.rs"),
                include_str!(concat!(
                    "../../../references/native-select-tsx/native-select-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::NativeSelect,
        "Choose one value from a compact, keyboard-accessible list.",
        "https://ui.shadcn.com/docs/components/base/native-select",
    )
    .child(demo!(demo, "demo", "Native Select demo"))
    .child(heading("Usage"))
    .child(code_block(r#"NativeSelect::new("Status")
    .option(NativeSelectOption::new("", "Select status"))
    .option(NativeSelectOption::new("todo", "Todo"))
    .option(NativeSelectOption::new("done", "Done"))"#))
    .child(muted("The Rust control uses Native Shell’s Select API. Desktop opens an anchored list, and touch devices use a bottom sheet."))
    .child(heading("Groups"))
    .child(demo!(groups, "groups", "Groups"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("NativeSelect", "Set a default value or bind a Rust signal. Use disabled or invalid for field state, and size to choose the control height."))
    .child(api_row("NativeSelectOption", "Provide a unique value and visible label. Disabled options are skipped during selection."))
    .child(api_row("NativeSelectOptGroup", "Group related options under a heading. Disabling a group disables each of its options."))
}

fn navigation_menu_page() -> View {
    component_article(
        DocsPage::NavigationMenu,
        "Group direct links and dropdown link collections in a site navigation bar.",
        "https://ui.shadcn.com/docs/components/base/navigation-menu",
    )
    .child(example(
        "Navigation Menu demo",
        navigation_menu_examples::demo(),
        include_str!("examples/navigation_menu_examples.rs"),
        include_str!("../../../references/navigation-menu-tsx/navigation-menu-demo.tsx"),
    ))
    .child(heading("Usage"))
    .child(code_block(r#"NavigationMenu::new(
    "Site navigation",
    NavigationMenuList::new()
        .item(NavigationMenuItem::new(
            NavigationMenuTrigger::new("Getting started"),
            NavigationMenuContent::new()
                .link(NavigationMenuLink::new("Installation", "https://ui.shadcn.com/docs/installation")),
        ))
        .item(NavigationMenuItem::link(
            NavigationMenuLink::new("Docs", "https://ui.shadcn.com/docs"),
        )),
)"#))
    .child(heading("Link Component"))
    .child(muted("Direct links and links inside a panel both accept a destination, optional icon, description, and current-page state."))
    .child(heading("RTL"))
    .child(example(
        "RTL",
        navigation_menu_examples::rtl(),
        include_str!("examples/navigation_menu_examples.rs"),
        include_str!("../../../references/navigation-menu-tsx/navigation-menu-rtl.tsx"),
    ))
    .child(heading("API Reference"))
    .child(api_row("NavigationMenu / NavigationMenuList", "Group top-level links and triggers under a named navigation landmark. Enable indicator to mark the open trigger."))
    .child(api_row("NavigationMenuItem / NavigationMenuTrigger", "Add a direct link, or pair a trigger with a panel. Arrow keys navigate the bar, and Escape closes the panel."))
    .child(api_row("NavigationMenuContent / NavigationMenuLink", "Compose an anchored list or grid of links. Set columns and width for larger panels."))
}

fn pagination_page() -> View {
    macro_rules! demo {
        ($function:ident, $reference:literal, $name:literal) => {
            example(
                $name,
                pagination_examples::$function(),
                include_str!("examples/pagination_examples.rs"),
                include_str!(concat!(
                    "../../../references/pagination-tsx/pagination-",
                    $reference,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Pagination,
        "Navigate a series of pages with numbered links and previous or next controls.",
        "https://ui.shadcn.com/docs/components/base/pagination",
    )
    .child(demo!(demo, "demo", "Pagination demo"))
    .child(heading("Usage"))
    .child(code_block(r#"let page = signal(2);
Pagination::new(
    PaginationContent::new().item(PaginationItem::new(
        PaginationLink::action("1", move || page.set(1))
            .label("Page 1")
            .current(move || page.get() == 1),
    )),
)"#))
    .child(heading("Simple"))
    .child(demo!(simple, "simple", "Simple"))
    .child(heading("Icons Only"))
    .child(demo!(icons_only, "icons-only", "Icons Only"))
    .child(heading("Page Links"))
    .child(muted("Use PaginationLink::new(label, href) for a destination or PaginationLink::action(label, handler) when the page changes in place. Bind current to mark the active page for assistive technology."))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Pagination / PaginationContent / PaginationItem", "Group page controls in a named navigation landmark."))
    .child(api_row("PaginationLink", "Set the current page and an accessible label. Disable unavailable actions or destinations."))
    .child(api_row("PaginationPrevious / PaginationNext / PaginationEllipsis", "Add directional controls and a visual marker for omitted pages. Previous and next collapse to chevrons below 640 points."))
}

fn popover_page() -> View {
    macro_rules! demo {
        ($function:ident, $reference:literal, $name:literal) => {
            example(
                $name,
                popover_examples::$function(),
                include_str!("examples/popover_examples.rs"),
                include_str!(concat!(
                    "../../../references/popover-tsx/popover-",
                    $reference,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Popover,
        "Show contextual information or a small form next to a trigger.",
        "https://ui.shadcn.com/docs/components/base/popover",
    )
    .child(demo!(demo, "demo", "Popover demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Popover::new(
    PopoverTrigger::new("Open Popover"),
    PopoverContent::new()
        .label("Dimensions")
        .align(Align::Start)
        .child(PopoverHeader::new()
            .child(PopoverTitle::new("Dimensions"))
            .child(PopoverDescription::new("Set the dimensions for the layer."))),
)"#))
    .child(heading("Basic"))
    .child(demo!(basic, "basic", "Basic"))
    .child(heading("Align"))
    .child(demo!(alignments, "alignments", "Align"))
    .child(heading("With Form"))
    .child(demo!(form, "form", "With Form"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Popover / PopoverTrigger", "Pair an overlay with a trigger. Use open to share state, or set default_open for the initial state."))
    .child(api_row("PopoverContent", "Set a side, alignment, offsets and preferred width. Native Shell keeps the overlay within the viewport and flips sides when space is limited."))
    .child(api_row("PopoverHeader / PopoverTitle / PopoverDescription", "Compose the popup text. Give the content an accessible label and set initial_focus for a desktop form."))
}

fn progress_page() -> View {
    macro_rules! demo {
        ($function:ident, $reference:literal, $name:literal) => {
            example(
                $name,
                progress_examples::$function(),
                include_str!("examples/progress_examples.rs"),
                include_str!(concat!(
                    "../../../references/progress-tsx/progress-",
                    $reference,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Progress,
        "Show how much of a task is complete.",
        "https://ui.shadcn.com/docs/components/base/progress",
    )
    .child(demo!(demo, "demo", "Progress demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Progress::new(56.0)
    .label(ProgressLabel::new("Upload progress"))
    .display(ProgressValue::new())"#))
    .child(heading("Label"))
    .child(demo!(label, "label", "Label"))
    .child(heading("Controlled"))
    .child(demo!(controlled, "controlled", "Controlled"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Progress", "Pass a number or Rust signal. The default range is 0–100. Indeterminate progress leaves the current value unset and the track empty."))
    .child(api_row("ProgressLabel / ProgressValue", "Add a visible label or percentage. A formatter can provide localized text for the display and assistive technology."))
    .child(api_row("ProgressTrack / ProgressIndicator", "Customize the track height and indicator color."))
}

fn questionnaire_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                questionnaire_examples::$function(),
                include_str!("examples/questionnaire_examples.rs"),
                include_str!(concat!(
                    "../../../references/questionnaire-tsx/questionnaire-",
                    $file,
                    ".tsx"
                )),
            )
        };
        ($function:ident, $file:literal, $name:literal, section) => {
            view()
                .gap_3()
                .child(heading($name))
                .child(demo!($function, $file, $name))
        };
    }
    component_article(
        DocsPage::Questionnaire,
        "Guide users through a sequence of questions with choices or custom answers.",
        "https://ui.shadcn.com/docs/components/base/questionnaire",
    )
    .child(demo!(demo, "demo", "Questionnaire demo"))
    .child(heading("Usage"))
    .child(code_block(r#"const ITEMS: &[QuestionnaireItem] = &[
    QuestionnaireItem::new("direction", "What should we build?")
        .choices(&[QuestionnaireChoice::new("plan", "A plan")])
        .input(QuestionnaireInput::new("Another idea", "Describe your idea…")),
];
Questionnaire::new(ITEMS).on_submit(move |answers| {
    let direction = answers.first("direction");
    // Use the answer in the host application.
})"#))
    .child(demo!(multiple, "multiple", "Multiple Selection", section))
    .child(demo!(freeform, "freeform", "Freeform Answer", section))
    .child(demo!(skip, "skip", "Explicit Skip", section))
    .child(demo!(shortcuts, "shortcuts", "Shortcuts", section))
    .child(demo!(validation, "validation", "Custom Validation", section))
    .child(demo!(controlled, "controlled", "Controlled", section))
    .child(demo!(resume, "resume", "Resume", section))
    .child(demo!(conditional, "conditional", "Conditional Items", section))
    .child(demo!(navigation_state, "navigation-state", "Navigation State", section))
    .child(demo!(progress, "progress", "Custom Progress", section))
    .child(demo!(animated, "animated", "Animated Items", section))
    .child(demo!(card, "card", "Card", section))
    .child(demo!(dialog, "dialog", "Dialog", section))
    .child(heading("API Reference"))
    .child(api_row("QuestionnaireItem / QuestionnaireChoice / QuestionnaireInput", "Declare named items and fixed answers, and optionally add a named text field. Items can be required, accept multiple answers, or depend on earlier answers."))
    .child(api_row("Questionnaire / QuestionnaireAnswers", "Provide defaults, share the active item with a Rust signal, and receive the answers from enabled items in the submission handler."))
    .child(api_row("QuestionnaireProgress / QuestionnaireError", "Render the current step and total, or report a validation error on a specific item."))
}

fn radio_group_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                radio_group_examples::$function(),
                include_str!("examples/radio_group_examples.rs"),
                include_str!(concat!(
                    "../../../references/radio-group-tsx/radio-group-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::RadioGroup,
        "Choose one option from a set with a visible selection and keyboard navigation.",
        "https://ui.shadcn.com/docs/components/base/radio-group",
    )
    .child(demo!(demo, "demo", "Radio Group demo"))
    .child(heading("Usage"))
    .child(code_block(r#"RadioGroup::new("Display density")
    .default_value("comfortable")
    .item(RadioGroupItem::new("default", "Default"))
    .item(RadioGroupItem::new("comfortable", "Comfortable"))
    .item(RadioGroupItem::new("compact", "Compact"))"#))
    .child(heading("Description"))
    .child(demo!(description, "description", "Description"))
    .child(heading("Choice Card"))
    .child(demo!(choice_card, "choice-card", "Choice Card"))
    .child(heading("Fieldset"))
    .child(demo!(fieldset, "fieldset", "Fieldset"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("RadioGroup", "Set a default value or bind a Rust signal. Use on_change to receive user selections, and invalid to expose an invalid group."))
    .child(api_row("RadioGroupItem", "Give each item a unique value and a label. Add a description, disable an option, or use card for a full-width choice."))
    .child(api_row("Keyboard", "Tab enters at the selected item or first enabled item. Arrow keys wrap and skip disabled options, and Space selects the focused one."))
}

fn resizable_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                resizable_examples::$function(),
                include_str!("examples/resizable_examples.rs"),
                include_str!(concat!(
                    "../../../references/resizable-tsx/resizable-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Resizable,
        "Divide a layout into panels that resize with a pointer, touch, or the keyboard.",
        "https://ui.shadcn.com/docs/components/base/resizable",
    )
    .child(demo!(demo, "demo", "Resizable demo"))
    .child(heading("Usage"))
    .child(code_block(r#"ResizablePanelGroup::new(ResizableOrientation::Horizontal)
    .panel(ResizablePanel::new(text("One")).default_size(50.0))
    .handle(ResizableHandle::new().with_handle(true))
    .panel(ResizablePanel::new(text("Two")).default_size(50.0))"#))
    .child(heading("Vertical"))
    .child(demo!(vertical, "vertical", "Vertical"))
    .child(heading("Handle"))
    .child(demo!(handle, "handle", "Handle"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("ResizablePanelGroup", "Set an orientation or nest groups on different axes. Bind layout to share percentages with the host, and use on_resize for completed changes."))
    .child(api_row("ResizablePanel", "Set a default size and optional minimum and maximum percentages. Percentages exclude the space occupied by dividers."))
    .child(api_row("ResizableHandle", "Set a label, show a grip or disable resizing. Focus a divider and press an arrow key to resize by one percentage point. Home and End move to the limits."))
}

fn scroll_area_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                scroll_area_examples::$function(),
                include_str!("examples/scroll_area_examples.rs"),
                include_str!(concat!(
                    "../../../references/scroll-area-tsx/scroll-area-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::ScrollArea,
        "Scroll clipped content within a fixed viewport with a draggable indicator.",
        "https://ui.shadcn.com/docs/components/base/scroll-area",
    )
    .child(demo!(demo, "demo", "Scroll Area demo"))
    .child(heading("Usage"))
    .child(code_block(r#"ScrollArea::new("Release tags", tags)
    .width(192.0)
    .height(288.0)
    .scroll_bar(ScrollBar::new(ScrollAreaOrientation::Vertical))"#))
    .child(heading("Horizontal"))
    .child(demo!(horizontal, "horizontal-demo", "Horizontal"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("ScrollArea", "Name and size a viewport. Give it an optional reference for programmatic scrolling."))
    .child(api_row("ScrollBar", "Choose the viewport's scrolling axis and when to show the draggable indicator. Each viewport scrolls on one axis."))
    .child(api_row("Keyboard", "Focus the viewport. Use arrow keys along the scrolling axis, Page Up and Page Down for larger steps, or Home and End for the boundaries."))
}

fn select_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                select_examples::$function(),
                include_str!("examples/select_examples.rs"),
                include_str!(concat!(
                    "../../../references/select-tsx/select-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Select,
        "Choose a value from a styled list of options.",
        "https://ui.shadcn.com/docs/components/base/select",
    )
    .child(demo!(demo, "demo", "Select demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Select::new(
    "Theme",
    SelectTrigger::new(SelectValue::new("Choose a theme")),
    SelectContent::new()
        .item(SelectItem::new("light", "Light"))
        .item(SelectItem::new("dark", "Dark"))
        .item(SelectItem::new("system", "System")),
)"#))
    .child(heading("Align Item With Trigger"))
    .child(demo!(align_item, "align-item", "Align Item With Trigger"))
    .child(heading("Groups"))
    .child(demo!(groups, "groups", "Groups"))
    .child(heading("Scrollable"))
    .child(demo!(scrollable, "scrollable", "Scrollable"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Select", "Use default_value for the initial selection or bind a Rust signal with value. Bind open to share the popup state with the host."))
    .child(api_row("SelectTrigger / SelectValue", "Set the trigger size and the placeholder shown before a choice is made."))
    .child(api_row("SelectContent / SelectGroup / SelectItem", "Add labeled groups and options. Set the maximum list height or choose whether a selected item overlaps the desktop trigger."))
    .child(muted("On touch devices, Native Shell displays the list in a bottom sheet. Longer lists use the Native Shell scrollbar and keyboard navigation."))
}

fn separator_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $name:literal) => {
            example(
                $name,
                separator_examples::$function(),
                include_str!("examples/separator_examples.rs"),
                include_str!(concat!(
                    "../../../references/separator-tsx/separator-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Separator,
        "Visually divide sections, controls, or rows.",
        "https://ui.shadcn.com/docs/components/base/separator",
    )
    .child(demo!(demo, "demo", "Separator demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Separator::new()

Separator::new().orientation(SeparatorOrientation::Vertical)"#))
    .child(heading("Vertical"))
    .child(demo!(vertical, "vertical", "Vertical"))
    .child(heading("Menu"))
    .child(demo!(menu, "menu", "Menu"))
    .child(heading("List"))
    .child(demo!(list, "list", "List"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Separator", "Horizontal lines fill their container width. Vertical lines stretch to the height of their row."))
    .child(api_row("Accessibility", "A line is decorative by default. Use label to expose a named break as static text in Native Shell."))
}

fn sheet_page() -> View {
    macro_rules! demo {
        ($example:ident, $source:literal, $title:literal) => {
            example(
                $title,
                sheet_examples::$example(),
                include_str!("examples/sheet_examples.rs"),
                include_str!(concat!(
                    "../../../references/sheet-tsx/sheet-",
                    $source,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Sheet,
        "Open a modal panel along an edge to show content related to the current screen.",
        "https://ui.shadcn.com/docs/components/base/sheet",
    )
    .child(demo!(demo, "demo", "Sheet demo"))
    .child(heading("Usage"))
    .child(code_block(r#"let sheet = Sheet::new(signal(false));
view()
    .child(SheetTrigger::new(sheet, "Open"))
    .child(
        SheetContent::new(sheet, SheetHeader::new(SheetTitle::new("Edit profile")))
            .side(SheetSide::Right)
            .footer(move || SheetFooter::new().child(SheetClose::new(sheet, "Close"))),
    )"#))
    .child(heading("Composition"))
    .child(muted("Use a trigger and a content panel with a title. Add a description, scrollable body, and footer actions as needed."))
    .child(heading("Side"))
    .child(demo!(sides, "side", "Side"))
    .child(heading("No Close Button"))
    .child(demo!(no_close_button, "no-close-button", "No Close Button"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Sheet / SheetTrigger / SheetClose", "Use a Rust signal for open state and open or close the same panel from separate controls."))
    .child(api_row("SheetContent", "Choose a physical edge, set text direction, and pass None to close_button to hide the corner control."))
    .child(api_row("SheetHeader / SheetTitle / SheetDescription / SheetFooter", "Name and describe the modal, with footer actions visible while the body scrolls."))
}

fn sidebar_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                sidebar_examples::$function(),
                include_str!("examples/sidebar_examples.rs"),
                include_str!(concat!(
                    "../../../references/sidebar-tsx/sidebar-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Sidebar,
        "Build a navigation sidebar that adapts to the available screen width.",
        "https://ui.shadcn.com/docs/components/base/sidebar",
    )
    .child(demo!(demo, "demo", "Sidebar demo"))
    .child(heading("Usage"))
    .child(code_block(r#"SidebarProvider::new(|state: SidebarState| {
    Sidebar::new(state, |parts| {
        sidebar_header().child(
            SidebarMenuButton::new(parts, "Home").icon(text("⌂")).surface(),
        )
    })
    .collapsible(SidebarCollapsible::Icon)
    .layout(sidebar_inset().child(SidebarTrigger::new(state)))
})"#))
    .child(heading("Composition"))
    .child(muted("Add a header and footer around scrollable content. Organize navigation into groups with menu buttons, submenus, actions, and badges."))
    .child(heading("Controlled Sidebar"))
    .child(demo!(controlled, "controlled", "Controlled Sidebar"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("SidebarProvider / SidebarState", "Own desktop and mobile state independently. Bind a desktop signal or persist the internal state. Command+B or Ctrl+B toggles the active layout."))
    .child(api_row("Sidebar / SidebarInset", "Choose a left or right edge, Sidebar/Floating/Inset appearance, and Offcanvas/Icon/None collapse mode. Resizable viewports switch to a modal below 768 points."))
    .child(api_row("SidebarTrigger / sidebar_rail", "Use the focusable trigger in main content and a pointer-operated rail along the desktop edge."))
    .child(api_row("SidebarMenuButton / SidebarParts", "Compose compact or expanded menu buttons with icons, detail text, active state, and links. Place secondary content in expanded_only."))
}

fn skeleton_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                skeleton_examples::$function(),
                include_str!("examples/skeleton_examples.rs"),
                include_str!(concat!(
                    "../../../references/skeleton-tsx/skeleton-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Skeleton,
        "Reserve space for content while it loads.",
        "https://ui.shadcn.com/docs/components/base/skeleton",
    )
    .child(demo!(demo, "demo", "Skeleton demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Skeleton::new()
    .surface()
    .w(px(100.0))
    .h(px(20.0))
    .rounded_full()"#))
    .child(heading("Avatar"))
    .child(demo!(avatar, "avatar", "Avatar"))
    .child(heading("Card"))
    .child(demo!(card, "card", "Card"))
    .child(heading("Text"))
    .child(demo!(text_lines, "text", "Text"))
    .child(heading("Form"))
    .child(demo!(form, "form", "Form"))
    .child(heading("Table"))
    .child(demo!(table, "table", "Table"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Skeleton", "Set the size and shape on the surface. Pass animated(false) to disable the pulse. Reduced-motion settings also disable it."))
    .child(api_row("Accessibility", "The shape is decorative. Set the loading region's busy state while waiting for data, then clear it when the content is ready."))
}

fn slider_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                slider_examples::$function(),
                include_str!("examples/slider_examples.rs"),
                include_str!(concat!(
                    "../../../references/slider-tsx/slider-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Slider,
        "Select one or more values along a track.",
        "https://ui.shadcn.com/docs/components/base/slider",
    )
    .child(demo!(demo, "demo", "Slider demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Slider::new([33.0]).range(0.0, 100.0).step(1.0)

Slider::new([25.0, 50.0]).step(5.0).labels(["Minimum", "Maximum"])"#))
    .child(heading("Range"))
    .child(demo!(range, "range", "Range"))
    .child(heading("Multiple Thumbs"))
    .child(demo!(multiple, "multiple", "Multiple Thumbs"))
    .child(heading("Vertical"))
    .child(demo!(vertical, "vertical", "Vertical"))
    .child(heading("Controlled"))
    .child(demo!(controlled, "controlled", "Controlled"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Slider", "Provide one value per thumb in a fixed-size array. Choose bounds, step, accessible labels, orientation, and disabled state. Dragging a thumb can push adjacent thumbs."))
    .child(api_row("Value and commit", "Bind a caller-owned signal with value. Listen for updates using on_value_change and for completed changes using on_value_commit."))
    .child(api_row("Keyboard", "Arrow keys move one step. Use Page Up, Page Down, or Shift with an arrow for the larger increment. Home and End move to the available endpoints."))
}

fn spinner_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                spinner_examples::$function(),
                include_str!("examples/spinner_examples.rs"),
                include_str!(concat!(
                    "../../../references/spinner-tsx/spinner-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Spinner,
        "Indicate that a task is still running.",
        "https://ui.shadcn.com/docs/components/base/spinner",
    )
    .child(demo!(demo, "demo", "Spinner demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Spinner::new()

Spinner::new().size(24.0).label("Loading messages")"#))
    .child(heading("Customization"))
    .child(demo!(custom, "custom", "Customization"))
    .child(heading("Size"))
    .child(demo!(size, "size", "Size"))
    .child(heading("Button"))
    .child(demo!(button, "button", "Button"))
    .child(heading("Badge"))
    .child(demo!(badge, "badge", "Badge"))
    .child(heading("Input Group"))
    .child(demo!(input_group, "input-group", "Input Group"))
    .child(heading("Empty"))
    .child(demo!(empty, "empty", "Empty"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Spinner", "Choose a size, an inherited or custom color, and an optional replacement icon. Rotation follows the reduced-motion setting."))
    .child(api_row("Accessibility", "A named spinner exposes an indeterminate progress indicator. Use decorative inside a control or message with its own loading description."))
}

fn switch_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                switch_examples::$function(),
                include_str!("examples/switch_examples.rs"),
                include_str!(concat!(
                    "../../../references/switch-tsx/switch-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Switch,
        "Turn a setting on or off.",
        "https://ui.shadcn.com/docs/components/base/switch",
    )
    .child(example_with_rsx(
        "Switch composition",
        composition::switch_rsx::example(),
        composition::switch_rust::CODE,
        Some(tsx_examples::switch::EXAMPLE),
        Some(composition::switch_rsx::CODE),
    ))
    .child(heading("Usage"))
    .child(muted("Use switch().accessibility_label(...) to name the setting. For Rust-owned state, use default_checked. When you bind checked, update it from on_change in Rust or onChange in Native Shell TSX."))
    .child(muted("To use the TSX example, call components::switch::tsx::register in your nsh-foundation/native module."))
    .child(demo!(demo, "demo", "Switch demo"))
    .child(heading("Description"))
    .child(demo!(description, "description", "Description"))
    .child(heading("Choice Card"))
    .child(demo!(choice_card, "choice-card", "Choice Card"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("Size"))
    .child(demo!(sizes, "sizes", "Size"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Switch", "Set the accessible name and optional description, initial or controlled value, size, disabled state, and validation state."))
    .child(api_row("Control reference", "Use control_ref to connect a visible label to the switch and apply label activation to the shared value."))
}

fn table_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                table_examples::$function(),
                include_str!("examples/table_examples.rs"),
                include_str!(concat!(
                    "../../../references/table-tsx/table-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }
    component_article(
        DocsPage::Table,
        "Display structured data in aligned rows and columns.",
        "https://ui.shadcn.com/docs/components/base/table",
    )
    .child(demo!(demo, "demo", "Table demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Table::new("Invoices", [TableColumn::fixed(100.0), TableColumn::flexible(100.0)])
    .header(TableHeader::new().row(
        TableRow::new().head(TableHead::new("Invoice")).head(TableHead::new("Amount").align_end()),
    ))
    .body(TableBody::new().row(
        TableRow::new().cell(TableCell::text("INV001")).cell(TableCell::text("$250.00").align_end()),
    ))
    .caption(TableCaption::new("Recent invoices"))"#))
    .child(heading("Composition"))
    .child(muted("Give header, body, and footer rows the same column layout. Use span to merge cells across adjacent columns."))
    .child(heading("Footer"))
    .child(demo!(footer, "footer", "Footer"))
    .child(heading("Actions"))
    .child(demo!(actions, "actions", "Actions"))
    .child(heading("Data Table"))
    .child(muted("Open Data Table from the component index for examples with sorting, filtering, and pagination."))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Table / TableColumn", "Set fixed and minimum widths. When the viewport is wider, flexible columns share the extra space. When it is narrower, the table scrolls horizontally."))
    .child(api_row("TableRow / TableHead / TableCell", "Use native table semantics, logical alignment, and optional selection styles. Cells can hold text or interactive elements."))
}

fn tabs_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                tabs_examples::$function(),
                include_str!("examples/tabs_examples.rs"),
                include_str!(concat!("../../../references/tabs-tsx/tabs-", $file, ".tsx")),
            )
        };
    }
    component_article(
        DocsPage::Tabs,
        "Switch between related sections of content.",
        "https://ui.shadcn.com/docs/components/base/tabs",
    )
    .child(demo!(demo, "demo", "Tabs demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Tabs::new(TabsList::new("Account")
    .trigger(TabsTrigger::new("account", "Account"))
    .trigger(TabsTrigger::new("password", "Password")))
    .default_value("account")
    .content(TabsContent::new("account", || text("Make changes to your account.")))
    .content(TabsContent::new("password", || text("Change your password.")))"#))
    .child(heading("Composition"))
    .child(muted("Give each trigger a distinct value. A content panel with the same value appears when its trigger is selected."))
    .child(heading("Line"))
    .child(demo!(line, "line", "Line"))
    .child(heading("Vertical"))
    .child(demo!(vertical, "vertical", "Vertical"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Icons"))
    .child(demo!(icons, "icons", "Icons"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Tabs", "Set the initial or bound value. Read changes with on_change. Use vertical orientation for a stacked list."))
    .child(api_row("TabsList / TabsTrigger", "Arrow keys, Home, and End move focus. Enter or Space selects the focused tab. Enable activate_on_focus to select as arrow keys move focus."))
    .child(api_row("TabsContent", "The active panel is exposed as a named, focusable Native Shell region."))
}

fn textarea_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $title:literal) => {
            example(
                $title,
                textarea_examples::$function(),
                include_str!("examples/textarea_examples.rs"),
                include_str!(concat!(
                    "../../../references/textarea-tsx/textarea-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::Textarea,
        "Collect and edit multiline text.",
        "https://ui.shadcn.com/docs/components/base/textarea",
    )
    .child(demo!(demo, "demo", "Textarea demo"))
    .child(heading("Usage"))
    .child(code_block(r#"textarea().label("Message").placeholder("Type your message here.")"#))
    .child(heading("Field"))
    .child(demo!(field, "field", "Field"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Invalid"))
    .child(demo!(invalid, "invalid", "Invalid"))
    .child(heading("Button"))
    .child(demo!(button, "button", "Button"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("textarea()", "Use Native Shell's Input methods to set a placeholder, initial or bound text, and change events. Enter inserts a new line and the textarea grows with its content."))
    .child(api_row("invalid_textarea(invalid)", "Bind a validation condition to both the focus ring and the accessible invalid state. Use FieldDescription to associate instructions or error text."))
}

fn toast_page() -> View {
    let toaster = Toaster::new();
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                toast_examples::$function(toaster),
                include_str!("examples/toast_examples.rs"),
                include_str!(concat!(
                    "../../../references/toast-tsx/toast-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::Toast,
        "Show a temporary notification without interrupting the page.",
        "https://ui.shadcn.com/docs/components/base/toast",
    )
    .child(demo!(demo, "demo", "Toast demo"))
    .child(heading("Usage"))
    .child(muted("Create a Toaster in your screen, mount its viewport once, and pass the handle to controls that show notifications."))
    .child(code_block(r#"let toaster = Toaster::new();
view()
    .child(Button::new("Show Toast").on_press(move || {
        toaster.show(Toast::new("Event created").description("Sunday, December 3 at 9:00 AM"));
    }))
    .child(toaster.viewport())"#))
    .child(heading("Types"))
    .child(demo!(types, "types", "Types"))
    .child(heading("Action"))
    .child(muted("Attach an action with a label and callback. The Undo action in the main example closes its notification."))
    .child(code_block(r#"Toast::new("Event created").action("Undo", move || undo_event())"#))
    .child(heading("Promise"))
    .child(demo!(promise, "promise", "Promise"))
    .child(heading("API Reference"))
    .child(api_row("Toast", "Set a title, description, type, timeout, or action. Use timeout(None) for a notification that stays open until closed."))
    .child(api_row("Toaster", "Show or close a toast by its ID. Use promise to update a pending toast from a Rust future."))
    .child(toaster.viewport())
}

fn toggle_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                toggle_examples::$function(),
                include_str!("examples/toggle_examples.rs"),
                include_str!(concat!(
                    "../../../references/toggle-tsx/toggle-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::Toggle,
        "Turn a setting or formatting option on and off.",
        "https://ui.shadcn.com/docs/components/base/toggle",
    )
    .child(demo!(demo, "demo", "Toggle demo"))
    .child(heading("Usage"))
    .child(code_block(r#"Toggle::new("Bookmark")
    .label("Toggle bookmark")
    .variant(ToggleVariant::Outline)
    .size(ToggleSize::Small)"#))
    .child(heading("Outline"))
    .child(demo!(outline, "outline", "Outline"))
    .child(heading("With Text"))
    .child(demo!(with_text, "text", "With Text"))
    .child(heading("Size"))
    .child(demo!(sizes, "sizes", "Size"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("Toggle", "Use default_pressed for local state, or bind pressed and apply changes in on_change. Enter and Space toggle the focused button."))
    .child(api_row("ToggleVariant / ToggleSize", "Choose a transparent or outlined surface and a small, default, or large size."))
    .child(api_row("icon", "Render a decorative icon with access to the current pressed state."))
}

fn toggle_group_page() -> View {
    macro_rules! demo {
        ($function:ident, $file:literal, $label:literal) => {
            example(
                $label,
                toggle_group_examples::$function(),
                include_str!("examples/toggle_group_examples.rs"),
                include_str!(concat!(
                    "../../../references/toggle-group-tsx/toggle-group-",
                    $file,
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::ToggleGroup,
        "Group related toggle buttons with single or multiple selection.",
        "https://ui.shadcn.com/docs/components/base/toggle-group",
    )
    .child(demo!(demo, "demo", "Toggle Group demo"))
    .child(heading("Usage"))
    .child(code_block(r#"ToggleGroup::new("Call filter")
    .default_value(["all"])
    .variant(ToggleVariant::Outline)
    .spacing(0.0)
    .items([
        ToggleGroupItem::new("all", "All"),
        ToggleGroupItem::new("missed", "Missed"),
    ])"#))
    .child(heading("Composition"))
    .child(muted("Give each item a distinct value and an accessible label. Use item content to render an icon or a custom layout."))
    .child(heading("Outline"))
    .child(demo!(outline, "outline", "Outline"))
    .child(heading("Size"))
    .child(demo!(sizes, "sizes", "Size"))
    .child(heading("Spacing"))
    .child(demo!(spacing, "spacing", "Spacing"))
    .child(heading("Vertical"))
    .child(demo!(vertical, "vertical", "Vertical"))
    .child(heading("Disabled"))
    .child(demo!(disabled, "disabled", "Disabled"))
    .child(heading("Custom"))
    .child(demo!(custom, "font-weight-selector", "Custom"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
    .child(heading("API Reference"))
    .child(api_row("ToggleGroup", "Set the default or bound values, optional multiple selection, size, spacing, and orientation. Arrow keys move focus along the group and Enter or Space changes selection."))
    .child(api_row("ToggleGroupItem", "Set a stable value and label. Optionally disable the item or replace its text with custom content."))
}

fn tooltip_page() -> View {
    macro_rules! demo {
        ($function:ident, $label:literal) => {
            example(
                $label,
                tooltip_examples::$function(),
                include_str!("examples/tooltip_examples.rs"),
                include_str!(concat!(
                    "../../../references/tooltip-tsx/tooltip-",
                    stringify!($function),
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::Tooltip,
        "Describe a control when the pointer rests on it or the keyboard focuses it.",
        "https://ui.shadcn.com/docs/components/base/tooltip",
    )
    .child(demo!(demo, "Tooltip demo"))
    .child(heading("Usage"))
    .child(code_block(r#"TooltipContent::new("Add to library")
    .attach(Button::new("Hover").variant(ButtonVariant::Outline).surface())"#))
    .child(heading("Composition"))
    .child(muted("Attach tooltip content to an interactive surface. The same control handles pointer and keyboard focus."))
    .child(heading("Side"))
    .child(demo!(sides, "Side"))
    .child(heading("With Keyboard Shortcut"))
    .child(demo!(keyboard, "With Keyboard Shortcut"))
    .child(heading("Disabled Button"))
    .child(demo!(disabled, "Disabled Button"))
    .child(heading("RTL"))
    .child(demo!(rtl, "RTL"))
    .child(heading("API Reference"))
    .child(api_row("TooltipContent", "Set the side, alignment, offset, or pointer delay. Use render for custom content."))
    .child(api_row("attach", "Attach the tooltip to a control or a focusable wrapper around a disabled control. Escape dismisses it. Touch alone follows Native Shell’s hint behavior."))
}

fn trace_page() -> View {
    macro_rules! demo {
        ($function:ident, $name:literal, $label:literal, $file:literal) => {
            example(
                $label,
                trace_examples::$function(),
                include_str!($file),
                include_str!(concat!(
                    "../../../references/trace-tsx/trace-",
                    $name,
                    ".tsx"
                )),
            )
        };
        ($function:ident, $name:literal, $label:literal) => {
            demo!($function, $name, $label, "examples/trace_examples.rs")
        };
    }

    component_article(
        DocsPage::Trace,
        "Show what happened in a process, with expandable output and related resources.",
        "https://ui.shadcn.com/docs/components/base/trace",
    )
    .child(demo!(demo, "demo", "Trace demo", "examples/trace_examples/replay.rs"))
    .child(heading("Usage"))
    .child(code_block(r#"Trace::new().child(
    TraceLine::new(TraceTitle::new("Ran pnpm test"))
        .description(TraceDescription::new("14s")),
)"#))
    .child(heading("Composition"))
    .child(muted("Compose each row with a title, optional icon, and description. Wrap the row in a disclosure, link, or modal trigger to add an action."))
    .child(heading("Trace Line"))
    .child(demo!(static_line, "static", "Trace Line"))
    .child(heading("Status"))
    .child(demo!(status, "status", "Status"))
    .child(heading("Description"))
    .child(demo!(description, "description", "Description"))
    .child(heading("Collapsible"))
    .child(demo!(collapsible, "collapsible", "Collapsible"))
    .child(heading("Open State"))
    .child(demo!(open, "open", "Open State"))
    .child(heading("Link"))
    .child(demo!(link, "link", "Link"))
    .child(heading("Dialog"))
    .child(demo!(dialog, "dialog", "Dialog"))
    .child(heading("Sheet"))
    .child(demo!(sheet, "sheet", "Sheet"))
    .child(heading("Group"))
    .child(demo!(group, "group", "Group"))
    .child(heading("Sources"))
    .child(demo!(sources, "sources", "Sources"))
    .child(heading("Chain of Thought"))
    .child(demo!(steps, "steps", "Chain of Thought"))
    .child(heading("Commit"))
    .child(demo!(commit, "commit", "Commit"))
    .child(heading("Whole Turn"))
    .child(demo!(turn, "turn", "Whole Turn"))
    .child(heading("Reasoning"))
    .child(demo!(thinking, "thinking", "Reasoning", "examples/trace_examples/replay.rs"))
    .child(heading("Tool"))
    .child(demo!(tool_calls, "tool-calls", "Tool", "examples/trace_examples/replay.rs"))
    .child(heading("API Reference"))
    .child(api_row("TraceLine", "Compose a title, icon, indicator, and description. Running titles respect reduced motion."))
    .child(api_row("TraceCollapsible", "Set a default or bound open state and compose a trigger and detail content."))
    .child(muted("The Rust chat, reasoning and tool previews play prerecorded examples. The TSX versions use the shadcn docs chat fixture helper."))
}

fn typography_page() -> View {
    macro_rules! demo {
        ($function:ident, $name:literal, $label:literal) => {
            example(
                $label,
                typography_examples::$function(),
                include_str!("examples/typography_examples.rs"),
                include_str!(concat!(
                    "../../../references/typography-tsx/typography-",
                    $name,
                    ".tsx"
                )),
            )
        };
    }

    component_article(
        DocsPage::Typography,
        "Set the hierarchy and rhythm of headings, paragraphs, and other article content.",
        "https://ui.shadcn.com/docs/components/base/typography",
    )
    .child(demo!(demo, "demo", "Typography demo"))
    .child(heading("h1"))
    .child(demo!(h1, "h1", "h1"))
    .child(heading("h2"))
    .child(demo!(h2, "h2", "h2"))
    .child(heading("h3"))
    .child(demo!(h3, "h3", "h3"))
    .child(heading("h4"))
    .child(demo!(h4, "h4", "h4"))
    .child(heading("p"))
    .child(demo!(p, "p", "p"))
    .child(heading("blockquote"))
    .child(demo!(blockquote, "blockquote", "blockquote"))
    .child(heading("table"))
    .child(demo!(table, "table", "table"))
    .child(heading("list"))
    .child(demo!(list, "list", "list"))
    .child(heading("Inline code"))
    .child(demo!(inline_code, "inline-code", "Inline code"))
    .child(heading("Lead"))
    .child(demo!(lead, "lead", "Lead"))
    .child(heading("Large"))
    .child(demo!(large, "large", "Large"))
    .child(heading("Small"))
    .child(demo!(small, "small", "Small"))
    .child(heading("Muted"))
    .child(demo!(muted, "muted", "Muted"))
    .child(heading("RTL"))
    .child(demo!(rtl, "rtl", "RTL"))
}

fn desktop_navigation() -> bool {
    window().width.0 >= 940.0
}

const DOCS_HEADER_HEIGHT: f32 = 58.0;
const WIDE_DOCS_WIDTH: f32 = 1220.0;
const THEME_ICON: &str = "M8 1.5v1.25M8 13.25v1.25M1.5 8h1.25M13.25 8h1.25M3.4 3.4l.9.9m7.4 7.4.9.9m0-9.2-.9.9m-7.4 7.4-.9.9M11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z";

fn docs_header(
    mobile_menu: impl Fn() -> AnyElement + Copy + 'static,
    desktop_sections: impl Fn() -> Option<AnyElement> + Copy + 'static,
    open_components: impl Fn() + Copy + 'static,
    open_blocks: impl Fn() + Copy + 'static,
    toggle_theme: impl Fn() + 'static,
) -> View {
    view()
        .flex_row()
        .items_center()
        .justify_between()
        .flex_shrink_0()
        .h(px(DOCS_HEADER_HEIGHT))
        .px(|| px(if desktop_navigation() { 24.0 } else { 12.0 }))
        .gap_1()
        .border_b(px(1.0))
        .border_color(|| theme().border)
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_2()
                .min_w(px(0.0))
                .child(when(|| !desktop_navigation(), mobile_menu))
                .child(
                    text("nsh-foundation")
                        .font_weight(FontWeight::SEMIBOLD)
                        .truncate(),
                )
                .child(when(desktop_navigation, move || {
                    view()
                        .flex_row()
                        .gap_1()
                        .child(button("Components", Variant::Ghost, open_components))
                        .child(button("Blocks", Variant::Ghost, open_blocks))
                })),
        )
        .child(
            view()
                .flex_row()
                .items_center()
                .flex_shrink_0()
                .gap_1()
                .child(when(desktop_navigation, move || dynamic(desktop_sections)))
                .child(
                    Button::new("NSH Shell")
                        .variant(ButtonVariant::Ghost)
                        .href("https://nsh.dev/")
                        .h(px(44.0)),
                )
                .child(
                    Button::new("GitHub")
                        .variant(ButtonVariant::Ghost)
                        .href("https://github.com/csdog/nsh-foundation")
                        .h(px(44.0)),
                )
                .child(view().w(px(1.0)).h(px(16.0)).bg(|| theme().border))
                .child(
                    Button::new("Theme")
                        .size(ButtonSize::Icon)
                        .variant(ButtonVariant::Ghost)
                        .icon_start(crate::examples::toggle_examples::icon(THEME_ICON, false))
                        .on_press(toggle_theme)
                        .size(px(44.0)),
                ),
        )
}

fn docs_menu(label: &'static str, content: impl Fn(Signal<bool>) -> View + 'static) -> AnyElement {
    let open = signal(false);
    let panel = element_ref();
    let class = "transition duration-100";
    popover()
        .open(move || Some(open.get()))
        .on_open_change(move |event: &PopoverEvent| {
            if let PopoverEvent::OpenChange(next, _) = event {
                open.set(*next);
            }
        })
        .on_open(move || panel.focus())
        // The panel fills the space below the header. Keep the header controls reachable.
        .close_on_press_outside(false)
        .placement(Placement::Bottom)
        .align(Align::Start)
        .margin(px(0.0))
        .offset(px(7.0))
        .popup_class(move || (!reduce_motion()).then(|| SharedString::from(class)))
        .popup_ending_class(move || (!reduce_motion()).then(|| SharedString::from("opacity-0")))
        .popup_style(|style| {
            style
                .w(|| window().width)
                .bg(|| theme().background.opacity(0.94))
                .backdrop_blur(px(12.0))
        })
        .trigger(
            Button::new(label)
                .variant(ButtonVariant::Ghost)
                .icon_start(
                    text(move || SharedString::from(if open.get() { "×" } else { "☰" }))
                        .text_lg()
                        .accessibility_elements_hidden(true),
                )
                .surface()
                .h(px(44.0))
                .test_id("docs-menu-trigger"),
        )
        .child(dynamic(move || {
            scroll()
                .h(|| px((window().height.0 - DOCS_HEADER_HEIGHT).max(0.0)))
                .scrollbar(false)
                .role(AccessibilityRole::Dialog)
                .label(label)
                .focusable(true)
                .element_ref(&panel)
                .test_id("docs-menu-panel")
                .child(content(open).p_4().gap_4())
        }))
        .into_element()
}

fn mobile_navigation(
    page: DocsPage,
    navigate: impl Fn(DocsPage) + Copy + 'static,
    local_sections: impl Fn(Signal<bool>) -> Option<View> + 'static,
) -> AnyElement {
    docs_menu("Menu", move |open| {
        view()
            .child(
                view()
                    .role(AccessibilityRole::Navigation)
                    .label("Sections")
                    .child(muted("Sections").px_3())
                    .child(
                        button("Blocks", Variant::Ghost, move || {
                            open.set(false);
                            navigate(DocsPage::Blocks);
                        })
                        .items_start()
                        .h(px(44.0)),
                    ),
            )
            .child(local_sections(open))
            .child(component_links(
                move || Some(page),
                move |target| {
                    open.set(false);
                    navigate(target);
                },
                44.0,
                None,
            ))
    })
}

fn component_links(
    page: impl Fn() -> Option<DocsPage> + Copy + 'static,
    navigate: impl Fn(DocsPage) + Copy + 'static,
    row_height: f32,
    sidebar_ready: Option<Signal<bool>>,
) -> View {
    let mut links = view()
        .role(AccessibilityRole::Navigation)
        .label("Components")
        .child(muted("Components").px_3().pb_1());
    // Drawer uses a separate page layout and keeps its catalogue position.
    let ordered = COMPONENTS.iter().map(|route| route.0).flat_map(|target| {
        std::iter::once(target).chain((target == DocsPage::Direction).then_some(DocsPage::Drawer))
    });
    for target in ordered {
        let current = move || page() == Some(target);
        let mut link = button(target.label(), Variant::Ghost, move || navigate(target))
            .items_start()
            .h(px(row_height))
            .style_when(current, |style| style.bg(|| theme().secondary))
            .accessibility_state(move || AccessibilityState {
                selected: current(),
                ..Default::default()
            });
        if let Some(ready) = sidebar_ready {
            let row = element_ref();
            effect(move || {
                if ready.get() && current() {
                    row.reveal(
                        RevealOptions::default()
                            .vertical(ScrollAlignment::Nearest)
                            .animated(false),
                    );
                }
            });
            link = link.element_ref(&row);
        }
        links = links.child(link);
    }
    links
}

#[derive(Clone, Copy)]
struct DrawerSections {
    scroll_ref: ElementRef,
    locations: Stored<[f32; SECTIONS.len()]>,
    selected: Signal<Option<usize>>,
}

impl DrawerSections {
    fn go(self, index: usize) {
        self.selected.set(Some(index));
        let y = self.locations.with(|items| items[index]);
        self.scroll_ref.scroll_to(
            Point {
                x: px(0.0),
                y: px((y - 18.0).max(0.0)),
            },
            false,
        );
    }
}

fn drawer_page(sections: DrawerSections, non_modal: Drawer) -> View {
    let DrawerSections {
        scroll_ref,
        locations,
        selected,
    } = sections;
    locations.replace([0.0; SECTIONS.len()]);
    selected.set(None);
    non_modal.close();
    let status = signal(None::<SharedString>);
    let go = move |index| sections.go(index);

    let delivery = drawer::delivery(status);
    let (custom_size, custom_size_panel) = drawer::custom_size();
    let (floating, floating_panel) = drawer::floating();
    let (positions, position_panels) = drawer::positions();
    let (swipe, swipe_panel) = drawer::swipe();
    let (nested, nested_panel) = drawer::nested();
    let non_modal = drawer::non_modal_trigger(non_modal);
    let (snap, snap_panel) = drawer::snap();
    let (responsive, responsive_panel) = drawer::responsive(status);

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
        .child(example_with_rsx("Delivery time", delivery, DemoSource::text(DRAWER_SOURCE), Some(tsx_examples::drawer::EXAMPLE), Some(drawer_rsx::CODE)))
        .child(
            section("Installation", 0, locations)
                .child(muted("Build the Native Shell Rust project for the Web target."))
                .child(code_panel("nsh package --target web --out dist", "bash", false)),
        )
        .child(
            section("Usage", 1, locations)
                .child(muted("Pass the Drawer state to DrawerRoot once. In Rust, compose the parts with .child(...). Pass the drawer_content body as a closure so it builds when the drawer opens."))
                .child(runnable_code_panel(drawer_rust::CODE))
                .child(muted("The TSX example imports Drawer parts from this app’s Native Shell module."))

        )
        .child(section("Composition", 2, locations)
            .child(muted("The trigger can sit anywhere in the layout. DrawerContent supplies the window layer and backdrop, and mounts its children when the drawer opens."))
            .child(code_panel("DrawerRoot\n├── DrawerTrigger\n└── DrawerContent\n    ├── DrawerPortal + DrawerOverlay\n    ├── DrawerHeader\n    │   ├── DrawerTitle\n    │   └── DrawerDescription\n    └── DrawerFooter\n        └── DrawerClose", "", false)))
        .child(section("Custom Sizes", 3, locations)
            .child(muted("Use height for vertical drawers and width for side drawers. Put long content in a scroll view."))
            .child(example("Custom Sizes", custom_size, DRAWER_SOURCE, TSX_SIZED))
            .child(custom_size_panel))
        .child(section("Styling", 4, locations)
            .child(muted("Edge drawers sit 8 points from the viewport by default. Set inset to override the gap. The header and footer add 16 points of padding at the panel edges. Use p_4() on the body for matching spacing or set its own padding for edge-to-edge content."))
            .child(example("Styling", floating, DRAWER_SOURCE, TSX_FLOATING))
            .child(floating_panel))
        .child(section("Examples", 5, locations))
        .child(section("Position", 6, locations)
            .child(muted("Open the drawer from any of the four screen edges."))
            .child(example("Position", positions, DRAWER_SOURCE, TSX_POSITION))
            .child(position_panels))
        .child(section("Swipe Handle", 7, locations)
            .child(muted("Drag the handle toward the edge to dismiss the drawer."))
            .child(example("Swipe Handle", swipe, DRAWER_SOURCE, TSX_HANDLE))
            .child(swipe_panel))
        .child(section("Nested", 8, locations)
            .child(muted("Open up to four drawers. Close the topmost drawer to return to the previous one."))
            .child(example("Nested", nested, DRAWER_SOURCE, TSX_NESTED))
            .child(nested_panel))
        .child(section("Non Modal", 9, locations)
            .child(muted("Scroll and interact with this page while the drawer stays open."))
            .child(example("Non Modal", non_modal, DRAWER_SOURCE, TSX_NON_MODAL)))
        .child(section("Snap Points", 10, locations)
            .child(muted("Drag the drawer between a compact peek and its full height."))
            .child(example("Snap Points", snap, DRAWER_SOURCE, TSX_SNAP))
            .child(snap_panel))
        .child(section("Responsive", 11, locations)
            .child(muted("Edit the profile in a desktop dialog or a mobile drawer."))
            .child(example("Responsive", responsive, DRAWER_SOURCE, TSX_RESPONSIVE))
            .child(responsive_panel))
        .child(section("API Reference", 12, locations)
            .child(muted("Use Drawer::new with an open signal. Drawer::nested links a child to a parent that remains mounted."))
            .child(api_row("DrawerTrigger / DrawerClose", "Use drawer_trigger().child(...) and drawer_close().child(...) in Rust. Use render to attach drawer behavior to your own control."))
            .child(api_row("DrawerContent", "Use drawer_content().child(|| ...) to build the panel when it opens. Set placement, size, and behavior with options."))
            .child(api_row("DrawerHeader / DrawerTitle / DrawerDescription / DrawerFooter", "Compose the heading with drawer_header().child(...) and group actions with drawer_footer().child(...)."))
            .child(api_row("DrawerPortal / DrawerOverlay / DrawerSwipeHandle", "Customize the window layer or backdrop, or style the generated drag handle."))
            .child(api_row("direction", "Edge of the viewport. Defaults to Down."))
            .child(api_row("modality", "Modal traps focus. NonModal leaves the page interactive."))
            .child(api_row("disable_outside_dismissal", "Keeps the drawer open on outside presses."))
            .child(api_row("height / width / inset", "Panel dimensions and distance from the viewport edges, in points."))
            .child(api_row("snap_points / snap_index", "Vertical detents and the active detent signal."))
            .child(api_row("centered", "Displays a centered dialog for the responsive example.")));

    view()
        .flex_1()
        .min_h(px(0.0))
        .child(
            view()
                .flex_1()
                .min_h(px(0.0))
                .flex_row()
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
                    || window().width.0 >= WIDE_DOCS_WIDTH,
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
                            .child(section_links(go, selected, 30.0))
                    },
                )),
        )
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

fn drawer_header(
    navigate: impl Fn(DocsPage) + Copy + 'static,
    toggle_theme: impl Fn() + 'static,
    sections: DrawerSections,
) -> View {
    let selected = sections.selected;
    let go = move |index| sections.go(index);
    docs_header(
        move || {
            mobile_navigation(DocsPage::Drawer, navigate, move |open| {
                Some(
                    Collapsible::new(
                        CollapsibleTrigger::render(|_| {
                            button_surface("On this page", Variant::Ghost)
                                .items_start()
                                .h(px(44.0))
                        }),
                        CollapsibleContent::new(section_links(
                            move |index| {
                                open.set(false);
                                go(index);
                            },
                            selected,
                            44.0,
                        )),
                    )
                    .surface(),
                )
            })
        },
        move || {
            (window().width.0 < WIDE_DOCS_WIDTH).then(|| {
                docs_menu("On this page", move |open| {
                    section_links(
                        move |index| {
                            open.set(false);
                            go(index);
                        },
                        selected,
                        44.0,
                    )
                })
            })
        },
        move || go(5),
        move || navigate(DocsPage::Blocks),
        toggle_theme,
    )
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

fn section_links(
    go: impl Fn(usize) + Copy + 'static,
    selected: Signal<Option<usize>>,
    row_height: f32,
) -> View {
    let mut links = view()
        .role(AccessibilityRole::Navigation)
        .label("Drawer sections");
    for (index, name) in SECTIONS.into_iter().enumerate() {
        links = links.child(
            view()
                .role(AccessibilityRole::Link)
                .label(name)
                .cursor_pointer()
                .h(px(row_height))
                .justify_center()
                .px_3()
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
    page: Signal<Option<DocsPage>>,
    navigate: impl Fn(DocsPage) + Copy + 'static,
    offset: Stored<Point<Points>>,
) -> impl IntoElement {
    let ready = signal(false);
    scroll()
        .w(px(216.0))
        .flex_shrink_0()
        .min_h(px(0.0))
        .border_r(px(1.0))
        .border_color(|| theme().border)
        .test_id("docs-sidebar")
        .content_offset(offset.get())
        .on_layout(move |_: &Bounds<Points>| ready.set(true))
        .on_scroll(move |event: &ScrollEvent| {
            offset.replace(event.offset);
        })
        .child(
            component_links(move || page.get(), navigate, 34.0, Some(ready))
                .px_3()
                .py_5(),
        )
}

fn example(
    label: &'static str,
    contents: impl IntoElement,
    rust: &'static str,
    _upstream: &'static str,
) -> View {
    let source = crate::native_examples::source(label);
    example_with_rsx(
        label,
        contents,
        DemoSource::text(rust),
        source.as_ref().map(|source| source.tsx),
        source.map(|source| source.rsx),
    )
}

fn example_with_rsx(
    label: &'static str,
    contents: impl IntoElement,
    rust: DemoSource,
    tsx: Option<TsxExample>,
    rsx: Option<DemoSource>,
) -> View {
    let preview = stored(Some(contents.into_element()));
    let mut list = TabsList::new(format!("{label} views"))
        .trigger(TabsTrigger::new("preview", "Preview"))
        .trigger(TabsTrigger::new("rust", "Rust"));
    if rsx.is_some() {
        list = list.trigger(TabsTrigger::new("rsx", "RSX"));
    }
    if tsx.is_some() {
        list = list.trigger(TabsTrigger::new("tsx", "TSX"));
    }
    let mut tabs = Tabs::new(list)
        .default_value("preview")
        .content(
            TabsContent::new("preview", move || {
                view()
                    .min_h(px(248.0))
                    .p_4()
                    .items_center()
                    .justify_center()
                    .child(preview.replace(None))
            })
            .keep_mounted(),
        )
        .content(TabsContent::new("rust", move || runnable_code_panel(rust)));
    if let Some(tsx) = tsx {
        tabs = tabs.content(TabsContent::new("tsx", move || tsx_code_panel(tsx)));
    }
    if let Some(rsx) = rsx {
        tabs = tabs.content(TabsContent::new("rsx", move || runnable_code_panel(rsx)));
    }
    view()
        .role(AccessibilityRole::Region)
        .label(label)
        .border(px(1.0))
        .border_color(move || theme().border)
        .rounded(px(12.0))
        .overflow_hidden()
        .child(tabs.surface().p_2())
}

fn tsx_code_panel(example: TsxExample) -> View {
    let panel = view()
        .gap_2()
        .child(view().px_3().py_2().child(muted(example.byline)))
        .child(code_panel(example.source, "tsx", true));
    let Some(runner) = use_context::<tsx_examples::TsxRunner>() else {
        return panel;
    };
    let selected = runner.selected;
    let running = move || selected.get() == Some(example.id);
    panel
        .child(
            view().items_start().child(
                Button::new(move || {
                    if running() {
                        "Hide example"
                    } else {
                        "Run example"
                    }
                })
                .variant(ButtonVariant::Outline)
                .on_press(move || {
                    let next = (selected.get_untracked() != Some(example.id)).then_some(example.id);
                    selected.set(next);
                    runner
                        .scope
                        .emit(crate::WebGalleryEvent::RunTsx(next.unwrap_or("").into()));
                }),
            ),
        )
        .child(when(running, move || {
            view()
                .min_h(px(80.0))
                .p_4()
                .items_center()
                .child(runner.slot.place(|slot| slot))
        }))
}

fn runnable_code_panel(source: DemoSource) -> View {
    let mut panel = view().gap_2().child(code_panel(source.text, "rust", true));
    if let Some(render) = source.render {
        let running = signal(false);
        panel = panel
            .child(
                view().items_start().child(
                    Button::new(move || {
                        if running.get() {
                            "Hide example"
                        } else {
                            "Run example"
                        }
                    })
                    .variant(ButtonVariant::Outline)
                    .on_press(move || running.update(|running| *running = !*running)),
                ),
            )
            .child(when(running, move || {
                view().p_4().items_center().child(render())
            }));
    }
    panel
}

fn code_block(value: &'static str) -> View {
    code_panel(value, "rust", false)
}

fn code_panel(value: &'static str, language: &'static str, scrollable: bool) -> View {
    let mut content = CodeBlockContent::new(value)
        .language(language)
        .wrap(!scrollable)
        .focusable(scrollable);
    if scrollable {
        content = content.max_height(340.0);
    }
    CodeBlock::new(content)
        .header(CodeBlockHeader::actions_only(
            CodeBlockActions::new().child(CodeBlockCopyButton::new(value)),
        ))
        .surface()
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

#[cfg(test)]
mod language_coverage {
    use super::*;
    use nsh_core::element::Root;
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    #[test]
    fn every_component_pages_first_example_offers_rust_rsx_and_native_tsx() {
        for route in std::iter::once("#drawer").chain(COMPONENTS.iter().map(|route| route.2)) {
            let mut config = EngineConfig::new(Root::of::<crate::App>());
            config.viewport = Size::new(px(390.0), px(844.0));
            let mut app =
                Headless::start_at_page(config, &format!("https://foundation.test/{route}"))
                    .unwrap();
            app.first_render().unwrap();
            let tree = app.screen().accessibility(None).unwrap().unwrap();
            let mut nodes = vec![&tree];
            let tabs = loop {
                let node = nodes
                    .pop()
                    .unwrap_or_else(|| panic!("{route} has no example tabs"));
                if node.role == Some(AccessibilityRole::Tablist) {
                    break node;
                }
                nodes.extend(node.children.iter().rev());
            };
            let first_tabs: Vec<_> = tabs.children.iter().map(|tab| tab.name.as_str()).collect();
            assert!(
                first_tabs == ["Preview", "Rust", "RSX", "TSX"],
                "{route} first example tabs: {first_tabs:?}"
            );
            assert_eq!(app.screen().count("role:tab[shadcn/ui]"), Ok(0), "{route}");
            let label = tabs.name.strip_suffix(" views").unwrap();
            let region = format!("role:region[{label}]");
            app.press(&format!("{region} >> RSX")).unwrap();
            app.press(&format!("{region} >> Copy")).unwrap();
            let code = app.screen().clipboard().unwrap();
            assert!(code.contains("rsx!"), "{route} has no RSX source");
            assert!(
                !code.contains("docs::") && !code.contains("Example />"),
                "{route} hides the component implementation: {code}"
            );
        }
    }
}
