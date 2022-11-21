/*!
falcon-512

These bindings use the falcon-c-near FFI functions.

*/

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialization")]
use serde_big_array::BigArray;

use crate::ffi::{self, shake256_init_prng_from_seed, Shake256Context, shake256_init_prng_from_system, falcon_make_public};
use pqcrypto_traits::sign as primitive;
use pqcrypto_traits::{Error, Result};



///SecretKey Structure
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct SecretKey (
    #[cfg_attr(feature = "serialization", serde(with = "BigArray"))] [u8; ffi::NEAR_FALCON512_PRIVKEY_SIZE],
);

impl SecretKey {
    /// Generates an uninitialized object
    ///
    /// Used to pass to ``ffi`` interfaces.
    ///
    /// Internal use only!
    fn new() -> Self {
        SecretKey([0u8; ffi::NEAR_FALCON512_PRIVKEY_SIZE])
    }
}

impl primitive::SecretKey for SecretKey {
    /// Get this object as a byte slice
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Construct this object from a byte slice
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != ffi::NEAR_FALCON512_PRIVKEY_SIZE {
            Err(Error::BadLength {
                name: stringify!(SecretKey),
                actual: bytes.len(),
                expected: ffi::NEAR_FALCON512_PRIVKEY_SIZE,
            })
        } else {
            let mut array = [0u8; ffi::NEAR_FALCON512_PRIVKEY_SIZE];
            array.copy_from_slice(bytes);
            Ok(SecretKey(array))
        }
    }
}

impl From<SecretKey> for [u8; ffi::NEAR_FALCON512_PRIVKEY_SIZE] {
    fn from(secret_key: SecretKey) -> Self {
        secret_key.0
    }
}

impl PartialEq for SecretKey {
    /// By no means constant time comparison
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .try_for_each(|(a, b)| if a == b { Ok(()) } else { Err(()) })
            .is_ok()
    }
}



///PublicKey Structure
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct PublicKey (
    #[cfg_attr(feature = "serialization", serde(with = "BigArray"))] [u8; ffi::NEAR_FALCON512_PUBKEY_SIZE],
);

impl PublicKey {
    fn new() -> Self {
        PublicKey([0u8; ffi::NEAR_FALCON512_PUBKEY_SIZE])
    }
}

impl primitive::PublicKey for PublicKey {
    /// Get this object as a byte slice
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Construct this object from a byte slice
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != ffi::NEAR_FALCON512_PUBKEY_SIZE {
            Err(Error::BadLength {
                name: stringify!(PublicKey),
                actual: bytes.len(),
                expected: ffi::NEAR_FALCON512_PUBKEY_SIZE,
            })
        } else {
            let mut array = [0u8; ffi::NEAR_FALCON512_PUBKEY_SIZE];
            array.copy_from_slice(bytes);
            Ok(PublicKey(array))
        }
    }
}

impl From<PublicKey> for [u8; ffi::NEAR_FALCON512_PUBKEY_SIZE] {
    fn from(pubkey: PublicKey) -> Self {
        pubkey.0
    }
}

impl PartialEq for PublicKey {
    /// By no means constant time comparison
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .try_for_each(|(a, b)| if a == b { Ok(()) } else { Err(()) })
            .is_ok()
    }
}

// Digital Signature structure
#[derive(Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetachedSignature(
    #[cfg_attr(feature = "serialization", serde(with = "BigArray"))]
    [u8; ffi::NEAR_FALCON512_SIG_PADDED_SIZE],
    usize,
);

// for internal use
impl DetachedSignature {
    fn new() -> Self {
        DetachedSignature([0u8; ffi::NEAR_FALCON512_SIG_PADDED_SIZE], 0)
    }
}

