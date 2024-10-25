#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use polymesh_api_codegen_macro::*;

#[cfg_attr(
  all(not(feature = "download_metadata"), feature = "polymesh_v6"),
  codegen_api(metadata_file = "specs/polymesh_dev_spec_6002000.meta")
)]
#[cfg_attr(
  all(not(feature = "download_metadata"), feature = "polymesh_v7"),
  codegen_api(metadata_file = "specs/polymesh_dev_spec_7000003.meta")
)]
#[cfg_attr(
  feature = "download_metadata",
  codegen_api(metadata_url = "POLYMESH_NODE_URL")
)]
mod polymesh {}

pub use polymesh::*;

// re-export core client and common types.
#[cfg(feature = "rpc")]
pub use polymesh_api_client as client;
#[cfg(feature = "rpc")]
pub use polymesh_api_client::{ChainApi, Client};

#[cfg(feature = "ink")]
pub use polymesh_api_ink as ink;
