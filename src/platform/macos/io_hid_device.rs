use core_foundation::{base::CFTypeRef, string::CFStringRef};

pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type UInt32 = ::std::os::raw::c_uint;
pub type Boolean = ::std::os::raw::c_uchar;
pub type CFTypeID = ::std::os::raw::c_ulong;
pub type CFIndex = ::std::os::raw::c_long;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOHIDDeviceRef(*const std::os::raw::c_void);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFAllocator {
    _unused: [u8; 0],
}
pub type CFAllocatorRef = *const __CFAllocator;
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
pub type CFTimeInterval = f64;
pub type mach_port_t = __darwin_mach_port_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFRunLoop {
    _unused: [u8; 0],
}
pub type CFRunLoopRef = *mut __CFRunLoop;
pub type dispatch_block_t = *mut ::std::os::raw::c_void;
pub type dispatch_queue_t = *mut dispatch_queue_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_queue_s {
    pub _address: u8,
}
pub type kern_return_t = ::std::os::raw::c_int;
pub type IOReturn = kern_return_t;
pub type IOOptionBits = UInt32;
pub type io_object_t = mach_port_t;
pub type io_service_t = io_object_t;
pub const IOHIDReportType_kIOHIDReportTypeInput: IOHIDReportType = 0;
pub const IOHIDReportType_kIOHIDReportTypeOutput: IOHIDReportType = 1;
pub const IOHIDReportType_kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const IOHIDReportType_kIOHIDReportTypeCount: IOHIDReportType = 3;
pub type IOHIDReportType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDElement {
    _unused: [u8; 0],
}
pub type IOHIDElementRef = *mut __IOHIDElement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDValue {
    _unused: [u8; 0],
}
pub type IOHIDValueRef = *mut __IOHIDValue;
pub type IOHIDCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
    ),
>;
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
pub type IOHIDValueMultipleCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        multiple: CFDictionaryRef,
    ),
>;
extern "C" {
    pub fn IOHIDDeviceGetTypeID() -> CFTypeID;
}
extern "C" {
    pub fn IOHIDDeviceCreate(allocator: CFAllocatorRef, service: io_service_t) -> IOHIDDeviceRef;
}
extern "C" {
    pub fn IOHIDDeviceGetService(device: IOHIDDeviceRef) -> io_service_t;
}
extern "C" {
    pub fn IOHIDDeviceOpen(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceConformsTo(device: IOHIDDeviceRef, usagePage: u32, usage: u32) -> Boolean;
}
extern "C" {
    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;
}
extern "C" {
    pub fn IOHIDDeviceSetProperty(
        device: IOHIDDeviceRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
extern "C" {
    pub fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: CFDictionaryRef,
        options: IOOptionBits,
    ) -> CFArrayRef;
}
extern "C" {
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
extern "C" {
    pub fn IOHIDDeviceUnscheduleFromRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
extern "C" {
    pub fn IOHIDDeviceSetDispatchQueue(device: IOHIDDeviceRef, queue: dispatch_queue_t);
}
extern "C" {
    pub fn IOHIDDeviceSetCancelHandler(device: IOHIDDeviceRef, handler: dispatch_block_t);
}
extern "C" {
    pub fn IOHIDDeviceActivate(device: IOHIDDeviceRef);
}
extern "C" {
    pub fn IOHIDDeviceCancel(device: IOHIDDeviceRef);
}
extern "C" {
    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDDeviceRegisterInputValueCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDDeviceRegisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDDeviceRegisterInputReportWithTimeStampCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn IOHIDDeviceSetInputValueMatching(device: IOHIDDeviceRef, matching: CFDictionaryRef);
}
extern "C" {
    pub fn IOHIDDeviceSetInputValueMatchingMultiple(device: IOHIDDeviceRef, multiple: CFArrayRef);
}
extern "C" {
    pub fn IOHIDDeviceSetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceSetValueMultiple(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceSetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceSetValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceGetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceGetValueWithOptions(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
        options: u32,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceCopyValueMultiple(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceGetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceCopyValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceSetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceSetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceGetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
    ) -> IOReturn;
}
extern "C" {
    pub fn IOHIDDeviceGetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
