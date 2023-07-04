#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use polymesh_api_codegen_macro::*;

#[cfg_attr(not(feature = "download_metadata"), codegen_api(metadata_file = "specs/polymesh_dev_spec_5004000.meta"))]
#[cfg_attr(feature = "download_metadata", codegen_api(metadata_url = "ws://localhost:9944"))]
mod polymesh {}

pub use polymesh::*;

// re-export core client and common types.
#[cfg(feature = "rpc")]
pub use polymesh_api_client as client;
#[cfg(feature = "rpc")]
pub use polymesh_api_client::{ChainApi, Client};

#[cfg(feature = "ink")]
pub use polymesh_api_ink as ink;
