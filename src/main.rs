extern crate clap;
extern crate libusb;

mod bindings;

use serde::Deserialize;
use std::sync::mpsc;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use clap::{App, Arg};
use std::path::Path;
use ::core::mem;

const PRODUCT_ID: u16 = 0x775;
const CONFIG_PATH: &str = "./../../config/";

fn send_data(data: &[u8]) {

    let context:libusb::Context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.product_id() == PRODUCT_ID {
            let handle = context.open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id());

            let mut dev_handle = handle.unwrap();
            let ret_claim = dev_handle.claim_interface(0);
            
            // let child = thread::spawn(move || 
            //     {
            //         let timeout = Duration::new(1, 0);
            //         let mut buf: [u8; 150] =  unsafe { mem::uninitialized() };      
            //         let ret_bulk_read = dev_handle.read_bulk(0x81, &mut buf ,timeout);
            
            //         match ret_bulk_read {
            //             Ok(val) => 
            //             {
            //                 print!("RX bytes len: {:?}\n", val);
            //                 let ackfrm : bindings::UCAN_AckFrameDef = bincode::deserialize(&buf).unwrap();
            //                 print!("ACK frame {:?}", ackfrm);
            
            //             },
            //             Err(e) => println!("error ACK ret_bulk: {:?}", e),
            //         }
            //     });

            if ret_claim.is_ok() {
                println!("Device claimed data to be send {:?} ",data);
                let data_to_send : &[u8] = data;

                let timeout = Duration::new(1, 0);
                let ret_bulk = dev_handle.write_bulk(0x01, data_to_send, timeout);

                match ret_bulk {
                    Ok(_v) => print!("Sent bytes: {:?}\n", ret_bulk.unwrap()),
                    Err(e) => println!("error ret_bulk: {:?}", e),
                }  
            }
            else {
                println!("Unable to access the requested interface");
            }
            
            // thread::sleep(time::Duration::from_millis(200));
            
            let timeout = Duration::new(20, 0);
            let mut buf: [u8; 512] =  unsafe { mem::uninitialized() };      
            let ret_bulk_read = dev_handle.read_bulk(0x81, &mut buf ,timeout);
            
            // FOR ACK
            // match ret_bulk_read {
            //     Ok(val) => 
            //     {
            //         print!("RX bytes len: {:?}\n", val);
            //         let ackfrm : bindings::UCAN_AckFrameDef = bincode::deserialize(&buf).unwrap();
            //         print!("ACK frame {:?}", ackfrm);
    
            //     },
            //     Err(e) => println!("error ACK ret_bulk: {:?}", e),
            // }
            match ret_bulk_read {
                Ok(val) => 
                {
                    print!("CAN RX bytes len: {:?}\n", val);
                    let ackfrm : bindings::UCAN_RxFrameDef = bincode::deserialize(&buf).unwrap();
                    print!("CAN RX frame {:?}", ackfrm.can_rx_header);
                    // ackfrm.can_data.iter().map(|x| print!(",{}",x));
                    println!(" ");
                    
                },
                Err(e) => println!("error ACK ret_bulk: {:?}", e),
            }
          

            // let res = child.join();
        }
    }
}

fn read_data_from_json(filename: &str) -> String {
    let path = Path::new(filename);
    // let mut file = File::open("UCAN_TxFrameDef.json").unwrap();
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}

