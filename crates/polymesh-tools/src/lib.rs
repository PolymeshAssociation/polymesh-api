//! Polymesh Tools - Helper utilities for Polymesh chain operations
//!
//! This crate provides various helper tools and utilities for working with Polymesh chains,
//! including chain upgrades, curve tree operations, and other administrative tasks.

use anyhow::{anyhow, Context, Result};
use polymesh_api::polymesh::types::{
  pallet_pips::types::{PipDescription, PipId},
  polymesh_primitives::Url,
  runtime::{events::PipsEvent, RuntimeEvent},
};
use polymesh_api_tester::PolymeshTester;
use polymesh_api_tester::{AccountId, AccountSigner, IdentityId, Signer};
use sp_weights::Weight;
use std::fs;

#[derive(Clone)]
struct CommitteeKey {
  source: String,
  account: AccountId,
  did: IdentityId,
  signer: AccountSigner,
}

fn required_votes(total_members: usize, threshold: (u32, u32)) -> Result<usize> {
  let (n, d) = threshold;
  if d == 0 {
    return Err(anyhow!(
      "Invalid committee vote threshold: denominator is zero"
    ));
  }

  let total = total_members as u128;
  let req = ((n as u128) * total).div_ceil(d as u128);
  usize::try_from(req).context("Required vote count does not fit in usize")
}

fn matching_key_indexes(keys: &[CommitteeKey], committee_members: &[IdentityId]) -> Vec<usize> {
  keys
    .iter()
    .enumerate()
    .filter_map(|(idx, k)| committee_members.contains(&k.did).then_some(idx))
    .collect()
}

fn bytes_label(input: &str) -> Vec<u8> {
  input.as_bytes().to_vec()
}

fn pip_id_from_events(events: &[polymesh_api::client::EventRecord<RuntimeEvent>]) -> Option<PipId> {
  for rec in events {
    if let RuntimeEvent::Pips(PipsEvent::ProposalCreated(_, _, pip_id, ..)) = &rec.event {
      return Some(pip_id.clone());
    }
  }
  None
}

/// Performs a chain upgrade with the provided WASM file
///
/// This function reads the specified WASM file and submits a sudo-wrapped set_code call
/// to upgrade the runtime.
///
/// # Arguments
///
/// * `wasm_path` - Path to the WASM file containing the new runtime code
///
/// # Returns
///
/// Returns `Ok(())` on successful upgrade, or an error if the operation fails
pub async fn upgrade_chain(wasm_path: &str) -> Result<()> {
  println!("Starting chain upgrade");
  println!("WASM file: {}", wasm_path);

  let code =
    fs::read(wasm_path).with_context(|| format!("Failed to read WASM file: {}", wasm_path))?;

  if code.is_empty() {
    return Err(anyhow!("WASM file is empty: {}", wasm_path));
  }

  println!("Loaded WASM file ({} bytes)", code.len());

  let tester = PolymeshTester::new()
    .await
    .context("Failed to initialize Polymesh API tester")?;

  let mut sudo = tester
    .sudo
    .clone()
    .ok_or_else(|| anyhow!("No sudo signer found for the connected chain"))?;

  println!("Building runtime upgrade call");
  let set_code = tester
    .api
    .call()
    .system()
    .set_code(code)
    .context("Failed to create system.set_code call")?;

  let weight = Weight::from_parts(1_000_000_000, 0);
  println!("Submitting sudo_unchecked_weight upgrade transaction");

  let mut res = tester
    .api
    .call()
    .sudo()
    .sudo_unchecked_weight(set_code.into(), weight)
    .context("Failed to create sudo_unchecked_weight call")?
    .submit_and_watch(&mut sudo)
    .await
    .context("Failed to submit runtime upgrade transaction")?;

  println!("Transaction submitted, waiting for finalization...");
  res
    .wait_finalized()
    .await
    .context("Runtime upgrade transaction was not finalized")?;

  println!("Chain upgrade finalized successfully");
  Ok(())
}

