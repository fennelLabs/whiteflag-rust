use super::error::{WhiteflagCryptoError, WhiteflagCryptoResult};

/// Whiteflag encryption parameters enum class
/// This is a non-instantiatable enum class that holds all
/// encryption parameters in accordance with the Whiteflag specification.
/// No implementation specific properties and methods are defined by this class.
///
/// Whiteflag Specification 5.2.3 Key and Token Derivation
#[derive(Clone)]
pub enum WhiteflagEncryptionMethod {
    /// Encryption Method 0: no encryption
    NoEncryption {
        field_value: String,
        algorithm_name: String,
        operation_mode: String,
        padding_scheme: String,
        key_length: usize,
        hkdf_salt: String,
    },

    /// Encryption Method 1: AES-256-CTR with negotiated key
    Aes256CtrEcdh {
        field_value: String,
        algorithm_name: String,
        operation_mode: String,
        padding_scheme: String,
        key_length: usize,
        hkdf_salt: String,
    },

    /// Encryption Method : AES-256-CTR with pre-shared key
    Aes256CtrPsk {
        field_value: String,
        algorithm_name: String,
        operation_mode: String,
        padding_scheme: String,
        key_length: usize,
        hkdf_salt: String,
    },

    Aes512IegEcdh {
        field_value: String,
        algorithm_name: String,
        operation_mode: String,
        padding_scheme: String,
        key_length: usize,
        hkdf_salt: String,
    },

    Aes512IegPsk {
        field_value: String,
        algorithm_name: String,
        operation_mode: String,
        padding_scheme: String,
        key_length: usize,
        hkdf_salt: String,
    },
}

/// Returns the encryption method from the indicator value.
pub fn encryption_method_from_field_value(
    field_value: String,
) -> WhiteflagCryptoResult<WhiteflagEncryptionMethod> {
    return match field_value.as_str() {
        "0" => Ok(WhiteflagEncryptionMethod::NoEncryption {
            field_value: "0".to_string(),
            algorithm_name: "NONE".to_string(),
            operation_mode: "NONE".to_string(),
            padding_scheme: "NoPadding".to_string(),
            key_length: 0,
            hkdf_salt: "".to_string(),
        }),
        "1" => Ok(WhiteflagEncryptionMethod::Aes256CtrEcdh {
            field_value: "1".to_string(),
            algorithm_name: "AES".to_string(),
            operation_mode: "CTR".to_string(),
            padding_scheme: "NoPadding".to_string(),
            key_length: 32,
            hkdf_salt: "8ddb03085a2c15e69c35c224bce2952dca7878770724741cbce5a135328be0c0"
                .to_string(),
        }),
        "2" => Ok(WhiteflagEncryptionMethod::Aes256CtrPsk {
            field_value: "2".to_string(),
            algorithm_name: "AES".to_string(),
            operation_mode: "CTR".to_string(),
            padding_scheme: "NoPadding".to_string(),
            key_length: 32,
            hkdf_salt: "c4d028bd45c876135e80ef7889835822a6f19a31835557d5854d1334e8497b56"
                .to_string(),
        }),
        "3" => Ok(WhiteflagEncryptionMethod::Aes512IegEcdh {
            field_value: "3".to_string(),
            algorithm_name: "AES".to_string(),
            operation_mode: "IEG".to_string(),
            padding_scheme: "x16".to_string(),
            key_length: 32,
            hkdf_salt: "".to_string(),
        }),
        "4" => Ok(WhiteflagEncryptionMethod::Aes512IegPsk {
            field_value: "4".to_string(),
            algorithm_name: "AES".to_string(),
            operation_mode: "IEG".to_string(),
            padding_scheme: "x16".to_string(),
            key_length: 32,
            hkdf_salt: "".to_string(),
        }),
        _ => Err(WhiteflagCryptoError::InvalidMethod),
    };
}
