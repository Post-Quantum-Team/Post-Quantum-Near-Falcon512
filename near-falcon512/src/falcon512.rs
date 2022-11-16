/*!
falcon-512

These bindings use the clean version from [PQClean][pqc]

# Example

use pqcrypto_falcon::falcon512::*;
let message = vec![0, 1, 2, 3, 4, 5];
let (pk, sk) = keypair();
let sm = sign(&message, &sk);
let verifiedmsg = open(&sm, &pk).unwrap();
assert!(verifiedmsg == message);


[pqc]: https://github.com/pqclean/pqclean/
*/

// This file is generated.

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialization")]
use serde_big_array::BigArray;

use crate::ffi::{self, shake256_init_prng_from_seed, Shake256Context, shake256_init_prng_from_system, falcon_make_public};
use pqcrypto_traits::sign as primitive;
use pqcrypto_traits::{Error, Result};

macro_rules! simple_struct {
    ($type: ident, $size: expr) => {
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
        pub struct $type(
            #[cfg_attr(feature = "serialization", serde(with = "BigArray"))] [u8; $size],
        );

        impl $type {
            /// Generates an uninitialized object
            ///
            /// Used to pass to ``ffi`` interfaces.
            ///
            /// Internal use only!
            fn new() -> Self {
                $type([0u8; $size])
            }
        }

        impl primitive::$type for $type {
            /// Get this object as a byte slice
            #[inline]
            fn as_bytes(&self) -> &[u8] {
                &self.0
            }

            /// Construct this object from a byte slice
            fn from_bytes(bytes: &[u8]) -> Result<Self> {
                if bytes.len() != $size {
                    Err(Error::BadLength {
                        name: stringify!($type),
                        actual: bytes.len(),
                        expected: $size,
                    })
                } else {
                    let mut array = [0u8; $size];
                    array.copy_from_slice(bytes);
                    Ok($type(array))
                }
            }
        }

        impl PartialEq for $type {
            /// By no means constant time comparison
            fn eq(&self, other: &Self) -> bool {
                self.0
                    .iter()
                    .zip(other.0.iter())
                    .try_for_each(|(a, b)| if a == b { Ok(()) } else { Err(()) })
                    .is_ok()
            }
        }
    };
}


simple_struct!(
    PublicKey,
    ffi::NEAR_FALCON512_PUBKEY_SIZE
);
simple_struct!(
    SecretKey,
    ffi::NEAR_FALCON512_PRIVKEY_SIZE
);

#[derive(Clone, Copy, Hash)]
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


/// Create a detached signature on the message
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
    use std::vec::Vec;

    #[test]
    pub fn test_sign_detached() {
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let (pk, sk) = keypair();
        let sig = detached_sign(&message, &sk);
        assert!(verify_detached_signature(&sig, &message, &pk).is_ok());
        assert!(!verify_detached_signature(&sig, &message[..message.len() - 1], &pk).is_ok());
    }

    #[test]
    pub fn test_sign_detached_seed() {
        let mut rng = rand::thread_rng();
        let len: u16 = rng.gen();
        let message = (0..len).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let seed = "Crypto is NEAR !";
        let (pk, sk) = keypair_from_seed(seed.as_bytes());
        let sig = detached_sign(&message, &sk);
        assert!(verify_detached_signature(&sig, &message, &pk).is_ok());
        assert!(!verify_detached_signature(&sig, &message[..message.len() - 1], &pk).is_ok());
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
    pub fn test_random_keygen() {
        let (pk, sk) = keypair();
        let (pk2, sk2) = keypair();
        assert_ne!(pk, pk2);
        assert_ne!(sk, sk2);
    }
}