impl primitive::DetachedSignature for DetachedSignature {
    /// Get this object as a byte slice
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.0[..self.1]
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let actual = bytes.len();
        let expected = ffi::NEAR_FALCON512_SIG_PADDED_SIZE;
        if actual > expected {
            return Err(Error::BadLength {
                name: "DetachedSignature",
                actual,
                expected,
            });
        }
        let mut array = [0u8; ffi::NEAR_FALCON512_SIG_PADDED_SIZE];
        array[..bytes.len()].copy_from_slice(bytes);
        Ok(DetachedSignature(array, actual))
    }
}

impl From<DetachedSignature> for [u8; ffi::NEAR_FALCON512_SIG_PADDED_SIZE] {
    fn from(sig: DetachedSignature) -> Self {
        sig.0
    }
}


/// Get the number of bytes for a public key
pub const fn public_key_bytes() -> usize {
    ffi::NEAR_FALCON512_PUBKEY_SIZE
}

/// Get the number of bytes for a secret key
pub const fn secret_key_bytes() -> usize {
    ffi::NEAR_FALCON512_PRIVKEY_SIZE
}

/// Get the number of bytes that a signature occupies
pub const fn signature_bytes() -> usize {
    ffi::NEAR_FALCON512_SIG_PADDED_SIZE
}

/// Initialize a Shake256Context from a seed parameter
pub fn generator_from_seed(sc: &mut Shake256Context, seed: &[u8]) {
    unsafe {
        let seed_len = seed.len();
        shake256_init_prng_from_seed(sc.0.as_mut_ptr(), seed.as_ptr(), seed_len);
    }
}

/// Initialize a Shake256Context from the OS Random Generator
pub fn generator_from_system_prng(sc: &mut Shake256Context) {
    unsafe {
        shake256_init_prng_from_system(sc.0.as_mut_ptr());
    }
}


/// Generate a falcon-512 keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    let mut pk = PublicKey::new();
    let mut sk = SecretKey::new();
    let mut tmp_keygen = [0u8; ffi::NEAR_FALCON512_TMPSIZE_KEYGEN];
    let mut sc = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
    generator_from_system_prng(&mut sc);
    unsafe {
        assert_eq!(
        ffi::falcon_keygen_make(sc.0.as_ptr(),
        ffi::NEAR_FALCON_DEGREE, sk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PRIVKEY_SIZE, pk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(),
        ffi::NEAR_FALCON512_TMPSIZE_KEYGEN),
        0); };
    (pk, sk)
}

/// Generate a falcon-512 keypair from a seed
pub fn keypair_from_seed(seed: &[u8]) -> (PublicKey, SecretKey) {
    let mut pk = PublicKey::new();
    let mut sk = SecretKey::new();
    let mut tmp_keygen = [0u8; ffi::NEAR_FALCON512_TMPSIZE_KEYGEN];
    let mut sc = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
    generator_from_seed(&mut sc, seed);
    unsafe {
        assert_eq!(
        ffi::falcon_keygen_make(sc.0.as_ptr(),
        ffi::NEAR_FALCON_DEGREE, sk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PRIVKEY_SIZE, pk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(),
        ffi::NEAR_FALCON512_TMPSIZE_KEYGEN),
        0); };
    (pk, sk)
}

/// Generate a falcon-512 keypair from a Shake256Context
pub fn keypair_from_shake256context(sc: Shake256Context) -> (PublicKey, SecretKey) {
    let mut pk = PublicKey::new();
    let mut sk = SecretKey::new();
    let mut tmp_keygen = [0u8; ffi::NEAR_FALCON512_TMPSIZE_KEYGEN];
    unsafe {
        assert_eq!(
        ffi::falcon_keygen_make(sc.0.as_ptr(),
        ffi::NEAR_FALCON_DEGREE, sk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PRIVKEY_SIZE, pk.0.as_mut_ptr(),
        ffi::NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(),
        ffi::NEAR_FALCON512_TMPSIZE_KEYGEN),
        0); };
    (pk, sk)
}

