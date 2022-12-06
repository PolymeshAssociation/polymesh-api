use codec::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use polymesh_api::client::basic_types::IdentityId;

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
