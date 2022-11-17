//! Foreign function interfaces
//!
//! This module defines the foreign function interface for the following
//! crypto implementations from Near:
//!  * falcon-512
//!
//! This file has been generated from falcon-c-near.
//! The ffi functions could be used to use Falcon-1024 too.
//! To use Falcon-1024, change the NEAR_FALCON_DEGREE constant from 9 to 10.
//! WARNING : if the NEAR_FALCON_DEGREE, the NEAR_FALCON512_* constants should not be used anymore

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shake256Context (pub [u64; SHAKE256_CONTEXT_SIZE]);

#[link(name = "falcon-512")]
extern "C" {
    // Get size from C implementation (allow verification of the Rust defined constants)
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
    pub fn falcon_get_logn(object: *const u8, object_len: usize) -> c_int;

    // Signature Generation
    pub fn falcon_sign_dyn(rng: *const u64, sig: *mut u8, sig_len: *const usize, sig_type: usize, privkey: *const u8, privkey_len: usize, data: *const u8, data_len: usize, tmp: *mut u8, tmp_len: usize) -> c_int;
    
    // Signature verification
    pub fn falcon_verify(sig: *const u8, sig_len: usize, sig_type: usize, pubkey: *const u8, pubkey_len: usize, data: *const u8, data_len: usize, tmp: *const u8, tmp_len: usize) -> c_int;
}



#[cfg(test)]
mod test_falcon512 {
    use hex::encode as hexencode;

    use super::*;

    #[test]
    fn test_shake256() {
        unsafe {
            let mut sc = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
            shake256_init(sc.0.as_mut_ptr());

            let injected_data = "This string will be injected in shake256_inject";
            let len_injected_data = injected_data.len();
            shake256_inject(sc.0.as_mut_ptr(), injected_data.as_ptr(), len_injected_data);

            shake256_flip(sc.0.as_mut_ptr());

            let mut extracted_data = [0u8; 64];
            let len_extracted_data = extracted_data.len();
            shake256_extract(sc.0.as_mut_ptr(), extracted_data.as_mut_ptr(), len_extracted_data);
            assert_eq!(hexencode(extracted_data), "bdfd07908ee0916269975c0200c7af08c5b5503fc4a3e3e978a0d44f24abd4592b955e526d0eb5d84320b9c7a986e9dfcd9b20aeb5badc262de1c785f2836640");
        };
    }

    #[test]
    fn test_shake256_from_seed() {
        let mut sc = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        let seed = "This string will be injected in shake256_inject";
        let len_seed = seed.len();
        unsafe {
            shake256_init_prng_from_seed(sc.0.as_mut_ptr(), seed.as_ptr(), len_seed);
            let mut extracted_data = [0u8; 64];
            let len_extracted_data = extracted_data.len();
            shake256_extract(sc.0.as_mut_ptr(), extracted_data.as_mut_ptr(), len_extracted_data);
            assert_eq!(hexencode(extracted_data), "bdfd07908ee0916269975c0200c7af08c5b5503fc4a3e3e978a0d44f24abd4592b955e526d0eb5d84320b9c7a986e9dfcd9b20aeb5badc262de1c785f2836640");            
        }
        
    }

    #[test]
    fn test_shake256_from_os_prng() {
        let mut sc1 = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        let mut sc2 = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        unsafe {
            assert_eq!(
                0,
                shake256_init_prng_from_system(sc1.0.as_mut_ptr())
            );
            assert_eq!(
                0,
                shake256_init_prng_from_system(sc2.0.as_mut_ptr())
            );
        }
        assert_ne!(sc1, sc2);
    }

    #[test]
    fn test_falcon512_keygen_from_os_prng() {
        // Initialize Keypair elements
        let mut pk1 = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut pk2 = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut sk1 = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut sk2 = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut tmp_keygen = [0u8; NEAR_FALCON512_TMPSIZE_KEYGEN];

        let mut sc1 = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        let mut sc2 = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);

        unsafe {
            assert_eq!(
                0,
                shake256_init_prng_from_system(sc1.0.as_mut_ptr())
            );
            assert_eq!(
                0,
                falcon_keygen_make(sc1.0.as_ptr(), NEAR_FALCON_DEGREE, sk1.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk1.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN)
            );
            assert_eq!(
                0,
                shake256_init_prng_from_system(sc2.0.as_mut_ptr())
            );
            assert_eq!(
                0,
                falcon_keygen_make(sc2.0.as_ptr(), NEAR_FALCON_DEGREE, sk2.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk2.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN)
            );
        }
        
