use codec::{Decode, Encode};

use polymesh_api::client::basic_types::{AccountId, IdentityId};
use polymesh_api::client::error::Result;
use polymesh_api::types::{
  polymesh_common_utilities::traits::checkpoint::ScheduleId,
  polymesh_primitives::asset::CheckpointId,
  polymesh_primitives::settlement::{InstructionId, VenueId},
  runtime::{events::*, RuntimeEvent},
};
use polymesh_api::TransactionResults;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreatedIds {
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
            #[cfg(feature = "polymesh_v6")]
            RuntimeEvent::MultiSig(MultiSigEvent::MultiSigCreated(_, multisig, .. )) => {
              ids.push(CreatedIds::MultiSigCreated(*multisig));
            }
            #[cfg(feature = "polymesh_v7")]
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
