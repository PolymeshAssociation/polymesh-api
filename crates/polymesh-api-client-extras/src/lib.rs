use codec::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use polymesh_api::TransactionResults;
use polymesh_api::client::basic_types::{AccountId, IdentityId};
use polymesh_api::client::error::Result;
use polymesh_api::types::{
  polymesh_primitives::settlement::{VenueId, InstructionId},
  polymesh_primitives::asset::CheckpointId,
  polymesh_common_utilities::traits::checkpoint::ScheduleId,
  runtime::{events::*, RuntimeEvent},
};

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
pub async fn get_created_ids(mut res: TransactionResults) -> Result<Vec<CreatedIds>> {
  Ok(res.events().await?.map(|events| {
    let mut ids = Vec::new();
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Settlement(SettlementEvent::VenueCreated(
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::VenueCreated(*id));
        }
        RuntimeEvent::Settlement(SettlementEvent::InstructionCreated(
          _,
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::InstructionCreated(*id));
        }
        RuntimeEvent::Checkpoint(CheckpointEvent::CheckpointCreated(
          _,
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::CheckpointCreated(id.clone()));
        }
        RuntimeEvent::Checkpoint(CheckpointEvent::ScheduleCreated(
          _,
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::ScheduleCreated(id.clone()));
        }
        RuntimeEvent::Identity(IdentityEvent::DidCreated(
          id,
          ..
        )) => {
          ids.push(CreatedIds::IdentityCreated(*id));
        }
        RuntimeEvent::Identity(IdentityEvent::ChildDidCreated(
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::ChildIdentityCreated(*id));
        }
        RuntimeEvent::MultiSig(MultiSigEvent::MultiSigCreated(
          _,
          id,
          ..
        )) => {
          ids.push(CreatedIds::MultiSigCreated(*id));
        }
        _ => (),
      }
    }
    ids
  }).unwrap_or_default())
}

/// Search transaction events for VenueId.
pub async fn get_venue_id(mut res: TransactionResults) -> Result<Option<VenueId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Settlement(SettlementEvent::VenueCreated(
          _,
          venue_id,
          ..
        )) => {
          return Some(*venue_id);
        }
        _ => (),
      }
    }
    None
  }))
}

/// Search transaction events for InstructionId.
pub async fn get_instruction_id(mut res: TransactionResults) -> Result<Option<InstructionId>> {
  Ok(res.events().await?.and_then(|events| {
    for rec in &events.0 {
      match &rec.event {
        RuntimeEvent::Settlement(SettlementEvent::InstructionCreated(
          _,
          _,
          instruction_id,
          ..
        )) => {
          return Some(*instruction_id);
        }
        _ => (),
      }
    }
    None
  }))
}
