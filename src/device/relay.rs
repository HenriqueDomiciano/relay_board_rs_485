use crate::device::error::{DeviceError, Result};
use clap::ValueEnum;

use crate::{
    protocol::modbus::{ModBusRequest, ModBusResponse},
    transport::generic::Transport,
};

pub trait Relay {
    fn read_status(
        &mut self,
        slave_addr: u8,
        starting_register: u16,
        register_length: u16,
    ) -> Result<StatusCommandResponse>;
    fn toogle_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()>;
    fn open_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()>;
    fn close_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()>;
    fn latch_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()>;
    fn delay_time(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()>;
    fn open_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()>;
    fn close_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()>;
}

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
    #[must_use]
    pub fn value(&self) -> u8 {
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

pub struct StatusCommandResponse {
    slave_id: u8,
    function: u8,
    data_lenght: u8,
    pub data: Box<Vec<u16>>,
    crc: u16,
}
impl StatusCommandResponse {
    pub fn from_modbus_response(response: ModBusResponse) -> Result<Self> {
        let slave_id = response.slave_addr;
        let data_raw = response
            .data
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        let data = Box::new(data_raw);
        Ok(Self {
            slave_id,
            function: response.function_code,
            data_lenght: response.quantitiy,
            data,
            crc: response.crc,
        })
    }
}

pub struct StatusCommand {
    slave_id: u8,
    function: u8,
    starting_register_address: u16,
    register_length: u16,
}
impl StatusCommand {
    pub fn to_mod_bus_command(&self) -> ModBusRequest {
        let mut buffer: [u8; 6] = [0; 6];
        buffer[2..4].copy_from_slice(&self.starting_register_address.to_be_bytes());
        buffer[4..6].copy_from_slice(&self.register_length.to_be_bytes());
        let start_address = buffer[2..4].to_vec();
        let quantity = buffer[4..6].to_vec();
        return ModBusRequest {
            slave_addr: self.slave_id,
            function_code: self.function,
            start_address,
            quantity,
        };
    }
}

pub struct ActionCommand {
    slave_id: u8,
    function: u8,
    address: u16,
    command: ActionCommandsEnum,
    delay_time: u8,
}
impl ActionCommand {
    pub fn to_mod_bus_command(&self) -> Result<ModBusRequest> {
        let address = self.address.to_be_bytes().to_vec();
        let mut command = self.command.value().to_be_bytes().to_vec();
        command.push(self.delay_time);
        Ok(ModBusRequest {
            slave_addr: self.slave_id,
            function_code: self.function,
            start_address: address,
            quantity: command,
        })
    }
}
pub struct RelayBoard<T: Transport> {
    protocol: T,
}
impl<T: Transport> RelayBoard<T> {
    fn get_status(&mut self, status_command: StatusCommand) -> Result<StatusCommandResponse> {
        self.protocol.flush().expect("Error flushing serial");
        let command = status_command.to_mod_bus_command();
        let _ = self
            .protocol
            .write_frame(command.to_vec_with_bytes().expect("Unable to parse"));
        let bytes = match self.protocol.read_frame() {
            Ok(result) => result,
            Err(_) => return Err(DeviceError::UnableToSendError),
        };
        let mod_bus_response = match ModBusResponse::from_vec(bytes) {
            Ok(value) => value,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        match StatusCommandResponse::from_modbus_response(mod_bus_response) {
            Ok(final_value) => return Ok(final_value),
            Err(_) => return Err(DeviceError::ParsingError),
        };
    }
    fn send_command(&mut self, command: ActionCommand) -> Result<()> {
        let mod_bus_command = match command.to_mod_bus_command() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        let final_command = match mod_bus_command.to_vec_with_bytes() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        match self.protocol.write_frame(final_command) {
            Ok(()) => return Ok(()),
            Err(_) => return Err(DeviceError::UnableToSendError),
        }
    }
}
impl<T: Transport> Relay for RelayBoard<T> {
    fn close_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: channel,
            command: ActionCommandsEnum::Close,
            delay_time: delay_time,
        };
        self.send_command(command)
    }

    fn open_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: channel,
            command: ActionCommandsEnum::Open,
            delay_time: delay_time,
        };
        self.send_command(command)
    }

    fn toogle_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: channel,
            command: ActionCommandsEnum::Toggle,
            delay_time: delay_time,
        };
        self.send_command(command)
    }

    fn latch_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: channel,
            command: ActionCommandsEnum::Latch,
            delay_time: delay_time,
        };
        self.send_command(command)
    }
    fn delay_time(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: channel,
            command: ActionCommandsEnum::Delay,
            delay_time: delay_time,
        };
        self.send_command(command)
    }
    fn open_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: 0,
            command: ActionCommandsEnum::OpenAll,
            delay_time: delay_time,
        };
        self.send_command(command)
    }

    fn close_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND,
            address: 0,
            command: ActionCommandsEnum::CloseAll,
            delay_time: delay_time,
        };
        self.send_command(command)
    }
    fn read_status(
        &mut self,
        slave_addr: u8,
        starting_register: u16,
        register_length: u16,
    ) -> Result<StatusCommandResponse> {
        let command = StatusCommand {
            slave_id: slave_addr,
            function: READ_STATUS_COMMAND,
            starting_register_address: starting_register,
            register_length,
        };
        self.get_status(command)
    }
}
