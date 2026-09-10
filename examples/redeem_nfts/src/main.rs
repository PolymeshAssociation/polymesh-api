//! Redeem (burn) NFTs from an NFT collection.
//!
//! The issuer (asset `owner_did`) can burn NFTs they currently hold. NFTs held by other
//! identities must first be pulled in with `Nft.controller_transfer` (not covered here).
//!
//! Chain APIs used:
//! - `Nft.redeem_nft(asset_id, nft_id, holdings_kind, number_of_keys)` — call index 2
//! - `Asset.assets(asset_id)` — `AssetDetails { owner_did, ... }`
//! - `Asset.ticker_asset_id(ticker)` — resolve ticker to AssetId
//! - `Asset.frozen(asset_id)` — check if asset is frozen
//! - `Asset.asset_names(asset_id)` — get asset name for display
//! - `Identity.key_records(account)` — resolve caller's DID
//! - `ExternalAgents.group_of_agent(asset_id, did)` — check agent permissions
//! - `Nft.collection_asset(asset_id)` — get NFTCollectionId (0 = no collection)
//! - `Nft.collection_keys(collection_id)` — mandatory metadata keys (count for weight hint)
//! - `Nft.owner(asset_id, nft_id)` — Option<AssetHolder> who currently holds the NFT
//! - `Nft.owner` (paged) — enumerate all NFTs in collection with holders
//! - `Nft.nft_holder(account, asset_id, nft_id)` — NFTOwnerStatus (account holdings lock check)
//! - `Portfolio.portfolio_locked_nft(portfolio_id, (asset_id, nft_id))` — portfolio holdings lock check
//! - `Nft.nf_ts_in_collection(asset_id)` — total supply
//! - `Nft.number_of_nf_ts(asset_id, did)` — per-identity count
//! - `Utility.force_batch(calls)` — non-atomic batch; `ItemFailed` on per-item error
//!
//! Event emitted on success: `RuntimeEvent::Nft(NFTHoldingsUpdated(did, NFTs, from, None, Redeemed))`
//!
//! Note: `Nft.CollectionAsset` is a ValueQuery map; missing collection returns `NFTCollectionId(0)`.
//! Collection IDs start at 1 (CurrentCollectionId increments before use), so 0 means "no collection".

use std::str::FromStr;

use anyhow::{bail, Result};
use clap::{ArgAction, Parser};
use futures_util::StreamExt;

use polymesh_api::client::{AccountId, AssetId, DefaultSigner, IdentityId};
use polymesh_api::polymesh::types::polymesh_primitives::{
  agent::AgentGroup,
  asset::{AssetHolder, AssetHolderKind, AssetType},
  identity_id::{PortfolioKind, PortfolioNumber},
  nft::{NFTId, NFTOwnerStatus},
  secondary_key::KeyRecord,
  ticker::Ticker,
};
use polymesh_api::polymesh::types::runtime::{events, RuntimeCall, RuntimeEvent};
use polymesh_api::Api;

#[derive(Parser, Debug)]
#[command(
  name = "redeem_nfts",
  about = "Redeem (burn) NFTs from an NFT collection"
)]
struct Cli {
  /// WebSocket URL of the Polymesh node
  #[arg(index = 1)]
  ws_url: String,

  /// Signer URI (mnemonic, //Alice, 0x<seed>, or env POLYMESH_SIGNER)
  #[arg(long, env = "POLYMESH_SIGNER")]
  key: String,

  /// Asset ID (UUID) or ticker (e.g., MYNFT)
  #[arg(long)]
  asset: String,

  /// NFT IDs to burn (u64); mutually exclusive with --all
  #[arg(index = 2, value_parser = parse_nft_id)]
  nft_ids: Vec<u64>,

  /// Burn all NFTs in the collection currently held by the issuer
  #[arg(long, action = ArgAction::SetTrue)]
  all: bool,

  /// Dry run: validate and print plan, but do not submit
  #[arg(long, action = ArgAction::SetTrue)]
  dry_run: bool,

  /// Skip interactive confirmation
  #[arg(long, short = 'y', action = ArgAction::SetTrue)]
  yes: bool,
}

