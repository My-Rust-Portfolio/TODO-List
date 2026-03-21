use crate::command_handler::Commands;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

pub fn get_input() -> Args {
    let args = Args::parse();
    println!("Parsed: {:?}", args.command);
    args
}
