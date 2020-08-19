extern crate clap;
extern crate libusb;

mod bindings;
use std::{thread, time};
use clap::{App, Arg};

mod common;

const CONFIG_PATH: &str = "./../../config/";

fn cli_interface() {
    let matches = App::new("ucan_send")
        .version("1.0")
        .author("https://ucandevices.github.io/")
        .about("Sending CAN frames over zeroMQ uCAN devices")
        .arg(Arg::with_name("interfaceName")
            .long("dev")
            .short("d")
            .help("Device/Interface name ex. can0")            
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("idFrame")
            .long("idFrame")
            .short("i")
            .help("Frame id CAN be hex or dec ex 0x123 or 291")            
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("idType")
            .short("T")
            .long("idType")
            .help("Frame id Ex|St type Ex for Extended St For standard")            
            .required(false)
            .takes_value(true)
            .value_name("Ex|St"))
        .arg(Arg::with_name("DataLength")
            .short("L")
            .long("DataLen")
            .help("Data Lenght in bytes for 0 to 64")            
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("FDFormat")
            .short("F")
            .long("FD")
            .help("IF present is fd frame ")            
            .required(false)
            .takes_value(false))
            
        .get_matches();

    if let Some(o) = matches.value_of("idFrame") {
        println!("Value for idFrame: {}", o);
    }

    if let Some(o) = matches.value_of("idType") {
        println!("Value for idType: {}", o);
    }

    if let Some(o) = matches.value_of("interfaceName") {
        println!("Value for interfaceName: {}", o);
    }
}

fn main() {

    cli_interface();

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    let buffer = common::read_data_from_json(&format!("{0}{1}",CONFIG_PATH,"UCAN_TxFrameDef.json"));
    let mut frame: bindings::UCAN_TxFrameDef = serde_json::from_str(&buffer).unwrap();

    frame.can_tx_header.Identifier = 0x22;

    let bytes = bincode::serialize(&frame).unwrap();
    
    requester.send(bytes, 0).unwrap();

    requester.recv(&mut msg, 0).unwrap();
    println!("Received {}:", msg.as_str().unwrap());
    
    println!("exit");

    // data transmited via zeroMQ as bytestream UCAN_TxFrameDef

}
