//! Reusable Rust and Native Shell exports for the gallery's runnable examples.

use nsh_core::prelude::*;

use crate::examples::*;

pub(crate) struct ExampleSource {
    pub rsx: &'static str,
    pub tsx: &'static str,
}

fn toast_preview() -> impl IntoElement {
    let toaster = components::toast::Toaster::new();
    view()
        .child(toast_examples::demo(toaster))
        .child(toaster.viewport())
}

macro_rules! example_rsx {
    ($component:ident, $factory:ident) => {
        concat!(
            "use components::rsx;\n",
            "use docs::native_examples::",
            stringify!($factory),
            ";\n",
            "use nsh_core::prelude::IntoElement;\n\n",
            "pub fn example() -> impl IntoElement {\n",
            "    rsx! { <",
            stringify!($component),
            " /> }\n",
            "}\n",
        )
    };
}

macro_rules! gallery_examples {
    ($($page:ident $component:ident $factory:ident $label:literal => $render:path;)*) => {
        $(
            #[doc = concat!("Builds the gallery's ", stringify!($page), " example.")]
            #[doc = concat!("\n```no_run\n", example_rsx!($component, $factory), "```")]
            pub fn $factory() -> impl IntoElement {
                $render()
            }
        )*

        mod bindings {
            use super::*;

            $(
                #[doc = concat!("Renders the gallery's ", stringify!($page), " example.")]
                #[component]
                pub fn $component() -> impl IntoElement {
                    super::$factory()
                }
            )*
        }

        pub(crate) fn register(registry: &mut Registry) {
            $(registry.register::<bindings::$component>();)*
        }

        pub(crate) fn source(label: &str) -> Option<ExampleSource> {
            match label {
                $(
                    $label => Some(ExampleSource {
                        rsx: example_rsx!($component, $factory),
                        tsx: concat!(
                            "import { ", stringify!($component), " } from \"nsh-foundation/native\";\n\n",
                            "export function Example() {\n",
                            "  return <", stringify!($component), " />;\n",
                            "}\n",
                        ),
                    }),
                )*
                _ => None,
            }
        }

    };
}

