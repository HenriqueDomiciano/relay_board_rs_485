use std::{
    fmt::{self, Display},
    time::Duration,
};

use clap::{Parser, ValueEnum};
use relay_board_rs_485::{
    ModBusSerialTransport, RelayBoardR4D8A08, device::relay::ActionCommandsEnum,
};

#[derive(Debug, Clone, ValueEnum)]
enum CommandTypes {
    Action,
    Status,
}

impl Display for CommandTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CommandTypes::Action => write!(f, "action"),
            CommandTypes::Status => write!(f, "status"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    serial_port: String,

    #[arg(short = 'b', long, default_value_t = 9600)]
    baud_rate: u32,

    #[arg(short = 's', long, default_value_t = 1)]
    slave_address: u8,

    #[arg(short = 'c',long, default_value_t = CommandTypes::Status)]
    command_type: CommandTypes,

    #[arg(short = 'a', long, required_if_eq("command_type", "Action"))]
    action_command: Option<ActionCommandsEnum>,

    #[arg(short, long)]
    relay_value: Option<u16>,

    #[arg(short = 't', long, default_value_t = 10)]
    timeout_ms: u64,

    #[arg(short = 'd', long, default_value_t = 0)]
    delay_time_ms: u8,
}

fn main() {
    let args = Args::parse();
    let serial_port = serialport::new(args.serial_port, args.baud_rate)
        .timeout(Duration::from_millis(args.timeout_ms))
        .open()
        .expect("Unable to open serial port");
    let transport = ModBusSerialTransport { port: serial_port };
    let mut relay_board = RelayBoardR4D8A08 {
        protocol: transport,
    };

    match args.relay_value {
        None => match args.command_type {
            CommandTypes::Action => match args.action_command {
                None => {
                    let status = relay_board.read_status(args.slave_address, 1, 8);
                    println!("{:?}", status.expect("Error on Status").data);
                }
                Some(action) => match action {
                    ActionCommandsEnum::CloseAll => {
                        let _ = relay_board.close_all(args.slave_address, args.delay_time_ms);
                    }
                    ActionCommandsEnum::OpenAll => {
                        let _ = relay_board.open_all(args.slave_address, args.delay_time_ms);
                    }
                    _ => panic!("Unable to define command without relay value"),
                },
            },
            CommandTypes::Status => {
                let status = relay_board.read_status(args.slave_address, 1, 8);
                println!("{:?}", status.unwrap().data);
            }
        },
        Some(value) => match args.command_type {
            CommandTypes::Action => match args.action_command {
                None => {
                    let status = relay_board
                        .read_status(args.slave_address, 1, 8)
                        .unwrap()
                        .data[value as usize];
                    println!("{}", status);
                }
                Some(action) => match action {
                    ActionCommandsEnum::Close => {
                        let _ = relay_board.close_channel(
                            args.slave_address,
                            value,
                            args.delay_time_ms,
                        );
                    }
                    ActionCommandsEnum::Open => {
                        let _ =
                            relay_board.open_channel(args.slave_address, value, args.delay_time_ms);
                    }
                    ActionCommandsEnum::Latch => {
                        let _ = relay_board.latch_channel(
                            args.slave_address,
                            value,
                            args.delay_time_ms,
                        );
                    }
                    ActionCommandsEnum::Toggle => {
                        let _ = relay_board.toggle_channel(
                            args.slave_address,
                            value,
                            args.delay_time_ms,
                        );
                    }
                    ActionCommandsEnum::Delay => {
                        let _ =
                            relay_board.delay_time(args.slave_address, value, args.delay_time_ms);
                    }
                    _ => panic!("Not supported type for specific relay command"),
                },
            },
            CommandTypes::Status => {
                let status = relay_board
                    .read_status(args.slave_address, 1, 8)
                    .unwrap()
                    .data[(value - 1) as usize];
                println!("{}", status);
            }
        },
    }
}
