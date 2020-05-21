extern crate clap;
use clap::{App, Arg, SubCommand};

fn cli_interface() {
     let matches = App::new("ucan_config")
          .version("1.0")
          .author("https://ucandevices.github.io/")
          .about("Configuration for CAN devices")
          .arg(Arg::with_name("protocol")
               .short("p")
               .long("protocol")
               .help("Device protocol")
               .value_name("usb|virtual")
               .takes_value(true))
          .arg(Arg::with_name("id")
               .short("i")
               .long("id")
               .help("Device hardware ID")
               .value_name("ex. 0x01234")
               .takes_value(true))
          .arg(Arg::with_name("name")
               .help("Device name on uCAN network ex. can0 vcan0 ...")
               .short("n")
               .long("name")
               .takes_value(true))
          .arg(Arg::with_name("baundrate")
               .help("CAN baudrate in kBPS ex 1M 100k 100000")
               .short("b")
               .long("baudrate")
               .takes_value(true))
          .arg(Arg::with_name("mode")
               .help("CAN mode")
               .short("m")
               .required(false)
               .value_name("loopback|normal|monitor")
               .long("mode")
               .takes_value(true))
          .arg(Arg::with_name("config_file_path")
               .help("Path with config file")
               .short("c")
               .required(false)
               .long("config")
               .takes_value(true))
          .arg(Arg::with_name("v")
               .short("v")
               .multiple(true)
               .help("Sets the level of verbosity"))
          .arg(Arg::with_name("s")
               .short("s")
               .long("state")
               .multiple(true)
               .value_name("open|close")
               .help("Set device state"))               
          .get_matches();

     // Gets a value for config if supplied by user, or defaults to "default.conf"
     let protocol = matches.value_of("protocol").unwrap_or("default.conf");
     println!("Value for protocol: {}", protocol);

     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
     // required we could have used an 'if let' to conditionally get the value)
     println!("Using name: {}", matches.value_of("name").unwrap());

     // Vary the output based on how many times the user used the "verbose" flag
     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
     match matches.occurrences_of("v") {
          0 => println!("No verbose info"),
          1 => println!("Some verbose info"),
          2 => println!("Tons of verbose info"),
          3 | _ => println!("Don't be crazy"),
     }
}

fn main() {
     
     cli_interface();

     // more program logic goes here...
     let context = zmq::Context::new();
     let responder = context.socket(zmq::REP).unwrap();
 
     assert!(responder.bind("tcp://*:5555").is_ok());
}
