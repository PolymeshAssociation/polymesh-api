use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};

use darling::{FromMeta, ToTokens};
use syn::parse_macro_input;

#[derive(Clone, FromMeta)]
struct CodegenArgs {
  metadata_file: Option<String>,
  metadata_url: Option<String>,
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn codegen_api(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(args as syn::AttributeArgs);
  let item = parse_macro_input!(input as syn::ItemMod);
  let args = CodegenArgs::from_list(&args).unwrap_or_else(|e| abort_call_site!(e));

  let buf = match args {
    CodegenArgs {
      metadata_file: Some(filename),
      metadata_url: None,
    } => {
      let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
      let filename = Path::new(&root).join(filename);
      let mut file = File::open(filename).unwrap_or_else(|e| abort_call_site!(e));
      let mut buf = Vec::new();
      file
        .read_to_end(&mut buf)
        .unwrap_or_else(|e| abort_call_site!(e));
      buf
    }
    #[cfg(feature = "download_metadata")]
    CodegenArgs {
      metadata_file: None,
      metadata_url: Some(url),
    } => {
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        use codec::Encode;
        use polymesh_api_client::*;

        let client = Client::new(&url)
          .await
          .unwrap_or_else(|e| abort_call_site!(e));

        // Get current Metadata.
        let metadata = client
          .get_block_metadata(None)
          .await
          .unwrap_or_else(|e| abort_call_site!(e))
          .expect("Chain metadata");

        metadata.encode()
      })
    }
    #[cfg(not(feature = "download_metadata"))]
    CodegenArgs {
      metadata_file: None,
      metadata_url: Some(_),
    } => {
      panic!("Support `metadata_url` disabled, add feature `download_metadata`.");
    }
    _ => {
      panic!("Must provide either `metadata_file` or `metadata_url`, but not both.");
    }
  };

  match polymesh_api_codegen::macro_codegen(&buf, item.ident.into_token_stream().into()) {
    Ok(out) => out.into(),
    Err(err) => abort_call_site!(err),
  }
}
