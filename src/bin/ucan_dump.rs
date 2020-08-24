extern crate clap;
extern crate libusb;

use clap::{App, Arg};

fn cli_interface() {
    let matches = App::new("ucan_dump")
        .version("1.0")
        .author("https://ucandevices.github.io/")
        .about("Usage: candump [options] <CAN interface>+. 
        Up to 16 CAN interfaces with optional filter sets can be specified
        on the commandline in the form: <ifname>[,filter]*
        
        Comma separated filters can be specified for each given CAN interface:
         <can_id>:<can_mask> (matches when <received_can_id> & mask == can_id & mask)
        
         Exmpales 
         candump -c -c -ta can0,123:7FF,400:700,#000000FF can2,400~7F0 can3 can8
         candump vcan2,92345678:DFFFFFFF (match only for extended CAN ID 12345678)
         candump vcan2,123:7FF (matches CAN ID 123 - including EFF and RTR frames)
         candump vcan2,123:C00007FF (matches CAN ID 123 - only SFF and non-RTR frames)
         ")
        .arg(Arg::with_name("interfaceName")
            .long("dev")
            .short("d")
            .help("Device/Interface number from 0.100 if not spefied monitrong all intefaces")            
            .required(false)
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name("type")
            .short("t")
            .help("(timestamp: (a)bsolute/(d)elta/(z)ero/(A)bsolute w date)")            
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("drop_count")
            .short("n")
            .help("-n <count>  (terminate after receiption of <count> CAN frames)")            
            .required(false)
            .takes_value(true))            
        .arg(Arg::with_name("terminate")
            .short("T")
            .help("-T <msecs>  (terminate after <msecs> without any reception)")            
            .required(false)
            .takes_value(true))                        
        .get_matches();

    // let frame_type = matches.value_of("frame_type");
}

fn main() {

    cli_interface();
    
    println!("exit");
}
