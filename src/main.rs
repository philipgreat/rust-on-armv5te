use serialport::prelude::*;
use std::io::Read;
use std::time::Duration;

fn main() {
    let port_name = "/dev/usb0";
    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };

    let mut port = serialport::open_with_settings(&port_name, &s).unwrap();

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    println!("Receiving data...");

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                for byte in serial_buf.iter().take(t) {
                    print!("{}", *byte as char);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
