use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Parsing Error")]
    ParsingError,

    #[error("command error")]
    UnableToSendError,

    #[error("command error")]
    UnknownError,

    #[error("command error")]
    UnsuportedCommand,
}

pub type Result<T> = std::result::Result<T, DeviceError>;
