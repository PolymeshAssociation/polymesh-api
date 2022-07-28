use polymesh_api_codegen_macro::*;

#[codegen_api(metadata_file = "specs/polymesh_dev_spec_5000002.meta")]
mod polymesh {}

pub use polymesh::*;

// re-export core client
pub use polymesh_api_client as client;
