use std::env;

use anyhow::Result;
use futures_util::StreamExt;

use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url - usage: dump_global_asset_metadata <ws_url>");

  let api = Api::new(&url).await?;
  println!("Connected to {url}");
  println!("Dumping global asset metadata keys...");

  // Paginate through AssetMetadataGlobalKey -> AssetMetadataName
  // Storage: asset.assetMetadataGlobalKeyToName (GlobalKey => Name)
  // Also available: assetMetadataGlobalNameToKey (Name => Key), assetMetadataGlobalSpecs (Key => Spec)
  let entries = api
    .paged_query()
    .asset()
    .asset_metadata_global_key_to_name()
    .entries();
  tokio::pin!(entries);

  let mut total = 0usize;
  while let Some(entry) = entries.next().await {
    let (key, name_opt) = entry?;
    // Paged storage returns Option<AssetMetadataName> for optional entries.
    let name = match name_opt {
      Some(n) => n,
      None => continue,
    };
    total += 1;
    let name_str = String::from_utf8_lossy(&name.0);
    // AssetMetadataGlobalKey is tuple struct (u64)
    println!("  Global id: {}  Name: {}", key.0, name_str);

    // Optionally fetch spec for the key (url, description, type_def) - not required but useful:
    // let spec = api.query().asset().asset_metadata_global_specs(key.clone()).await?;
    // if let Some(spec) = spec { println!("    spec: {:?}", spec); }
  }

  if total == 0 {
    println!("No global asset metadata keys found.");
  } else {
    println!("Total global asset metadata keys: {total}");
  }

  // Alternative view: Name -> Key (demonstrates reverse mapping)
  // Uncomment to dump via the reverse map as well:
  // let rev_entries = api.paged_query().asset().asset_metadata_global_name_to_key().entries();
  // tokio::pin!(rev_entries);
  // while let Some(entry) = rev_entries.next().await {
  //   let (name, key) = entry?;
  //   println!("  Name: {} -> id: {}", String::from_utf8_lossy(&name.0), key.0);
  // }

  Ok(())
}
