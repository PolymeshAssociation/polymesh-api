use std::env;

use anyhow::Result;

use sub_api::*;

use codegen::*;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");

  let client = Client::new(&url).await?;

  // Get current Metadata.
  let metadata = client.get_block_metadata(None).await?;

  //let version = client.get_runtime_version(None).await?;
  //println!("{version:#?}");

  let code = generate(metadata)?;
  eprintln!("------------- Generated code.  Now formatting.");
  println!("{}", rustfmt_wrapper::rustfmt(code).unwrap());
  //println!("{code}");

  Ok(())
}
