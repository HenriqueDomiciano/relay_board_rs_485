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

const ACTION_COMMAND_R4: u8 = 6;
const READ_STATUS_COMMAND_R4: u8 = 3;

const OPERATE_ALL_COMMAND_WAVE_SHARE: u8 = 0xff;
const ACTION_COMMAND_WAVE_SHARE: u8 = 5;
const READ_STATUS_COMMAND_WAVE_SHARE: u8 = 1;

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
    pub fn to_value_r4(&self) -> u8 {
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
    pub fn to_value_wave_share(&self) -> Result<u16> {
        match *self {
            ActionCommandsEnum::Open => Ok(0xFF00),
            ActionCommandsEnum::Close => Ok(0x0000),
            ActionCommandsEnum::Toggle => Ok(0x5500),
            ActionCommandsEnum::Latch => Ok(0x0200),
            ActionCommandsEnum::Momentary => Ok(0x0400),
            ActionCommandsEnum::Delay => Err(DeviceError::UnsuportedCommand),
            ActionCommandsEnum::OpenAll => Err(DeviceError::UnsuportedCommand),
            ActionCommandsEnum::CloseAll => Err(DeviceError::UnsuportedCommand),
        }
    }
}

pub struct StatusCommandResponse {
    pub slave_id: u8,
    pub function: u8,
    pub data_lenght: u8,
    pub data: Box<Vec<u16>>,
    pub crc: u16,
}
impl StatusCommandResponse {
    pub fn from_modbus_r4_response(response: ModBusResponse) -> Result<Self> {
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
    fn mask_to_vec(mask: u8) -> Vec<u16> {
        (0..16)
            .map(|i| {
                if i < 8 && (mask & (1 << i)) != 0 {
                    1u16 // Or any specific u16 value you want for "on"
                } else {
                    0u16 // "off" or padding
                }
            })
            .collect()
    }
    pub fn from_modbus_wave_share_response(response: ModBusResponse) -> Result<Self> {
        let slave_id = response.slave_addr;
        let data_lenght: u8 = 8;
        let data = Box::new(Self::mask_to_vec(response.data[0]));
        Ok(Self {
            slave_id,
            function: response.function_code,
            data_lenght,
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
        ModBusRequest {
            slave_addr: self.slave_id,
            function_code: self.function,
            start_address,
            quantity,
        }
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
    pub fn to_r4_mod_bus_command(&self) -> Result<ModBusRequest> {
        let address = self.address.to_be_bytes().to_vec();
        let mut command = self.command.to_value_r4().to_be_bytes().to_vec();
        command.push(self.delay_time);
        Ok(ModBusRequest {
            slave_addr: self.slave_id,
            function_code: self.function,
            start_address: address,
            quantity: command,
        })
    }
    pub fn to_wave_share_mod_bus_command(&self) -> Result<ModBusRequest> {
        let address = self.address.to_be_bytes().to_vec();
        let mut command = self.command.to_value_wave_share()?.to_be_bytes().to_vec();
        command.push(self.delay_time);
        Ok(ModBusRequest {
            slave_addr: self.slave_id,
            function_code: self.function,
            start_address: address,
            quantity: command,
        })
    }
}
pub struct RelayBoardWaveShare<T: Transport> {
    protocol: T,
}
impl<T: Transport> RelayBoardWaveShare<T> {
    pub fn get_status(&mut self, status_command: StatusCommand) -> Result<StatusCommandResponse> {
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
        match StatusCommandResponse::from_modbus_wave_share_response(mod_bus_response) {
            Ok(final_value) => Ok(final_value),
            Err(_) => Err(DeviceError::ParsingError),
        }
    }
    pub fn send_command(&mut self, command: ActionCommand) -> Result<()> {
        let mod_bus_command = match command.to_wave_share_mod_bus_command() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        let final_command = match mod_bus_command.to_vec_with_bytes() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        match self.protocol.write_frame(final_command) {
            Ok(()) => Ok(()),
            Err(_) => Err(DeviceError::UnableToSendError),
        }
    }
    pub fn close_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_WAVE_SHARE,
            address: channel,
            command: ActionCommandsEnum::Close,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn open_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_WAVE_SHARE,
            address: channel,
            command: ActionCommandsEnum::Open,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn toogle_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_WAVE_SHARE,
            address: channel,
            command: ActionCommandsEnum::Toggle,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn latch_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_WAVE_SHARE,
            address: channel,
            command: ActionCommandsEnum::Latch,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn delay_time(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_WAVE_SHARE,
            address: channel,
            command: ActionCommandsEnum::Delay,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn open_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: OPERATE_ALL_COMMAND_WAVE_SHARE,
            address: 0,
            command: ActionCommandsEnum::OpenAll,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn close_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: OPERATE_ALL_COMMAND_WAVE_SHARE,
            address: 0,
            command: ActionCommandsEnum::CloseAll,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn read_status(
        &mut self,
        slave_addr: u8,
        starting_register: u16,
        register_length: u16,
    ) -> Result<StatusCommandResponse> {
        let command = StatusCommand {
            slave_id: slave_addr,
            function: READ_STATUS_COMMAND_WAVE_SHARE,
            starting_register_address: starting_register,
            register_length,
        };
        self.get_status(command)
    }
}

pub struct RelayBoardR4D8A08<T: Transport> {
    pub protocol: T,
}
impl<T: Transport> RelayBoardR4D8A08<T> {
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
        match StatusCommandResponse::from_modbus_r4_response(mod_bus_response) {
            Ok(final_value) => Ok(final_value),
            Err(_) => Err(DeviceError::ParsingError),
        }
    }
    fn send_command(&mut self, command: ActionCommand) -> Result<()> {
        let mod_bus_command = match command.to_r4_mod_bus_command() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        let final_command = match mod_bus_command.to_vec_with_bytes() {
            Ok(command) => command,
            Err(_) => return Err(DeviceError::ParsingError),
        };
        match self.protocol.write_frame(final_command) {
            Ok(()) => Ok(()),
            Err(_) => Err(DeviceError::UnableToSendError),
        }
    }
    pub fn close_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: channel,
            command: ActionCommandsEnum::Close,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn open_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: channel,
            command: ActionCommandsEnum::Open,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn toogle_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: channel,
            command: ActionCommandsEnum::Toggle,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn latch_channel(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: channel,
            command: ActionCommandsEnum::Latch,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn delay_time(&mut self, slave_addr: u8, channel: u16, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: channel,
            command: ActionCommandsEnum::Delay,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn open_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: 0,
            command: ActionCommandsEnum::OpenAll,
            delay_time,
        };
        self.send_command(command)
    }

    pub fn close_all(&mut self, slave_addr: u8, delay_time: u8) -> Result<()> {
        let command = ActionCommand {
            slave_id: slave_addr,
            function: ACTION_COMMAND_R4,
            address: 0,
            command: ActionCommandsEnum::CloseAll,
            delay_time,
        };
        self.send_command(command)
    }
    pub fn read_status(
        &mut self,
        slave_addr: u8,
        starting_register: u16,
        register_length: u16,
    ) -> Result<StatusCommandResponse> {
        let command = StatusCommand {
            slave_id: slave_addr,
            function: READ_STATUS_COMMAND_R4,
            starting_register_address: starting_register,
            register_length,
        };
        self.get_status(command)
    }
}
