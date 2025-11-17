
use std::io;
use std::io::Read;
use std::time::Duration;

use super::*;
use serialport::ClearBuffer;
use serialport::DataBits;
use serialport::Error;
use serialport::FlowControl;
use serialport::Parity;
use serialport::StopBits;
// Just For Testing purposes; 

#[allow(dead_code)]
pub struct DummySerialPort{
    buffer: Vec<u8>, 
    pos: usize
}

impl Read for DummySerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Simulate reading from the buffer
        let remaining = &self.buffer[self.pos..];
        let n = remaining.len().min(buf.len());
        buf[..n].copy_from_slice(&remaining[..n]);
        self.pos += n;
        Ok(n)
    }
}
#[allow(dead_code)]
impl Write for DummySerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Just swallow writes (or store them if you prefer)
        println!("DummySerialPort received write: {:?}", buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
impl SerialPort for DummySerialPort {
    fn name(&self) -> Option<String> { None }

    fn baud_rate(&self) -> Result<u32, Error> { Ok(9600) }
    fn data_bits(&self) -> Result<DataBits, Error> { Ok(DataBits::Eight) }
    fn flow_control(&self) -> Result<FlowControl, Error> { Ok(FlowControl::None) }
    fn parity(&self) -> Result<Parity, Error> { Ok(Parity::None) }
    fn stop_bits(&self) -> Result<StopBits, Error> { Ok(StopBits::One) }
    fn timeout(&self) -> std::time::Duration{
        std::time::Duration::from_millis(0)
    }


    fn set_baud_rate(&mut self, _baud_rate: u32) -> serialport::Result<()> {
        todo!()
    }

    fn set_data_bits(&mut self, _data_bits: serialport::DataBits) -> serialport::Result<()> {
        todo!()
    }
    
    fn set_flow_control(&mut self, _flow_control: serialport::FlowControl) -> serialport::Result<()> {
        todo!()
    }
    
    fn set_parity(&mut self, _parity: serialport::Parity) -> serialport::Result<()> {
        todo!()
    }
    
    fn set_stop_bits(&mut self, _stop_bits: serialport::StopBits) -> serialport::Result<()> {
        todo!()
    }
    
    fn set_timeout(&mut self, _timeout: Duration) -> serialport::Result<()> {
        todo!()
    }
    
    fn write_request_to_send(&mut self, _level: bool) -> serialport::Result<()> {
        todo!()
    }
    
    fn write_data_terminal_ready(&mut self, _level: bool) -> serialport::Result<()> {
        todo!()
    }
    
    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        todo!()
    }
    
    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        todo!()
    }
    
    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        todo!()
    }
    
    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        todo!()
    }
    
    fn bytes_to_read(&self) -> serialport::Result<u32> {
        todo!()
    }
    
    fn bytes_to_write(&self) -> serialport::Result<u32> {
        todo!()
    }
    
    fn clear(&self, _buffer: ClearBuffer) -> Result<(), Error> { Ok(()) }
    
    fn try_clone(&self) -> Result<Box<dyn SerialPort>, Error> {
        Ok(Box::new(DummySerialPort{buffer:Vec::new(),pos:10}))
    }
    
    fn set_break(&self) -> serialport::Result<()> {
        todo!()
    }
    
    fn clear_break(&self) -> serialport::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    use crate::RelayBoardRS485; 
    #[test]
    fn test_crc(){
        let command:[u8;6] = [0x01,0x06,0x00,0x01,0x01,0x00];
        let crc  = RelayBoardRS485::mod_bus_crc_calculation(&command); 
        assert_eq!(crc, 0xd99a);     
    }
    #[test]
    fn test_build_action_command()
    {
        let expected_command: Vec<u8> = [0x01,0x06,0x0,1,1,0,0xd9,0x9a].to_vec();
        let relay_object = RelayBoardRS485{serial_port:Box::new(DummySerialPort{buffer:Vec::new(),pos:0}),address:0x01};
        let command = relay_object.build_action_command(1, ActionCommandsEnum::Open, 0);
        assert_eq!(expected_command,command);
    }
    #[test]
    fn test_build_status_command()
    {
        let expected_command:Vec<u8> = [0x01,0x03,0,0x01,0x00,0x01,0xd5,0xca].to_vec(); 
        let relay_object = RelayBoardRS485{serial_port:Box::new(DummySerialPort{buffer:Vec::new(),pos:0}),address:0x01};
        let command = relay_object.build_status_command(1, 1);
        assert_eq!(expected_command, command);
    }
    #[test]
    fn test_response_parsing_1_entry()
    {
        let expected_result = StatusCommandResponseStruct{
            slave_id:1, 
            function:3, 
            data_lenght:2,
            data: Box::new(vec![1]), 
            crc: 0x7984
        }; 
        let command:Vec<u8> = [0x01,0x03,0x02,0x00,0x01,0x79,0x84].to_vec();
        let command_result = StatusCommandResponseStruct::from_bytes(&command); 
        assert_eq!(expected_result,command_result);
    }
}
