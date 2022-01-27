mod platform_impl;

#[test]
fn hid_test() {
    use platform_impl::HID;

    let hid = HID::new();
    let devices = hid.get_devices();
    for device in devices {
        println!(
            "product_id: 0x{:04x}",
            device.product_id.unwrap_or_default()
        );
    }
}
