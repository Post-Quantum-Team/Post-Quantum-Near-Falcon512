//! # falcon
//!
//! This crate provides bindings to and wrappers around the following
//! implementations from Post-Quantum-Blockchain :
//!
//! * falcon-512 - clean
//!
//! 
//!

#![no_std]
#![allow(clippy::len_without_is_empty)]

// For no-std vectors
extern crate alloc;

// For tests
#[cfg(feature = "std")]
extern crate std;

pub mod falcon512;
pub mod ffi;

// to add : open as falcon512_open, sign as falcon512_sign,

pub use crate::falcon512::{
    detached_sign as falcon512_detached_sign, keypair as falcon512_keypair,
    public_key_bytes as falcon512_public_key_bytes, secret_key_bytes as falcon512_secret_key_bytes,
    signature_bytes as falcon512_signature_bytes,
    verify_detached_signature as falcon512_verify_detached_signature, generator_from_seed,
    generator_from_system_prng, public_key_from_secret_key as falcon512_public_key_from_secret_key,
    keypair_from_shake256context as falcon512_keypair_from_shake256context, 
    keypair_from_seed as falcon512_keypair_from_seed,
};

pub use crate::ffi::{
    Shake256Context, shake256_extract, shake256_flip, 
    shake256_init, shake256_init_prng_from_seed,
    shake256_init_prng_from_system, shake256_inject,
    NEAR_FALCON512_PRIVKEY_SIZE, NEAR_FALCON512_PUBKEY_SIZE, NEAR_FALCON512_SIG_PADDED_SIZE as NEAR_FALCON512_SIG_SIZE, SHAKE256_CONTEXT_SIZE,
};