gallery_examples! {
    Accordion AccordionExample accordion_example "Accordion demo" => accordion_examples::demo::preview;
    Alert AlertExample alert_example "Alert demo" => alert_examples::demo::preview;
    AlertDialog AlertDialogExample alert_dialog_example "Alert Dialog demo" => alert_dialog_examples::demo::preview;
    AspectRatio AspectRatioExample aspect_ratio_example "Aspect Ratio demo" => aspect_ratio_examples::demo::preview;
    Attachment AttachmentExample attachment_example "Attachment demo" => attachment_examples::demo::preview;
    Avatar AvatarExample avatar_example "Avatar demo" => avatar_examples::demo::preview;
    Breadcrumb BreadcrumbExample breadcrumb_example "Breadcrumb demo" => breadcrumb_examples::demo::preview;
    Bubble BubbleExample bubble_example "Bubble demo" => bubble_examples::demo::preview;
    ButtonGroup ButtonGroupExample button_group_example "Button Group demo" => button_examples::group_demo::preview;
    Calendar CalendarExample calendar_example "Calendar demo" => calendar_examples::demo::preview;
    Card CardExample card_example "Card composition" => composition::card_rsx::example;
    Carousel CarouselExample carousel_example "Carousel composition" => composition::carousel_rsx::example;
    Chart ChartExample chart_example "Interactive Bar Chart" => chart_examples::demo;
    Checkbox CheckboxExample checkbox_example "Checkbox demo" => checkbox_examples::demo;
    CodeBlock CodeBlockExample code_block_example "Code Block demo" => code_block_examples::demo;
    Collapsible CollapsibleExample collapsible_example "Collapsible demo" => collapsible_examples::demo;
    Combobox ComboboxExample combobox_example "Combobox demo" => combobox_examples::demo;
    Command CommandExample command_example "Command demo" => command_examples::demo;
    ContextMenu ContextMenuExample context_menu_example "Context Menu demo" => context_menu_examples::demo;
    DataTable DataTableExample data_table_example "Data Table demo" => data_table_examples::demo;
    DatePicker DatePickerExample date_picker_example "Date Picker demo" => date_picker_examples::demo;
    Dialog DialogExample dialog_example "Dialog demo" => dialog_examples::demo;
    Direction DirectionExample direction_example "Direction demo" => card_examples::login::direction;
    DropdownMenu DropdownMenuExample dropdown_menu_example "Dropdown Menu demo" => dropdown_menu_examples::demo;
    Empty EmptyExample empty_example "Empty demo" => empty_examples::demo;
    Field FieldExample field_example "Field demo" => field_examples::demo;
    HoverCard HoverCardExample hover_card_example "Hover Card demo" => hover_card_examples::demo;
    Input InputExample input_example "Input demo" => input_examples::demo;
    InputGroup InputGroupExample input_group_example "Input Group demo" => input_group_examples::demo;
    InputOtp InputOtpExample input_otp_example "Input OTP demo" => input_otp_examples::demo;
    Item ItemExample item_example "Item demo" => item_examples::demo;
    Kbd KbdExample kbd_example "Kbd demo" => kbd_examples::demo;
    Label LabelExample label_example "Label demo" => label_examples::demo;
    Marker MarkerExample marker_example "Marker demo" => marker_examples::demo;
    Menubar MenubarExample menubar_example "Menubar demo" => menubar_examples::demo;
    Message MessageExample message_example "Message demo" => message_examples::demo;
    MessageScroller MessageScrollerExample message_scroller_example "Message Scroller demo" => message_scroller_examples::demo;
    NativeSelect NativeSelectExample native_select_example "Native Select demo" => native_select_examples::demo;
    NavigationMenu NavigationMenuExample navigation_menu_example "Navigation Menu demo" => navigation_menu_examples::demo;
    Pagination PaginationExample pagination_example "Pagination demo" => pagination_examples::demo;
    Popover PopoverExample popover_example "Popover demo" => popover_examples::demo;
    Progress ProgressExample progress_example "Progress demo" => progress_examples::demo;
    Questionnaire QuestionnaireExample questionnaire_example "Questionnaire demo" => questionnaire_examples::demo;
    RadioGroup RadioGroupExample radio_group_example "Radio Group demo" => radio_group_examples::demo;
    Resizable ResizableExample resizable_example "Resizable demo" => resizable_examples::demo;
    ScrollArea ScrollAreaExample scroll_area_example "Scroll Area demo" => scroll_area_examples::demo;
    Select SelectExample select_example "Select demo" => select_examples::demo;
    Separator SeparatorExample separator_example "Separator demo" => separator_examples::demo;
    Sheet SheetExample sheet_example "Sheet demo" => sheet_examples::demo;
    Sidebar SidebarExample sidebar_example "Sidebar demo" => sidebar_examples::demo;
    Skeleton SkeletonExample skeleton_example "Skeleton demo" => skeleton_examples::demo;
    Slider SliderExample slider_example "Slider demo" => slider_examples::demo;
    Spinner SpinnerExample spinner_example "Spinner demo" => spinner_examples::demo;
    Table TableExample table_example "Table demo" => table_examples::demo;
    Tabs TabsExample tabs_example "Tabs demo" => tabs_examples::demo;
    Textarea TextareaExample textarea_example "Textarea demo" => textarea_examples::demo;
    Toast ToastExample toast_example "Toast demo" => toast_preview;
    Toggle ToggleExample toggle_example "Toggle demo" => toggle_examples::demo;
    ToggleGroup ToggleGroupExample toggle_group_example "Toggle Group demo" => toggle_group_examples::demo;
    Tooltip TooltipExample tooltip_example "Tooltip demo" => tooltip_examples::demo;
    Trace TraceExample trace_example "Trace demo" => trace_examples::demo;
    Typography TypographyExample typography_example "Typography demo" => typography_examples::demo;
}
