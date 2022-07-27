/// Whiteflag encryption parameters enum class
/// This is a non-instantiatable enum class that holds all
/// encryption parameters in accordance with the Whiteflag specification.
/// No implementation specific properties and methods are defined by this class.
///
/// Whiteflag Specification 5.2.3 Key and Token Derivation
pub enum WhiteflagEncryptionMethod {
    /// Encryption Method 0: no encryption
    NO_ENCRYPTION {
        field_value: &str,
        algorithm_name: &str,
        operation_mode: &str,
        padding_scheme: &str,
        key_length: usize,
        hkdf_salt: &str,
    },

    /// Encryption Method 1: AES-256-CTR with negotiated key
    AES_256_CTR_ECDH {
        field_value: &str,
        algorithm_name: &str,
        operation_mode: &str,
        padding_scheme: &str,
        key_length: usize,
        hkdf_salt: &str,
    },

    /// Encryption Method : AES-256-CTR with pre-shared key
    AES_256_CTR_PSK {
        field_value: &str,
        algorithm_name: &str,
        operation_mode: &str,
        padding_scheme: &str,
        key_length: usize,
        hkdf_salt: &str,
    },
}

/// Returns the encryption method from the indicator value.
pub fn encryption_method_from_field_value(
    field_value: &str,
) -> WhiteflagCryptoOption<WhiteflagEncryptionMethod> {
    return match field_value {
        "0" => WhiteflagEncryptionMethod::NO_ENCRYPTION("0", "NONE", "NONE", "NoPadding", 0, ""),
        "1" => WhiteflagEncryptionMethod::AES_256_CTR_ECDH(
            "1",
            "AES",
            "CTR",
            "NoPadding",
            32,
            "8ddb03085a2c15e69c35c224bce2952dca7878770724741cbce5a135328be0c0",
        ),
        "2" => WhiteflagEncryptionMethod::AES_256_CTR_PSK(
            "2",
            "AES",
            "CTR",
            "NoPadding",
            32,
            "c4d028bd45c876135e80ef7889835822a6f19a31835557d5854d1334e8497b56",
        ),
        _ => Err(WhiteflagCryptoError::InvalidMethod),
    };
}