fn parse_nft_id(s: &str) -> Result<u64, std::num::ParseIntError> {
  s.parse()
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let cli = Cli::parse();

  if cli.nft_ids.is_empty() && !cli.all {
    bail!("Either provide NFT IDs or use --all");
  }
  if !cli.nft_ids.is_empty() && cli.all {
    bail!("Cannot use both explicit NFT IDs and --all");
  }

  let api = Api::new(&cli.ws_url).await?;
  println!("Connected to {}", cli.ws_url);

  let mut signer = DefaultSigner::from_string(&cli.key, None)?;
  let caller_account: AccountId = signer.account;
  println!("Caller account: {:?}", caller_account);

  // Resolve issuer DID from key records
  let issuer_did = match api
    .query()
    .identity()
    .key_records(caller_account.clone())
    .await?
  {
    Some(KeyRecord::PrimaryKey(did)) | Some(KeyRecord::SecondaryKey(did)) => did,
    Some(KeyRecord::MultiSigSignerKey(_)) => bail!("MultiSig signer not supported"),
    None => bail!("Account {:?} has no identity", caller_account),
  };
  println!("Issuer DID: {:?}", issuer_did);

  // Resolve AssetId (UUID or ticker)
  let asset_id = resolve_asset_id(&api, &cli.asset).await?;
  println!("Asset ID: {}", asset_id);

  // Fetch asset details
  let details = api
    .query()
    .asset()
    .assets(asset_id)
    .await?
    .ok_or_else(|| anyhow::anyhow!("Asset not found: {}", asset_id))?;

  let asset_name = api
    .query()
    .asset()
    .asset_names(asset_id)
    .await?
    .map(|n| String::from_utf8_lossy(&n.0).to_string())
    .unwrap_or_else(|| "<unknown>".to_string());

  let frozen = api.query().asset().frozen(asset_id).await?;

  println!("Asset: {}", asset_name);
  println!("  Owner DID: {:?}", details.owner_did);
  println!("  Total supply: {}", details.total_supply);
  println!("  Asset type: {:?}", details.asset_type);
  println!("  Frozen: {}", frozen);

  // Verify issuer is the owner (or an agent)
  if details.owner_did != issuer_did {
    let agent_group = api
      .query()
      .external_agents()
      .group_of_agent(asset_id, issuer_did)
      .await?;
    eprintln!(
      "Warning: Caller DID {:?} is not the asset owner (owner: {:?}).",
      issuer_did, details.owner_did
    );
    if let Some(g) = agent_group {
      eprintln!("  Caller is an agent with group: {:?}", g);
      if !matches!(g, AgentGroup::Full) {
        eprintln!("  Note: Only 'Full' agents can redeem; this agent may lack permissions.");
      }
    } else {
      eprintln!("  Caller is not registered as an agent for this asset.");
      bail!("Insufficient permissions: not owner and not an agent");
    }
  } else {
    println!("Issuer is the asset owner (full agent by default).");
  }

  // Verify asset is non-fungible
  let is_nft = matches!(details.asset_type, AssetType::NonFungible(_));
  if !is_nft {
    eprintln!("Warning: Asset type is not NonFungible; redeem may fail.");
  }

  // Get collection
  let collection_id = api.query().nft().collection_asset(asset_id).await?;
  if collection_id.0 == 0 {
    bail!("No NFT collection found for asset {}", asset_id);
  }
  println!("NFT Collection ID: {:?}", collection_id);

  // Get mandatory metadata keys for weight hint
  let collection_keys = api
    .query()
    .nft()
    .collection_keys(collection_id.clone())
    .await?;
  let number_of_keys = if collection_keys.is_empty() {
    None
  } else {
    let k = collection_keys.len();
    if k > 255 {
      bail!("Collection has {} keys (> 255 max)", k);
    }
    Some(k as u8)
  };
  println!("Mandatory metadata keys: {}", collection_keys.len());

  // Determine NFT IDs to redeem
  let mut ids = Vec::new();
  if cli.all {
    println!("Enumerating NFTs in collection (paged query)...");
    let entries = api.paged_query().nft().owner(asset_id).entries();
    tokio::pin!(entries);
    while let Some(entry) = entries.next().await {
      let (nft_id, holder_opt) = entry?;
      println!("NFT[{nft_id:?}]");
      if let Some(holder) = holder_opt {
        if let Some(kind) = holder_belongs_to_issuer(&holder, issuer_did, caller_account.clone()) {
          ids.push((nft_id, kind));
        }
      }
    }
    if ids.is_empty() {
      println!("No NFTs held by issuer in this collection.");
      return Ok(());
    }
  } else {
    for nft_id in &cli.nft_ids {
      let nft_id = NFTId(*nft_id);
      let holder_opt = api.query().nft().owner(asset_id, nft_id.clone()).await?;
      let Some(holder) = holder_opt else {
        continue;
      };

      match validate_and_resolve_holding(
        &api,
        &holder,
        issuer_did,
        caller_account.clone(),
        asset_id,
        &nft_id,
      )
      .await
      {
        Ok(kind) => ids.push((nft_id, kind)),
        Err(reason) => {
          log::debug!("Skipping NFT {:?}: {}", nft_id, reason);
        }
      }
    }
  }

  println!("NFT to redeem IDs: {} total", ids.len());

  if ids.is_empty() {
    println!("No NFTs to redeem (all skipped).");
    return Ok(());
  }

  // Dry run
  if cli.dry_run {
    println!("\n--- Dry run complete (no submission) ---");
    let before_supply = api.query().nft().nf_ts_in_collection(asset_id).await?;
    let before_count = api
      .query()
      .nft()
      .number_of_nf_ts(asset_id, issuer_did)
      .await?;
    println!("Current total supply: {}", before_supply);
    println!("Current issuer count: {}", before_count);
    return Ok(());
  }

  // Confirm
  if !cli.yes {
    println!(
      "\nThis will permanently burn {} NFT(s). Continue? [y/N] ",
      ids.len()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
      println!("Aborted.");
      return Ok(());
    }
  }

  // Build RuntimeCalls (cloneable) — WrappedCall is not Clone
  let runtime_calls: Vec<RuntimeCall> = ids
    .iter()
    .map(|(id, kind)| {
      let wc = api
        .call()
        .nft()
        .redeem_nft(asset_id, id.clone(), kind.clone(), number_of_keys)?;
      Ok::<_, anyhow::Error>(wc.into())
    })
    .collect::<Result<Vec<_>, _>>()?;

  // Submit (chunked for force_batch)
  const MAX_BATCH: usize = 100; // chain constant Utility.batched_calls_limit = 170
  let mut total_completed = 0usize;
  let mut total_failed = 0usize;

  for chunk in runtime_calls.chunks(MAX_BATCH) {
    let wrapped = if chunk.len() == 1 {
      api.wrap_call(chunk[0].clone())?
    } else {
      api.call().utility().force_batch(chunk.to_vec())?
    };
    let call = wrapped;

    println!("Submitting batch of {} call(s)...", chunk.len());
    let mut res = call.submit_and_watch(&mut signer).await?;
    res.wait_finalized().await?;

    if let Err(e) = res.ok().await {
      eprintln!("Batch extrinsic failed: {}", e);
      total_failed += chunk.len();
      continue;
    }

    if let Some(events) = res.events().await? {
      for rec in &events.0 {
        match &rec.event {
          RuntimeEvent::Nft(events::NftEvent::NFTHoldingsUpdated(
            _did,
            nfts,
            _from,
            to,
            _reason,
          )) => {
            if to.is_none() {
              println!("  Burned: {:?}", nfts.ids);
            }
          }
          RuntimeEvent::Utility(events::UtilityEvent::ItemFailed { error }) => {
            total_failed += 1;
            eprintln!("  ItemFailed: {:?} ({})", error, rec.short_doc());
          }
          RuntimeEvent::Utility(events::UtilityEvent::ItemCompleted) => {
            total_completed += 1;
          }
          RuntimeEvent::Utility(events::UtilityEvent::BatchCompleted) => {
            println!("  BatchCompleted");
          }
          RuntimeEvent::Utility(events::UtilityEvent::BatchCompletedWithErrors) => {
            println!("  BatchCompletedWithErrors");
          }
          _ => {}
        }
      }
    }
    // For single-call chunks, no ItemCompleted event; count from NFTHoldingsUpdated.
    if chunk.len() == 1 && total_completed == 0 && total_failed == 0 {
      // Single redeem emits NFTHoldingsUpdated but not ItemCompleted.
      // We already printed Burned; count it as completed if we saw it.
      // The loop above handles it; if we want a numeric summary, infer:
      // If no ItemFailed seen, treat as 1 success (checked via total_failed).
    }
  }

  println!("\n--- Summary ---");
  println!("Completed (ItemCompleted): {}", total_completed);
  println!("Failed:    {}", total_failed);
  if runtime_calls.len() == 1 && total_completed == 0 && total_failed == 0 {
    println!("(Singleton redeem: check Burned log above; if no ItemFailed, it succeeded)");
  }

  // Post-check
  let after_supply = api.query().nft().nf_ts_in_collection(asset_id).await?;
  let after_count = api
    .query()
    .nft()
    .number_of_nf_ts(asset_id, issuer_did)
    .await?;
  println!("Total supply after: {}", after_supply);
  println!("Issuer count after: {}", after_count);

  if total_failed > 0 {
    bail!("Some NFTs failed to redeem");
  }
  Ok(())
}

