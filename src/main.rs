
extern crate serialport;
use serialport::SerialPort;

fn main() {
    // Open the serial port.
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}