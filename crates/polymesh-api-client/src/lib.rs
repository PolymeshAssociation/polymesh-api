#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod basic_types;
pub use basic_types::*;

pub mod error;
pub use error::*;

pub mod rpc;

pub mod block;
pub use block::*;

pub mod signer;
pub use signer::*;

pub mod transaction;
pub use transaction::*;

pub mod client;
pub use client::*;

#[cfg(feature = "type_info")]
pub mod type_def;

#[cfg(feature = "type_info")]
pub mod type_codec;

#[cfg(feature = "type_info")]
pub mod schema;

#[cfg(feature = "type_info")]
pub mod metadata;
