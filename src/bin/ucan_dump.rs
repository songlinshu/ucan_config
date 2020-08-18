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
    let matches = App::new("ucan_dump")
        .version("1.0")
        .author("https://ucandevices.github.io/")
        .about("Sending CAN frames over zeroMQ uCAN devices")
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
}

use std::{thread, time};
fn main() {

    cli_interface();
    
    println!("exit");
}
