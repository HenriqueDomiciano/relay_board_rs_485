use std::time::Duration;

use relay_board_rs_485::{ModBusSerialTransport, RelayBoardR4D8A08};
use serialport::SerialPort;

fn main() {
    let serial_port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Unable to open port");
    let protocol = ModBusSerialTransport{port:serial_port}; 
    let mut relay_board = RelayBoardR4D8A08{protocol};
    //Open All relays
    relay_board.open_all(1,100).expect("Error");
    //Close All relays
    relay_board.close_all(1,0).expect("Error");
    // Open Specific relay
    relay_board.open_channel(1,1, 0).expect("Error");
    // Delay time command
    relay_board.delay_time(1,1, 0).expect("Error");
    // Latch Channel
    relay_board.latch_channel(1,1, 0).expect("Error");
    // Toogle specific channel
    relay_board.toogle_channel(1,1, 0).expect("Error");
    // Close specific close channel
    relay_board.close_channel(1,3, 10).expect("Error");
    //Open the relay board
    relay_board.open_channel(1,2, 10).expect("Error");
    // Toggle Channel command
    relay_board.toogle_channel(1,3, 10).expect("Error");
    // Return the status command Structure, that it's the return value
    relay_board.read_status(1,1, 7).expect("Error");

    let result = relay_board.read_status(1,1, 8);
    println!("{:?}", result.unwrap().data);
}
