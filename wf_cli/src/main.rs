#[cfg(test)]
mod test;

use clap::{AppSettings, Parser, Subcommand};
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};
use wf_field::Header;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Encode { json } => fennel_whiteflag::encode_from_json(json)?,
        Commands::Decode { hex } => fennel_whiteflag::decode_from_hex(hex)?,
        Commands::Authenticate => {
            let keypair = wf_crypto::ecdh_keypair::WhiteflagECDHKeyPair::default();
            serde_json::json!({"something": "" }).to_string()
        }
        Commands::Message { code } => {
            let header = Header::new(code);
            let body = header.to_body();
            body.to_string()?
        }
    };

    println!("{}", result);
    Ok(())
}

/// Generates an authentication lock.
fn acquire_auth_lock() {
    // Some good, old-fashioned semaphore action.
    match File::create(Path::new(".authlock")) {
        Err(why) => panic!("couldn't create .authlock file: {}", why),
        Ok(file) => file,
    };
}

/// Removes the authentication lock.
fn release_auth_lock() -> bool {
    match fs::remove_file(".authlock") {
        Err(_) => false,
        Ok(_) => true,
    }
}

/// Checks whether a lockfile exists.
fn check_auth_lock() -> bool {
    match File::open(Path::new(".authlock")) {
        Err(_) => false,
        Ok(_) => true,
    }
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
