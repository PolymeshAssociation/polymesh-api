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
  metadata_file: String,
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn sub_api(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(args as syn::AttributeArgs);
  let item = parse_macro_input!(input as syn::ItemMod);
  let args = CodegenArgs::from_list(&args).unwrap_or_else(|e| abort_call_site!(e));

  let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
  let filename = Path::new(&root).join(args.metadata_file);
  let mut file = File::open(filename).unwrap_or_else(|e| abort_call_site!(e));
  let mut buf = Vec::new();
  file
    .read_to_end(&mut buf)
    .unwrap_or_else(|e| abort_call_site!(e));

  match codegen::macro_codegen(&buf, item.ident.into_token_stream().into()) {
    Ok(out) => out.into(),
    Err(err) => abort_call_site!(err),
  }
}
