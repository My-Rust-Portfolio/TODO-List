// "Parser" autogerentes args from struct
use clap::{Parser};
use crate::command_handler::Commands;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)] // Tells clap to parse the subcommand into this field
    pub command: Commands,
}

pub fn get_input() -> Args {
    let args = Args::parse();
    println!("Parsed: {:?}", args.command);
    args
}