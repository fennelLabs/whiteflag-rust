mod auth;
#[cfg(test)]
mod test;

use clap::{AppSettings, Parser, Subcommand};
use std::error::Error;
use wf_cli::WhiteflagCLICommands;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Encode { json } => WhiteflagCLICommands::encode(json)?,
        Commands::Decode { hex } => WhiteflagCLICommands::decode(hex)?,
        Commands::Auth { logout } => WhiteflagCLICommands::auth(logout)?,
        Commands::Message { code } => {
            let hex = WhiteflagCLICommands::message(code)?.as_hex()?;
            hex
        }
    };

    println!("{}", result);
    Ok(())
}

#[derive(Parser)]
#[clap(name = "wf")]
#[clap(about = "A tool for interacting with the Whiteflag Protocol", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode { json: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode { hex: String },

    #[clap()]
    Auth { logout: bool },

    #[clap()]
    Message { code: String },
}