/// Generate public key from secret key
pub fn public_key_from_secret_key(sk: SecretKey) -> PublicKey {

    let mut pk = PublicKey::new();
    let mut tmp_makepub = [0u8; ffi::NEAR_FALCON512_TMPSIZE_MAKEPUB];
    unsafe{
        assert_eq!(falcon_make_public(pk.0.as_mut_ptr(), ffi::NEAR_FALCON512_PUBKEY_SIZE, sk.0.as_ptr(), ffi::NEAR_FALCON512_PRIVKEY_SIZE, tmp_makepub.as_mut_ptr(), ffi::NEAR_FALCON512_TMPSIZE_MAKEPUB),
            0);
        }
    pk
}


/// Create a detached signature on the message, using random as Shake256Context input
pub fn detached_sign(msg: &[u8], sk: &SecretKey) -> DetachedSignature {
    let mut sig = DetachedSignature::new();
    let sig_len = ffi::NEAR_FALCON512_SIG_PADDED_SIZE;
    let mut tmp_signdyn = [0u8; ffi::NEAR_FALCON512_TMPSIZE_SIGNDYN];
    let mut sc = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
    generator_from_system_prng(&mut sc);
    unsafe {
        ffi::falcon_sign_dyn(
            sc.0.as_ptr(),
            sig.0.as_mut_ptr(),
            &sig_len,
            ffi::FALCON_SIG_PADDED,
            sk.0.as_ptr(),
            ffi::NEAR_FALCON512_PRIVKEY_SIZE,
            msg.as_ptr(),
            msg.len(),
            tmp_signdyn.as_mut_ptr(),
            ffi::NEAR_FALCON512_TMPSIZE_SIGNDYN);
    }
    sig.1 = sig.0.len();
    sig
}

/// Create a detached signature on the message, using the seed as Shake256Context input
pub fn detached_sign_with_seed(msg: &[u8], sk: &SecretKey, seed: &[u8]) -> DetachedSignature {
    let mut sig = DetachedSignature::new();
    let sig_len = ffi::NEAR_FALCON512_SIG_PADDED_SIZE;
    let mut tmp_signdyn = [0u8; ffi::NEAR_FALCON512_TMPSIZE_SIGNDYN];
    let mut sc = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
    generator_from_seed(&mut sc, seed);
    unsafe {
        ffi::falcon_sign_dyn(
            sc.0.as_ptr(),
            sig.0.as_mut_ptr(),
            &sig_len,
            ffi::FALCON_SIG_PADDED,
            sk.0.as_ptr(),
            ffi::NEAR_FALCON512_PRIVKEY_SIZE,
            msg.as_ptr(),
            msg.len(),
            tmp_signdyn.as_mut_ptr(),
            ffi::NEAR_FALCON512_TMPSIZE_SIGNDYN);
    }
    sig.1 = sig.0.len();
    sig
}



