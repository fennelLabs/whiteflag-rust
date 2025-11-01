//! Whiteflag ECDH Key Pair - brainpoolP256r1 implementation
//!
//! Implements RFC 5639 brainpoolP256r1 curve as mandated by
//! Whiteflag Protocol Specification v1 Section 5.2.2

use aes_tools::AESCipher;
use bp256::r1::{
    AffinePoint as BrainpoolAffine,
    NonZeroScalar as BrainpoolNonZeroScalar,
    ProjectivePoint as BrainpoolProjective,
    SecretKey as BrainpoolSecretKey,
    BrainpoolP256r1,
};
use elliptic_curve::{
    sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint},
};
use hkdf::Hkdf;
use rand_core::{OsRng, TryRngCore};
use sha2::Sha256;
use zeroize::ZeroizeOnDrop;

/// Whiteflag ECDH Key Pair class
///
/// Implements brainpoolP256r1 as specified in:
/// - Whiteflag Specification 5.2.2 Key Agreement
/// - RFC 5639 Section 4.2.2
#[derive(ZeroizeOnDrop, Clone)]
pub struct WhiteflagECDHKeyPair {
    #[zeroize(skip)]
    secret_scalar: BrainpoolNonZeroScalar,
    #[zeroize(skip)]
    public_affine: BrainpoolAffine,
}

impl AsRef<[u8]> for WhiteflagECDHKeyPair {
    fn as_ref(&self) -> &[u8] {
        // Return compressed public key bytes (leaks memory to maintain trait)
        Box::leak(Box::new(self.public_key_compressed())).as_slice()
    }
}

impl WhiteflagECDHKeyPair {
    /// Creates a new random ECDH key pair using brainpoolP256r1
    pub fn new() -> Self {
        let secret_key = BrainpoolSecretKey::random(&mut OsRng.unwrap_mut());
        let secret_scalar = secret_key.to_nonzero_scalar();
        
        let public_projective = BrainpoolProjective::GENERATOR * secret_scalar.as_ref();
        let public_affine = public_projective.to_affine();
        
        WhiteflagECDHKeyPair {
            secret_scalar,
            public_affine,
        }
    }

    /// Creates an ECDH key pair from existing 32-byte secret key
    pub fn from_bytes(secret_bytes: &[u8; 32]) -> Self {
        let secret_key = BrainpoolSecretKey::from_bytes(secret_bytes.into())
            .expect("Invalid secret key bytes");
        let secret_scalar = secret_key.to_nonzero_scalar();
        
        let public_projective = BrainpoolProjective::GENERATOR * secret_scalar.as_ref();
        let public_affine = public_projective.to_affine();
        
        WhiteflagECDHKeyPair {
            secret_scalar,
            public_affine,
        }
    }
    
    /// Get SEC1 compressed public key (33 bytes)
    pub fn public_key_compressed(&self) -> Vec<u8> {
        let encoded = self.public_affine.to_encoded_point(true);
        encoded.as_bytes().to_vec()
    }
    
    /// Get public key as hexadecimal string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key_compressed())
    }

    /// Calculates the negotiated shared secret with a peer
    pub fn negotiate(&self, peer_public_bytes: &[u8]) -> Vec<u8> {
        self.negotiate_internal(peer_public_bytes)
            .expect("ECDH negotiation failed")
    }
    
    fn negotiate_internal(&self, peer_public_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let peer_encoded = EncodedPoint::<BrainpoolP256r1>::from_bytes(peer_public_bytes)
            .map_err(|e| format!("Invalid SEC1 encoding: {:?}", e))?;
        
        let peer_affine = Option::<BrainpoolAffine>::from(
            BrainpoolAffine::from_encoded_point(&peer_encoded)
        ).ok_or_else(|| "Peer public key not on curve".to_string())?;
        
        let peer_projective = BrainpoolProjective::from(peer_affine);
        let shared_projective = peer_projective * self.secret_scalar.as_ref();
        let shared_affine = shared_projective.to_affine();
        
        let shared_uncompressed = shared_affine.to_encoded_point(false);
        let uncompressed_bytes = shared_uncompressed.as_bytes();
        
        let mut x_coord = vec![0u8; 32];
        x_coord.copy_from_slice(&uncompressed_bytes[1..33]);
        
        Ok(x_coord)
    }
    
    fn derive_aes256_key(&self, peer_public_bytes: &[u8]) -> Result<[u8; 32], String> {
        let shared_secret = self.negotiate_internal(peer_public_bytes)?;
        
        let salt = b"whiteflag-ecdh-v1";
        let info = b"wf-aes256-ctr";
        
        let hkdf = Hkdf::<Sha256>::new(Some(salt), &shared_secret);
        let mut output_key_material = [0u8; 32];
        hkdf.expand(info, &mut output_key_material)
            .map_err(|e| format!("HKDF expansion failed: {:?}", e))?;
        
        Ok(output_key_material)
    }

    pub fn create_aes_cipher(&self, peer_public_bytes: &[u8]) -> AESCipher {
        let aes_key = self.derive_aes256_key(peer_public_bytes)
            .expect("Failed to derive AES key from ECDH");
        AESCipher::new_from_shared_secret(&aes_key)
    }
}

impl Default for WhiteflagECDHKeyPair {
    fn default() -> Self {
        Self::new()
    }
}
