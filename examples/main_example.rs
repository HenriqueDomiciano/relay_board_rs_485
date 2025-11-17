use std::time::Duration;

use relay_board_rs_485::RelayBoardRS485;

fn main() {
    let serial_port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Unable to open port");
    let mut relay_board = RelayBoardRS485::new(serial_port, 0x01);
    //Open All relays
    relay_board.open_all(100);
    //Close All relays
    relay_board.close_all(0);
    //Change relay address
    relay_board.change_address(2);
    // Open Specific relay
    relay_board.open_channel(1, 0);
    // Delay time command
    relay_board.delay_time(1, 0);
    // Latch Channel
    relay_board.latch_channel(1, 0);
    // Toogle specific channel
    relay_board.toogle_channel(1, 0);
    // Close specific close channel
    relay_board.close_channel(3, 10);
    //Open the relay board
    relay_board.open_channel(2, 10);
    // Toggle Channel command
    relay_board.toogle_channel(3, 10);
    // Return the status command Structure, that it's the return value
    relay_board.get_status(1, 7);

    let result = relay_board.get_status(1, 8);
    println!("{:?}", result);
}
