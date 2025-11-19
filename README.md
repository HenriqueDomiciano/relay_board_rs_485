[![CI](https://github.com/HenriqueDomiciano/RelayBoardRS485/actions/workflows/CI.yml/badge.svg)](https://github.com/HenriqueDomiciano/RelayBoardRS485/actions/workflows/CI.yml)
[![Release](https://github.com/HenriqueDomiciano/RelayBoardRS485/actions/workflows/release.yml/badge.svg)](https://github.com/HenriqueDomiciano/RelayBoardRS485/actions/workflows/release.yml)

# Relay RS-485. 

This project provides a command-line interface (CLI) and lib for interacting
with an RS-485 MOD BUS RTU relay board using the `relay_board_rs_485` Rust crate. 
It allows sending action commands, querying status, and controlling
specific relay channels.

Obs: This project does not support async commands and this may be added in the future. 

There is also an rust lib on the project that can be ported or used in other projects, examples on how to use it 
are on the examples folder of this project. 

## Tested Boards 
The board that was tested and validated is the following one
<p align="center">
<img  width="400" height="400" alt="image" src="https://github.com/user-attachments/assets/4af70340-fc21-4be4-8053-aa1f87eabf7c" />
</p>
    
Other boards may or may not work. 

## 📦 Features

-   Open and close individual relays
-   Open and close all relays
-   Toggle and latch relays
-   Read status of all channels or a specific relay
-   Configure serial port, baud rate, slave address, and timeout

## 🚀 CLI Usage

### **Basic Command Structure**

    ./relay_board_rs_485 <SERIAL_PORT> [FLAGS] [OPTIONS]

### **Arguments and Options**

| Argument / Flag       | Description                                  | Default |
|------------------------|----------------------------------------------|---------|
| `<SERIAL_PORT>`        | Serial port path (e.g. `/dev/ttyUSB0`, `COM3`) | — |
| `-b, --baud-rate`      | Serial baud rate                              | `9600` |
| `-s, --slave-address`  | RS-485 device slave address                   | `1` |
| `-c, --command-type`   | Command type: `action` or `status`            | `action` |
| `-a, --action-command` | One of `Open`, `Close`, `Toggle`, `Latch`, `OpenAll`, `CloseAll`, etc. | — |
| `-r, --relay-value`    | Relay channel number (`u16`)                  | — |
| `-t, --timeout-ms`     | Serial timeout in milliseconds                | `10` |
| `-d, --delay-time-ms`  | Delay time for commands                       | `0` |


## 🧪 Examples

### **Check status of all relays**

    ./relay_board_rs_485 /dev/ttyUSB0 -c status

### **Close all relays**

    ./relay_board_rs_485 /dev/ttyUSB0 -a CloseAll

### **Open relay #3**

    ./relay_board_rs_485 /dev/ttyUSB0 -a Open --relay-value 3

### **Toggle relay #5 with 200 ms delay**

    ./relay_board_rs_485 /dev/ttyUSB0 -a Toggle --relay-value 5 -d 200

### **Read status of relay #2**

    ./relay_board_rs_485 /dev/ttyUSB0 --relay-value 2 -c status

## 🧩 Code Overview

### **Main Components**

-   **Argument parsing**: Implemented with `clap`'s `Parser` and
    `ValueEnum`.
-   **CommandTypes enum**: Defines `action` and `status` modes.
-   **Main logic**: Matches `command_type` and `relay_value` to
    determine which command to send.
-   **RelayBoardRS485**: Handles low-level serial communication with the
    relay board.


## lib
If you want to use the lib use the following command in your Cargo.toml file 

    [dependencies]
    relay_board_rs_485 = { git = "https://github.com/HenriqueDomiciano/RelayBoardRS485.git" }

## Releases 

if you do not want to build the binary there is the release page where you can find the latest and gratest cli version of this project. 
The current OS supported are linux and windows. 

## 🛠️ Building

    cargo build --release

Binary output:

    target/release/relay_board_rs485 


## 📜 License

MIT --- feel free to use and modify this project.
