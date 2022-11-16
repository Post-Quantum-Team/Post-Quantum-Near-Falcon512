//! Foreign function interfaces
//!
//! This module defines the foreign function interface for the following
//! crypto implementations from Near:
//!
//!  * falcon-512
// This file has been generated from falcon512-c-near.

use libc::c_int;

// ensures we link correctly
#[allow(unused_imports)]
use pqcrypto_internals::*;

// Falcon Error Codes
pub const FALCON_ERR_RANDOM: isize = -1;
pub const FALCON_ERR_SIZE: isize = -2;
pub const FALCON_ERR_FORMAT: isize = -3;
pub const FALCON_ERR_BADSIG: isize = -4;
pub const FALCON_ERR_BADARG: isize = -5;
pub const FALCON_ERR_INTERNAL: isize = -6;

// Falcon Signature Formats
pub const FALCON_SIG_COMPRESSED: usize = 1;
pub const FALCON_SIG_PADDED: usize = 2;
pub const FALCON_SIG_CT: usize = 3;

// Falcon512 degree
pub const NEAR_FALCON_DEGREE: usize = 9;

// Shake256 array size
pub const SHAKE256_CONTEXT_SIZE: usize = 26;

// Sizes used for Falcon-512 C implementation
pub const NEAR_FALCON512_PRIVKEY_SIZE: usize = 1281;
pub const NEAR_FALCON512_PUBKEY_SIZE: usize = 897;
pub const NEAR_FALCON512_SIG_COMPRESSED_MAXSIZE: usize = 752;
pub const NEAR_FALCON512_SIG_PADDED_SIZE: usize = 666;
pub const NEAR_FALCON512_SIG_CT_SIZE: usize = 809;
pub const NEAR_FALCON512_TMPSIZE_KEYGEN: usize = 15879;
pub const NEAR_FALCON512_TMPSIZE_MAKEPUB: usize = 3073;
pub const NEAR_FALCON512_TMPSIZE_SIGNDYN: usize = 39943;
pub const NEAR_FALCON512_TMPSIZE_VERIFY: usize = 4097;

#[derive(Clone, Copy)]
pub struct Shake256Context (pub [u64; SHAKE256_CONTEXT_SIZE]);

#[link(name = "falcon-512")]
extern "C" {
    // Get size from C implementation
    pub fn falcon_privkey_size(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_pubkey_size(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_sig_compressed_maxsize(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_sig_padded_size(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_sig_ct_size(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_tmpsize_keygen(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_tmpsize_makepub(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_tmpsize_signdyn(NEAR_FALCON_DEGREE: usize) -> c_int;
    pub fn falcon_tmpsize_verify(NEAR_FALCON_DEGREE: usize) -> c_int;

    // SHAKE256 implementation
    pub fn shake256_init(sc: *mut u64);
    pub fn shake256_inject(sc: *mut u64, data: *const u8, len: usize);
    pub fn shake256_flip(sc: *mut u64);
    pub fn shake256_extract(sc: *mut u64, out: *const u8, len: usize);
    pub fn shake256_init_prng_from_seed(sc: *mut u64, seed: *const u8, seed_len: usize);
    pub fn shake256_init_prng_from_system(sc: *mut u64) -> c_int;

    // KeyPair Generation
    pub fn falcon_keygen_make(rng: *const u64, logn: usize, privkey: *mut u8, privkey_len: usize, pubkey: *mut u8, pubkey_len: usize, tmp: *mut u8, tmp_len: usize) -> c_int;
    pub fn falcon_make_public(pubkey: *const u8, pubkey_len: usize, privkey: *const u8, privkey_len: usize, tmp: *mut u8, tmp_len: usize) -> c_int;

    // Signature Generation
    pub fn falcon_sign_dyn(rng: *const u64, sig: *mut u8, sig_len: *const usize, sig_type: usize, privkey: *const u8, privkey_len: usize, data: *const u8, data_len: usize, tmp: *mut u8, tmp_len: usize) -> c_int;
    
    // Signature verification
    pub fn falcon_verify(sig: *const u8, sig_len: usize, sig_type: usize, pubkey: *const u8, pubkey_len: usize, data: *const u8, data_len: usize, tmp: *const u8, tmp_len: usize) -> c_int;
}



#[cfg(test)]
mod test_falcon512_clean {
    use std::dbg;

    use super::*;

    #[test]
    fn test_ffi() {
        let mut test = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        unsafe { shake256_init(test.0.as_mut_ptr()) }
        let seed = "ceci est la seed";
        let seed_size = seed.len();
        unsafe {
            shake256_init_prng_from_seed(test.0.as_mut_ptr(), seed.as_ptr(), seed_size);
        }
        let mut output = [0u8; 128];
        unsafe {
            shake256_extract(test.0.as_mut_ptr(), output.as_mut_ptr(), 128);
        }

        // Initialize Keypair elements
        let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut pk2 = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut sk = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut sk2 = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut sig = [0u8; NEAR_FALCON512_SIG_PADDED_SIZE];
        let mut tmp_keygen = [0u8; NEAR_FALCON512_TMPSIZE_KEYGEN];
        let mut tmp_makepub = [0u8; NEAR_FALCON512_TMPSIZE_MAKEPUB];
        let mut tmp_signdyn = [0u8; NEAR_FALCON512_TMPSIZE_SIGNDYN];
        let tmp_verify = [0u8; NEAR_FALCON512_TMPSIZE_VERIFY];

        unsafe{
            let mut test = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
            shake256_init_prng_from_system(test.0.as_mut_ptr());
            falcon_keygen_make(test.0.as_ptr(), NEAR_FALCON_DEGREE, sk.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN);
            dbg!("{}", pk);

            let mut test = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
            shake256_init_prng_from_system(test.0.as_mut_ptr());
            falcon_keygen_make(test.0.as_ptr(), NEAR_FALCON_DEGREE, sk2.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk2.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN);
            dbg!("{}", pk2);
            // Test prng
            assert_ne!(pk, pk2);
            // regenerate pubkey
            let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
            falcon_make_public(pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, tmp_makepub.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_MAKEPUB);

            shake256_init_prng_from_system(test.0.as_mut_ptr());

            let text = "Bonjour ceci est un petit test";
            let text_size = text.len();
            let sig_len = NEAR_FALCON512_SIG_PADDED_SIZE;
            falcon_sign_dyn(test.0.as_ptr(), sig.as_mut_ptr(), &sig_len, FALCON_SIG_PADDED, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, text.as_ptr(), text_size, tmp_signdyn.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_SIGNDYN);

            falcon_verify(sig.as_ptr(), sig_len, FALCON_SIG_PADDED, pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE, text.as_ptr(), text_size, tmp_verify.as_ptr(), NEAR_FALCON512_TMPSIZE_VERIFY);

        } ;
    }
}
