# Telecommand Sender

A Rust application to send telecommands to a microcontroller (MCU) over UART and receive sensor data. You can also simulate the MCU using virtual serial ports (e.g., socat) for development and testing.

## Features
- List available serial ports
- Send commands (`on`, `off`, `exit`) to MCU
- Display responses from MCU
- Supports manual entry of serial port name (for virtual ports)
- Can be tested without hardware using socat

## Requirements
- Rust (cargo)
- [serialport crate](https://crates.io/crates/serialport)
- Linux (for socat virtual ports)
- socat (for simulation)

## Usage

### 1. Build the Project
```bash
cargo build --release
```

### 2. Run the Program
```bash
cargo run
```

- The program will list available serial ports.
- Enter the port number or type the port name (e.g., `/dev/pts/3`).
- Type `on` to start sensor, `off` to stop, or `exit` to quit.

### 3. Simulate MCU with socat

#### a. Start socat to create virtual serial ports:
```bash
socat -d -d PTY,raw,echo=0 PTY,raw,echo=0
```
- socat will print two device names, e.g., `/dev/pts/3` and `/dev/pts/4`.
- **These are simulated virtual serial ports, not real hardware devices.**
- Use one port (e.g., `/dev/pts/3`) in the Rust app.

#### b. Simulate MCU responses:
- In another terminal, send responses to the other port:
  ```bash
  echo "simulated mcu response" | sudo tee /dev/pts/4
  ```
- Or interactively:
  ```bash
  screen /dev/pts/4 115200
  # Type responses and press Enter
  ```

#### c. See what the Rust app sends:
- In another terminal:
  ```bash
  cat /dev/pts/4
  ```
- When you type `on` in the Rust app, you'll see `turn_on` appear here.

## Notes
- Always use the device names printed by socat when it starts.
- You may need `sudo` to write to `/dev/pts/*` devices.
- The Rust app continuously prints any incoming serial data.

## Example Workflow
1. Start socat and note the device names.
2. Run the Rust app and select one device.
3. In another terminal, send simulated responses to the Rust app by writing to the other simulated port (e.g., `/dev/pts/4`).
4. The Rust app will immediately display any received messages from the simulated MCU port.
5. When you send commands from the Rust app, they will appear on the other simulated port, allowing you to observe them as if you were the MCU.