        assert_ne!(sc1, sc2);
        assert_ne!(pk1, pk2);
    }

    #[test]
    fn test_falcon512_keygen_from_seed() {
        //Shake256 part
        let mut sc = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);
        let seed = "This string will be injected in shake256_inject";
        let len_seed = seed.len();
        unsafe {
            shake256_init_prng_from_seed(sc.0.as_mut_ptr(), seed.as_ptr(), len_seed);
            let mut extracted_data = [0u8; 64];
            let len_extracted_data = extracted_data.len();
            shake256_extract(sc.0.as_mut_ptr(), extracted_data.as_mut_ptr(), len_extracted_data);
            assert_eq!(hexencode(extracted_data), "bdfd07908ee0916269975c0200c7af08c5b5503fc4a3e3e978a0d44f24abd4592b955e526d0eb5d84320b9c7a986e9dfcd9b20aeb5badc262de1c785f2836640");            
        }

        // Keygen part
        let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut sk = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut tmp_keygen = [0u8; NEAR_FALCON512_TMPSIZE_KEYGEN];
        let mut tmp_makepub = [0u8; NEAR_FALCON512_TMPSIZE_MAKEPUB];
        unsafe {
            assert_eq!(
                0,
                falcon_keygen_make(sc.0.as_ptr(), NEAR_FALCON_DEGREE, sk.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN)
            );
            let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
            assert_eq!(
                0,
                falcon_make_public(pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, tmp_makepub.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_MAKEPUB)
            );
            assert_eq!(
                NEAR_FALCON_DEGREE as i32,
                falcon_get_logn(pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE)
            );

        }
    }

    #[test]
    fn test_falcon512_signature() {
        // Initialize Keypair elements
        let mut pk = [0u8; NEAR_FALCON512_PUBKEY_SIZE];
        let mut sk = [0u8; NEAR_FALCON512_PRIVKEY_SIZE];
        let mut sig = [0u8; NEAR_FALCON512_SIG_PADDED_SIZE];
        let mut tmp_keygen = [0u8; NEAR_FALCON512_TMPSIZE_KEYGEN];
        let mut tmp_signdyn = [0u8; NEAR_FALCON512_TMPSIZE_SIGNDYN];
        let mut tmp_verify = [0u8; NEAR_FALCON512_TMPSIZE_VERIFY];

        let mut sc = Shake256Context([0u64; SHAKE256_CONTEXT_SIZE]);

        let sig_size = NEAR_FALCON512_SIG_PADDED_SIZE;
        let data_to_sign = "Hello World !";
        let len_data_to_sign = data_to_sign.len();

        unsafe{
            shake256_init_prng_from_system(sc.0.as_mut_ptr());
            assert_eq!(
                0,
                falcon_keygen_make(sc.0.as_ptr(), NEAR_FALCON_DEGREE, sk.as_mut_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, pk.as_mut_ptr(), NEAR_FALCON512_PUBKEY_SIZE, tmp_keygen.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_KEYGEN)
            );
            shake256_init_prng_from_system(sc.0.as_mut_ptr());
            assert_eq!(
                0,
                falcon_sign_dyn(sc.0.as_ptr(), sig.as_mut_ptr(), &sig_size, FALCON_SIG_PADDED, sk.as_ptr(), NEAR_FALCON512_PRIVKEY_SIZE, data_to_sign.as_ptr(), len_data_to_sign, tmp_signdyn.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_SIGNDYN)
            );

            // Signature verifications
            assert_eq!(
                0,
                falcon_verify(sig.as_ptr(), NEAR_FALCON512_SIG_PADDED_SIZE, FALCON_SIG_PADDED, pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE, data_to_sign.as_ptr(), len_data_to_sign, tmp_verify.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_VERIFY)
            );
            assert_ne!(
                0,
                falcon_verify(sig.as_ptr(), NEAR_FALCON512_SIG_PADDED_SIZE, FALCON_SIG_PADDED, pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE, data_to_sign.as_ptr(), len_data_to_sign-1, tmp_verify.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_VERIFY)
            );
            assert_ne!(
                0,
                falcon_verify(sig.as_ptr(), NEAR_FALCON512_SIG_PADDED_SIZE-1, FALCON_SIG_PADDED, pk.as_ptr(), NEAR_FALCON512_PUBKEY_SIZE, data_to_sign.as_ptr(), len_data_to_sign, tmp_verify.as_mut_ptr(), NEAR_FALCON512_TMPSIZE_VERIFY)
            );
        }
    }
}
