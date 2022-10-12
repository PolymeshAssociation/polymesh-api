#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use polymesh_api_codegen_macro::*;

#[codegen_api(metadata_file = "specs/polymesh_dev_spec_5000002.meta")]
mod polymesh {}

pub use polymesh::*;

// re-export core client and common types.
#[cfg(feature = "rpc")]
pub use polymesh_api_client as client;
#[cfg(feature = "rpc")]
pub use polymesh_api_client::{ChainApi, Client};

#[cfg(feature = "ink")]
pub use polymesh_api_ink as ink;
