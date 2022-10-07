use clap::{AppSettings, Parser, Subcommand};
use std::{error::Error, str::FromStr};
use wf_field::{Header, MessageCodeType};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Encode { json } => fennel_whiteflag::encode_from_json(json)?,
        Commands::Decode { hex } => fennel_whiteflag::decode_from_hex(hex)?,
        Commands::Authenticate => {
            let keypair = wf_crypto::ecdh_keypair::WhiteflagECDHKeyPair::default();
            serde_json::json!({"something": "" }).to_string()
        }
        Commands::Message { code } => serde_json::to_string(&Header::new(code))?,
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
    Encode {
        json: String,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode {
        hex: String,
    },

    Authenticate,

    Message {
        code: String,
    },
}
