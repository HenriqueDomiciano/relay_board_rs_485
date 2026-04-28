use crate::transport::generic::Transport;
use crate::transport::error::Result;

pub struct MockTransport {
    pub sent_frames: Vec<Vec<u8>>,
    pub queued_responses: Vec<Vec<u8>>,
}

impl Transport for MockTransport {
    fn write_frame(&mut self, data: Vec<u8>) -> Result<()> {
        self.sent_frames.push(data);
        Ok(())
    }

    fn read_frame(&mut self) -> Result<Vec<u8>> {
        Ok(self.queued_responses.remove(0))
    }
}