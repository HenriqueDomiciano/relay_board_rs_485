use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("timeout")]
    Timeout,

    #[error("crc error")]
    InvalidCrc,

    #[error("crc error")]
    UnknownError,

    #[error("crc error")]
    UnableToGetBaudRate,

    #[error("crc error")]
    UnableToSetTimeout,
}

pub type Result<T> = std::result::Result<T, TransportError>;
