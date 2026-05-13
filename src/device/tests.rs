use crate::{RelayBoardR4D8A08, device::relay::Relay, transport::generic::Transport};

#[test]
fn test_r4_relay_action_commands() {
    let expected_result = crate::protocol::modbus::ModBusRequest {
        slave_addr: 1,
        function_code: 6,
        start_address: vec![0, 1],
        quantity: vec![1, 0],
    };
    let response = crate::device::relay::ActionCommand {
        slave_id: 1,
        function: 6,
        address: 1,
        command: crate::device::relay::ActionCommandsEnum::Open,
        delay_time: 0,
    }
    .to_r4_mod_bus_command();
    assert_eq!(expected_result, response.expect("Unable to parse"));
}

#[test]
fn test_r4_relay_status_commands() {
    let expected_result = crate::protocol::modbus::ModBusRequest {
        slave_addr: 1,
        function_code: 3,
        start_address: vec![0, 1],
        quantity: vec![0, 1],
    };
    let response = crate::device::relay::StatusCommand {
        slave_id: 1,
        function: 3,
        starting_register_address: 1,
        register_length: 1,
    }
    .to_mod_bus_command();
    assert_eq!(expected_result, response);
}

#[test]
fn test_Rs4_close_all_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.close_all(1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,0,8,0,0x8e,0x0a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_open_all_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.open_all(1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,0,7,0,0x8b,0xfa];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_open_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.open_channel(1,1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,1,0,0xd9,0x9a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_open_channel_2_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.open_channel(1,2,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,2,1,0,0x29,0x9a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_close_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.close_channel(1,1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,2,0,0xd9,0x6a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_close_channel_2_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.close_channel(1,2,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,2,2,0,0x29,0x6a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_toogle_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.toggle_channel(1,1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,3,0,0xd8,0xfa];
    assert_eq!(expected_command,response); 
}
#[test]
fn test_Rs4_latch_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.latch_channel(1,1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,4,0,0xda,0xca];
    assert_eq!(expected_command,response); 
}
#[test]
fn test_Rs4_momentary_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.momentary_channel(1,1,0); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,5,0,0xdb,0x5a];
    assert_eq!(expected_command,response); 
}

#[test]
fn test_Rs4_delay_channel_1_board_commands()
{   
    let mut transport = crate::transport::mock::MockTransport{sent_frames:Vec::new(), queued_responses:Vec::new()};
    let mut relay = RelayBoardR4D8A08{protocol:transport};
    let _ = relay.delay_time(1,1,10); 
    let mut response = relay.protocol.sent_frames.pop().expect("No value received");
    let expected_command:Vec<u8> = vec![1,6,0,1,6,0x0a,0x5b,0xad];
    assert_eq!(expected_command,response); 
}