/// Verify the detached signature
pub fn verify_detached_signature(
    sig: &DetachedSignature,
    msg: &[u8],
    pk: &PublicKey,
) -> core::result::Result<(), primitive::VerificationError> {
    let mut tmp_verify = [0u8; ffi::NEAR_FALCON512_TMPSIZE_VERIFY];
        let res = unsafe {
                ffi::falcon_verify(
                sig.0.as_ptr(),
                ffi::NEAR_FALCON512_SIG_PADDED_SIZE,
                ffi::FALCON_SIG_PADDED,
                pk.0.as_ptr(),
                ffi::NEAR_FALCON512_PUBKEY_SIZE,
                msg.as_ptr(),
                msg.len(),
                tmp_verify.as_mut_ptr(),
                ffi::NEAR_FALCON512_TMPSIZE_VERIFY
            )
        };
        match res {
            0 => Ok(()),
            -4 => Err(primitive::VerificationError::InvalidSignature),
            _ => Err(primitive::VerificationError::UnknownVerificationError),
        }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use std::{vec::Vec};

    #[test]
    pub fn test_shake256_generator_from_seed() {
        let mut sc = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
        let seed = [12u8; 37];
        generator_from_seed(&mut sc, &seed);

        let mut expected_result: [u64; ffi::SHAKE256_CONTEXT_SIZE] =
        [
            868082074056920076,868082074056920076,868082074056920076,
            868082074056920076,34136602184716,0,0,0,0,0,0,0,0,0,0,0,
            9223372036854775808,0,0,0,0,0,0,0,0,136,
        ];
        assert_eq!(expected_result, sc.0);
        expected_result[1] = 124;
        assert_ne!(expected_result, sc.0);
    }

    #[test]
    pub fn test_shake256_generator_from_random() {
        let mut sc1 = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
        let mut sc2 = Shake256Context([0u64; ffi::SHAKE256_CONTEXT_SIZE]);
        generator_from_system_prng(&mut sc1);
        generator_from_system_prng(&mut sc2);

        assert_ne!(sc1, sc2);
    }

    #[test]
    pub fn test_sign_falcon() {
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let seed1 = "Crypto is NEAR !";

        let (pk1, sk1) = keypair_from_seed(seed1.as_bytes());
        let (_, sk2) = keypair();
        let pk2 = public_key_from_secret_key(sk2);
        let sig1 = detached_sign(&message, &sk1);
        let sig2 = detached_sign(&message, &sk2);
        assert!(verify_detached_signature(&sig1, &message, &pk1).is_ok());
        assert!(verify_detached_signature(&sig2, &message, &pk2).is_ok());
        assert!(!verify_detached_signature(&sig2, &message, &pk1).is_ok());
        assert!(!verify_detached_signature(&sig1, &message, &pk2).is_ok());
        assert!(!verify_detached_signature(&sig1, &message[..message.len() - 1], &pk1).is_ok());
        assert!(!verify_detached_signature(&sig2, &message[..message.len() - 1], &pk2).is_ok());
    }

    #[test]
    pub fn test_sign_detached_shake() {
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let sc = Shake256Context([3u64; ffi::SHAKE256_CONTEXT_SIZE]);
        let (pk, sk) = keypair_from_shake256context(sc);
        let sig = detached_sign(&message, &sk);
        assert!(verify_detached_signature(&sig, &message, &pk).is_ok());
        assert!(!verify_detached_signature(&sig, &message[..message.len() - 1], &pk).is_ok());
    }

    #[test]
    pub fn test_sign_detached_seed() {
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let (pk, sk) = keypair();
        let seed_str = [1u8; 12];
        let sig1 = detached_sign(&message, &sk);
        let sig2 = detached_sign_with_seed(&message, &sk, &seed_str);
        assert!(verify_detached_signature(&sig1, &message, &pk).is_ok());
        assert!(verify_detached_signature(&sig2, &message, &pk).is_ok());
        assert!(!verify_detached_signature(&sig1, &message[..message.len() - 1], &pk).is_ok());
        assert!(!verify_detached_signature(&sig2, &message[..message.len() - 1], &pk).is_ok());
        assert_ne!(sig1.0, sig2.0);
    }

    #[test]
    pub fn test_random_keygen() {
        let (pk, sk) = keypair();
        let (pk2, sk2) = keypair();
        assert_ne!(pk, pk2);
        assert_ne!(sk, sk2);
    }

    #[test]
    pub fn test_from_trait() {
        let (pk, sk) = keypair();
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();
        let sig = detached_sign(&message, &sk);

        let sk_array = <[u8; ffi::NEAR_FALCON512_PRIVKEY_SIZE]>::from(sk);
        let pk_array = <[u8; ffi::NEAR_FALCON512_PUBKEY_SIZE]>::from(pk);
        let sig_array = <[u8; ffi::NEAR_FALCON512_SIG_PADDED_SIZE]>::from(sig);
        assert_eq!(sk_array, sk.0);
        assert_eq!(pk_array, pk.0);
        assert_eq!(sig_array, sig.0);
    }
}
