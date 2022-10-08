mod auth;
#[cfg(test)]
mod test;

use clap::{AppSettings, Parser, Subcommand};
use std::error::Error;
use wf_field::Header;

use crate::auth::UserAuthenticationState;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Encode { json } => fennel_whiteflag::encode_from_json(json)?,
        Commands::Decode { hex } => fennel_whiteflag::decode_from_hex(hex)?,
        Commands::Auth { logout } => {
            if logout {
                UserAuthenticationState::logout()
            } else {
                UserAuthenticationState::login()
            }
            .to_string()
        }
        Commands::Message { code } => {
            if UserAuthenticationState::is_authenticated() == false {
                "error: must authenticate using `wf auth`".to_string()
            } else {
                let header = Header::new(code);
                let body = header.to_body();
                body.to_string()?
            }
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
