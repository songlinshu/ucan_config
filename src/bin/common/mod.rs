use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const ZERO_MQ_STARTING_PORT:u16 = 5555;

pub fn read_data_from_json(filename: &str) -> String {
    let path = Path::new(filename);
    // let mut file = File::open("UCAN_TxFrameDef.json").unwrap();
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}