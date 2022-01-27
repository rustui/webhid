use std::{os::raw::c_void, ptr};

use core_foundation::{
    base::kCFAllocatorDefault,
    set::{CFSetGetCount, CFSetGetValues},
};

use super::{
    hid_device::HIDDevice,
    io_hid_device::IOHIDDeviceRef,
    io_hid_manager::{
        IOHIDManagerCopyDevices, IOHIDManagerCreate, IOHIDManagerRef, IOHIDManagerSetDeviceMatching,
    },
};

pub struct HID {
    manager: IOHIDManagerRef,
}

impl HID {
    pub fn new() -> Self {
        let manager = unsafe { IOHIDManagerCreate(kCFAllocatorDefault, 0) };
        Self { manager }
    }
    pub fn get_devices(&self) -> impl Iterator<Item = HIDDevice> {
        unsafe { IOHIDManagerSetDeviceMatching(self.manager, ptr::null()) };
        let cf_set_ref = unsafe { IOHIDManagerCopyDevices(self.manager) };
        let devices_len = unsafe { CFSetGetCount(cf_set_ref) } as usize;
        let mut devices: Vec<IOHIDDeviceRef> = Vec::with_capacity(devices_len);
        unsafe { devices.set_len(devices_len) };
        let devices_ptr = devices.as_mut_ptr();
        unsafe { CFSetGetValues(cf_set_ref, devices_ptr as *mut *const c_void) };
        devices.into_iter().map(|device| HIDDevice::new(device))
    }
}

#[test]
fn hid_test() {
    let hid = HID::new();
    let devices = hid.get_devices();
    for device in devices {
        println!(
            "product_id: 0x{:04x}",
            device.product_id.unwrap_or_default()
        );
    }
}
