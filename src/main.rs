use std::{
    fmt::{self, Display},
    time::Duration,
};

use clap::{Parser, ValueEnum};
use relay_board_rs_485::{ActionCommandsEnum, RelayBoardRS485};

#[derive(Debug, Clone, ValueEnum)]
enum CommandTypes {
    Action,
    Status,
}

impl Display for CommandTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CommandTypes::Action => write!(f, "ACTION"),
            CommandTypes::Status => write!(f, "STATUS"),
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

    #[arg(short = 'c',long, default_value_t = CommandTypes::Action)]
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
    let mut relay_board = RelayBoardRS485::new(serial_port, args.slave_address);

    match args.relay_value {
        None => match args.command_type {
            CommandTypes::Action => match args.action_command {
                None => {
                    let status = relay_board.get_status(1, 8);
                    println!("{:?}", status.data);
                }
                Some(action) => match action {
                    ActionCommandsEnum::CloseAll => relay_board.close_all(args.delay_time_ms),
                    ActionCommandsEnum::OpenAll => relay_board.open_all(args.delay_time_ms),
                    _ => panic!("Unable to define command without relay value"),
                },
            },
            CommandTypes::Status => {
                let status = relay_board.get_status(1, 8);
                println!("{:?}", status.data);
            }
        },
        Some(value) => match args.command_type {
            CommandTypes::Action => match args.action_command {
                None => {
                    let status = relay_board.get_status(1, 8).data[value as usize];
                    println!("{}", status);
                }
                Some(action) => match action {
                    ActionCommandsEnum::Close => {
                        relay_board.close_channel(value, args.delay_time_ms)
                    }
                    ActionCommandsEnum::Open => relay_board.open_channel(value, args.delay_time_ms),
                    ActionCommandsEnum::Latch => {
                        relay_board.latch_channel(value, args.delay_time_ms)
                    }
                    ActionCommandsEnum::Toggle => {
                        relay_board.toogle_channel(value, args.delay_time_ms)
                    }
                    ActionCommandsEnum::Delay => relay_board.delay_time(value, args.delay_time_ms),
                    _ => panic!("Not supported type for specific relay command"),
                },
            },
            CommandTypes::Status => {
                let status = relay_board.get_status(1, 8).data[value as usize];
                println!("{}", status);
            }
        },
    }
}
