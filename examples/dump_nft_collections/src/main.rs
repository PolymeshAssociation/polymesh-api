use std::env;

use anyhow::Result;
use futures_util::StreamExt;

use polymesh_api::polymesh::types::polymesh_primitives::asset_metadata::AssetMetadataKey;
use polymesh_api::Api;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url - usage: dump_nft_collections <ws_url>");

  let api = Api::new(&url).await?;
  println!("Connected to {url}");

  // Paginate through all NFT collections: NFTCollectionId => NFTCollection { id, asset_id }
  let collections = api.paged_query().nft().collection().entries();
  tokio::pin!(collections);

  let mut total = 0usize;
  while let Some(entry) = collections.next().await {
    let (collection_id, collection_opt) = entry?;
    // StoragePaged returns Option<NFTCollection> because the map is optional.
    let collection = match collection_opt {
      Some(c) => c,
      None => continue,
    };
    total += 1;
    let asset_id = collection.asset_id;

    // Number of NFTs in this collection (by AssetId).
    let nft_count = api.query().nft().nf_ts_in_collection(asset_id).await?;

    // Mandatory metadata keys for the collection.
    let keys = api
      .query()
      .nft()
      .collection_keys(collection_id.clone())
      .await?;

    println!(
      "Collection {:?} (AssetId: {:?}) - NFTs: {}",
      collection_id, asset_id, nft_count
    );

    if keys.is_empty() {
      println!("  Mandatory keys: none");
    } else {
      println!("  Mandatory keys ({}):", keys.len());
      for key in keys {
        match &key {
          AssetMetadataKey::Global(g) => {
            let name_opt = api
              .query()
              .asset()
              .asset_metadata_global_key_to_name(g.clone())
              .await?;
            let name = name_opt
              .map(|n| String::from_utf8_lossy(&n.0).to_string())
              .unwrap_or_else(|| "<unknown>".to_string());
            println!("    - Global({:?}) Name: {}", g.0, name);
          }
          AssetMetadataKey::Local(l) => {
            let name_opt = api
              .query()
              .asset()
              .asset_metadata_local_key_to_name(asset_id, l.clone())
              .await?;
            let name = name_opt
              .map(|n| String::from_utf8_lossy(&n.0).to_string())
              .unwrap_or_else(|| "<unknown>".to_string());
            println!("    - Local({:?}) Name: {}", l.0, name);
          }
        }
      }
    }
    // Optional: show storage key details for debugging
    // println!("  raw key set: {:?}", keys);
  }

  if total == 0 {
    println!("No NFT collections found.");
  } else {
    println!("Total NFT collections: {total}");
  }

  Ok(())
}
