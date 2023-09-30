use serialport::SerialPort;

fn main() {
    // Open the serial port.
    let mut port = SerialPort::open("/dev/usb0", 115200).unwrap();

    // Create a buffer to store the incoming data.
    let mut buffer = [0; 128];

    // Read data from the serial port.
    let bytes_read = port.read(&mut buffer).unwrap();

    // Print the incoming data to the console.
    println!("Received {} bytes: {}", bytes_read, String::from_utf8_lossy(&buffer[..bytes_read]));
}