/// Performs a committee-driven chain upgrade using UpgradeCommittee and PolymeshCommittee.
///
/// Flow:
/// 1. Resolve provided signing keys to on-chain DIDs.
/// 2. Verify committee membership and vote-threshold sufficiency for both committees.
/// 3. Use UpgradeCommittee vote_or_propose to create a committee PIP that proposes set_code.
/// 4. Use PolymeshCommittee vote_or_propose to approve the committee PIP.
/// 5. Use the PolymeshCommittee release coordinator to reschedule execution to the next block.
pub async fn committee_upgrade(
  wasm_path: &str,
  version: &str,
  committee_keys: &[String],
) -> Result<()> {
  if committee_keys.is_empty() {
    return Err(anyhow!("At least one committee key must be provided"));
  }

  println!("Starting committee-based chain upgrade");
  println!("WASM file: {}", wasm_path);
  println!("Target version: {}", version);

  let code =
    fs::read(wasm_path).with_context(|| format!("Failed to read WASM file: {}", wasm_path))?;
  if code.is_empty() {
    return Err(anyhow!("WASM file is empty: {}", wasm_path));
  }
  println!("Loaded WASM file ({} bytes)", code.len());

  let tester = PolymeshTester::new()
    .await
    .context("Failed to initialize Polymesh API tester")?;

  println!("Resolving key-to-DID mappings...");
  let mut resolved_keys = Vec::with_capacity(committee_keys.len());
  for key in committee_keys {
    let signer = AccountSigner::from_string(key)
      .with_context(|| format!("Failed to parse committee key: {}", key))?;
    let account = signer.account();
    let did = tester
      .get_did(account)
      .await
      .with_context(|| format!("Failed to query DID for key: {}", key))?
      .ok_or_else(|| anyhow!("Key is not linked to an on-chain identity: {}", key))?;

    println!("  key: {} -> account: {:?}, did: {:?}", key, account, did);
    resolved_keys.push(CommitteeKey {
      source: key.clone(),
      account,
      did,
      signer,
    });
  }

  for i in 0..resolved_keys.len() {
    for j in (i + 1)..resolved_keys.len() {
      if resolved_keys[i].did == resolved_keys[j].did {
        return Err(anyhow!(
          "Duplicate DID detected across provided keys: {:?}. Provide at most one key per DID",
          resolved_keys[i].did
        ));
      }
    }
  }

  let upgrade_members = tester
    .api
    .query()
    .upgrade_committee()
    .members()
    .await
    .context("Failed to query UpgradeCommittee members")?;
  let upgrade_threshold = tester
    .api
    .query()
    .upgrade_committee()
    .vote_threshold()
    .await
    .context("Failed to query UpgradeCommittee vote threshold")?;

  let polymesh_members = tester
    .api
    .query()
    .polymesh_committee()
    .members()
    .await
    .context("Failed to query PolymeshCommittee members")?;
  let polymesh_threshold = tester
    .api
    .query()
    .polymesh_committee()
    .vote_threshold()
    .await
    .context("Failed to query PolymeshCommittee vote threshold")?;

  let required_upgrade_votes = required_votes(upgrade_members.len(), upgrade_threshold)?;
  let required_polymesh_votes = required_votes(polymesh_members.len(), polymesh_threshold)?;

  let upgrade_key_indexes = matching_key_indexes(&resolved_keys, &upgrade_members);
  let polymesh_key_indexes = matching_key_indexes(&resolved_keys, &polymesh_members);

  println!(
    "UpgradeCommittee: members={}, threshold={}/{}, required_yes_votes={}, matching_keys={}",
    upgrade_members.len(),
    upgrade_threshold.0,
    upgrade_threshold.1,
    required_upgrade_votes,
    upgrade_key_indexes.len()
  );
  println!(
    "PolymeshCommittee: members={}, threshold={}/{}, required_yes_votes={}, matching_keys={}",
    polymesh_members.len(),
    polymesh_threshold.0,
    polymesh_threshold.1,
    required_polymesh_votes,
    polymesh_key_indexes.len()
  );

  if upgrade_key_indexes.len() < required_upgrade_votes {
    return Err(anyhow!(
      "Not enough provided committee keys for UpgradeCommittee: have {}, need {}",
      upgrade_key_indexes.len(),
      required_upgrade_votes
    ));
  }
  if polymesh_key_indexes.len() < required_polymesh_votes {
    return Err(anyhow!(
      "Not enough provided committee keys for PolymeshCommittee: have {}, need {}",
      polymesh_key_indexes.len(),
      required_polymesh_votes
    ));
  }

  let release_coordinator_did = tester
    .api
    .query()
    .polymesh_committee()
    .release_coordinator()
    .await
    .context("Failed to query PolymeshCommittee release coordinator")?
    .ok_or_else(|| anyhow!("PolymeshCommittee release coordinator is not set"))?;

  let release_coordinator_idx = resolved_keys
    .iter()
    .position(|k| k.did == release_coordinator_did)
    .ok_or_else(|| {
      anyhow!(
        "Release coordinator DID {:?} is not in the provided committee keys",
        release_coordinator_did
      )
    })?;

  println!("Release coordinator DID: {:?}", release_coordinator_did);

  let url = Url(bytes_label(&format!(
    "https://github.com/PolymeshAssociation/Polymesh/releases/tag/v{}",
    version
  )));
  let description = PipDescription(bytes_label(&format!("Polymesh v{}", version)));

  println!("Step 1/3: proposing upgrade through UpgradeCommittee...");
  let mut pip_id: Option<PipId> = None;
  for (vote_idx, key_idx) in upgrade_key_indexes
    .iter()
    .take(required_upgrade_votes)
    .enumerate()
  {
    let key = &mut resolved_keys[*key_idx];
    println!(
      "  UpgradeCommittee vote {}/{} from DID {:?} (account {:?})",
      vote_idx + 1,
      required_upgrade_votes,
      key.did,
      key.account
    );

    let set_code = tester
      .api
      .call()
      .system()
      .set_code(code.clone())
      .context("Failed to build system.set_code call")?;
    let propose_pip = tester
      .api
      .call()
      .pips()
      .propose(
        set_code.into(),
        0,
        Some(url.clone()),
        Some(description.clone()),
      )
      .context("Failed to build pips.propose call")?;
    let committee_vote = tester
      .api
      .call()
      .upgrade_committee()
      .vote_or_propose(true, propose_pip.into())
      .context("Failed to build UpgradeCommittee.vote_or_propose call")?;

    let mut res = committee_vote
      .submit_and_watch(&mut key.signer)
      .await
      .with_context(|| {
        format!(
          "Failed to submit UpgradeCommittee vote from key {}",
          key.source
        )
      })?;

    if let Some(events) = res
      .events()
      .await
      .context("Failed to load transaction events")?
    {
      if let Some(id) = pip_id_from_events(&events.0) {
        pip_id = Some(id);
      }
    }

    res.wait_finalized().await.with_context(|| {
      format!(
        "UpgradeCommittee vote from key {} was not finalized",
        key.source
      )
    })?;
  }

  let pip_id = pip_id.ok_or_else(|| {
    anyhow!(
      "Upgrade committee proposal did not emit Pips.ProposalCreated; unable to determine PipId"
    )
  })?;
  println!("Created committee PIP id: {:?}", pip_id);

  println!("Step 2/3: approving committee PIP through PolymeshCommittee...");
  let mut scheduled_block: Option<u32> = None;
  for (vote_idx, key_idx) in polymesh_key_indexes
    .iter()
    .take(required_polymesh_votes)
    .enumerate()
  {
    let key = &mut resolved_keys[*key_idx];
    println!(
      "  PolymeshCommittee vote {}/{} from DID {:?} (account {:?})",
      vote_idx + 1,
      required_polymesh_votes,
      key.did,
      key.account
    );

    let approve_pip = tester
      .api
      .call()
      .pips()
      .approve_committee_proposal(pip_id.clone())
      .context("Failed to build pips.approve_committee_proposal call")?;
    let committee_vote = tester
      .api
      .call()
      .polymesh_committee()
      .vote_or_propose(true, approve_pip.into())
      .context("Failed to build PolymeshCommittee.vote_or_propose call")?;

    let mut res = committee_vote
      .submit_and_watch(&mut key.signer)
      .await
      .with_context(|| {
        format!(
          "Failed to submit PolymeshCommittee vote from key {}",
          key.source
        )
      })?;

    if let Some(events) = res
      .events()
      .await
      .context("Failed to load transaction events")?
    {
      for rec in &events.0 {
        if let RuntimeEvent::Pips(PipsEvent::ExecutionScheduled(_, scheduled_pip_id, block)) =
          &rec.event
        {
          if *scheduled_pip_id == pip_id {
            scheduled_block = Some(*block);
          }
        }
      }
    }

    res.wait_finalized().await.with_context(|| {
      format!(
        "PolymeshCommittee vote from key {} was not finalized",
        key.source
      )
    })?;
  }

  println!(
    "Committee PIP approved and scheduled{}",
    scheduled_block
      .map(|b| format!(" at block {}", b))
      .unwrap_or_default()
  );

  println!("Step 3/3: rescheduling execution to the next block as release coordinator...");
  let release_key = &mut resolved_keys[release_coordinator_idx];
  let reschedule = tester
    .api
    .call()
    .pips()
    .reschedule_execution(pip_id.clone(), None)
    .context("Failed to build pips.reschedule_execution call")?;

  let mut res = reschedule
    .submit_and_watch(&mut release_key.signer)
    .await
    .with_context(|| {
      format!(
        "Failed to submit pips.reschedule_execution as release coordinator key {}",
        release_key.source
      )
    })?;

  res
    .wait_finalized()
    .await
    .context("Reschedule execution transaction was not finalized")?;

  println!(
    "Committee upgrade flow completed successfully for PipId {:?}",
    pip_id
  );
  Ok(())
}
