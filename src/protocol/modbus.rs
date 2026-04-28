use crate::protocol::crc::mod_bus_crc_calculation;
use serialport::SerialPort;
use std::io::Error;
use std::time::Duration;
use std::u8;

use super::crc;
use super::utils;

pub struct ModBusRequest {
    slave_addr: u8,
    function_code: u8,
    start_address: Vec<u8>,
    quantitiy: Vec<u8>,
    crc: u16,
}

impl ModBusRequest {
    pub fn to_vec_with_bytes(self) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(self.slave_addr);
        buffer.push(self.function_code);
        buffer.extend(&self.start_address);
        buffer.extend(&self.quantitiy);
        let crc_16 = crc::mod_bus_crc_calculation(&buffer);
        let crc_bytes = &crc_16.to_be_bytes();
        buffer.extend_from_slice(crc_bytes);
        Ok(buffer)
    }
}

pub struct ModBusResponse {
    slave_addr: u8,
    function_code: u8,
    quantitiy: u8,
    data: Vec<u8>,
    crc: u16,
}

impl ModBusResponse {
    #[must_use]
    pub fn from_vec(response: Vec<u8>) -> Self {
        let slave_id = response.get(0).expect("Unable to find slave id");
        let function_code = response.get(1).expect("Unable to get function code");
        let quantity = response.get(2).expect("Unable to get quantity");
        let __data_to_read: usize = (quantity + 3) as usize;
        let __data_value_u8 = response
            .get(3..__data_to_read)
            .expect("Unable to get data value");
        let data = __data_value_u8.to_vec();
        let mut buffer: Vec<u8> = vec![*slave_id, *function_code, *quantity];
        buffer.extend(__data_value_u8);
        let calculated_crc_16 = crc::mod_bus_crc_calculation(&buffer);
        let obtained_crc_u8_slice = response.get(__data_to_read..).expect("Unable to get crc 16");
        let obtained_crc = u16::from_be_bytes(obtained_crc_u8_slice[0..2].try_into().expect("Slice must have at least 2 bytes"));
        assert!(calculated_crc_16 == obtained_crc);
        return Self {
            slave_addr: *slave_id,
            function_code: *function_code,
            quantitiy: *quantity,
            data: data,
            crc: obtained_crc,
        };
    }
}