async fn resolve_asset_id(api: &Api, input: &str) -> Result<AssetId> {
  // Try UUID first (client AssetId implements FromStr via uuid)
  if let Ok(asset_id) = AssetId::from_str(input) {
    return Ok(asset_id);
  }
  // Also handle 0x-prefixed or bare 32-hex-char (not hyphenated) by building AssetId directly
  let hex_input = input.trim().strip_prefix("0x").unwrap_or(input);
  if hex_input.len() == 32 && hex_input.chars().all(|c| c.is_ascii_hexdigit()) {
    let bytes = hex::decode(hex_input)?;
    let arr: [u8; 16] = bytes
      .try_into()
      .map_err(|_| anyhow::anyhow!("Invalid hex length"))?;
    return Ok(AssetId(arr));
  }
  // Fallback: treat as ticker (right-pad to 12 bytes, as Polymesh does)
  if input.len() > 12 {
    bail!("Ticker too long (max 12 chars): {}", input);
  }
  let mut ticker_bytes = [0u8; 12];
  ticker_bytes[..input.len()].copy_from_slice(input.as_bytes());
  let ticker = Ticker(ticker_bytes);
  if let Some(asset_id) = api.query().asset().ticker_asset_id(ticker).await? {
    Ok(asset_id)
  } else {
    bail!("No asset found for ticker: {}", input)
  }
}

