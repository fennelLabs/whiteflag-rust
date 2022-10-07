use clap::{AppSettings, Parser, Subcommand};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Encode { json } => fennel_whiteflag::encode_from_json(json),
        Commands::Decode { hex } => fennel_whiteflag::decode_from_hex(hex),
        Commands::Authenticate => {
            let keypair = wf_crypto::ecdh_keypair::WhiteflagECDHKeyPair::default();
            Ok(serde_json::json!({"something": "" }).to_string())
        },
    }?;

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

    Authenticate
    /* #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encrypt { plaintext: String, identity: u32 },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decrypt { ciphertext: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    GenerateEncryptionChannel {},
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AcceptEncryptionChannel {
        identity_id: u32,
        secret_key: String,
        public_key: String,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    SendSecureMessage {
        sender_id: u32,
        message: String,
        recipient_id: u32,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AESEncrypt {
        secret: String,
        public_key: String,
        plaintext: String,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    AESDecrypt {
        secret: String,
        public_key: String,
        ciphertext: String,
    },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Verify {
        message: String,
        signature: String,
        identity: u32,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Sign { message: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    DecryptBacklog { identity: u32 },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    SendMessage {
        sender_id: u32,
        message: String,
        recipient_id: u32,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    GetMessages { id: u32 },

    #[clap()]
    CreateIdentity {},
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    RetrieveIdentity { id: u32 },
    #[clap()]
    RetrieveIdentities {},

    #[clap()]
    StartRPC {}, */
}
