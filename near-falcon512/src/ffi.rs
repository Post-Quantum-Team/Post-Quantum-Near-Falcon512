//! Foreign function interfaces
//!
//! This module defines the foreign function interface for the following
//! crypto implementations from Near:
//!
//!  * falcon-512
// This file has been generated from falcon512-c-near.
// Find the templates in pqcrypto-template
use libc::c_int;

// ensures we link correctly
#[allow(unused_imports)]
use pqcrypto_internals::*;

/*
pub const PQCLEAN_FALCON512_CLEAN_CRYPTO_SECRETKEYBYTES: usize = 1281;
pub const PQCLEAN_FALCON512_CLEAN_CRYPTO_PUBLICKEYBYTES: usize = 897;
pub const PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES: usize = 690;

#[cfg(enable_x86_avx2)]
pub const PQCLEAN_FALCON512_AVX2_CRYPTO_SECRETKEYBYTES: usize = 1281;
#[cfg(enable_x86_avx2)]
pub const PQCLEAN_FALCON512_AVX2_CRYPTO_PUBLICKEYBYTES: usize = 897;
#[cfg(enable_x86_avx2)]
pub const PQCLEAN_FALCON512_AVX2_CRYPTO_BYTES: usize = 690;
*/

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



#[link(name = "falcon-512_clean")]
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


    /*pub fn PQCLEAN_FALCON512_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn PQCLEAN_FALCON512_CLEAN_crypto_sign(
        sm: *mut u8,
        smlen: *mut usize,
        msg: *const u8,
        len: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_FALCON512_CLEAN_crypto_sign_open(
        m: *mut u8,
        mlen: *mut usize,
        sm: *const u8,
        smlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_FALCON512_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_FALCON512_CLEAN_get_seed(
        seed: *const u8,
        seed_len:  usize,
    ) -> c_int; */
}
/*
#[cfg(enable_x86_avx2)]
#[link(name = "falcon-512_avx2")]
extern "C" {
    #[cfg(enable_x86_avx2)]
    pub fn PQCLEAN_FALCON512_AVX2_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    #[cfg(enable_x86_avx2)]
    pub fn PQCLEAN_FALCON512_AVX2_crypto_sign(
        sm: *mut u8,
        smlen: *mut usize,
        msg: *const u8,
        len: usize,
        sk: *const u8,
    ) -> c_int;
    #[cfg(enable_x86_avx2)]
    pub fn PQCLEAN_FALCON512_AVX2_crypto_sign_open(
        m: *mut u8,
        mlen: *mut usize,
        sm: *const u8,
        smlen: usize,
        pk: *const u8,
    ) -> c_int;
    #[cfg(enable_x86_avx2)]
    pub fn PQCLEAN_FALCON512_AVX2_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> c_int;
    #[cfg(enable_x86_avx2)]
    pub fn PQCLEAN_FALCON512_AVX2_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_FALCON512_AVX2_get_seed(
        seed: *const u8,
        seed_len:  usize,
    ) -> c_int;
}


#[cfg(test)]
mod test_falcon512_clean {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use rand::prelude::*;
    use std::dbg;

    #[test]
    fn get_seed_test() {
        unsafe {
            let mut tab = vec![0u8; 16];;
            let size = 16;
            PQCLEAN_FALCON512_CLEAN_get_seed(tab.as_mut_ptr(), size);
            dbg!(tab);

        }
    }

    #[test]
    fn test_ffi() {
        unsafe {
            let mut rng = rand::thread_rng();
            let mut mlen: usize = rng.gen::<u16>() as usize;
            let msg: Vec<u8> = (0..mlen).map(|_| rng.gen()).collect();

            let mut pk = vec![0u8; PQCLEAN_FALCON512_CLEAN_CRYPTO_PUBLICKEYBYTES];
            let mut sk = vec![0u8; PQCLEAN_FALCON512_CLEAN_CRYPTO_SECRETKEYBYTES];
            let mut pk_alt = vec![0u8; PQCLEAN_FALCON512_CLEAN_CRYPTO_PUBLICKEYBYTES];
            let mut sk_alt = vec![0u8; PQCLEAN_FALCON512_CLEAN_CRYPTO_SECRETKEYBYTES];
            let mut detached_sig = vec![0u8; PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES];
            let mut sm = Vec::with_capacity(mlen + PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES);
            let mut smlen = 0;
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr())
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign(
                    sm.as_mut_ptr(),
                    &mut smlen as *mut usize,
                    msg.as_ptr(),
                    mlen,
                    sk.as_ptr()
                )
            );
            sm.set_len(smlen);

            let mut unpacked_m = Vec::with_capacity(mlen + PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES);
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_open(
                    unpacked_m.as_mut_ptr(),
                    &mut mlen as *mut usize,
                    sm.as_ptr(),
                    sm.len(),
                    pk.as_ptr()
                )
            );
            unpacked_m.set_len(mlen);
            assert_eq!(unpacked_m, msg);

            // check verification fails with wrong pk
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_keypair(
                    pk_alt.as_mut_ptr(),
                    sk_alt.as_mut_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_open(
                    unpacked_m.as_mut_ptr(),
                    &mut mlen as *mut usize,
                    sm.as_ptr(),
                    sm.len(),
                    pk_alt.as_ptr()
                )
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_signature(
                    detached_sig.as_mut_ptr(),
                    &mut smlen as *mut usize,
                    msg.as_ptr(),
                    msg.len(),
                    sk.as_ptr()
                )
            );
            assert!(
                smlen <= PQCLEAN_FALCON512_CLEAN_CRYPTO_BYTES,
                "Signed message length should be ≤ CRYPTO_BYTES"
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len(),
                    pk.as_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len(),
                    pk_alt.as_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len() - 1,
                    pk.as_ptr()
                )
            );
        }
    }
}

#[cfg(all(test, enable_x86_avx2, feature = "avx2"))]
mod test_falcon512_avx2 {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use rand::prelude::*;
    use std::is_x86_feature_detected;

    #[test]
    fn test_ffi() {
        if !is_x86_feature_detected!("avx2") {
            return;
        }
        unsafe {
            let mut rng = rand::thread_rng();
            let mut mlen: usize = rng.gen::<u16>() as usize;
            let msg: Vec<u8> = (0..mlen).map(|_| rng.gen()).collect();

            let mut pk = vec![0u8; PQCLEAN_FALCON512_AVX2_CRYPTO_PUBLICKEYBYTES];
            let mut sk = vec![0u8; PQCLEAN_FALCON512_AVX2_CRYPTO_SECRETKEYBYTES];
            let mut pk_alt = vec![0u8; PQCLEAN_FALCON512_AVX2_CRYPTO_PUBLICKEYBYTES];
            let mut sk_alt = vec![0u8; PQCLEAN_FALCON512_AVX2_CRYPTO_SECRETKEYBYTES];
            let mut detached_sig = vec![0u8; PQCLEAN_FALCON512_AVX2_CRYPTO_BYTES];
            let mut sm = Vec::with_capacity(mlen + PQCLEAN_FALCON512_AVX2_CRYPTO_BYTES);
            let mut smlen = 0;
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr())
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign(
                    sm.as_mut_ptr(),
                    &mut smlen as *mut usize,
                    msg.as_ptr(),
                    mlen,
                    sk.as_ptr()
                )
            );
            sm.set_len(smlen);

            let mut unpacked_m = Vec::with_capacity(mlen + PQCLEAN_FALCON512_AVX2_CRYPTO_BYTES);
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign_open(
                    unpacked_m.as_mut_ptr(),
                    &mut mlen as *mut usize,
                    sm.as_ptr(),
                    sm.len(),
                    pk.as_ptr()
                )
            );
            unpacked_m.set_len(mlen);
            assert_eq!(unpacked_m, msg);

            // check verification fails with wrong pk
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign_keypair(
                    pk_alt.as_mut_ptr(),
                    sk_alt.as_mut_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_AVX2_crypto_sign_open(
                    unpacked_m.as_mut_ptr(),
                    &mut mlen as *mut usize,
                    sm.as_ptr(),
                    sm.len(),
                    pk_alt.as_ptr()
                )
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign_signature(
                    detached_sig.as_mut_ptr(),
                    &mut smlen as *mut usize,
                    msg.as_ptr(),
                    msg.len(),
                    sk.as_ptr()
                )
            );
            assert!(
                smlen <= PQCLEAN_FALCON512_AVX2_CRYPTO_BYTES,
                "Signed message length should be ≤ CRYPTO_BYTES"
            );
            assert_eq!(
                0,
                PQCLEAN_FALCON512_AVX2_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len(),
                    pk.as_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_AVX2_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len(),
                    pk_alt.as_ptr()
                )
            );
            assert_eq!(
                -1,
                PQCLEAN_FALCON512_AVX2_crypto_sign_verify(
                    detached_sig.as_ptr(),
                    smlen,
                    msg.as_ptr(),
                    msg.len() - 1,
                    pk.as_ptr()
                )
            );
        }
    }
}
*/