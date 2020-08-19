extern crate clap;
extern crate libusb;

mod bindings;
mod common;

use serde::Deserialize;
use std::sync::mpsc;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use clap::{App, Arg};
use std::{thread, time};
use std::path::Path;
use ::core::mem;

const PRODUCT_ID: u16 = 0x775;
const CONFIG_PATH: &str = "./../../config/";

fn send_data(data: &[u8], wait_for_rx: bool) {

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
                print!("Bytes {:?} ", data.len());
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
            match ret_bulk_read {
                Ok(val) => 
                {
                    println!("RX bytes len: {:?}\n", val);
                    let ackfrm : bindings::UCAN_AckFrameDef = bincode::deserialize(&buf).unwrap();
                    println!("ACK frame {:?}", ackfrm);
    
                },
                Err(e) => println!("error ACK ret_bulk: {:?}", e),
            }

            if wait_for_rx == true
            {
                println!("Wait for CAN frame reception");
                let timeout = Duration::new(20, 0);
                let mut buf: [u8; 512] =  unsafe { mem::uninitialized() };      
                let ret_bulk_read = dev_handle.read_bulk(0x81, &mut buf ,timeout);
            
                match ret_bulk_read {
                Ok(val) => 
                {
                    print!("CAN RX bytes len: {:?}\n", val);
                    let ackfrm : bindings::UCAN_RxFrameDef = bincode::deserialize(&buf).unwrap();
                    print!("CAN RX frame {:?}", ackfrm.can_rx_header);
                    // ackfrm.can_data.iter().map(|x| print!(",{}",x));
                    println!(" ");
                    
                },
                    Err(e) => println!("error CAN_RX ret_bulk: {:?}", e),
                }
            }
            // let res = child.join();
        }
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
            .help("")
            .value_name("1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16")
            .required(false)
            .takes_value(true))
        .get_matches();

    // let frame_type = matches.value_of("frame_type");

    // parse_frame(frame_type);
    println!("Init CAN frame");
    let buffer = common::read_data_from_json(&format!("{0}{1}",CONFIG_PATH,"UCAN_InitFrameDef.json"));
            let frame: bindings::UCAN_InitFrameDef = serde_json::from_str(&buffer).unwrap();
            let bytes = bincode::serialize(&frame).unwrap();
            send_data(bytes.as_slice(), false);

    
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        // thread::sleep(Duration::from_millis(10));
        // responder.recv(&mut msg, 0).unwrap();
        println!("TX frame in loopback");
        let zmqData = responder.recv_bytes(0).unwrap();
        // println!("Received {}", zmqData);  
        // let frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();
        // let bytes = bincode::serialize(&frame).unwrap();
        send_data(&zmqData, true);    
        // send_data(bytes.as_slice(), true);    
        responder.send("OK", 0).unwrap();
    }
}

fn main() {

    cli_interface();
    
    println!("exit");
}
