use fennel_whiteflag::{encode_from_json, decode_from_hex};
use serde_json::json;

fn main() {
    let public_key = "025fa69cf6f273128e9c9a30589287f1974de9753e16d5be5fcf7fa38b636eb02d";
    
    let message = json!({
        "prefix": "WF",
        "version": "1",
        "encryptionIndicator": "0",
        "duressIndicator": "0",
        "messageCode": "K",
        "referenceIndicator": "0",
        "referencedMessage": "0000000000000000000000000000000000000000000000000000000000000000",
        "cryptoDataType": "0A",
        "cryptoData": public_key
    });
    
    println!("=== Original Message ===");
    println!("{}", serde_json::to_string_pretty(&message).unwrap());
    println!();
    
    println!("=== Encoding ===");
    match encode_from_json(message.to_string()) {
        Ok(encoded) => {
            println!("✅ Encoded successfully!");
            println!("Encoded hex: {}", encoded);
            println!("Length: {} chars", encoded.len());
            println!();
            
            println!("=== Decoding Back ===");
            match decode_from_hex(encoded.clone()) {
                Ok(decoded_json) => {
                    println!("✅ Decoded successfully!");
                    println!("{}", decoded_json);
                    
                    let decoded: serde_json::Value = serde_json::from_str(&decoded_json).unwrap();
                    if let Some(crypto_data) = decoded.get("cryptoData") {
                        let crypto_data_str = crypto_data.as_str().unwrap_or("");
                        println!();
                        if crypto_data_str == public_key {
                            println!("✅ PUBLIC KEY MATCHES!");
                        } else {
                            println!("❌ PUBLIC KEY MISMATCH!");
                            println!("Expected: {}", public_key);
                            println!("Got:      {}", crypto_data_str);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Decode error: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Encode error: {:?}", e);
        }
    }
}
