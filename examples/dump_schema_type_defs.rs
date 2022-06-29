use anyhow::Result;

use sub_api::schema::*;

fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let mut types = Types::new();
  types.load_schema("./schemas/init_types.json")?;
  types.load_schema("./schemas/polymesh/3000.json")?;

  types.dump_types();
  types.dump_unresolved();
  Ok(())
}
