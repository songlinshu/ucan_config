extern crate clap;
extern crate libusb;

mod bindings;
mod common;
mod usbcfdc;

use ::core::mem;
use clap::{App, Arg};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::Duration;
use std::{thread, time};

const PRODUCT_ID: u16 = 0x775;
const CONFIG_PATH: &str = "./../../config/";

// fn cli_interface() {
//     let matches = App::new("ucan_config")
//     .version("1.0")
//     .author("https://ucandevices.github.io/")
//     .about("Configuration for CAN devices")
//     .arg(Arg::with_name("protocol")
//          .short("p")
//          .long("protocol")
//          .help("Device protocol")
//          .value_name("usb|virtual")
//          .required(false)
//          .takes_value(true))
//     .arg(Arg::with_name("id")
//          .short("i")
//          .long("id")
//          .help("Device hardware ID")
//          .required(false)
//          .value_name("ex. 0x01234")
//          .takes_value(true))
//     .arg(Arg::with_name("dev_number")
//          .help("Device number on uCAN network in fomat from 0 .. 100")
//          .required(false)
//          .short("d")
//          .long("devno")
//          .takes_value(true))
//     .arg(Arg::with_name("baundrate")
//          .help("CAN baudrate in kBPS ex 1M 100k 100000")
//          .required(false)
//          .short("b")
//          .long("baudrate")
//          .takes_value(true))
//     .arg(Arg::with_name("mode")
//          .help("CAN mode")
//          .short("m")
//          .required(false)
//          .value_name("loopback|normal|monitor")
//          .long("mode")
//          .takes_value(true))
//     .arg(Arg::with_name("config_file_path")
//          .help("Path with config file")
//          .short("c")
//          .required(false)
//          .long("config")
//          .takes_value(true))
//     .get_matches();

//     let mut devNo: u16 = 0;
//     if let Some(o) = matches.value_of("dev_number") {
//         devNo = u16::from_str(o).unwrap_or(0);
//         println!("Value for dev_number: {}", devNo);
//     }

//     if let Some(p) = matches.value_of("protocol") {
//         println!("Value for protocol: {}", p);
//     }

//     // let frame_type = matches.value_of("frame_type");

//     // parse_frame(frame_type);
//     println!("Init CAN frame");
//     let buffer = common::read_data_from_json(&format!("{0}{1}",CONFIG_PATH,"UCAN_InitFrameDef.json"));
//             let frame: bindings::UCAN_InitFrameDef = serde_json::from_str(&buffer).unwrap();
//             let bytes = bincode::serialize(&frame).unwrap();
//             usb_send_data(bytes.as_slice(), false);

//     let context = zmq::Context::new();
//     let responder = context.socket(zmq::REP).unwrap();

//     // let mut w = Vec::new();
//     // write!(&mut w, "tcp://*:{}!", "{}");

//     let bb: &str = &(common::ZERO_MQ_STARTING_PORT + devNo).to_string();
//     println!("zeroMQ port:{}",bb);
//     assert!(responder.bind(&format!("tcp://*:{}",bb)).is_ok());
//     let mut msg = zmq::Message::new();
//     loop {
//         // thread::sleep(Duration::from_millis(10));
//         // responder.recv(&mut msg, 0).unwrap();
//         println!("TX frame in loopback");
//         let zmqData = responder.recv_bytes(0).unwrap();
//         println!("Received zmq");
//         // let frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();
//         // let bytes = bincode::serialize(&frame).unwrap();
//         usb_send_data(&zmqData, true);
//         // usb_send_data(bytes.as_slice(), true);
//         responder.send("OK", 0).unwrap();
//     }
// }

use std::sync::{Arc, Mutex};

fn main() {
    let mut devNo: u16 = 0;
    // cli_interface();

    let ctx = libusb::Context::new().unwrap();
    for device in ctx.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.product_id() == PRODUCT_ID {
            let mut handle = ctx
                .open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id())
                .unwrap();
            let ret_claim = handle.claim_interface(0);

            let mut cfdc = usbcfdc::UsbCfdc::new(&handle);

            println!("Init CAN frame");
            let buffer = common::read_data_from_json(&format!(
                "{0}{1}",
                CONFIG_PATH, "UCAN_InitFrameDef.json"
            ));
            let frame: bindings::UCAN_InitFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();

            cfdc.send_cmd(bytes.as_slice());

            let context = zmq::Context::new();
            let responder = context.socket(zmq::REP).unwrap();

            //     // let mut w = Vec::new();
            //     // write!(&mut w, "tcp://*:{}!", "{}");

            let bb: &str = &(common::ZERO_MQ_STARTING_PORT + devNo).to_string();
            println!("zeroMQ port:{}", bb);
            assert!(responder.bind(&format!("tcp://*:{}", bb)).is_ok());

            let mut msg = zmq::Message::new();
            loop {
                // thread::sleep(Duration::from_millis(10));
                // responder.recv(&mut msg, 0).unwrap();
                println!("TX frame in loopback");
                let zmqData = responder.recv_bytes(0).unwrap();
                println!("Received zmq");
                // let frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();
                // let bytes = bincode::serialize(&frame).unwrap();
                cfdc.send_cmd(&zmqData);
                // usb_send_data(bytes.as_slice(), true);
                responder.send("OK", 0).unwrap();
            }
        }
    }
    // let mut usb_cfdc = Arc::new(Mutex::new(UsbCfdc));
    // let p2 = usb_cfdc.clone();

    // let handle = thread::spawn(move || {
    //     let mut buffer: Vec<u8> = Vec::new();
    //     for j in 0..100 {
    //         let _bytes_read = p2.lock().unwrap().read(&mut buffer);
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // });

    // let request_temperature: Vec<u8> = vec![0xAA];
    // for i in 0..10 {
    //     usb_cfdc.lock().unwrap().write(&request_temperature);
    //     thread::sleep(Duration::from_millis(100));
    // }

    // handle.join();

    println!("exit");
}
