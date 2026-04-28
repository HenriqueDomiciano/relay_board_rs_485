use clap::ValueEnum;

use crate::transport::generic::Transport;


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

pub struct RelayBoard<T: Transport> {
    port: T,
    address: u8,
}

