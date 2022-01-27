mod platform_impl;

#[test]
fn hid_test() {
    use platform_impl::HID;

    let hid = HID::new();
    let devices = hid.get_devices();
    for device in devices {
        println!(
            "vendor_id: {:04x}, product_id: 0x{:04x}",
            device.vendor_id.unwrap_or_default(),
            device.product_id.unwrap_or_default()
        );
    }
}
