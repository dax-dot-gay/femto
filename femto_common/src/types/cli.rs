use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct FemtoCli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {}
