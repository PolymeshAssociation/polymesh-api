use std::env;

use anyhow::Result;

use polymesh_api_client::*;

use polymesh_api_codegen::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = Client::new(&url).await?;

  // Get current Metadata.
  let metadata = client.get_block_metadata(None).await?.expect("Metadata");

  //let version = client.get_runtime_version(None).await?;
  //println!("{version:#?}");

  let code = generate(metadata).map_err(|e| anyhow::anyhow!("{e:?}"))?;
  eprintln!("------------- Generated code.  Now formatting.");
  println!("{}", rustfmt_wrapper::rustfmt(code).unwrap());
  //println!("{code}");

  Ok(())
}