fn parse_frame(frame_type : Option<&str>) {
    match frame_type {
        Some("1") => {
            let buffer = read_data_from_json("UCAN_AckFrameDef.json");
            let frame: bindings::UCAN_AckFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("2") => {
            let buffer = read_data_from_json("UCAN_DeinitFrameDef.json");
            let frame: bindings::UCAN_DeinitFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("3") => {
            let buffer = read_data_from_json("UCAN_Get_CAN_Status.json");
            let frame: bindings::UCAN_Get_CAN_Status = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("4") => {
            let buffer = read_data_from_json("UCAN_GoToBootladerFrameDef.json");
            let frame: bindings::UCAN_GoToBootladerFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("5") => {
            let buffer = read_data_from_json("UCAN_InitFrameDef.json");
            let frame: bindings::UCAN_InitFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("6") => {
            let buffer = read_data_from_json("UCAN_RxFrameDef.json");
            let frame: bindings::UCAN_RxFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("7") => {
            let buffer = read_data_from_json("UCAN_SaveConfigFrameDef.json");
            let frame: bindings::UCAN_SaveConfigFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("8") => {
            let buffer = read_data_from_json("UCAN_TxFrameDef.json");
            let frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("9") => {
            let buffer = read_data_from_json("FDCAN_Device_DescritionDef.json");
            let frame: bindings::FDCAN_Device_DescritionDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("10") => {
            let buffer = read_data_from_json("FDCAN_ErrorCountersTypeDef.json");
            let frame: bindings::FDCAN_ErrorCountersTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("11") => {
            let buffer = read_data_from_json("FDCAN_InitTypeDef.json");
            let frame: bindings::FDCAN_InitTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("12") => {
            let buffer = read_data_from_json("FDCAN_MsgRamAddressTypeDef.json");
            let frame: bindings::FDCAN_MsgRamAddressTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("13") => {
            let buffer = read_data_from_json("FDCAN_ProtocolStatusTypeDef.json");
            let frame: bindings::FDCAN_ProtocolStatusTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("14") => {
            let buffer = read_data_from_json("FDCAN_RxHeaderTypeDef.json");
            let frame: bindings::FDCAN_RxHeaderTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("15") => {
            let buffer = read_data_from_json("FDCAN_TxEventFifoTypeDef.json");
            let frame: bindings::FDCAN_TxEventFifoTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        Some("16") => {
            let buffer = read_data_from_json("FDCAN_TxHeaderTypeDef.json");
            let frame: bindings::FDCAN_TxHeaderTypeDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());
        },
        _ => { println!("Unknown parameter")}
    }
}

fn cli_interface() {
    let matches = App::new("ucan_config")
        .version("1.0")
        .author("https://ucandevices.github.io/")
        .about("Configuration for CAN devices")
        .arg(Arg::with_name("frame_type")
            .short("f")
            .long("frame_type")
            .help("1 - UCAN_AckFrameDef\n\
                    2 - UCAN_DeinitFrameDef\n\
                    3 - UCAN_Get_CAN_Status\n\
                    4 - UCAN_GoToBootladerFrameDef\n\
                    5 - UCAN_InitFrameDef\n\
                    6 - UCAN_RxFrameDef\n\
                    7 - UCAN_SaveConfigFrameDef\n\
                    8 - UCAN_TxFrameDef\n\
                    9 - FDCAN_Device_DescritionDef\n\
                    10 - FDCAN_ErrorCountersTypeDef\n\
                    11 - FDCAN_InitTypeDef\n\
                    12 - FDCAN_MsgRamAddressTypeDef\n\
                    13 - FDCAN_ProtocolStatusTypeDef\n\
                    14 - FDCAN_RxHeaderTypeDef\n\
                    15 - FDCAN_TxEventFifoTypeDef\n\
                    16 - FDCAN_TxHeaderTypeDef")
            .value_name("1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16")
            .required(true)
            .takes_value(true))
        .get_matches();

    // let frame_type = matches.value_of("frame_type");

    // parse_frame(frame_type);
    // let buffer = read_data_from_json(&format!("{0}{1}",CONFIG_PATH,"UCAN_InitFrameDef.json"));
    //         let frame: bindings::UCAN_InitFrameDef = serde_json::from_str(&buffer).unwrap();
    //         let bytes = bincode::serialize(&frame).unwrap();
    //         send_data(bytes.as_slice());

    println!("TX frame");
    let buffer = read_data_from_json(&format!("{0}{1}",CONFIG_PATH,"UCAN_TxFrameDef.json"));
            let frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice());

    
}

use std::{thread, time};
fn main() {

    cli_interface();
    
    println!("exit");
}