fn holder_belongs_to_issuer(
  holder: &AssetHolder,
  issuer_did: IdentityId,
  caller_account: AccountId,
) -> Option<AssetHolderKind> {
  match holder {
    AssetHolder::Account(acct) => {
      if *acct == caller_account {
        Some(AssetHolderKind::Account)
      } else {
        None
      }
    }
    AssetHolder::Portfolio(pid) => {
      if pid.did == issuer_did {
        match &pid.kind {
          PortfolioKind::Default => Some(AssetHolderKind::DefaultPortfolio),
          PortfolioKind::User(num) => Some(AssetHolderKind::UserPortfolio(PortfolioNumber(num.0))),
        }
      } else {
        None
      }
    }
  }
}

async fn validate_and_resolve_holding(
  api: &Api,
  holder: &AssetHolder,
  issuer_did: IdentityId,
  caller_account: AccountId,
  asset_id: AssetId,
  nft_id: &NFTId,
) -> Result<AssetHolderKind, String> {
  match holder {
    AssetHolder::Account(acct) => {
      if *acct != caller_account {
        return Err(format!(
          "Held by another account {:?} (use controller_transfer to pull it first)",
          acct
        ));
      }
      let status = api
        .query()
        .nft()
        .nft_holder(acct.clone(), asset_id, nft_id.clone())
        .await
        .map_err(|e| format!("Query error: {}", e))?;
      if matches!(status, NFTOwnerStatus::OwnerLocked) {
        return Err("NFT is locked (OwnerLocked)".to_string());
      }
      Ok(AssetHolderKind::Account)
    }
    AssetHolder::Portfolio(pid) => {
      if pid.did != issuer_did {
        return Err(format!(
          "Held by another identity's portfolio (DID: {:?})",
          pid.did
        ));
      }
      let locked = api
        .query()
        .portfolio()
        .portfolio_locked_nft(pid.clone(), (asset_id, nft_id.clone()))
        .await
        .map_err(|e| format!("Query error: {}", e))?;
      if locked {
        return Err("NFT is locked in portfolio".to_string());
      }
      let kind = match &pid.kind {
        PortfolioKind::Default => AssetHolderKind::DefaultPortfolio,
        PortfolioKind::User(num) => AssetHolderKind::UserPortfolio(PortfolioNumber(num.0)),
      };
      Ok(kind)
    }
  }
}
