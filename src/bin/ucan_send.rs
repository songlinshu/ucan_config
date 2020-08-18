extern crate clap;
extern crate libusb;

// mod bindings;

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

fn cli_interface() {
    let matches = App::new("ucan_send")
        .version("1.0")
        .author("https://ucandevices.github.io/")
        .about("Sending CAN frames over zeroMQ uCAN devices")
        .arg(Arg::with_name("idFrame")
            .long("idFrame")
            .short("i")
            .help("Frame id CAN be hex or dec ex 0x123 or 291")            
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("idType")
            .short("I")
            .long("idType")
            .help("Frame id Ex|St type Ex for Extended St For standard")            
            .required(false)
            .takes_value(true)
            .value_name("Ex|St"))
            
        .get_matches();

    if let Some(o) = matches.value_of("idFrame") {
        println!("Value for idFrame: {}", o);
    }

    if let Some(o) = matches.value_of("idType") {
        println!("Value for idType: {}", o);
    }
}

use std::{thread, time};
use std::process::Command;
fn main() {

    cli_interface();
    
    println!("exit");

    // data transmited via zeroMQ as bytestream UCAN_TxFrameDef

}
