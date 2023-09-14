use anyhow::Result;

use polymesh_api_client::schema::*;
use polymesh_api_client::RuntimeVersion;

fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let mut types = Types::new(RuntimeVersion::default());
  types.try_load_schema("./schemas/init_types.json");
  types.try_load_schema("./schemas/polymesh/3000.json");

  types.dump_types();
  types.dump_unresolved();
  Ok(())
}
