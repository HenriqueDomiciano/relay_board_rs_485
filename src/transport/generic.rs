use crate::transport::error::Result;
pub trait Transport {
    fn write_frame(&mut self, data: Vec<u8>) -> Result<()>;
    fn read_frame(&mut self) -> Result<Vec<u8>>;
    fn flush(&mut self) -> Result<()>;
}
