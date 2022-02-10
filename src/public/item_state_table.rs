use std::{isize, mem};

use super::Tag;
#[derive(Default)]
pub struct GlobalItemState {
    pub usage_page: u32,
    pub logical_minimum: i32,
    pub logical_maximum: i32,
    pub physical_minimum: i32,
    pub physical_maximum: i32,
    pub unit_exponent: u32,
    pub unit: u32,
    pub report_size: u32,
    pub report_count: u32,
}

#[derive(Default)]
pub struct LocalItemState {
    pub usages: Vec<u32>,
    pub usage_minimum: u32,
    pub usage_maximum: u32,
    pub designator_index: u32,
    pub designator_minimum: u32,
    pub designator_maximum: u32,
    pub string_index: u32,
    pub string_minimum: u32,
    pub string_maximum: u32,
    pub delimiter: u32,
}

impl LocalItemState {
    pub fn reset(&mut self) {
        self.usages.clear();
        self.usage_minimum = 0;
        self.usage_maximum = 0;
        self.designator_index = 0;
        self.designator_minimum = 0;
        self.designator_maximum = 0;
        self.string_index = 0;
        self.string_minimum = 0;
        self.string_maximum = 0;
        self.delimiter = 0;
    }
}

#[derive(Default)]
pub struct ItemStateTable {
    pub report_id: u32,
    pub global_stack: Vec<GlobalItemState>,
    pub local: LocalItemState,
}

impl ItemStateTable {
    pub fn set_item_value(&mut self, tag: Tag, value: u32, payload_size: isize) {
        if is_global_item(tag) {
            if self.global_stack.is_empty() {
                self.global_stack.push(GlobalItemState::default());
            }
            if let Some(global) = self.global_stack.last_mut() {
                match tag {
                    Tag::UsagePage => global.usage_page = value,
                    Tag::LogicalMinimum => {
                        global.logical_minimum = i32_from_value_and_size(value, payload_size)
                    }
                    Tag::LogicalMaximum => {
                        global.logical_maximum = i32_from_value_and_size(value, payload_size)
                    }
                    Tag::PhysicalMinimum => {
                        global.physical_minimum = i32_from_value_and_size(value, payload_size)
                    }
                    Tag::PhysicalMaximum => {
                        global.physical_maximum = i32_from_value_and_size(value, payload_size)
                    }
                    Tag::UnitExponent => global.unit_exponent = value,
                    Tag::Unit => global.unit = value,
                    Tag::ReportSize => global.report_size = value,
                    Tag::ReportCount => global.report_count = value,
                    _ => (),
                }
            }
        } else {
            match tag {
                Tag::Usage => self.local.usages.push(maybe_combine_usage_and_usage_page(
                    value,
                    &self.global_stack,
                )),
                Tag::UsageMinimum => {
                    self.local.usage_minimum =
                        maybe_combine_usage_and_usage_page(value, &self.global_stack)
                }
                Tag::UsageMaximum => {
                    self.local.usage_maximum =
                        maybe_combine_usage_and_usage_page(value, &self.global_stack)
                }
                Tag::DesignatorIndex => self.local.designator_index = value,
                Tag::DesignatorMinimum => self.local.designator_minimum = value,
                Tag::DesignatorMaximum => self.local.designator_maximum = value,
                Tag::StringIndex => self.local.string_index = value,
                Tag::StringMinimum => self.local.string_minimum = value,
                Tag::StringMaximum => self.local.string_maximum = value,
                Tag::Delimiter => self.local.delimiter = value,
                _ => (),
            }
        }
    }
}

fn is_global_item(tag: Tag) -> bool {
    match tag {
        Tag::UsagePage
        | Tag::LogicalMinimum
        | Tag::LogicalMaximum
        | Tag::PhysicalMinimum
        | Tag::PhysicalMaximum
        | Tag::UnitExponent
        | Tag::Unit
        | Tag::ReportSize
        | Tag::ReportId
        | Tag::ReportCount => true,
        _ => false,
    }
}

fn maybe_combine_usage_and_usage_page(usage: u32, global_stack: &Vec<GlobalItemState>) -> u32 {
    if let Some(last) = global_stack.last() {
        if usage > u16::MAX as u32 {
            usage
        } else {
            (last.usage_page << (mem::size_of::<u16>() * 8)) | usage
        }
    } else {
        usage
    }
}

fn i32_from_value_and_size(value: u32, payload_size: isize) -> i32 {
    match payload_size {
        0 => 0,
        1 => (value & 0xFF) as i8 as i32,
        2 => (value & 0xFFFF) as i16 as i32,
        _ => value as i32,
    }
}
