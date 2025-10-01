use serialport::{SerialPort, SerialPortType};
use std::io::{self, Read, Write};
use std::time::Duration;

const BAUD_RATE: u32 = 115200; // Adjust this to match your MCU's baud rate

fn main() -> io::Result<()> {
    // List available ports
    let ports = serialport::available_ports().expect("No serial ports found!");
    println!("Available serial ports:");
    for (i, port) in ports.iter().enumerate() {
        println!("{}: {}", i, port.port_name);
    }

    // Get port selection from user
    println!("Enter the number of the port to use:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let port_num: usize = input.trim().parse().expect("Please enter a valid number");
    let port_name = &ports[port_num].port_name;

    // Open the serial port
    let mut port = serialport::new(port_name, BAUD_RATE)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    println!("Connected to {}. Type 'on' to start sensor, 'off' to stop, or 'exit' to quit.", port_name);

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        
        match command.trim() {
            "on" => {
                port.write_all(b"turn_on\n")?;
                println!("Sent turn on command");
            }
            "off" => {
                port.write_all(b"turn_off\n")?;
                println!("Sent turn off command");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command. Use 'on', 'off', or 'exit'");
                continue;
            }
        }

        // Read response from MCU
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let response = String::from_utf8_lossy(&serial_buf[..t]);
                println!("Received: {}", response);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("No response received (timeout)");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
            }
        }
    }

    Ok(())
}
