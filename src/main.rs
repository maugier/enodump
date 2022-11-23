use colored::Colorize;
use enocean::{port::{Port}, frame::{ESP3Frame}, FrameReadError};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short)] device: Option<String>,
}

fn main() {
    let args = Args::parse();
    let port_name = args.device.or_else(|| {
        match &serialport::available_ports().expect("Could not list serial ports")[..] {
            &[] => { eprintln!("No serial ports found."); None },
            &[ref port] => Some(port.port_name.clone()),
            more => { eprintln!("Multiple ports found. Use one of {:?}", more.iter().map(|i| &i.port_name)); None}
        }
    }).expect("No useable serial port.");
    
    let mut port = Port::open(&port_name)
        .expect("Could not open port");

    eprintln!("Port {} opened.", port_name);

    let version =  port.read_version_information().expect("Could not read version");

    eprintln!("Port version: {}", version);

    loop {
        match port.read_frame() {
            Ok(frame) => pretty_print(frame),
            Err(FrameReadError::DataCRC { frame, data_crc }) => {
                eprintln!("BAD CRC {:x} : {:?}", data_crc, frame)
            },
            Err(e) => { eprintln!("Error: {:?}", e); break }
        }
        eprintln!(".");
    }

}

fn pretty_print<'a>(frame: ESP3Frame) {
    let pkt_type = format!("{:02x}", frame.packet_type()).bright_yellow();
    let data = format!("{:x?}", frame.data()).bright_blue();
    let opt = format!("{:x?}", frame.optional_data()).bright_green();
    println!("TYPE: {} {} {}", pkt_type, data, opt);
}
