use std::io::{self, Write};
use std::{thread::sleep, time::Duration};

use crate::protocol::utils;

use crate::transport::error::{Result, TransportError};
use crate::transport::generic::Transport;

const TIMEOUT_OFF_SET: u64 = 100;
const TIMEOUT_MULTIPLIER: f64 = 1000.0;

pub struct ModBusSerialTransport {
    pub port: Box<dyn serialport::SerialPort>,
}
impl Transport for ModBusSerialTransport {
    fn write_frame(&mut self, data: Vec<u8>) -> Result<()> {
        match self.port.write_all(&data) {
            Ok(()) => {
                sleep(Duration::from_millis(30));
                Ok(())
            }
            Err(_) => Err(TransportError::UnknownError),
        }
    }

    fn flush(&mut self) -> Result<()> {
        let _ = self.port.flush();

        match self.port.clear(serialport::ClearBuffer::All) {
            Ok(()) => Ok(()),
            Err(_e) => Err(TransportError::UnknownError),
        }
    }

    fn read_frame(&mut self) -> Result<Vec<u8>> {
        let baud_rate = match self.port.baud_rate() {
            Ok(baud_rate) => baud_rate,
            Err(_) => return Err(TransportError::UnableToGetBaudRate),
        };
        let timeout_ms: f64 = (35_f64 / f64::from(baud_rate)) * TIMEOUT_MULTIPLIER;
        match self.port.set_timeout(Duration::from_millis(
            timeout_ms.ceil() as u64 + TIMEOUT_OFF_SET,
        )) {
            Ok(()) => (),
            Err(_) => return Err(TransportError::UnableToSetTimeout),
        }
        let mut serial_buffer: Vec<u8> = vec![0; 256];
        let mut final_buffer: Vec<u8> = Vec::new();
        loop {
            match self.port.read(&mut serial_buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        final_buffer.extend_from_slice(&serial_buffer[..bytes_read]);
                        io::stdout().flush().unwrap();
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    let data = utils::remove_trailing_zeros(final_buffer);
                    return Ok(data);
                }
                Err(_e) => {
                    return Err(TransportError::UnknownError);
                }
            }
        }
    }
}
