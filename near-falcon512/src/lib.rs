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
    verify_detached_signature as falcon512_verify_detached_signature,
};
