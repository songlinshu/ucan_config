extern crate libusb;
use ::core::mem;
use libusb::*;
use std::time::Duration;

const PRODUCT_ID: u16 = 0x775;

pub struct UsbCfdc<'a> {
    handle: &'a DeviceHandle<'a>,
}

impl<'a> UsbCfdc<'a> {
    pub fn new(handle: &'a DeviceHandle<'a>) -> UsbCfdc<'a> {
        UsbCfdc { handle }
    }

    pub fn send_cmd(&mut self, data: &[u8]) {
        print!("Bytes {:?} ", data.len());
        println!("Device claimed data to be send {:?} ", data);

        let data_to_send: &[u8] = data;
        let timeout = Duration::new(1, 0);
        let ret_bulk = self.handle.write_bulk(0x01, data_to_send, timeout);

        match ret_bulk {
            Ok(_v) => print!("Sent bytes: {:?}\n", ret_bulk.unwrap()),
            Err(e) => println!("error ret_bulk: {:?}", e),
        }

        let timeout = Duration::new(20, 0);
        let mut buf: [u8; 512] = unsafe { mem::uninitialized() };
        let ret_bulk_read = self.handle.read_bulk(0x81, &mut buf, timeout);
        // FOR ACK
        match ret_bulk_read {
            Ok(val) => {
                println!("RX bytes len: {:?}\n", val);
                // let ackfrm : bindings::UCAN_AckFrameDef = bincode::deserialize(&buf).unwrap();
                // println!("ACK frame {:?}", ackfrm);
            }
            Err(e) => println!("error ACK ret_bulk: {:?}", e),
        }
    }

    pub fn get_CAN_data(&mut self)
    {
        println!("Wait for CAN frame reception");
        let timeout = Duration::new(20, 0);
        let mut buf: [u8; 512] =  unsafe { mem::uninitialized() };      
        let ret_bulk_read = self.handle.read_bulk(0x81, &mut buf ,timeout);
    
        match ret_bulk_read {
        Ok(val) => 
        {
            print!("CAN RX bytes len: {:?}\n", val);
            // let ackfrm : bindings::UCAN_RxFrameDef = bincode::deserialize(&buf).unwrap();
            // print!("CAN RX frame {:?}", ackfrm.can_rx_header);

            // ackfrm.can_data.iter().map(|x| print!(",{}",x));
            println!(" ");
            
        },
            Err(e) => println!("error CAN_RX ret_bulk: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let ctx = libusb::Context::new().unwrap();
        for device in ctx.devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.product_id() == PRODUCT_ID {
                let mut handle = ctx
                    .open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id())
                    .unwrap();
                let ret_claim = handle.claim_interface(0);

                let mut cfdc = UsbCfdc::new(&handle);

                let bytes = vec![1, 2, 3];
                cfdc.send_cmd(bytes.as_slice());
            }
        }
    }
}

//     pub fn read(&mut self, _v: &mut Vec<u8>) {
//         println!("READING...");
//     }
//     pub fn write(&mut self, _v: &Vec<u8>) {
//         println!("WRITING...");
//     }
