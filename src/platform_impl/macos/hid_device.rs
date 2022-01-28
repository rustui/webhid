use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFGetTypeID, CFIndexConvertible, FromVoid},
    data::{CFData, CFDataGetTypeID},
    number::{CFNumber, CFNumberGetTypeID},
    runloop::{kCFRunLoopDefaultMode, CFRunLoopGetMain},
    string::{
        kCFStringEncodingUTF8, CFString, CFStringCreateWithBytesNoCopy, CFStringGetTypeID,
        CFStringRef,
    },
};

use super::{
    io_hid_device::{
        IOHIDDeviceGetProperty, IOHIDDeviceOpen, IOHIDDeviceRef, IOHIDDeviceScheduleWithRunLoop,
    },
    io_hid_device_keys::{
        kIOHIDProductIDKey, kIOHIDProductKey, kIOHIDReportDescriptorKey, kIOHIDVendorIDKey,
    },
    io_kit::{kIOHIDOptionsTypeNone, kIOReturnSuccess},
};

#[derive(Debug)]
pub struct HIDDevice {
    device_ref: IOHIDDeviceRef,
    opened: bool,
}

impl HIDDevice {
    pub fn new(device_ref: IOHIDDeviceRef) -> Self {
        Self {
            device_ref,
            opened: false,
        }
    }

    pub fn is_opened(&self) -> bool {
        self.opened
    }

    pub fn get_vendor_id(&self) -> i32 {
        unsafe { self.get_property_i32(kIOHIDVendorIDKey) }
    }

    pub fn get_product_id(&self) -> i32 {
        unsafe { self.get_property_i32(kIOHIDProductIDKey) }
    }

    pub fn get_product_name(&self) -> String {
        unsafe { self.get_property_string(kIOHIDProductKey) }
    }

    pub fn get_report_descriptor(&self) -> Vec<u8> {
        unsafe { self.get_property_data(kIOHIDReportDescriptorKey) }
    }

    pub fn open(&mut self) {
        let io_return = unsafe { IOHIDDeviceOpen(self.device_ref, kIOHIDOptionsTypeNone) };
        if io_return == kIOReturnSuccess {
            self.opened = true;
            unsafe {
                IOHIDDeviceScheduleWithRunLoop(
                    self.device_ref,
                    CFRunLoopGetMain(),
                    kCFRunLoopDefaultMode,
                )
            };
        }
    }

    unsafe fn create_cf_string(bytes: &[u8]) -> CFStringRef {
        CFStringCreateWithBytesNoCopy(
            kCFAllocatorDefault,
            bytes.as_ptr(),
            bytes.len().to_CFIndex(),
            kCFStringEncodingUTF8,
            false as Boolean,
            kCFAllocatorDefault,
        )
    }

    unsafe fn get_property_i32(&self, bytes: &[u8]) -> i32 {
        let cf_string_ref = Self::create_cf_string(bytes);
        let cf_type_ref = IOHIDDeviceGetProperty(self.device_ref, cf_string_ref);
        if cf_type_ref.is_null() {
            0
        } else {
            if CFGetTypeID(cf_type_ref) != CFNumberGetTypeID() {
                panic!("Invalid bytes");
            }
            CFNumber::from_void(cf_type_ref)
                .to_i32()
                .unwrap_or_default()
        }
    }

    unsafe fn get_property_string(&self, bytes: &[u8]) -> String {
        let cf_string_ref = Self::create_cf_string(bytes);
        let cf_type_ref = IOHIDDeviceGetProperty(self.device_ref, cf_string_ref);
        if cf_type_ref.is_null() {
            String::new()
        } else {
            if CFGetTypeID(cf_type_ref) != CFStringGetTypeID() {
                panic!("Invalid bytes");
            }
            CFString::from_void(cf_type_ref).to_string()
        }
    }

    unsafe fn get_property_data(&self, bytes: &[u8]) -> Vec<u8> {
        let cf_string_ref = Self::create_cf_string(bytes);
        let cf_type_ref = IOHIDDeviceGetProperty(self.device_ref, cf_string_ref);
        if cf_type_ref.is_null() {
            Vec::new()
        } else {
            if CFGetTypeID(cf_type_ref) != CFDataGetTypeID() {
                panic!("Invalid bytes");
            }
            CFData::from_void(cf_type_ref).to_vec()
        }
    }
}
