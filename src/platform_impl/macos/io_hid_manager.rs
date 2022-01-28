#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core_foundation::{mach_port::CFAllocatorRef, set::CFSetRef, string::CFStringRef, runloop::CFRunLoopRef};

use super::io_hid_device::IOHIDDeviceRef;

pub type kern_return_t = ::std::os::raw::c_int;
pub type IOReturn = kern_return_t;
pub type UInt32 = ::std::os::raw::c_uint;
pub type Boolean = ::std::os::raw::c_uchar;
pub type IOOptionBits = UInt32;
pub type CFTypeID = ::std::os::raw::c_ulong;
pub type CFIndex = ::std::os::raw::c_long;
pub type CFTypeRef = *const ::std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFArray {
    _unused: [u8; 0],
}
pub type CFArrayRef = *const __CFArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFDictionary {
    _unused: [u8; 0],
}
pub type CFDictionaryRef = *const __CFDictionary;

pub type dispatch_block_t = *mut ::std::os::raw::c_void;
pub type dispatch_queue_t = *mut dispatch_queue_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_queue_s {
    pub _address: u8,
}
pub const IOHIDReportType_kIOHIDReportTypeInput: IOHIDReportType = 0;
pub const IOHIDReportType_kIOHIDReportTypeOutput: IOHIDReportType = 1;
pub const IOHIDReportType_kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const IOHIDReportType_kIOHIDReportTypeCount: IOHIDReportType = 3;
pub type IOHIDReportType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDValue {
    _unused: [u8; 0],
}
pub type IOHIDValueRef = *mut __IOHIDValue;
pub type IOHIDReportCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        type_: IOHIDReportType,
        reportID: u32,
        report: *mut u8,
        reportLength: CFIndex,
    ),
>;
pub type IOHIDReportWithTimeStampCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        type_: IOHIDReportType,
        reportID: u32,
        report: *mut u8,
        reportLength: CFIndex,
        timeStamp: u64,
    ),
>;
pub type IOHIDValueCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        value: IOHIDValueRef,
    ),
>;
pub type IOHIDDeviceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        device: IOHIDDeviceRef,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDManager {
    _unused: [u8; 0],
}
pub type IOHIDManagerRef = *mut __IOHIDManager;
extern "C" {
    pub fn IOHIDManagerGetTypeID() -> CFTypeID;
}
extern "C" {
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;
}
extern "C" {
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
}
extern "C" {
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
}
extern "C" {
    pub fn IOHIDManagerGetProperty(manager: IOHIDManagerRef, key: CFStringRef) -> CFTypeRef;
}
extern "C" {
    pub fn IOHIDManagerSetProperty(
        manager: IOHIDManagerRef,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> Boolean;
}
extern "C" {
    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
extern "C" {
    pub fn IOHIDManagerUnscheduleFromRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
extern "C" {
    pub fn IOHIDManagerSetDispatchQueue(manager: IOHIDManagerRef, queue: dispatch_queue_t);
}
extern "C" {
    pub fn IOHIDManagerSetCancelHandler(manager: IOHIDManagerRef, handler: dispatch_block_t);
}
extern "C" {
    pub fn IOHIDManagerActivate(manager: IOHIDManagerRef);
}
extern "C" {
    pub fn IOHIDManagerCancel(manager: IOHIDManagerRef);
}
extern "C" {
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
}
extern "C" {
    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: CFArrayRef);
}
extern "C" {
    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;
}
extern "C" {
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDManagerRegisterInputReportCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDManagerRegisterInputReportWithTimeStampCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
}
extern "C" {
    pub fn IOHIDManagerSetInputValueMatchingMultiple(
        manager: IOHIDManagerRef,
        multiple: CFArrayRef,
    );
}
extern "C" {
    pub fn IOHIDManagerSaveToPropertyDomain(
        manager: IOHIDManagerRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
        options: IOOptionBits,
    );
}
