//! Native Shell documentation and component examples.

extern crate self as nsh_foundation;

pub mod accordion;
mod accordion_examples;
pub mod alert;
pub mod alert_dialog;
mod alert_dialog_examples;
mod alert_examples;
pub mod aspect_ratio;
mod aspect_ratio_examples;
pub mod attachment;
mod attachment_examples;
pub mod avatar;
mod avatar_examples;
pub mod badge;
mod badge_examples;
pub mod breadcrumb;
mod breadcrumb_examples;
pub mod bubble;
mod bubble_examples;
pub mod button;
mod button_examples;
pub mod button_group;
mod button_group_examples;
pub mod calendar;
mod calendar_examples;
pub mod card;
mod card_examples;
pub mod carousel;
mod carousel_examples;
pub mod chart;
mod chart_examples;
pub mod checkbox;
mod checkbox_examples;
pub mod code_block;
mod code_block_examples;
pub mod collapsible;
mod collapsible_examples;
pub mod combobox;
mod combobox_examples;
pub mod command;
mod command_examples;
pub mod context_menu;
mod context_menu_examples;
pub mod data_table;
mod data_table_examples;
pub mod date_picker;
mod date_picker_examples;
pub mod dialog;
mod dialog_examples;
pub mod direction;
mod docs;
pub mod drawer;
pub mod dropdown_menu;
mod dropdown_menu_examples;
pub mod empty;
mod empty_examples;
mod examples;
pub mod field;
mod field_examples;
pub mod hover_card;
mod hover_card_examples;
pub mod input;
mod input_examples;
pub mod input_group;
mod input_group_examples;
pub mod input_otp;
mod input_otp_examples;
pub mod item;
mod item_examples;
pub mod kbd;
mod kbd_examples;
pub mod label;
mod label_examples;
pub mod marker;
mod marker_examples;
mod menu_parts;
pub mod menubar;
mod menubar_examples;
pub mod message;
mod message_examples;
pub mod message_scroller;
mod message_scroller_examples;
pub mod native_select;
mod native_select_examples;
pub mod navigation_menu;
mod navigation_menu_examples;
pub mod pagination;
mod pagination_examples;
pub mod popover;
mod popover_examples;
pub mod progress;
mod progress_examples;
pub mod questionnaire;
mod questionnaire_examples;
pub mod radio_group;
mod radio_group_examples;
pub mod resizable;
mod resizable_examples;
pub mod scroll_area;
mod scroll_area_examples;
pub mod select;
mod select_examples;
pub mod separator;
mod separator_examples;
pub mod sheet;
mod sheet_examples;
pub mod sidebar;
mod sidebar_examples;
pub mod skeleton;
mod skeleton_examples;
pub mod slider;
mod slider_examples;
pub mod spinner;
mod spinner_examples;
pub mod switch;
mod switch_examples;
pub mod table;
mod table_examples;
pub mod tabs;
mod tabs_examples;
mod ui;

use nsh_core::prelude::*;

#[native_module("nsh-foundation/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
    registry.assets(assets!());
}

#[component]
pub fn App() -> impl IntoElement {
    docs::page()
}
