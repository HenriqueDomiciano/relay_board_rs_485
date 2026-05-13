use crate::protocol::{
    crc,
    modbus::{ModBusRequest, ModBusResponse},
    utils,
};

#[test]
fn test_crc() {
    let command: [u8; 6] = [0x01, 0x06, 0x00, 0x01, 0x01, 0x00];
    let crc = crc::mod_bus_crc_calculation(&command);
    assert_eq!(crc, 0xd99a);
}

#[test]
fn test_to_vec_with_bytes() {
    let expected_command: Vec<u8> = [0x01, 0x06, 0x0, 1, 1, 0, 0xd9, 0x9a].to_vec();
    let mod_bus_struct = ModBusRequest {
        slave_addr: 1,
        function_code: 6,
        start_address: vec![0, 1],
        quantity: vec![1, 0],
    };
    assert_eq!(
        expected_command,
        mod_bus_struct.to_vec_with_bytes().expect("Unable to parse")
    );
}

#[test]
fn test_parse_response_1_entry() {
    let command_response: Vec<u8> = [0x01, 0x03, 0x02, 0x00, 0x01, 0x79, 0x84].to_vec();
    let expected_response = ModBusResponse {
        slave_addr: 1,
        function_code: 3,
        quantitiy: 2,
        data: vec![0, 1],
        crc: 0x7984,
    };
    let processed_command = ModBusResponse::from_vec(command_response).expect("Unable to process");
    assert_eq!(expected_response, processed_command);
}

#[test]
fn test_remove_trailing_zeros() {
    let response: Vec<u8> = vec![0, 0, 0, 0, 1, 2, 0];
    let result = utils::remove_trailing_zeros(response);
    let expected_result = vec![0, 0, 0, 0, 1, 2];
    assert_eq!(expected_result, result);
}
