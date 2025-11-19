# Relay RS-485 CLI Tool

This project provides a command-line interface (CLI) for interacting
with an RS-485 relay board using the `relay_board_rs_485` Rust crate. It
allows sending action commands, querying status, and controlling
specific relay channels.

This project was used to implement an simple, small and easy to use CLI and lib to interface 
with RS485 Modbus RTU Boards.

Obs: This project does not support async actions and this may be added in the future. 

## 📦 Features

-   Open and close individual relays
-   Open and close all relays
-   Toggle and latch relays
-   Read status of all channels or a specific relay
-   Configure serial port, baud rate, slave address, and timeout

## 🚀 Usage

### **Basic Command Structure**

    relay_board_rs_485 <SERIAL_PORT> [FLAGS] [OPTIONS]

### **Arguments and Options**

  -----------------------------------------------------------------------
  Argument / Flag                Description             Default
  ------------------------------ ----------------------- ----------------
  `<SERIAL_PORT>`                Serial port path        ---
                                 (e.g. `/dev/ttyUSB0`,   
                                 `COM3`)                 

  `-b, --baud-rate`              Serial baud rate        `9600`

  `-s, --slave-address`          RS-485 device slave     `1`
                                 address                 

  `-c, --command-type`           `action` or `status`    `action`

  `-a, --action-command`         One of `Open`, `Close`, ---
                                 `Toggle`, `Latch`,      
                                 `OpenAll`, `CloseAll`,  
                                 etc.                    

  `--relay-value`                Relay channel number    ---
                                 (`u16`)                 

  `-t, --timeout-ms`             Serial timeout in       `10`
                                 milliseconds            

  `-d, --delay-time-ms`          Delay time for commands `0`
  -----------------------------------------------------------------------

## 🧪 Examples

### **Check status of all relays**

    relay_board_rs_485 /dev/ttyUSB0 -c status

### **Close all relays**

    relay_board_rs_485 /dev/ttyUSB0 -a CloseAll

### **Open relay #3**

    relay_board_rs_485 /dev/ttyUSB0 -a Open --relay-value 3

### **Toggle relay #5 with 200 ms delay**

    relay_board_rs_485 /dev/ttyUSB0 -a Toggle --relay-value 5 -d 200

### **Read status of relay #2**

    relay_board_rs_485 /dev/ttyUSB0 --relay-value 2 -c status

## 🧩 Code Overview

### **Main Components**

-   **Argument parsing**: Implemented with `clap`'s `Parser` and
    `ValueEnum`.
-   **CommandTypes enum**: Defines `action` and `status` modes.
-   **Main logic**: Matches `command_type` and `relay_value` to
    determine which command to send.
-   **RelayBoardRS485**: Handles low-level serial communication with the
    relay board.

## 🛠️ Building

    cargo build --release

Binary output:

    target/release/

## 📜 License

MIT --- feel free to use and modify this project.
