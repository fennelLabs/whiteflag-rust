use fennel_whiteflag::decode_from_hex;

fn main() {
    let encoded = "57463130258000000000000000000000000000000000000000000000000000000000000000005012fd34e7b793989474e4d182c4943f8cba6f4ba9f0b6adf2fe7bfd1c5b1b758168";
    
    println!("=== Decoding K Message ===");
    println!("Encoded: {}...", &encoded[..80]);
    println!();
    
    match decode_from_hex(encoded.to_string()) {
        Ok(json) => {
            println!("✅ Decoded successfully!");
            println!();
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("❌ Error: {:?}", e);
        }
    }
}
