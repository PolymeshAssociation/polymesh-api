use codec::{Decode, Encode};

use polymesh_api::client::basic_types::{AccountId, AssetId, IdentityId};
use polymesh_api::client::error::Result;
use polymesh_api::client::BlockHash;
use polymesh_api::types::{
  polymesh_primitives::{
    asset::CheckpointId,
    checkpoint::ScheduleId,
    settlement::{InstructionId, LegId, VenueId},
    ticker::Ticker,
  },
  runtime::{events::*, RuntimeEvent},
};
use polymesh_api::{Api, ChainApi, TransactionResults};

mod user;
pub use user::*;

pub const ONE_POLYX: u128 = 1_000_000;

pub type Moment = u64;
pub type AuthorizationNonce = u64;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct TargetIdAuthorization {
  /// Target identity which is authorized to make an operation.
  pub target_id: IdentityId,
  /// It HAS TO be `target_id` authorization nonce: See `Identity::offchain_authorization_nonce`.
  pub nonce: AuthorizationNonce,
  /// Expire timestamp to limit how long the authorization is valid for.
  pub expires_at: Moment,
}

/// An offchain transaction receipt.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Receipt {
  /// Unique receipt number set by the signer for their receipts.
  #[cfg(feature = "polymesh_v7")]
  pub uid: u64,
  /// The [`InstructionId`] of the instruction for which the receipt is for.
  pub instruction_id: InstructionId,
  /// The [`LegId`] of the leg for which the receipt is for.
  pub leg_id: LegId,
  /// The [`IdentityId`] of the sender.
  pub sender_identity: IdentityId,
  /// The [`IdentityId`] of the receiver.
  pub receiver_identity: IdentityId,
  /// [`Ticker`] of the asset being transferred.
  pub ticker: Ticker,
  /// The amount transferred.
  pub amount: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreatedIds {
  AssetCreated(AssetId),
  IdentityCreated(IdentityId),
  ChildIdentityCreated(IdentityId),
  MultiSigCreated(AccountId),
  VenueCreated(VenueId),
  InstructionCreated(InstructionId),
  CheckpointCreated(CheckpointId),
  ScheduleCreated(ScheduleId),
}

/// Get ids from *Created events.
pub async fn get_created_ids(res: &mut TransactionResults) -> Result<Vec<CreatedIds>> {
  Ok(
    res
      .events()
      .await?
      .map(|events| {
        let mut ids = Vec::new();
        for rec in &events.0 {
          match &rec.event {
            RuntimeEvent::Asset(AssetEvent::AssetCreated(_, id, ..)) => {
              ids.push(CreatedIds::AssetCreated(*id));
            }
            RuntimeEvent::Settlement(SettlementEvent::VenueCreated(_, id, ..)) => {
              ids.push(CreatedIds::VenueCreated(*id));
            }
            RuntimeEvent::Settlement(SettlementEvent::InstructionCreated(_, _, id, ..)) => {
              ids.push(CreatedIds::InstructionCreated(*id));
            }
            RuntimeEvent::Checkpoint(CheckpointEvent::CheckpointCreated(_, _, id, ..)) => {
              ids.push(CreatedIds::CheckpointCreated(id.clone()));
            }
            RuntimeEvent::Checkpoint(CheckpointEvent::ScheduleCreated(_, _, id, ..)) => {
              ids.push(CreatedIds::ScheduleCreated(id.clone()));
            }
            RuntimeEvent::Identity(IdentityEvent::DidCreated(id, ..)) => {
              ids.push(CreatedIds::IdentityCreated(*id));
            }
            RuntimeEvent::Identity(IdentityEvent::ChildDidCreated(_, id, ..)) => {
              ids.push(CreatedIds::ChildIdentityCreated(*id));
            }
            RuntimeEvent::MultiSig(MultiSigEvent::MultiSigCreated { multisig, .. }) => {
              ids.push(CreatedIds::MultiSigCreated(*multisig));
            }
            _ => (),
          }
        }
        ids
      })
      .unwrap_or_default(),
  )
}

/// Search transaction events for newly created identity id.
pub async fn get_identity_id(res: &mut TransactionResults) -> Result<Option<IdentityId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Identity(IdentityEvent::DidCreated(id, ..)) => {
          return Some(*id);
        }
        RuntimeEvent::Identity(IdentityEvent::ChildDidCreated(_, id, ..)) => {
          return Some(*id);
        }
        _ => (),
      }
    }
    None
  }))
}

/// Search transaction events for VenueId.
pub async fn get_venue_id(res: &mut TransactionResults) -> Result<Option<VenueId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Settlement(SettlementEvent::VenueCreated(_, venue_id, ..)) => {
          return Some(*venue_id);
        }
        _ => (),
      }
    }
    None
  }))
}

/// Search transaction events for InstructionId.
pub async fn get_instruction_id(res: &mut TransactionResults) -> Result<Option<InstructionId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Settlement(SettlementEvent::InstructionCreated(_, _, instruction_id, ..)) => {
          return Some(*instruction_id);
        }
        _ => (),
      }
    }
    None
  }))
}

/// Search transaction events for AssetId.
pub async fn get_asset_id(res: &mut TransactionResults) -> Result<Option<AssetId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Asset(AssetEvent::AssetCreated(_, asset_id, ..)) => {
          return Some(*asset_id);
        }
        _ => (),
      }
    }
    None
  }))
}

/// Label for signing identity secondary key addition authorization.
pub const IDENTITY_ADD_SECONDARY_KEY_LABEL: &[u8] = b"Polymesh Identity Add Secondary Key";

/// Label for signing relay transactions.
pub const RELAY_TX_LABEL: &[u8] = b"Polymesh Relay Transaction";

/// Label for signing STO fundraiser receipts.
pub const STO_FUNDRAISER_RECEIPT_LABEL: &[u8] = b"Polymesh STO Fundraiser Receipt";

/// Label for signing settlement receipts.
pub const SETTLEMENT_RECEIPT_LABEL: &[u8] = b"Polymesh Settlement Receipt";

/// Chain scoped message for signing and verifying signatures. This is used to prevent signature replay attacks across different chains.
#[derive(Encode)]
pub struct ChainScopedMessage<M: Encode> {
  pub genesis_hash: BlockHash,
  pub nonce_or_id: u64,
  pub label: &'static [u8],
  pub expires_at: Moment,
  pub message: M,
}

impl<M: Encode> ChainScopedMessage<M> {
  /// Create a new [`ChainScopedMessage`] unchecked.
  pub async fn new(
    api: &Api,
    nonce_or_id: u64,
    label: &'static [u8],
    expires_at: Option<Moment>,
    message: M,
  ) -> Result<ChainScopedMessage<M>> {
    let genesis_hash = api.client().get_genesis_hash();
    let expires_at = if let Some(expires_at) = expires_at {
      expires_at
    } else {
      // Default to 1 hour from now.
      let now = api.query().timestamp().now().await?;
      now + 3600_000
    };
    Ok(ChainScopedMessage {
      genesis_hash,
      nonce_or_id,
      label,
      expires_at,
      message,
    })
  }
}
