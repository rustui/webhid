use core_foundation::{
    base::{kCFAllocatorDefault, Boolean, CFGetTypeID, CFIndexConvertible, FromVoid},
    number::{CFNumber, CFNumberGetTypeID},
    string::{kCFStringEncodingUTF8, CFStringCreateWithBytesNoCopy},
};

use super::{
    io_hid_device::{IOHIDDeviceGetProperty, IOHIDDeviceRef},
    io_hid_device_keys::{kIOHIDProductIDKey, kIOHIDVendorIDKey},
};

pub struct HIDDevice {
    device_ref: IOHIDDeviceRef,
    pub vendor_id: Option<i32>,
    pub product_id: Option<i32>,
}

impl HIDDevice {
    pub fn new(device_ref: IOHIDDeviceRef) -> Self {
        let vendor_id = Self::get_property_i32(device_ref, kIOHIDVendorIDKey);
        let product_id = Self::get_property_i32(device_ref, kIOHIDProductIDKey);
        Self {
            device_ref,
            vendor_id,
            product_id,
        }
    }

    fn get_property_i32(device_ref: IOHIDDeviceRef, bytes: &[u8]) -> Option<i32> {
        let cf_string_ref = unsafe {
            CFStringCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                bytes.as_ptr(),
                bytes.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
                kCFAllocatorDefault,
            )
        };
        let cf_type_ref = unsafe { IOHIDDeviceGetProperty(device_ref, cf_string_ref) };
        if cf_type_ref.is_null() {
            None
        } else {
            if unsafe { CFGetTypeID(cf_type_ref) != CFNumberGetTypeID() } {
                panic!("Invalid bytes");
            }
            unsafe { CFNumber::from_void(cf_type_ref) }.to_i32()
        }
    }
}
