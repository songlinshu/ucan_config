extern crate clap;
extern crate libusb;
use std::time::Duration;

fn main() {

    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        // if device_desc.product_id() == 0x775
        if device_desc.product_id() == 14155
        {
            let handle = context.open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id());
            println!("{:?}", handle.is_some());

            let mut dev_handle = handle.unwrap();
            let ret_claim = dev_handle.claim_interface(0);

            let data_to_send : &[u8] = b"hello";
            println!("{:?}\n{:?}", ret_claim, data_to_send);

            let timeout = Duration::new(2, 0);
            let ret_bulk = dev_handle.write_bulk(0x01, data_to_send, timeout);

            print!("{:?}\n", ret_bulk);
        }
    }
}