mod platform_impl;
mod public;

#[test]
fn hid_test() {
    use platform_impl::HID;

    let hid = HID::new();
    let devices = hid.get_devices();
    for device in devices {
        println!("product_name: {}", device.get_product_name());
        println!("vendor_id: {}", device.get_vendor_id());
        println!("product_id: {}", device.get_product_id());
        println!("{:x?}", device.get_report_descriptor());
    }
}
