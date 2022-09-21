use super::crypto_util::Hkdf;
use super::error::{CryptoError, CryptoResult};

pub struct EncryptionAlgorithm {
    field_value: usize,
    algorithm_name: &'static str,
    operation_mode: &'static str,
    padding_scheme: &'static str,
    key_length: usize,
    hkdf_salt: &'static str,
}

impl EncryptionAlgorithm {
    pub fn derive_secret_key(&self, psk: &[u8], context: &[u8]) -> Vec<u8> {
        let salt = hex::decode(self.hkdf_salt).unwrap();
        Hkdf::extract(psk, &salt)
            .expand(context, self.key_length)
            .unwrap()
    }
}

/// Whiteflag encryption parameters enum class
/// This is a non-instantiatable enum class that holds all
/// encryption parameters in accordance with the Whiteflag specification.
/// No implementation specific properties and methods are defined by this class.
///
/// Whiteflag Specification 5.2.3 Key and Token Derivation
#[derive(Clone)]
pub enum WhiteflagEncryptionMethod {
    /// Encryption Method 0: no encryption
    NoEncryption,
    /// Encryption Method 1: AES-256-CTR with negotiated key
    Aes256CtrEcdh,
    /// Encryption Method : AES-256-CTR with pre-shared key
    Aes256CtrPsk,
    Aes512IegEcdh,
    Aes512IegPsk,
}

impl std::str::FromStr for WhiteflagEncryptionMethod {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.parse::<usize>().unwrap();
        Self::from_number(n)
    }
}

impl WhiteflagEncryptionMethod {
    pub fn from_number(number: usize) -> CryptoResult<Self> {
        let method = match number {
            0 => Self::NoEncryption,

            1 => Self::Aes256CtrEcdh,
            2 => Self::Aes256CtrPsk,

            3 => Self::Aes512IegEcdh,
            4 => Self::Aes512IegPsk,
            _ => return Err(CryptoError::InvalidMethod),
        };

        Ok(method)
    }

    pub fn alg(&self) -> EncryptionAlgorithm {
        match self {
            Self::NoEncryption => NO_ENCRYPTION,
            Self::Aes256CtrEcdh => AES_256_CTR_ECDH,
            Self::Aes256CtrPsk => AES_256_CTR_PSK,
            Self::Aes512IegEcdh => AES_512_IEG_ECDH,
            Self::Aes512IegPsk => AES_512_IEG_PSK,
        }
    }
}

const NO_ENCRYPTION: EncryptionAlgorithm = EncryptionAlgorithm {
    field_value: 0,
    algorithm_name: "NONE",
    operation_mode: "NONE",
    padding_scheme: "NoPadding",
    key_length: 0,
    hkdf_salt: "",
};

const AES_256_CTR_ECDH: EncryptionAlgorithm = EncryptionAlgorithm {
    field_value: 1,
    algorithm_name: "AES",
    operation_mode: "CTR",
    padding_scheme: "NoPadding",
    key_length: 32,
    hkdf_salt: "8ddb03085a2c15e69c35c224bce2952dca7878770724741cbce5a135328be0c0",
};

const AES_256_CTR_PSK: EncryptionAlgorithm = EncryptionAlgorithm {
    field_value: 2,
    algorithm_name: "AES",
    operation_mode: "CTR",
    padding_scheme: "NoPadding",
    key_length: 32,
    hkdf_salt: "c4d028bd45c876135e80ef7889835822a6f19a31835557d5854d1334e8497b56",
};

const AES_512_IEG_ECDH: EncryptionAlgorithm = EncryptionAlgorithm {
    field_value: 3,
    algorithm_name: "AES",
    operation_mode: "IEG",
    padding_scheme: "x16",
    key_length: 32,
    hkdf_salt: "",
};

const AES_512_IEG_PSK: EncryptionAlgorithm = EncryptionAlgorithm {
    field_value: 4,
    algorithm_name: "AES",
    operation_mode: "IEG",
    padding_scheme: "x16",
    key_length: 32,
    hkdf_salt: "",
};
