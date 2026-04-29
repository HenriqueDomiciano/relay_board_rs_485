use crate::transport::error::Result;
use crate::transport::generic::Transport;

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

    fn flush(&mut self) -> Result<()> {
        for _ in self.queued_responses.clone() {
            self.queued_responses.remove(0);
        }
        return Ok(());
    }
}
