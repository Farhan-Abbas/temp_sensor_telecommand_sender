use serialport::{SerialPort, SerialPortType};
use std::io::{self, Read, Write};
use std::time::Duration;

const BAUD_RATE: u32 = 115200; // Adjust this to match your MCU's baud rate

fn main() -> io::Result<()> {
    // List available ports
    let ports = serialport::available_ports().unwrap_or_default();
    println!("Available serial ports:");
    for (i, port) in ports.iter().enumerate() {
        println!("{}: {}", i, port.port_name);
    }

    // Allow manual entry of port name
    println!("Enter the number of the port to use, or type the port name (e.g. /dev/pts/1):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    let port_name = if let Ok(port_num) = input.parse::<usize>() {
        if port_num < ports.len() {
            ports[port_num].port_name.clone()
        } else {
            println!("Invalid port number, exiting.");
            return Ok(());
        }
    } else {
        input.to_string()
    };

    // Open the serial port
    let mut port = serialport::new(&port_name, BAUD_RATE)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    println!("Connected to {}. Type 'on' to start sensor, 'off' to stop, or 'exit' to quit.", &port_name);

    use std::thread;
    use std::sync::mpsc;

    // Spawn a thread to continuously read and print from the serial port
    let mut port_reader = port.try_clone().expect("Failed to clone serial port");
    thread::spawn(move || {
        loop {
            let mut serial_buf: Vec<u8> = vec![0; 32];
            match port_reader.read(serial_buf.as_mut_slice()) {
                Ok(t) if t > 0 => {
                    let response = String::from_utf8_lossy(&serial_buf[..t]);
                    println!("Received: {}", response);
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    // Ignore timeouts
                }
                Err(e) => {
                    eprintln!("Error reading from serial port: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

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
    }

    Ok(())
}
