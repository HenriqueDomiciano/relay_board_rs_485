pub mod protocol;
pub mod transport;
pub mod device ; 
use clap::ValueEnum;
use serialport::SerialPort;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;



const ACTION_COMMAND: u8 = 6;
const READ_STATUS_COMMAND: u8 = 3;
#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum ActionCommandsEnum {
    Open,
    Close,
    Toggle,
    Latch,
    Momentary,
    Delay,
    OpenAll,
    CloseAll,
}

impl ActionCommandsEnum {   
    #[must_use] pub fn value(&self) -> u8 {
        match *self {
            ActionCommandsEnum::Open => 0x01,
            ActionCommandsEnum::Close => 0x02,
            ActionCommandsEnum::Toggle => 0x03,
            ActionCommandsEnum::Latch => 0x04,
            ActionCommandsEnum::Momentary => 0x05,
            ActionCommandsEnum::Delay => 0x06,
            ActionCommandsEnum::OpenAll => 0x07,
            ActionCommandsEnum::CloseAll => 0x08,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StatusCommandResponseStruct {
    slave_id: u8,
    function: u8,
    data_lenght: u8,
    pub data: Box<Vec<u16>>,
    crc: u16,
}

impl StatusCommandResponseStruct {
    fn from_bytes(data: &[u8]) -> Self {
        let slave_id = *data[0..1].first().expect("Failed to get Slave Id");
        let function = *data[1..2].first().expect("Failed to get byte function");
        let data_lenght = *data[2..3].first().expect("Failed to get byte data lenght");
        let __data_to_read: usize = (data_lenght + 3) as usize;
        let __data_value_u8 = data
            .get(3..__data_to_read)
            .expect("Unable to get data value");
        let data_value_u16: Vec<u16> = __data_value_u8
            .chunks(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        let __crc_u8: [u8; 2] = data
            .get(__data_to_read..__data_to_read + 2)
            .expect("Unable to get CRC")
            .try_into()
            .expect("Unable to parse CRC");
        let crc = u16::from_be_bytes(__crc_u8);
        let validation_data = &data[..data.len() - 2];
        let validation_crc = RelayBoardRS485::mod_bus_crc_calculation(validation_data);
        assert!(!(validation_crc != crc), "CRC Received is different than calculated"); 
        Self {
            slave_id,
            function,
            data_lenght,
            data: Box::new(data_value_u16),
            crc,
        }
    }
}

pub struct StatusCommandStruct {
    slave_id: u8,
    function: u8,
    starting_register_address: u16,
    register_length: u16,
}
impl StatusCommandStruct {
    fn to_bytes(&self) -> [u8; 6] {
        let mut buffer: [u8; 6] = [0; 6];
        buffer[0..1].copy_from_slice(&self.slave_id.to_be_bytes());
        buffer[1..2].copy_from_slice(&self.function.to_be_bytes());
        buffer[2..4].copy_from_slice(&self.starting_register_address.to_be_bytes());
        buffer[4..6].copy_from_slice(&self.register_length.to_be_bytes());
        buffer
    }
}

pub struct ActionCommandStruct {
    slave_id: u8,
    function: u8,
    address: u16,
    command: u8,
    delay_time: u8,
}

impl ActionCommandStruct {
    fn to_bytes(&self) -> [u8; 6] {
        let mut buffer: [u8; 6] = [0; 6];
        buffer[0..1].copy_from_slice(&self.slave_id.to_be_bytes());
        buffer[1..2].copy_from_slice(&self.function.to_be_bytes());
        buffer[2..4].copy_from_slice(&self.address.to_be_bytes());
        buffer[4..5].copy_from_slice(&self.command.to_be_bytes());
        buffer[5..6].copy_from_slice(&self.delay_time.to_be_bytes());
        buffer
    }
}

pub struct RelayBoardRS485 {
    serial_port: Box<dyn SerialPort>,
    address: u8,
}
impl RelayBoardRS485 {
    #[must_use] pub fn new(serial_port: Box<dyn SerialPort>, address: u8) -> Self {
        Self {
            serial_port,
            address,
        }
    }
    fn build_status_command(
        &self,
        starting_register_address: u16,
        register_length: u16,
    ) -> Vec<u8> {
        let mut final_command: Vec<u8> = Vec::new();
        let command = StatusCommandStruct {
            slave_id: self.address,
            function: READ_STATUS_COMMAND,
            starting_register_address,
            register_length,
        };
        let crc = &RelayBoardRS485::mod_bus_crc_calculation(&command.to_bytes()).to_be_bytes();
        final_command.extend_from_slice(&command.to_bytes());
        final_command.extend_from_slice(crc);
        final_command
    }
    pub fn change_address(&mut self, new_address: u8) {
        self.address = new_address;
    }
    fn build_action_command(
        &self,
        channel: u16,
        command: ActionCommandsEnum,
        delay_time: u8,
    ) -> Vec<u8> {
        let mut final_command: Vec<u8> = Vec::new();
        let command = ActionCommandStruct {
            slave_id: self.address,
            function: ACTION_COMMAND,
            address: channel,
            command: command.value(),
            delay_time,
        };
        let crc = RelayBoardRS485::mod_bus_crc_calculation(&command.to_bytes());
        let crc_u8 = &crc.to_be_bytes();
        final_command.extend_from_slice(&command.to_bytes());
        final_command.extend_from_slice(crc_u8);
        final_command
    }
    pub fn remove_trailing_zeros(mut vec: Vec<u8>) -> Vec<u8> {
        let last_non_zero_index = vec
            .iter()
            .rfind(|&&x| x != 0)
            .map(|x| std::ptr::from_ref::<u8>(x) as usize - vec.as_ptr() as usize);

        if let Some(index) = last_non_zero_index {
            vec.truncate(index / std::mem::size_of::<u8>() + 1);
        } else {
            vec.clear();
        }
        vec
    }

    fn read_mod_bus_until_timeout(&mut self) -> Vec<u8> {
        let timeout_ms: f64 = (35_f64
            / self
                .serial_port
                .baud_rate()
                .expect("Failed to get baud rate") as f64)
            * 1000.0;
        self.serial_port
            .set_timeout(Duration::from_millis(timeout_ms.ceil() as u64 + 100))
            .expect("Failed to set timeout");
        let mut serial_buffer: Vec<u8> = vec![0; 256];
        let mut final_buffer: Vec<u8> = Vec::new();
        loop {
            match self.serial_port.read(&mut serial_buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        final_buffer.extend_from_slice(&serial_buffer[..bytes_read]);
                        io::stdout().flush().unwrap();
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return RelayBoardRS485::remove_trailing_zeros(final_buffer);
                }
                Err(e) => {
                    eprintln!("Error {e:?} reading from serial rtu");
                    panic!("Error reading modbus");
                }
            }
        }
    }
    pub fn send_command(&mut self, command: Vec<u8>) {
        self.serial_port
            .write_all(&command)
            .expect("Failed to send data to relay");
        sleep(Duration::from_millis(30));
    }
    pub fn get_status(
        &mut self,
        starting_register_adress: u16,
        register_lenght: u16,
    ) -> StatusCommandResponseStruct {
        self.serial_port.flush().expect("Error flushing serial");
        self.serial_port
            .clear(serialport::ClearBuffer::All)
            .expect("Fail to clear buffer !!!");
        let command = self.build_status_command(starting_register_adress, register_lenght);
        self.send_command(command);
        let buffer = self.read_mod_bus_until_timeout();
        StatusCommandResponseStruct::from_bytes(&buffer)
    }
    pub fn close_channel(&mut self, channel: u16, delay_time: u8) {
        let command = self.build_action_command(channel, ActionCommandsEnum::Close, delay_time);
        self.send_command(command);
    }

    pub fn open_channel(&mut self, channel: u16, delay_time: u8) {
        let command = self.build_action_command(channel, ActionCommandsEnum::Open, delay_time);
        self.send_command(command);
    }

    pub fn toogle_channel(&mut self, channel: u16, delay_time: u8) {
        let command = self.build_action_command(channel, ActionCommandsEnum::Toggle, delay_time);
        self.send_command(command);
    }

    pub fn latch_channel(&mut self, channel: u16, delay_time: u8) {
        let command = self.build_action_command(channel, ActionCommandsEnum::Latch, delay_time);
        self.send_command(command);
    }
    pub fn delay_time(&mut self, channel: u16, delay_time: u8) {
        let command = self.build_action_command(channel, ActionCommandsEnum::Delay, delay_time);
        self.send_command(command);
    }
    pub fn open_all(&mut self, delay_time: u8) {
        let command = self.build_action_command(0, ActionCommandsEnum::OpenAll, delay_time);
        self.send_command(command);
    }

    pub fn close_all(&mut self, delay_time: u8) {
        let command = self.build_action_command(0, ActionCommandsEnum::CloseAll, delay_time);
        self.send_command(command);
    }

    fn mod_bus_crc_calculation(command: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        let crc_polinomial: u16 = 0xA001;
        for &byte in command {
            crc ^= u16::from(byte);
            for _ in 0..8 {
                if crc & 0x0001 != 0 {
                    crc = (crc >> 1) ^ crc_polinomial;
                } else {
                    crc >>= 1;
                }
            }
        }
        u16::swap_bytes(crc)
    }
}

mod lib_test;
